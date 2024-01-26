use std::{net::{IpAddr, Ipv4Addr, SocketAddr},
          thread};

use ibkr_rust_api::{client,
                    prelude::{Contract, *}};
fn get_client_addr() -> SocketAddr {
    // SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 62)), 4444)
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 62)), 1111)
}
#[tokio::test]
async fn request_account_updates() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 99).await?;
    client
        .request_account_updates(true, "DU3293378".to_string())
        .await?;
    // tokio::spawn(async move {
    //    let mut account_receiver = client.account_tracker;
    //    while let summary = account_receiver.recv() {
    //        //tracing::debug!(
    //        //    "{:?}\t-\tunrealized pnl:  {:?}",
    //        //    account_receiver.unrealized_pnl()
    //        //);
    //    }
    //});
    thread::spawn(move || {
        let account_receiver = client.account_tracker;
        loop {
            tracing::debug!("got updates at:  {:?}", account_receiver.recv());
        }
        // tracing::debug!("got order with state:  {:?}", order_state);
    });
    tokio::time::sleep(std::time::Duration::from_secs(180)).await;
    // tracing::debug!("next valid order id: {:?}", order_id);
    Ok(())
}

#[tokio::test]
async fn request_next_valid_order_id() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 99).await?;
    let order_id = &client.request_ids().await?;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    tracing::debug!("next valid order id: {:?}", order_id);
    Ok(())
}
#[tokio::test]
async fn request_contract_details() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect(get_client_addr(), 1).await?;
    let contracts = client.subscribe_contract_details().clone();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    thread::spawn(move || {
        while let Ok(contract) = contracts.recv() {
            tracing::error!("got contracts:  {:#?}", contract);
        }
    });
    let _contract2 = Contract {
        symbol: "AMD".to_string(),
        exchange: Some("SMART".to_string()),
        sec_type: SecType::Stock,
        currency: "USD".to_string(),
        ..Default::default()
    };

    let _ = &client.request_contract_details(1, _contract2).await?;
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    Ok(())
}

#[tokio::test]
async fn orders_auto_open() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 0).await?;
    let orders = client.order_tracker.order.clone();
    let executions = client.order_tracker.executions.clone();
    thread::spawn(move || {
        while let Ok(order) = orders.recv() {
            tracing::debug!("got order:  {:?}", order);
        }
    });
    thread::spawn(move || {
        while let Ok(order) = executions.recv() {
            tracing::debug!("got executions:  {:?}", order);
        }
    });
    let _ = client.request_auto_open_orders(true).await?;
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    Ok(())
}

#[tokio::test]
async fn orders_all_open() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let rec_order = client.subscribe_orders().order.clone();
    let rec_state = client.subscribe_orders().order_state.clone();
    let rec_status = client.subscribe_orders().order_status.clone();
    thread::spawn(move || {
        while let Ok(order) = rec_order.recv() {
            tracing::debug!("got order:  {:#?}", order);
        }
    });
    thread::spawn(move || {
        while let Ok(state) = rec_state.recv() {
            tracing::debug!("got state:  {:#?}", state);
        }
    });
    thread::spawn(move || {
        while let Ok(status) = rec_status.recv() {
            tracing::debug!("got status:  {:?}", status);
        }
    });
    let _ = client.request_all_open_orders().await?;
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    // signal::ctrl_c().await.expect("failed to listen for event");
    Ok(())
}

#[tokio::test]
async fn orders_completed() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let rec_order = client.subscribe_orders().order.clone();
    let rec_state = client.subscribe_orders().order_state.clone();
    let rec_status = client.subscribe_orders().order_status.clone();
    thread::spawn(move || {
        loop {
            tracing::error!("got order:  {:#?}", rec_order.recv());
        }
        // while let Ok(order) = rec_order.recv() {
        //     tracing::error!("got order:  {:#?}", order);
        // }
    });
    thread::spawn(move || {
        while let Ok(state) = rec_state.recv() {
            tracing::error!("got state:  {:#?}", state);
        }
    });
    thread::spawn(move || {
        while let Ok(status) = rec_status.recv() {
            tracing::error!("got status:  {:?}", status);
        }
    });
    let _ = client.request_completed_orders(false).await?;
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    // signal::ctrl_c().await.expect("failed to listen for event");
    Ok(())
}
#[tokio::test]
async fn executions_filtered() -> Result<()> {
    let mut client = client::connect(get_client_addr(), 0).await?;
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    let receiver = client.order_tracker.executions.clone();
    thread::spawn(move || {
        while let Ok(fill) = receiver.recv() {
            tracing::error!("got order:  {:?}", fill);
            // tracing::debug!("got order with state:  {:?}", order_state);
        }
    });
    let filter = Some(ExecutionFilter {
        client_id: None,
        account_code: "U11213636".to_string(),
        // symbol: "NIO".into(),
        time: "20220824-00:12:59".to_string(),
        ..Default::default()
    });
    client.request_executions(1, filter).await?;
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    // signal::ctrl_c().await.expect("failed to listen for event");
    Ok(())
}

#[tokio::test]
async fn place_market_order() -> Result<()> {
    // let mut client = client::connect(get_client_addr(), 2).await?;
    //// client.request_auto_open_orders(true).await?;
    // let order_id = &client.get_next_valid_order_id().await?;
    // let contract = Contract {
    //    symbol: "AAPL".to_string(),
    //    exchange: Some("SMART".to_string()),
    //    sec_type: SecType::Stock,
    //    currency: "USD".to_string(),
    //    ..Default::default()
    //};
    // let order = Order::market(contract, Action::Buy, Decimal::new(1, 0));
    // match &mut client.place_order(*order_id, order).await {
    //    Ok(()) => {
    //        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    //        // assert_eq!(tracker.status(), Some("Filled".to_string()));
    //    },
    //    Err(_err) => panic!("Error during order submission."),
    //}
    // tokio::spawn(async move {
    //    while let order = client.order_tracker.order.recv() {
    //        tracing::debug!("got order:  {:?}", order);
    //    }
    //});
    //// signal::ctrl_c().await.expect("failed to listen for event");
    // tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    Ok(())
}

//#[tokio::test]
// async fn request_orders() {
//    let mut client = match IBClient::connect("192.168.1.96".to_string(), 4444,
// 0, "").await {        Ok(client) => client,
//        Err(_error) => panic!("Connection not successful!"),
//    };
//    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//    match &mut client.req_auto_open_orders(true).await {
//        Ok(order) => {
//            // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//            info!("got order: {:?}", order);
//            // assert_eq!(order.status(), Some("Filled".to_string()));
//        },
//        Err(err) => panic!("Error during order submission."),
//    }
//    //match &mut client.req_all_open_orders().await {
//    //    Ok(orders) => {
//    //        // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//    //        info!("got existing orders: {:?}", orders);
//    //        // assert_eq!(order.status(), Some("Filled".to_string()));
//    //    },
//    //    Err(err) => panic!("Error during order submission."),
//    //}
//    loop {}
//}
//
//#[tokio::test]
// async fn place_spread_market_order() {
//    let mut client = match IBClient::connect("192.168.1.96".to_string(),4444,
// 1, "").await {        Ok(client) => client,
//        Err(_error) => panic!("Connection not successful!")
//    };
//    let mut legs = Vec::new();
//    legs.push(ComboLeg::new(43645865, 1, ComboAction::Buy, "SMART")); //IBKR
//    legs.push(ComboLeg::new(9408, 1, ComboAction::Sell, "SMART")); //MCD
//    let contract = Contract {
//        symbol: Some("IBKR,MCD".to_string()),
//        exchange: Some("SMART".to_string()),
//        sec_type: Some(SecType::Combo),
//        currency: Some("USD".to_string()),
//        combo_legs: Some(legs),
//        ..Default::default()
//    };
//
//    let mut order = Order::market(contract, Action::Buy, Decimal::new(10,0));
//    order.smart_combo_routing_params = Some(vec![("NonGuaranteed".to_string(),
// "1".to_string())]);    match &mut client.place_order(&order).await {
//        Ok(tracker) => {
//            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//            assert_eq!(tracker.status(), Some("Filled".to_string()));
//        }
//        Err(err)    => panic!("Error during order submission.")
//    }
//}
//
//#[tokio::test]
// async fn market_data() {
//    let mut client = match IBClient::connect("192.168.1.96".to_string(), 4444,
// 1, "").await {        Ok(client) => client,
//        Err(_error) => panic!("Connection not successful!"),
//    };
//    let contract = Contract {
//        symbol: Some("NIO".to_string()),
//        exchange: Some("SMART".to_string()),
//        sec_type: Some(SecType::Stock),
//        currency: Some("USD".to_string()),
//        ..Default::default()
//    };
//    match &client
//        .req_market_data(&contract, false, false, vec![
//            GenericTickType::ShortableData,
//        ])
//        .await
//    {
//        Ok(ticker) => {
//            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//            //loop {
//                println!("ticker: bid:{}\tmid:{}\task:{}",
// ticker.bid().unwrap(),ticker.midpoint().unwrap(),ticker.ask().unwrap());
//            //}
//            assert!(ticker.midpoint().is_some());
//        },
//        Err(_error) => panic!("Market data request not successful"),
//    }
//}
//
//#[tokio::test]
// async fn historical_data() {
//    let mut client = match IBClient::connect("192.168.1.96".to_string(),4444,
// 1, "").await {        Ok(client) => client,
//        Err(_error) => panic!("Connection not successful!")
//    };
//    let contract = Contract {
//        symbol: Some("AAPL".to_string()),
//        exchange: Some("SMART".to_string()),
//        sec_type: Some(SecType::Stock),
//        currency: Some("USD".to_string()),
//        ..Default::default()
//    };
//    let end_dt = Utc.datetime_from_str("2020-03-01 00:00:00", "%Y-%m-%d
// %H:%M:%S");
//
//    match &client.req_historical_data(&contract, &end_dt.unwrap(), "1 M", "1
// day",    "MIDPOINT", true).await {
//        Ok(bars) => {
//            assert!(bars.n_bars > 0);
//        },
//        Err(_error) => panic!("Bar series loading not successful!")
//    }
//}
