use crate::models::daily_statistic::{DailyStatistic, Period};
use crate::AppState;
use actix_web::web;
use actix_web::web::{Data, HttpRequest, HttpResponse, Query};
use actix_web::Error;
use futures::Future;
use log::debug;

pub struct ChartData;

impl ChartData {
    pub fn get_daily_statistic(
        state: Data<AppState>,
        _req: HttpRequest,
        params: Query<Period>,
    ) -> impl Future<Item = HttpResponse, Error = Error> {
        debug!("get_daily_statistic");
        let conn = &state.db.connection();
        let redis = &state.cache.into_inner();
        let res =
            DailyStatistic::get_period(conn, redis, params.start.as_str(), params.end.as_str());
        match res {
            Ok(d) => api_resp_data!(d),
            Err(e) => api_resp_err!(format!("get_daily_statistic failed: {}", e.as_str())),
        }
    }

    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("statistic/period").route(web::get().to_async(Self::get_daily_statistic)),
        );
    }
}
