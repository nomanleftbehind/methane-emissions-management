use crate::graphql::models::DeleteEntryInput;
use common::DeleteEntryVariant::{
    Compressor, Controller, ControllerChange, ControllerMonthHours, ControllerMonthVent,
    ControllerMonthVentOverride, TankFarm,
};
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
        ControllerChange => query!("DELETE FROM controller_changes WHERE id = $1", id,),
        ControllerMonthHours => query!("DELETE FROM controller_month_hours WHERE id = $1", id,),
        ControllerMonthVentOverride => query!(
            "DELETE FROM controller_month_vent_override WHERE id = $1",
            id,
        ),
        ControllerMonthVent => query!("DELETE FROM controller_month_vent WHERE id = $1", id,),
    };

    Ok(query.execute(pool).await?.rows_affected())
}
