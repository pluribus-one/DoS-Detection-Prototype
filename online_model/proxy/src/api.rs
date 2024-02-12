//! A module defining available APIs on the system.

use salvo::{
    hyper::Method,
    prelude::*
};

use super::rate_limiter::{
    Metrics,
    SYSTEM_METRICS,
    QuotaMetrics
};


/// Definition of POST `/metrics` API.
#[handler]
pub async fn update_metrics(
    &self,
    req: &mut Request,
    res: &mut Response,
)
{
    if req.method() == Method::POST {
        if let Ok(new_metrics) = req.parse_json::<Metrics>().await {
            let mut metrics = SYSTEM_METRICS.write().await;

            if let Some(metrics) = metrics.as_mut() {
                metrics.update_metrics(new_metrics);
            } else {
                *metrics = Some(QuotaMetrics::new(new_metrics))
            }

            res.render(
                Text::Json(r#"{"status": "Updated"}"#)
            );
        } else {
            res.render(
                Text::Json(r#"{"status":"Invalid metrics"}"#)
            );
        }
    } else {
        res.render(
            StatusError::method_not_allowed()
        )
    }
}
