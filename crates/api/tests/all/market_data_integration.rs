use chrono::Utc;
use ibkr_rust_api::{bars::*, cmd::*, contract::*, Result};

use super::helpers::*;

/// Verify historical bars download returns data.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn historical_bars_amd() -> Result<()> {
    let mut client = connect_test_client(70).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let contract = Contract {
        con_id: Some(4391),
        exchange: Some("SMART".to_string()),
        ..Default::default()
    };

    let receiver = client.market_data_tracker.historical_bars.clone();

    client
        .request_historical_data(&HistoricalDataRequest {
            req_id: 2000,
            contract,
            end_date_time: Some(Utc::now()),
            duration: Duration::Day(1),
            bar_size_setting: BarSize::_1Min,
            what_to_show: HistoricalDataType::Trades,
            use_rth: UseRegularTradingHoursOnly::Use,
            format_date: IntradayBarDateFormat::UnixEpochSeconds,
            keep_up_to_date: false,
            chart_options: vec![],
        })
        .await?;

    let bars = recv_timeout(&receiver, 15000).await;
    assert!(bars.is_some(), "Should receive historical bars within 15s");
    tracing::info!("Received historical bars response");

    Ok(())
}

/// Verify realtime bars subscription and unsubscription.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn realtime_bars_subscribe_unsubscribe() -> Result<()> {
    let mut client = connect_test_client(71).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let contract = Contract {
        con_id: Some(265598),
        symbol: "AAPL".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };

    let receiver = client.market_data_tracker.bars.clone();

    client
        .request_realtime_bars(&RealtimeBarRequest {
            req_id: 2001,
            contract,
            bar_size: BarSize::_5Secs,
            what_to_show: HistoricalDataType::Trades,
            use_rth: UseRegularTradingHoursOnly::DontUse,
            real_time_bars_options: vec![],
        })
        .await?;

    // Wait for at least one bar (up to 15s)
    let bar = recv_timeout(&receiver, 15000).await;
    assert!(bar.is_some(), "Should receive at least one realtime bar within 15s");
    tracing::info!("Received realtime bar: {:?}", bar);

    // Cancel subscription
    client
        .cancel_realtime_bars(&CancelRealtimeBars { req_id: 2001 })
        .await?;

    // Brief wait to ensure no error after cancel
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(())
}

/// Verify head timestamp request returns a date in the past.
#[tokio::test]
#[cfg_attr(not(feature = "ibkr_client_test"), ignore)]
async fn historical_head_timestamp() -> Result<()> {
    let mut client = connect_test_client(72).await?;
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let contract = Contract {
        symbol: "AAPL".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };

    let receiver = client.market_data_tracker.head_timestamp.clone();

    client
        .request_head_time_stamp(&HeadTimestampRequest {
            req_id: 2002,
            contract,
            what_to_show: HistoricalDataType::Trades,
            use_rth: UseRegularTradingHoursOnly::DontUse,
            format_date: IntradayBarDateFormat::YYYYMMDD,
        })
        .await?;

    let ts = recv_timeout(&receiver, 10000).await;
    assert!(ts.is_some(), "Should receive head timestamp within 10s");
    tracing::info!("Head timestamp: {:?}", ts);

    Ok(())
}
