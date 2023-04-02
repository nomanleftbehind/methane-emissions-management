use actix_web::web::Data;
use emissions_app_derive::FigureOut;
use sqlx::PgPool;

trait FigureOut {
    fn printff(&self);
}

#[derive(Debug, FigureOut)]
pub struct ControllerLoader {
    pool: Data<PgPool>,
}
