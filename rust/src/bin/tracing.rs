use opentelemetry::trace::{Tracer, TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::TracerProvider;
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

fn main() {
    // // Create a new OpenTelemetry trace pipeline that prints to stdout
    // let provider = TracerProvider::builder()
    //     .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
    //     .build();
    // let tracer = provider.tracer("readme_example");

    // // Create a tracing layer with the configured tracer
    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // // Use the tracing subscriber `Registry`, or any other subscriber
    // // that impls `LookupSpan`
    // let subscriber = Registry::default().with(telemetry);

    let otel_tracer = {
        let endpoint = "http://localhost:3200";
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .thread_name("rw-otel")
            .worker_threads(2)
            .build()
            .unwrap();
        let runtime = Box::leak(Box::new(runtime));

        // Installing the exporter requires a tokio runtime.
        let _entered = runtime.enter();

        opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(endpoint),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .unwrap()
    };

    let layer = tracing_opentelemetry::layer().with_tracer(otel_tracer);
    // .with_filter(reload_filter);

    tracing_subscriber::registry().with(layer).init();

    // Spans will be sent to the configured OpenTelemetry exporter
    let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();

    error!("This event will be logged in the root span.");
}
