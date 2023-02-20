use crate::{
    configuration::DefaultMoleFractions,
    graphql::models::{ControllerMonthVentCalculated, ControllerMonthVentInsertRow},
};
use chrono::NaiveDate;
use itertools::Itertools;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_controller_month_vents(
    pool: &PgPool,
    user_id: Uuid,
    month: NaiveDate,
    DefaultMoleFractions { c1, co2 }: &DefaultMoleFractions,
) -> Result<u64, sqlx::Error> {
    let controller_month_vents_calculated = sqlx::query_file_as!(
        ControllerMonthVentCalculated,
        "src/graphql/sql/statements/controller_month_vent_calculated.sql",
        month,
        c1,
        co2
    )
    .fetch_all(pool)
    .await?;

    if !controller_month_vents_calculated.is_empty() {
        let insert_rows_string = controller_month_vents_calculated
            .into_iter()
            .map(|controller_month_vent_calculated| {
                let insert_row =
                    ControllerMonthVentInsertRow::new(user_id, controller_month_vent_calculated);
                let insert_row_string: String = insert_row.into();
                insert_row_string
            })
            .join(",");

        let insert_statement = format!("
			INSERT INTO controller_month_vent (id, month, gas_volume, c1_volume, co2_volume, controller_id, created_by_id, created_at, updated_by_id, updated_at)
			VALUES {}
			ON CONFLICT (controller_id, month)
			DO
				UPDATE 
				SET gas_volume = EXCLUDED.gas_volume,
                    c1_volume = EXCLUDED.c1_volume,
                    co2_volume = EXCLUDED.co2_volume,
					updated_by_id = EXCLUDED.updated_by_id,
					updated_at = EXCLUDED.updated_at;
			",
		insert_rows_string);

        let rows_inserted = sqlx::query(&insert_statement)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_inserted)
    } else {
        Ok(0)
    }
}
