//! API

use salvo::prelude::*;

use super::rate_limiter::{
    Metrics,
    QuotaMetrics
};
use super::TEST;


///
#[handler]
pub async fn home(
    res: &mut Response
)
{
    res.render(Text::Plain("Test Rate Limit"))
}

///
#[handler]
pub async fn update_metrics(
    &self,
    req: &mut Request,
    res: &mut Response,
)
{
    if let Ok(new_metrics) = req.parse_json::<Metrics>().await {
        let mut metrics = TEST.write().await;

        if let Some(metrics) = metrics.as_mut() {
            metrics.update_metrics(new_metrics);
        } else {
            *metrics = Some(QuotaMetrics::new(new_metrics))
        }

        dbg!(&metrics);

        res.render(Text::Json(r#"{"status": "Updated"}"#));
    } else {
        res.render(Text::Json(r#"{"status":"Invalid metrics"}"#));
    }
}
