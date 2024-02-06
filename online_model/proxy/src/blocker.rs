use salvo::prelude::*;
use crate::cache::Entry;


pub struct Blocker;

#[handler]
impl Blocker {
    async fn handle(
        &self,
        req     : &mut Request,
        depot   : &mut Depot,
        res     : &mut Response,
        ctrl    : &mut FlowCtrl,
    )
    {
        if let Ok(entry) = depot.get::<Entry>("n_req") {
            // TODO: Must be dynamic, not hardcoded
            if entry.counter() == 2 {
                res.status_code(StatusCode::TOO_MANY_REQUESTS);

                ctrl.skip_rest()
            }
        } else {
            ctrl.call_next(req, depot, res).await;
        }
    }
}

