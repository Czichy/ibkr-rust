use fastnum::{decimal::Context, D256};
use ibkr_rust_api::{api_message::TwsApiMessage,
                    contract::*,
                    enums::*,
                    orders::Order,
                    Result};

use super::helpers::*;

/// Helper: build a far-from-market limit buy order (price $1.00).
/// This order will be accepted by TWS but never fill.
fn far_limit_order(order_id: i32) -> Order {
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
        order: ibkr_rust_api::orders::order::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("1", Context::default()).unwrap(),
            order_type: OrderType::Limit,
            lmt_price: Some(D256::from_str("1.00", Context::default()).unwrap()),
            tif: Some(TimeInForce::Day),
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    }
}

/// Place a far-from-market limit order, verify it gets accepted (no Error 320),
/// and that we receive an order status update.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn place_limit_order_far_from_market() -> Result<()> {
    let mut client = connect_test_client(80).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let order_id = client.get_next_valid_order_id().await?;
    let order = far_limit_order(order_id);

    let status_rx = client.order_tracker.order_status.clone();
    let message_rx = client.subscribe_message_updates();

    client.place_order(order).await?;

    // Wait for order status
    let status = recv_timeout(&status_rx, 10000).await;
    assert!(
        status.is_some(),
        "Should receive order status within 10s after placing order"
    );
    let status = status.unwrap();
    tracing::info!("Order status: {:?}", status);

    // Check no Error 320 was received
    // Drain the message channel briefly
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    while let Ok(msg) = message_rx.try_recv() {
        if let TwsApiMessage::TwsError { code, message, .. } = &msg {
            assert_ne!(
                *code, 320,
                "Error 320 should not occur! Message: {:?}",
                message
            );
        }
    }

    Ok(())
}

/// Regression test: place an order and explicitly verify no Error 320.
/// This is the exact error that the `Option<bool>` encoding bug caused.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn place_order_no_error_320() -> Result<()> {
    let mut client = connect_test_client(81).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let order_id = client.get_next_valid_order_id().await?;
    let order = far_limit_order(order_id);

    let message_rx = client.subscribe_message_updates();

    client.place_order(order).await?;

    // Wait and collect all messages for up to 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let mut errors = Vec::new();
    while let Ok(msg) = message_rx.try_recv() {
        if let TwsApiMessage::TwsError { code, message, .. } = msg {
            errors.push((code, message));
        }
    }

    // No Error 320
    for (code, msg) in &errors {
        assert_ne!(
            *code, 320,
            "Error 320 (misc_options parsing) should not occur! Got: {:?}",
            msg
        );
    }

    Ok(())
}

/// Place a market order for 1 share and verify we receive execution details.
/// WARNING: This test actually executes a trade on the paper account!
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn place_market_order_and_receive_execution() -> Result<()> {
    let mut client = connect_test_client(82).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let order_id = client.get_next_valid_order_id().await?;
    let order = Order {
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
        order: ibkr_rust_api::orders::order::OrderData {
            action: Action::Buy,
            total_qty: D256::from_str("1", Context::default()).unwrap(),
            order_type: OrderType::Market,
            transmit: true,
            ..Default::default()
        },
        order_state: Default::default(),
    };

    let status_rx = client.order_tracker.order_status.clone();
    let exec_rx = client.order_tracker.executions.clone();

    client.place_order(order).await?;

    // Wait for order status (Filled)
    let status = recv_timeout(&status_rx, 30000).await;
    assert!(
        status.is_some(),
        "Should receive order status within 30s after market order"
    );

    // Wait for execution report
    let exec = recv_timeout(&exec_rx, 30000).await;
    assert!(
        exec.is_some(),
        "Should receive execution report within 30s after market order"
    );
    let exec = exec.unwrap();
    tracing::info!("Execution: {:?}", exec);
    assert_eq!(exec.contract.symbol, "AAPL", "Execution should be for AAPL");

    Ok(())
}
