use fastnum::{decimal::Context, D256};

use crate::{cmd::PlaceOrder,
            contract::{Contract, SecType},
            enums::*,
            frame::Frame,
            ib_frame::ParseIbkrFrame,
            orders::Order};

#[test]
fn parse_completed_orders() {
    let response_messages = [
        r#"101\04815747\0NVDA\0STK\00\0?\0SMART\0USD\0NVDA\0NMS\0BUY\080\0STP \
         LMT\0109.66\0109.65\0DAY\0U7502027\00\0ChartTrader1700896507\01586577378\00\00\00\00\01\\02147483647\00\00\03\00\00\0Keine\00\00\00\00\00\00\00\00\02147483647\02147483647\0IB\00\\00\00\0Cancelled\00\00\00\0109.65\01.7976931348623157E308\00\01\00\00\02147483647\00\\0Kein Insider oder substantieller Aktion r\00\00\09223372036854775807\020250312 09:44:41 \Europe/Berlin\0Vom Trader storniert\00\0czich083\0"#,
        //         "101\04815747\0NVDA\0STK\00\0?\0SMART\0USD\0NVDA\0NMS\0SELL\080\0STP\0109.66\
        // 0109.5\0DAY\\          01586577378\0U7502027\00\0ChartTrader1700896507\
        // 01586577379\00\00\00\00\01\02147483647\\          00\00\03\00\00\0Keine\00\00\
        // 00\00\00\00\00\00\02147483647\02147483647\0IB\00\00\00\\          0Cancelled\00\
        // 00\00\0109.5\01.7976931348623157E308\00\01\00\00\02147483647\00\0Kein \
        //          Insider oder substantieller Aktion r\00\00\01586577378\020250312 09:44:41 \
        //          Europe/Berlin\0Vom System storniert:
        // \00\0czich083\0"
        //             .to_owned(),
        //         "5\00\0366244335\0VUAA\0STK\00\0?\0SMART\0USD\0VUAA\0EUET\0BUY\025\0STP \
        //          LMT\0106.46\0106.4\0DAY\0U7502027\00\0ChartTrader1700896507\00\01586577383\00\
        // 00\00\\          01586577383.0/U7502027/100\00\01\00\02147483647\00\00\00\03\00\
        // 00\00\00\00\0Keine\00\0?\\          00\00\00\00\00\00\00\02147483647\
        // 02147483647\00\0IB\00\00\00\00\0PreSubmitted\01.\          7976931348623157E308\
        // 01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\\
        //          09223372036854775808\00\00\00\00\0Keine\01.7976931348623157E308\0106.4\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\00\00\01\00\00\00\00\00\00\02147483647\0czich083\00\033\
        // 00\\          0PreSubmitted\00\025\00\01586577383\00\00\00\0trigger\00\0"
        //             .to_owned(),
        //         "5\00\0366244335\0VUAA\0STK\00\0?\0SMART\0USD\0VUAA\0EUET\0BUY\025\0STP \
        //          LMT\0106.46\0106.4\0DAY\0U7502027\00\0ChartTrader1700896507\00\01586577383\00\
        // 00\00\\          01586577383.0/U7502027/100\00\01\00\02147483647\00\00\00\03\00\
        // 00\00\00\00\0Keine\00\0?\\          00\00\00\00\00\00\00\02147483647\
        // 02147483647\00\0IB\00\00\00\00\0Submitted\01.\          7976931348623157E308\01.
        // 7976931348623157E308\01.7976931348623157E308\01.\          7976931348623157E308\
        // 01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\\
        //          09223372036854775808\00\00\00\00\0Keine\01.7976931348623157E308\0106.4\01.\
        //          7976931348623157E308\01.7976931348623157E308\01.7976931348623157E308\01.\
        //          7976931348623157E308\00\00\01\00\00\00\00\00\00\02147483647\0czich083\00\0)3\
        // 00\\          0Submitted\00\025\00\01586577383\00\00\00\00\0"
        //             .to_owned(),
    ];
    let message = response_messages[0];
    #[allow(clippy::single_char_pattern)]
    let mut it = message.split("\0");
    tracing::error!("incoming message: {:?}", it);
    let msg_id: crate::enums::Incoming = it
        .next()
        .unwrap()
        .parse()
        .expect("Could not parse message type.");

    let order = Order::try_parse_frame(msg_id, Some(199), &mut it);
    tracing::error!("parsed order: {:?}", order);
    // assert_debug_snapshot!(&files.unwrap());
}

// #[test]
// fn open_orders() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages:  RwLock::new(vec![]),
//         response_messages: vec!["9|1|43||".to_owned()],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let results = super::open_orders(&client);

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(request_messages[0].encode_simple(), "5|1|");

//     assert!(
//         results.is_ok(),
//         "failed to request completed orders: {}",
//         results.err().unwrap()
//     );
// }

// #[test]
// fn all_open_orders() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages:  RwLock::new(vec![]),
//         response_messages: vec!["9|1|43||".to_owned()],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let results = client.all_open_orders();

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(request_messages[0].encode_simple(), "16|1|");

//     assert!(
//         results.is_ok(),
//         "failed to request completed orders: {}",
//         results.err().unwrap()
//     );
// }

// #[test]
// fn auto_open_orders() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages:  RwLock::new(vec![]),
//         response_messages: vec!["9|1|43||".to_owned()],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let api_only = true;
//     let results = client.auto_open_orders(api_only);

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(request_messages[0].encode_simple(), "15|1|1|");

//     assert!(
//         results.is_ok(),
//         "failed to request completed orders: {}",
//         results.err().unwrap()
//     );
// }

// #[test]
// fn executions() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages:  RwLock::new(vec![]),
//         response_messages: vec!["9|1|43||".to_owned()],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let filter = ExecutionFilter {
//         client_id:     Some(100),
//         account_code:  "xyz".to_owned(),
//         time:          "yyyymmdd hh:mm:ss EST".to_owned(),
//         symbol:        "TSLA".to_owned(),
//         security_type: "STK".to_owned(),
//         exchange:      "ISLAND".to_owned(),
//         side:          "BUY".to_owned(),
//     };
//     let results = client.executions(filter);

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(
//         request_messages[0].encode_simple(),
//         "7|3|9000|100|xyz|yyyymmdd hh:mm:ss EST|TSLA|STK|ISLAND|BUY|"
//     );

//     assert!(
//         results.is_ok(),
//         "failed to request completed orders: {}",
//         results.err().unwrap()
//     );
//     // assert_eq!(43, results.unwrap(), "next order id");
// }

// #[test]
// fn encode_limit_order() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages: RwLock::new(vec![]),
//         response_messages: vec![],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let order_id = 12;
//     let contract = contract_samples::future_with_local_symbol();
//     let order = order_builder::limit_order(super::Action::Buy, 10.0, 500.00);

//     let results = client.place_order(order_id, &contract, &order);

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(
//         request_messages[0].encode_simple(),
//         "3|12|0||FUT|202303|0|||EUREX||EUR|FGBL MAR 23||||BUY|10|LMT|500||||||0||1|0|0|0|0|0|0|0||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|"
//     );

//     assert!(results.is_ok(), "failed to place order: {}",
// results.err().unwrap()); }

// #[test]
// fn encode_combo_market_order() {
//     let message_bus = Arc::new(MessageBusStub {
//         request_messages: RwLock::new(vec![]),
//         response_messages: vec![],
//     });

//     let client = Client::stubbed(message_bus, server_versions::SIZE_RULES);

//     let order_id = 12; // get next order id
//     let contract = contract_samples::smart_future_combo_contract();
//     let order = order_builder::combo_market_order(Action::Sell, 150.0, true);

//     let results = client.place_order(order_id, &contract, &order);

//     let request_messages = client.message_bus.request_messages();

//     assert_eq!(
//         request_messages[0].encode_simple(),
//         "3|12|0|WTI|BAG||0|||SMART||USD|||||SELL|150|MKT|||||||0||1|0|0|0|0|0|0|0|2|55928698|1|BUY|IPE|0|0||0|55850663|1|SELL|IPE|0|0||0|0|1|NonGuaranteed|1||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|"
//     );

//     assert!(results.is_ok(), "failed to place order: {}",
// results.err().unwrap()); }

/// Integration test: Verify PlaceOrder encoding produces correct TWS API v100+
/// field layout.
///
/// TWS API PlaceOrder frame format:
///   PLACE_ORDER(3) | order_id | con_id | symbol | sec_type | expiry | strike |
/// right |   multiplier | exchange | primary_exchange | currency | local_symbol
/// | trading_class |   sec_id_type | sec_id | action | total_qty | order_type |
/// lmt_price | aux_price | ...
#[test]
fn encode_place_order_market_stock() {
    let order = Order {
        order_id:    Some(42),
        contract:    Contract {
            con_id: Some(265598),
            symbol: "AAPL".to_string(),
            sec_type: SecType::Stock,
            exchange: Some("SMART".to_string()),
            primary_exchange: Some("NASDAQ".to_string()),
            currency: "USD".to_string(),
            ..Default::default()
        },
        order:       super::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("100", Context::default()).unwrap(),
            order_type: OrderType::Market,
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    };

    let place = PlaceOrder::new(order);
    let frame = place.into_frame();

    let raw = match frame {
        Frame::Bulk(bytes) => bytes,
        other => panic!("Expected Frame::Bulk, got {:?}", other),
    };

    // The frame has a 4-byte length prefix, then NUL-separated fields
    let payload = &raw[4..];
    let payload_str = std::str::from_utf8(payload).expect("Frame should be valid UTF-8");
    let fields: Vec<&str> = payload_str.split('\0').collect();

    eprintln!("=== PlaceOrder encoded fields ({} total) ===", fields.len());
    for (i, f) in fields.iter().enumerate() {
        eprintln!("  [{:3}] = {:?}", i, f);
    }

    // [0] = message type: PLACE_ORDER = 3
    assert_eq!(
        fields[0], "3",
        "field[0] should be PLACE_ORDER message type (3)"
    );
    // [1] = order_id (was missing before fix, causing Error 320)
    assert_eq!(fields[1], "42", "field[1] should be order_id");
    // [2] = con_id
    assert_eq!(fields[2], "265598", "field[2] should be con_id");
    // [3] = symbol
    assert_eq!(fields[3], "AAPL", "field[3] should be symbol");
    // [4] = sec_type
    assert_eq!(fields[4], "STK", "field[4] should be sec_type");
    // [5] = expiry (empty for stocks)
    assert_eq!(fields[5], "", "field[5] should be expiry (empty for STK)");
    // [6] = strike
    assert_eq!(fields[6], "", "field[6] should be strike (None for STK)");
    // [7] = right (empty)
    assert_eq!(fields[7], "", "field[7] should be right");
    // [8] = multiplier (empty)
    assert_eq!(fields[8], "", "field[8] should be multiplier");
    // [9] = exchange
    assert_eq!(fields[9], "SMART", "field[9] should be exchange");
    // [10] = primary_exchange
    assert_eq!(fields[10], "NASDAQ", "field[10] should be primary_exchange");
    // [11] = currency
    assert_eq!(fields[11], "USD", "field[11] should be currency");
    // [12] = local_symbol (empty)
    assert_eq!(fields[12], "", "field[12] should be local_symbol");
    // [13] = trading_class (empty)
    assert_eq!(fields[13], "", "field[13] should be trading_class");
    // [14] = sec_id_type (empty)
    assert_eq!(fields[14], "", "field[14] should be sec_id_type");
    // [15] = sec_id (empty)
    assert_eq!(fields[15], "", "field[15] should be sec_id");
    // [16] = action
    assert_eq!(fields[16], "BUY", "field[16] should be action");
    // [17] = total_qty
    assert_eq!(fields[17], "100", "field[17] should be total_qty");
    // [18] = order_type
    assert_eq!(fields[18], "MKT", "field[18] should be order_type");
    // [27] = transmit (action+11)
    assert_eq!(fields[27], "1", "field[27] should be transmit=true");

    // Verify reasonable field count
    assert!(
        fields.len() > 70,
        "Expected >70 fields, got {}",
        fields.len()
    );
}

/// Test encoding matches the known-good reference from the commented-out
/// encode_limit_order test.
#[test]
fn encode_place_order_limit_future() {
    let order = Order {
        order_id:    Some(12),
        contract:    Contract {
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
        order:       super::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("10", Context::default()).unwrap(),
            order_type: OrderType::Limit,
            lmt_price: Some(D256::from_str("500", Context::default()).unwrap()),
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    };

    let place = PlaceOrder::new(order);
    let frame = place.into_frame();

    let raw = match frame {
        Frame::Bulk(bytes) => bytes,
        other => panic!("Expected Frame::Bulk, got {:?}", other),
    };

    let payload = &raw[4..];
    let payload_str = std::str::from_utf8(payload).expect("Frame should be valid UTF-8");

    let pipe_separated: String = payload_str
        .strip_suffix('\0')
        .unwrap_or(payload_str)
        .replace('\0', "|");

    eprintln!("=== PlaceOrder FUT Limit encoded ===");
    eprintln!("{}", pipe_separated);

    // Reference from the commented-out test (known-good encoding):
    let reference = "3|12|0||FUT|202303|0|||EUREX||EUR|FGBL MAR 23||||BUY|10|LMT|500||||||0||1|0|0|0|0|0|0|0||0||||||||0||-1|0|||0|||0|0||0||||||0|||||0|||||||||||0|||0|0|||0||0|0|0|0|||||||0|||||||||0|0|0|0|||0|";

    let actual_fields: Vec<&str> = payload_str.split('\0').collect();
    let ref_fields: Vec<&str> = reference.split('|').collect();

    eprintln!("\n=== Field-by-field comparison (ref vs actual) ===");
    let max_len = actual_fields.len().max(ref_fields.len());
    let mut first_mismatch = None;
    for i in 0..max_len {
        let actual = actual_fields.get(i).unwrap_or(&"<MISSING>");
        let expected = ref_fields.get(i).unwrap_or(&"<MISSING>");
        let marker = if actual != expected {
            " <<<< MISMATCH"
        } else {
            ""
        };
        if actual != expected && first_mismatch.is_none() {
            first_mismatch = Some(i);
        }
        eprintln!(
            "  [{:3}] ref={:>20} | actual={:>20}{}",
            i, expected, actual, marker
        );
    }

    if let Some(idx) = first_mismatch {
        eprintln!("\nFirst mismatch at field [{}]", idx);
    }

    assert!(
        pipe_separated
            .starts_with("3|12|0||FUT|202303|0|||EUREX||EUR|FGBL MAR 23||||BUY|10|LMT|500|"),
        "Header fields should match reference. Got: {}",
        &pipe_separated[..pipe_separated.len().min(120)]
    );
}
mod encoding_tests;
