use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor_loader::CompressorLoader, user_loader::UserLoader},
    models::{compressor::Compressor, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(SimpleObject, Clone, FromRow, Debug)]
#[graphql(complex)]
pub struct CompressorBlowdown {
    pub id: Uuid,
    pub date: NaiveDate,
    pub gas_volume: f64,
    pub compressor_id: Uuid,
    pub created_by_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_by_id: Uuid,
    pub updated_at: NaiveDateTime,
}

#[ComplexObject]
impl CompressorBlowdown {
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

    async fn compressor(&self, ctx: &Context<'_>) -> Result<Option<Compressor>, Error> {
        let loader = ctx.get_loader::<DataLoader<CompressorLoader>>();
        let compressor = loader.load_one(self.compressor_id).await;

        compressor
    }
}

#[derive(Clone, Debug)]
pub struct CompressorBlowdownInterim {
    pub fdc_rec_id: String,
    pub date: NaiveDate,
    pub gas_volume: f64,
}

pub struct MssqlCompressorBlowdownRows {
    pub mssql_compressor_blowdown_rows: Vec<tiberius::Row>,
}

impl From<MssqlCompressorBlowdownRows>
    for Result<Vec<CompressorBlowdownInterim>, tiberius::error::Error>
{
    fn from(
        MssqlCompressorBlowdownRows {
            mssql_compressor_blowdown_rows,
        }: MssqlCompressorBlowdownRows,
    ) -> Self {
        let mut v = Vec::with_capacity(mssql_compressor_blowdown_rows.len());

        for row in mssql_compressor_blowdown_rows.into_iter() {
            // Match arms to return error if any of the columns were not found or wrong types were detected
            // and to filter out rows with null values and rows without matching MSSQL and Postgres ID crossreference
            match (
                row.try_get::<&str, _>("fdc_rec_id"),
                row.try_get("date"),
                row.try_get("gas_volume"),
            ) {
                (Ok(Some(fdc_rec_id)), Ok(Some(date)), Ok(Some(gas_volume))) => {
                    v.push(CompressorBlowdownInterim {
                        fdc_rec_id: fdc_rec_id.to_string(),
                        date,
                        gas_volume,
                    });
                }
                (Err(e), ..) | (_, Err(e), _) | (.., Err(e)) => return Err(e),
                _ => (),
            }
        }
        Ok(v)
    }
}

#[derive(Debug)]
pub struct CompressorBlowdownInterimUnnestedRows {
    pub user_id: Uuid,
    pub compressor_blowdown_interims: Vec<CompressorBlowdownInterim>,
}

#[derive(Debug)]
pub struct CompressorBlowdownInterimNestedRows {
    pub id: Vec<Uuid>,
    pub fdc_rec_id: Vec<String>,
    pub date: Vec<NaiveDate>,
    pub gas_volume: Vec<f64>,
    pub created_by_id: Vec<Uuid>,
    pub created_at: Vec<NaiveDateTime>,
    pub updated_by_id: Vec<Uuid>,
    pub updated_at: Vec<NaiveDateTime>,
}

impl From<CompressorBlowdownInterimUnnestedRows> for CompressorBlowdownInterimNestedRows {
    fn from(
        CompressorBlowdownInterimUnnestedRows {
            user_id,
            compressor_blowdown_interims,
        }: CompressorBlowdownInterimUnnestedRows,
    ) -> Self {
        let (
            id,
            fdc_rec_id,
            date,
            gas_volume,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        ): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = compressor_blowdown_interims
            .into_iter()
            .map(|cmvc| {
                (
                    Uuid::new_v4(),
                    cmvc.fdc_rec_id,
                    cmvc.date,
                    cmvc.gas_volume,
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                    user_id.clone(),
                    chrono::Utc::now().naive_utc(),
                )
            })
            .multiunzip();

        CompressorBlowdownInterimNestedRows {
            id,
            fdc_rec_id,
            date,
            gas_volume,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
