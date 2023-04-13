use crate::graphql::models::IdSelection;
use common::IdSelectionVariant::{self, ControllerApplicationId, ControllerManufacturerId};
use sqlx::{query_as, Error, PgPool};

pub async fn id_selection(
    pool: &PgPool,
    variant: IdSelectionVariant,
) -> Result<Vec<IdSelection>, Error> {
    println!("calling id selection: {:?}", &variant);
    match variant {
        ControllerManufacturerId => {
            query_as!(
            IdSelection,
            "SELECT id, manufacturer as name FROM controller_manufacturers ORDER BY manufacturer"
        )
            .fetch_all(pool)
            .await
        }
        ControllerApplicationId => {
            query_as!(
                IdSelection,
                "SELECT id, application as name FROM controller_applications ORDER BY application"
            )
            .fetch_all(pool)
            .await
        }
    }
}
