use crate::graphql::domain::{ControllerMonthVentCalculated, ControllerMonthVentInsertValuesRow};
use itertools::Itertools;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert_controller_month_vents(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let controller_month_vents_calculated = sqlx::query_as!(ControllerMonthVentCalculated,
      r#"SELECT

      cmv.controller_id,
      cmv.month::date as "month!",
      SUM(cmv.hours_on * cmv.rate * cmv.percent) as "volume!"
      
      FROM (
      
      SELECT
      
      cmh.controller_id,
      cmh.month_beginning_guarantee as month,
      cmh.hours_on,
      cc.rate,
      EXTRACT(DAY FROM (LEAST(cc.date_end, cmh.month_end_guarantee) + INTERVAL '1 day' - GREATEST(cc.date, cmh.month_beginning_guarantee))) / DATE_PART('days', cmh.month_end_guarantee) percent
      
      FROM            (
                        SELECT
                        
                        controller_id,
                        month,
                        DATE_TRUNC('month', month) month_beginning_guarantee,
                        DATE_TRUNC('month', month) + INTERVAL '1 month - 1 day' month_end_guarantee,
                        hours_on
      
      
                        FROM controller_month_hours
                      ) cmh
      LEFT OUTER JOIN (
                        SELECT
      
                        controller_id,
                        DATE_TRUNC('month', date) month_join_beginning,
                        DATE_TRUNC('month', COALESCE(LEAD(date) OVER (PARTITION BY controller_id ORDER BY date) - INTERVAL '1 day', CURRENT_DATE)) + INTERVAL '1 month - 1 day' month_join_end,
                        date,
                        COALESCE(LEAD(date) OVER (PARTITION BY controller_id ORDER BY date) - INTERVAL '1 day', CURRENT_DATE) date_end,
                        rate
      
                        FROM controller_changes
                      ) cc ON cc.controller_id = cmh.controller_id AND cmh.month BETWEEN cc.month_join_beginning AND cc.month_join_end
      
      --WHERE cmh.controller_id IN ('45c84ed1-5fc5-4b56-801a-9a96c86f947e','292d0c53-536d-43d4-b627-acddd0f585aa')
      ) cmv
      
      GROUP BY
      cmv.controller_id,
      cmv.month"#)
        .fetch_all(pool)
        .await?;

    let insert_rows_string = controller_month_vents_calculated
        .into_iter()
        .map(|controller_month_vent_calculated| {
            let insert_row =
                ControllerMonthVentInsertValuesRow::new(user_id, controller_month_vent_calculated);
            let insert_row_string: String = insert_row.into();
            insert_row_string
        })
        .join(",");

    let insert_statement = format!("INSERT INTO controller_month_vent (id, month, volume, controller_id, created_by_id, created_at, updated_by_id, updated_at) VALUES {};", insert_rows_string);

    let rows_inserted = sqlx::query(&insert_statement)
        .execute(pool)
        .await?
        .rows_affected();

    Ok(rows_inserted)
}
