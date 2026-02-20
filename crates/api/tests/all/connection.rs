use ibkr_rust_api::{api_message::TwsApiMessage, contract::*, Result};

use super::helpers::*;

/// Verify basic TWS connection works.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn connect_and_disconnect() -> Result<()> {
    let client = connect_test_client(90).await?;
    // If we got here, connection succeeded and handshake passed
    assert_eq!(client.conn_state as i32, 3, "Connection should be READY");
    drop(client);
    Ok(())
}

/// Verify `request_current_time` returns a plausible server time.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn request_server_time() -> Result<()> {
    let mut client = connect_test_client(91).await?;
    client.request_current_time().await?;

    let msg = recv_timeout(&client.message_tracker, 5000).await;
    assert!(msg.is_some(), "Should receive a message within 5s");

    match msg.unwrap() {
        TwsApiMessage::ServerTime(dt) => {
            let now = chrono::Utc::now();
            let diff = (now - dt).num_seconds().abs();
            assert!(diff < 60, "Server time should be within 60s of local time, diff={}s", diff);
        },
        other => panic!("Expected ServerTime, got {:?}", other),
    }
    Ok(())
}

/// Verify `get_next_valid_order_id` returns a positive order ID.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn request_next_order_id() -> Result<()> {
    let mut client = connect_test_client(92).await?;
    let order_id = client.get_next_valid_order_id().await?;
    assert!(order_id > 0, "Next order id should be > 0, got {}", order_id);
    Ok(())
}

/// Verify contract details lookup by symbol.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn request_contract_details_by_symbol() -> Result<()> {
    let mut client = connect_test_client(93).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let contracts = client.subscribe_contract_details();
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    client.request_contract_details(1, contract).await?;

    let details = recv_timeout(&contracts, 10000).await;
    assert!(details.is_some(), "Should receive contract details within 10s");

    let resp = details.unwrap();
    assert!(resp.response.is_some(), "Contract details response should not be empty");
    let cd = resp.response.unwrap();
    assert_eq!(cd.contract.con_id, Some(4391), "AMD con_id should be 4391");
    Ok(())
}

/// Verify contract details lookup by con_id.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn request_contract_details_by_con_id() -> Result<()> {
    let mut client = connect_test_client(94).await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let contracts = client.subscribe_contract_details();
    let contract = Contract {
        con_id: Some(265598),
        ..Default::default()
    };
    client.request_contract_details(2, contract).await?;

    let details = recv_timeout(&contracts, 10000).await;
    assert!(details.is_some(), "Should receive contract details within 10s");

    let resp = details.unwrap();
    assert!(resp.response.is_some(), "Contract details response should not be empty");
    let cd = resp.response.unwrap();
    assert_eq!(cd.contract.symbol, "AAPL", "con_id 265598 should be AAPL");
    Ok(())
}
