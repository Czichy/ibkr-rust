// use futures_util::{pin_mut, stream::StreamExt};
use std::{net::{IpAddr, Ipv4Addr, SocketAddr},
          thread};

use chrono::Utc;
use ibkr_rust_api::{client,
                    prelude::{Contract, *}};
fn get_client_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 62)), 1111)
}
#[tokio::test]
async fn market_data_realtime_bars() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "EUR".to_string(),
        exchange: Some("IDEALPRO".to_string()),
        sec_type: SecType::Forex,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.bars.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(tick) = receiver.recv() {
            tracing::error!("received: {:?}", tick);
            count += 1;
            if count > 3 {}
        }
        assert!(count > 0);
    });
    let _orders = client
        .request_realtime_bars(&RealtimeBarRequest {
            req_id: 1000,
            contract,
            bar_size: BarSize::_1Min,
            what_to_show: HistoricalDataType::BidAsk,
            use_rth: UseRegularTradingHoursOnly::DontUse,
            real_time_bars_options: vec![],
        })
        .await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    let _ = client
        .cancel_realtime_bars(&CancelRealtimeBars { req_id: 1000 })
        .await?;
    Ok(())
}

#[tokio::test]
async fn market_data_market_data() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "EUR".to_string(),
        exchange: Some("IDEALPRO".to_string()),
        sec_type: SecType::Forex,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.tick_by_tick.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(tick) = receiver.recv() {
            tracing::error!("received: {:?}", tick);
            count += 1;
            if count > 3 {}
        }
        assert!(count > 0);
    });
    let _orders = client
        .request_market_data(&MarketDataRequest {
            req_id: 1000,
            contract,
            generic_tick_list: vec![],
            snapshot: false,
            regulatory: false,
            additional_data: vec![],
        }) //           1000, contract, false, false, vec![])
        .await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    let _ = client
        .cancel_market_data(&CancelMarketDataRequest { req_id: 1000 })
        .await?;
    Ok(())
}

#[tokio::test]
async fn market_data_historical_data() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        con_id: Some(76792991),
        // symbol: "TSLA".to_string(),
        exchange: Some("SMART".to_string()),
        // sec_type: SecType::Stock,
        // currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.historical_bars.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(_fill) = receiver.recv() {
            tracing::error!("received: {:?}", _fill);
            count += 1;
            if count > 3 {}
        }
        tracing::error!("received: {:?}", count);
        assert!(count > 0);
    });
    let _ = client
        .request_historical_data(&HistoricalDataRequest {
            req_id: 1010,
            contract,
            end_date_time: Utc::now(),
            duration: Duration::Seconds(1800),
            bar_size_setting: BarSize::_1Secs,
            what_to_show: HistoricalDataType::Trades,
            use_rth: UseRegularTradingHoursOnly::Use,
            format_date: IntradayBarDateFormat::UnixEpochSeconds,
            keep_up_to_date: false,
            chart_options: vec![],
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Ok(())
}

#[tokio::test]
async fn market_data_historical_schedule() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("ISLAND".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.historical_schedule.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(_fill) = receiver.recv() {
            count += 1;
            tracing::error!("received: {:#?}", _fill);
            if count > 3 {}
        }
        tracing::error!("received: {:?}", count);
        assert!(count > 0);
    });
    let _ = client
        .request_historical_data(&HistoricalDataRequest {
            req_id: 1010,
            contract,
            end_date_time: Utc::now(),
            duration: Duration::Year(20),
            bar_size_setting: BarSize::_1Day,
            what_to_show: HistoricalDataType::Schedule,
            use_rth: UseRegularTradingHoursOnly::Use,
            format_date: IntradayBarDateFormat::UnixEpochSeconds,
            keep_up_to_date: false,
            chart_options: vec![],
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Ok(())
}

#[tokio::test]
async fn market_data_historical_tick() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.historical_ticks.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(_tick) = receiver.recv() {
            count += 1;
        }
        assert!(count > 0);
    });
    let _ = client
        .request_historical_ticks(&HistoricalTicksRequest {
            req_id:          1020,
            contract:        contract.clone(),
            date_time:       HistoricalTickDateTime::End(Utc::now()),
            number_of_ticks: 10,
            what_to_show:    HistoricalDataType::Trades,
            use_rth:         UseRegularTradingHoursOnly::Use,
            ignore_size:     1,
            misc_options:    Vec::new(),
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(())
}
#[tokio::test]
async fn market_data_historical_bars() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.historical_bars.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(_tick) = receiver.recv() {
            count += 1;
        }
        assert!(count > 0);
    });
    let _ = client
        .request_historical_data(&HistoricalDataRequest {
            req_id:           1020,
            contract:         contract.clone(),
            end_date_time:    Utc::now(),
            duration:         Duration::Seconds(1800),
            bar_size_setting: BarSize::_1Secs,
            format_date:      IntradayBarDateFormat::YYYYMMDD,
            keep_up_to_date:  false,
            chart_options:    vec![],
            what_to_show:     HistoricalDataType::Trades,
            use_rth:          UseRegularTradingHoursOnly::Use,
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(())
}
#[tokio::test]
async fn market_data_historical_head_timestamp() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.head_timestamp.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(time) = receiver.recv() {
            count += 1;
            tracing::error!("head timestamp: {:#?}", time);
        }
        assert!(count > 0);
    });
    let _ = client
        .request_head_time_stamp(&HeadTimestampRequest {
            req_id: 1010,
            contract,
            what_to_show: HistoricalDataType::Trades,
            use_rth: UseRegularTradingHoursOnly::DontUse,
            format_date: IntradayBarDateFormat::YYYYMMDD,
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    Ok(())
}
#[tokio::test]
async fn market_data_tick_by_tick() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 10).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let contract = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };
    let receiver = client.market_data_tracker.tick_by_tick.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(tick) = receiver.recv() {
            count += 3;
            tracing::error!("got tick: {:#?}", tick);
        }
        assert!(count > 0);
    });
    let receiver = client.market_data_tracker.historical_ticks.clone();
    thread::spawn(move || {
        let mut count: i32 = 0;
        while let Ok(tick) = receiver.recv() {
            count += 3;
            tracing::error!("got tick: {:#?}", tick);
        }
        assert!(count > 0);
    });
    let _ = client
        .request_tick_by_tick_data(&TickByTickRequest {
            req_id: 200,
            contract,
            tick_type: TickByTickType::BidAsk,
            number_of_ticks: 100,
            ignore_size: false,
        })
        .await?;

    tokio::time::sleep(std::time::Duration::from_secs(6)).await;
    Ok(())
}
