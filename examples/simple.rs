use std::{net::{IpAddr, Ipv4Addr, SocketAddr},
          thread};

use ibkr_rust_api::{client,
                    prelude::{Contract, *}};
#[tokio::main]
pub async fn main() -> Result<()> {

    tracing_subscriber::fmt::init();

    let mut client = client::connect(get_client_addr(), 99).await?;


    Ok(())
}
fn get_client_addr() -> SocketAddr {
    // SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 62)), 4444)
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 62)), 1111)
}

async fn contract_details_stream() -> impl Stream<Item = ContractDetails> {
    
}
