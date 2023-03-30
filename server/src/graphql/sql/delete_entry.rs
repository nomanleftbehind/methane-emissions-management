use crate::graphql::models::DeleteEntryInput;
use common::DeleteEntryVariant::{Compressor, Controller, TankFarm};
use sqlx::{query, Error, PgPool};

pub async fn delete_entry(
    pool: &PgPool,
    DeleteEntryInput {
        id,
        delete_entry_variant,
    }: DeleteEntryInput,
) -> Result<u64, Error> {
    let query = match delete_entry_variant {
        Controller => query!("DELETE FROM controllers WHERE id = $1", id,),
        Compressor => query!("DELETE FROM compressors WHERE id = $1", id,),
        TankFarm => query!("DELETE FROM tank_farms WHERE id = $1", id,),
    };

    Ok(query.execute(pool).await?.rows_affected())
}