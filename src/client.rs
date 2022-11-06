use lib::{init_tracing, SplinterClient};
use std::{
    net::{IpAddr, Ipv6Addr},
    time::Duration,
};
use tarpc::{client, context, tokio_serde::formats::Bincode};
use tokio::time::sleep;
use tracing::Instrument;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing("Tarpc Example Client")?;
    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), 8247);
    let transport = tarpc::serde_transport::tcp::connect(server_addr, Bincode::default);

    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    let client = SplinterClient::new(client::Config::default(), transport.await?).spawn();

    let hello = async move {
        // Send the request twice, just to be safe! ;)
        tokio::select! {
            hello1 = client.hello(context::current(), format!("{}1", "Tom")) => { hello1 }
            hello2 = client.hello(context::current(), format!("{}2", "Astra")) => { hello2 }
        }
    }
    .instrument(tracing::info_span!("Two Hellos"))
    .await;

    tracing::info!("{:?}", hello);

    // Let the background span processor finish.
    sleep(Duration::from_micros(1)).await;

    Ok(())
}
