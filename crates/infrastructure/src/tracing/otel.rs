// use opentelemetry::global::tracer;
// use opentelemetry::runtime::Tokio;
// use opentelemetry::sdk::export::trace::stdout;
// use opentelemetry_otlp::WithExportConfig;
//
// use crate::config::Config;
//
// pub fn setup_otel(conf: &Config) {
//     let otel_layer = conf.tracing.otel_address.as_ref().map(|addr| {
//         // Configure the OpenTelemetry tracing layer
//         opentelemetry::global::set_text_map_propagator(
//             opentelemetry::sdk::propagation::TraceContextPropagator::new(),
//         );
//
//         // let exporter = opentelemetry_otlp::new_exporter()
//         //     .tonic()
//         //     .with_endpoint(addr);
//         //
//         // let tracer = opentelemetry_otlp::new_pipeline()
//         //     .tracing()
//         //     .with_exporter(exporter)
//         //     .with_trace_config(
//         //         opentelemetry::sdk::trace::config()
//         //             .with_sampler(
//         //                 conf.tracing.otel_sample_ratio
//         //
// .map(opentelemetry::sdk::trace::Sampler::TraceIdRatioBased)         //
// .unwrap_or(opentelemetry::sdk::trace::Sampler::AlwaysOn),         //
// )         //
// .with_resource(opentelemetry::sdk::Resource::new(vec![         //
// opentelemetry::KeyValue::new(         //                     "service.name",
//         //                     env!("SERVICE_NAME"),
//         //                 ),
//         //             ])),
//         //     )
//         //     .install_batch(Tokio).unwrap();
//
//         let tracer = stdout::new_pipeline().install_simple();
//         let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
//         tracing_opentelemetry::layer().with_tracer(tracer)
//     });
// }
