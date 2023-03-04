use crate::graphql::{
    context::ContextExt,
    dataloaders::{compressor_loader::CompressorLoader, user_loader::UserLoader},
    models::{Compressor, User},
};
use async_graphql::{dataloader::DataLoader, ComplexObject, Context, Error, SimpleObject};
use chrono::{NaiveDate, NaiveDateTime};
use itertools::MultiUnzip;
use sqlx::FromRow;
use std::collections::HashMap;
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
    pub compressor_id: Uuid,
    pub date: NaiveDate,
    pub gas_volume: f64,
}

/// This data structure takes two fields:
/// * `crossref` is reference to [`HashMap`](std::collections::HashMap) where keys are unique IDs of Compressors in third party FDC Microsoft SQL Server database, and values are corresponding unique IDs of compressors in this application's Postgres database.
/// * `mssql_server_rows` is [`Vec`](alloc::vec) of [`tiberius::Row`](tiberius::Row)s
///
/// This struct implements [`From`](core::convert::From) trait that converts `Vec<tiberius::Row>` into `Vec<CompressorBlowdownInterim>` by mapping Postgres compressor IDs onto SQL Server compressor IDs using provided `crossref` `HashMap`.
pub struct CompressorBlowdownDbCrossrefRows<'m> {
    pub crossref: &'m HashMap<String, Uuid>,
    pub mssql_server_rows: Vec<tiberius::Row>,
}

impl From<CompressorBlowdownDbCrossrefRows<'_>>
    for Result<Vec<CompressorBlowdownInterim>, tiberius::error::Error>
{
    fn from(
        CompressorBlowdownDbCrossrefRows {
            crossref,
            mssql_server_rows,
        }: CompressorBlowdownDbCrossrefRows,
    ) -> Self {
        let mut v = Vec::with_capacity(mssql_server_rows.len());

        for row in mssql_server_rows.into_iter() {
            // Nested matches to return error if any of the columns were not found or wrong types were detected
            match row.try_get::<&str, _>("fdc_rec_id") {
                Ok(fdc_rec_id) => {
                    match row.try_get("date") {
                        Ok(date) => {
                            match row.try_get("gas_volume") {
                                Ok(gas_volume) => {
                                    // Nested if let to filter out rows with null values and rows without matching MSSQL and Postgres ID crossreference
                                    if let Some(fdc_rec_id) = fdc_rec_id {
                                        // This block is where MSSQL and Postgres ID crossreference is checked
                                        if let Some(compressor_id) =
                                            crossref.get(fdc_rec_id).copied()
                                        {
                                            if let Some(date) = date {
                                                if let Some(gas_volume) = gas_volume {
                                                    v.push(CompressorBlowdownInterim {
                                                        compressor_id,
                                                        date,
                                                        gas_volume,
                                                    })
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            };
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
    pub compressor_id: Vec<Uuid>,
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
            compressor_id,
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
                    cmvc.compressor_id,
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
            compressor_id,
            date,
            gas_volume,
            created_by_id,
            created_at,
            updated_by_id,
            updated_at,
        }
    }
}
