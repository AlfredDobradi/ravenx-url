use opentelemetry::{global, trace::TracerProvider as _, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
    Resource,
};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_core::Level;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;

fn resource() -> Resource {
    Resource::builder()
        .with_service_name("ravenx")
        .with_schema_url(
            [
                KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
                KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "development"),
            ],
            SCHEMA_URL,
        )
        .build()
}

// Construct Tracer for OpenTelemetryLayer
fn init_tracer(endpoint: String) -> Result<Tracer, anyhow::Error> {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()?;

    let provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build();

    global::set_tracer_provider(provider.clone());
    Ok(provider.tracer("tracing-otel-subscriber"))
}

// Initialize tracing-subscriber and return OtelGuard for opentelemetry-related termination processing
pub fn init_tracing_subscriber(config: &Config, level: Level) -> Result<(), anyhow::Error> {
    let tracer = init_tracer(
        config
            .clone()
            .otlp_endpoint
            .unwrap_or("http://localhost:4317".to_string()),
    )?;

    let int = tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(level))
        .with(tracing_subscriber::fmt::layer());

    if config.otlp_endpoint.is_some() {
        tracing::debug!("initializing OpenTelemetry tracing export layer");
        int.with(OpenTelemetryLayer::new(tracer)).init();
    } else {
        int.init();
    }

    Ok(())
}
