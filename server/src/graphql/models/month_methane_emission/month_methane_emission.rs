use crate::graphql::{
    context::ContextExt,
    dataloaders::{facility::FacilityLoader, site::SiteLoader, user::UserLoader},
    models::{facility::Facility, site::Site, user::User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use common::{MethaneEmissionCategory, MethaneEmissionSource};
// use itertools::Itertools;
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct MonthMethaneEmission {
    pub id: Uuid,
    pub facility_id: Uuid,
    pub site_id: Uuid,
    pub source: MethaneEmissionSource,
    pub source_id: Uuid,
    pub category: MethaneEmissionCategory,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl MonthMethaneEmission {
    async fn created_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let created_by = loader.load_one(self.created_by_id).await;

        created_by
    }

    async fn updated_by(&self, ctx: &Context<'_>) -> Result<Option<User>, Error> {
        let loader = ctx.get_loader::<DataLoader<UserLoader>>();
        let updated_by = loader.load_one(self.updated_by_id).await;

        updated_by
    }

    async fn facility(&self, ctx: &Context<'_>) -> Result<Option<Facility>, Error> {
        let loader = ctx.get_loader::<DataLoader<FacilityLoader>>();
        let facility = loader.load_one(self.facility_id).await;

        facility
    }

    async fn site(&self, ctx: &Context<'_>) -> Result<Option<Site>, Error> {
        let loader = ctx.get_loader::<DataLoader<SiteLoader>>();
        let site = loader.load_one(self.site_id).await;

        site
    }
}

#[derive(SimpleObject, Clone, FromRow, Debug)]
pub struct MonthMethaneEmissionCalculated {
    pub facility_id: Uuid,
    pub site_id: Uuid,
    pub source: MethaneEmissionSource,
    pub source_id: Uuid,
    pub category: MethaneEmissionCategory,
    pub month: NaiveDate,
    pub gas_volume: f64,
    pub c1_volume: f64,
    pub co2_volume: f64,
}

#[derive(Debug)]
pub struct MonthMethaneEmissionUnnestedRows(pub Vec<MonthMethaneEmissionCalculated>);

#[derive(Debug)]
pub struct MonthMethaneEmissionNestedRows {
    pub id: Vec<Uuid>,
    pub facility_id: Vec<Uuid>,
    pub site_id: Vec<Uuid>,
    pub source: Vec<MethaneEmissionSource>,
    pub source_id: Vec<Uuid>,
    pub category: Vec<MethaneEmissionCategory>,
    pub month: Vec<NaiveDate>,
    pub gas_volume: Vec<f64>,
    pub c1_volume: Vec<f64>,
    pub co2_volume: Vec<f64>,
    pub created_at: Vec<NaiveDateTime>,
    // pub created_by_id: Vec<Uuid>,
    // pub updated_by_id: Vec<Uuid>,
    // pub updated_at: Vec<NaiveDateTime>,
}

impl From<MonthMethaneEmissionUnnestedRows> for MonthMethaneEmissionNestedRows {
    fn from(
        MonthMethaneEmissionUnnestedRows(month_methane_emissions_calculated): MonthMethaneEmissionUnnestedRows,
    ) -> Self {
        let (
            id,
            facility_id,
            site_id,
            source,
            source_id,
            category,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            created_at,
            // created_by_id,
            // updated_by_id,
            // updated_at,
        ): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            // Vec<_>,
            // Vec<_>,
            // Vec<_>,
        ) = month_methane_emissions_calculated
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.facility_id,
                    cmvc.site_id,
                    cmvc.source,
                    cmvc.source_id,
                    cmvc.category,
                    cmvc.month,
                    cmvc.gas_volume,
                    cmvc.c1_volume,
                    cmvc.co2_volume,
                    chrono::Utc::now().naive_utc(),
                    // user_id.clone(),
                    // user_id.clone(),
                    // chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        MonthMethaneEmissionNestedRows {
            id,
            facility_id,
            site_id,
            source,
            source_id,
            category,
            month,
            gas_volume,
            c1_volume,
            co2_volume,
            created_at,
            // created_by_id,
            // updated_by_id,
            // updated_at,
        }
    }
}
