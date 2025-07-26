use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;

pub fn get_subscriber(
    app_name: String,
    default_env_filter: String,
) -> impl Subscriber + Sync + Send {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // 把 log 宏打印的日志转换成 tracing 的 event
    LogTracer::init().expect("Failed to setup log tracer.");
    set_global_default(subscriber).expect("Failed to set subscriber.");
}