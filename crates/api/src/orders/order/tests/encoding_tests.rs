use fastnum::{decimal::Context, D256};

use crate::{cmd::PlaceOrder,
            contract::{Contract, SecType},
            enums::*,
            frame::Frame,
            orders::Order,
            utils::ib_message::Encodable};

/// Helper: encode an Order into PlaceOrder frame and return NUL-separated fields.
fn encode_to_fields(order: &Order) -> Vec<String> {
    let place = PlaceOrder::new(order.clone());
    let frame = place.into_frame();
    let raw = match frame {
        Frame::Bulk(bytes) => bytes,
        other => panic!("Expected Frame::Bulk, got {:?}", other),
    };
    // Skip 4-byte length prefix
    let payload = &raw[4..];
    let payload_str = std::str::from_utf8(payload).expect("Frame should be valid UTF-8");
    payload_str.split('\0').map(String::from).collect()
}

/// Helper: build a simple AAPL market order for testing.
fn aapl_market_order(order_id: i32) -> Order {
    Order {
        order_id: Some(order_id),
        contract: Contract {
            con_id: Some(265598),
            symbol: "AAPL".to_string(),
            sec_type: SecType::Stock,
            exchange: Some("SMART".to_string()),
            primary_exchange: Some("NASDAQ".to_string()),
            currency: "USD".to_string(),
            ..Default::default()
        },
        order: super::super::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("100", Context::default()).unwrap(),
            order_type: OrderType::Market,
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    }
}

/// Verify that a default market order produces a reasonable number of fields
/// and that the header fields are in correct positions.
#[test]
fn encode_market_order_field_layout() {
    let order = aapl_market_order(42);
    let fields = encode_to_fields(&order);

    // [0] = PLACE_ORDER message type (3)
    assert_eq!(fields[0], "3", "field[0] should be PLACE_ORDER (3)");
    // [1] = order_id
    assert_eq!(fields[1], "42", "field[1] should be order_id");
    // [2] = con_id
    assert_eq!(fields[2], "265598", "field[2] should be con_id");
    // [3] = symbol
    assert_eq!(fields[3], "AAPL", "field[3] should be symbol");
    // [4] = sec_type
    assert_eq!(fields[4], "STK", "field[4] should be sec_type");
    // [16] = action
    assert_eq!(fields[16], "BUY", "field[16] should be action");
    // [17] = total_qty
    assert_eq!(fields[17], "100", "field[17] should be total_qty");
    // [18] = order_type
    assert_eq!(fields[18], "MKT", "field[18] should be order_type");

    // Reasonable field count (PlaceOrder has 70+ fields)
    assert!(
        fields.len() > 70,
        "Expected >70 fields, got {}",
        fields.len()
    );
}

/// Regression test for Error 320: all bool fields that were previously
/// `Option<bool>` must encode as exactly "0\0" (2 bytes for false) or
/// "1\0" (2 bytes for true), never as "\0" alone (1 byte).
/// A 1-byte encoding shifts all subsequent fields, causing TWS to
/// misparse `order_misc_options`.
#[test]
fn encode_bool_fields_not_single_null() {
    let order = aapl_market_order(1);

    // Verify bool fields via the Encodable trait directly
    assert_eq!(false.encode(), "0\0", "bool false should encode as '0\\0'");
    assert_eq!(true.encode(), "1\0", "bool true should encode as '1\\0'");

    // Verify the specific OrderData bool fields that were formerly Option<bool>
    let od = &order.order;
    assert_eq!(od.block_order.encode(), "0\0", "block_order default should be '0\\0'");
    assert_eq!(od.e_trade_only.encode(), "0\0", "e_trade_only default should be '0\\0'");
    assert_eq!(od.firm_quote_only.encode(), "0\0", "firm_quote_only default should be '0\\0'");
    assert_eq!(
        od.opt_out_smart_routing.encode(),
        "0\0",
        "opt_out_smart_routing default should be '0\\0'"
    );
    assert_eq!(od.what_if.encode(), "0\0", "what_if default should be '0\\0'");
}

/// Verify that `what_if = true` encodes correctly and does not shift
/// subsequent fields.
#[test]
fn encode_what_if_true() {
    let mut order = aapl_market_order(1);
    order.order.what_if = true;

    let fields = encode_to_fields(&order);

    // Find the what_if field by looking for the sequence:
    // "1" (what_if=true), "" (misc_options=empty), "0" (solicited),
    // "0" (randomize_size), "0" (randomize_price)
    let mut found = false;
    for i in 0..fields.len().saturating_sub(4) {
        if fields[i] == "1"
            && fields[i + 1].is_empty() // order_misc_options (None -> "")
            && fields[i + 2] == "0" // solicited
            && fields[i + 3] == "0" // randomize_size
            && fields[i + 4] == "0" // randomize_price
        {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "Could not find what_if=true followed by misc_options, solicited, randomize_size, randomize_price"
    );
}

/// Verify that `what_if = false` (default) produces the correct
/// sequence of fields — this is the common case that Error 320 broke.
#[test]
fn encode_what_if_false_field_sequence() {
    let order = aapl_market_order(1);
    let fields = encode_to_fields(&order);

    // With what_if=false and all defaults, we expect the sequence:
    // algo_id="", what_if="0", misc_options="", solicited="0",
    // randomize_size="0", randomize_price="0"
    let mut found = false;
    for i in 0..fields.len().saturating_sub(5) {
        if fields[i].is_empty()  // algo_id
            && fields[i + 1] == "0" // what_if
            && fields[i + 2].is_empty() // order_misc_options
            && fields[i + 3] == "0" // solicited
            && fields[i + 4] == "0" // randomize_size
            && fields[i + 5] == "0" // randomize_price
        {
            found = true;
            break;
        }
    }
    assert!(
        found,
        "Could not find algo_id->what_if->misc_options->solicited->randomize_size->randomize_price sequence"
    );
}

/// Verify that `order_misc_options` with actual key=value pairs encodes correctly.
#[test]
fn encode_order_misc_options_with_values() {
    let mut order = aapl_market_order(1);
    order.order.order_misc_options =
        Some(vec![("key1".to_string(), "val1".to_string()), ("key2".to_string(), "val2".to_string())]);

    let fields = encode_to_fields(&order);

    let misc_opts_found = fields.iter().any(|f| f == "key1=val1;key2=val2;");
    assert!(
        misc_opts_found,
        "order_misc_options should encode as 'key1=val1;key2=val2;', fields: {:?}",
        fields
    );
}

/// Verify the limit order encoding matches the known-good reference.
#[test]
fn encode_limit_order_reference_header() {
    let order = Order {
        order_id: Some(12),
        contract: Contract {
            con_id: Some(0),
            symbol: String::new(),
            sec_type: SecType::Future,
            last_trade_date_or_contract_month: Some("202303".to_string()),
            strike: Some(D256::from_str("0", Context::default()).unwrap()),
            right: None,
            multiplier: None,
            exchange: Some("EUREX".to_string()),
            primary_exchange: None,
            currency: "EUR".to_string(),
            local_symbol: Some("FGBL MAR 23".to_string()),
            trading_class: None,
            ..Default::default()
        },
        order: super::super::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("10", Context::default()).unwrap(),
            order_type: OrderType::Limit,
            lmt_price: Some(D256::from_str("500", Context::default()).unwrap()),
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    };

    let fields = encode_to_fields(&order);
    let pipe_separated: String = fields
        .iter()
        .take(fields.len().saturating_sub(1))
        .cloned()
        .collect::<Vec<_>>()
        .join("|");

    assert!(
        pipe_separated
            .starts_with("3|12|0||FUT|202303|0|||EUREX||EUR|FGBL MAR 23||||BUY|10|LMT|500|"),
        "Header fields should match reference. Got: {}",
        &pipe_separated[..pipe_separated.len().min(120)]
    );
}

/// Verify the field count is stable — detects accidental field additions/removals.
#[test]
fn encode_field_count_stable() {
    let order = aapl_market_order(1);
    let fields = encode_to_fields(&order);
    let count = fields.len();

    // Current encoding produces ~85-95 fields. Detect large unexpected changes.
    assert!(
        (80..=100).contains(&count),
        "Expected 80-100 fields, got {}. This may indicate an encoding change.",
        count
    );
}
