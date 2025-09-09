use crate::metrics::encode_metrics;
use ic_http_types::HttpResponseBuilder;
use ic_cdk::query;
use ic_http_types::{HttpRequest, HttpResponse};

#[query(hidden = true)]
pub fn http_request(request: HttpRequest) -> HttpResponse {
    if request.path() == "/metrics" {
        let now_millis = ic_cdk::api::time() / 1_000_000;
        let mut writer =
            ic_metrics_encoder::MetricsEncoder::new(Vec::new(), now_millis as i64);

        match encode_metrics(&mut writer) {
            Ok(()) => HttpResponseBuilder::ok()
                .header("Content-Type", "text/plain")
                .header("Cache-Control", "no-store")
                .with_body_and_content_length(writer.into_inner())
                .build(),
            Err(err) => {
                HttpResponseBuilder::server_error(format!("Failed to encode metrics: {}", err)).build()
            }
        }
    } else {
        HttpResponseBuilder::not_found().build()
    }
}
