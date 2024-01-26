mod all;
use std::env;

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, registry::Registry, EnvFilter};

#[ctor::ctor]
fn init() {
    LogTracer::init().expect("Unable to setup log tracer!");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    ////let (non_blocking_writer, _guard) =
    ////let tracing_appender::non_blocking(std::io::stdout());
    // let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name,
    // std::io::stdout);//non_blocking_writer);
    let formatting_layer = BunyanFormattingLayer::new(app_name, std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    tracing::error!("{:#?}", &env::var("RUST_LOG"));
}
