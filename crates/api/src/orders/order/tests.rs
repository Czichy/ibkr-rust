use crate::{ib_frame::ParseIbkrFrame, orders::Order};

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
