use crate::graphql::domain::ControllerMonthVentCalculated;
use sqlx::PgExecutor;

pub async fn query_all_controller_month_vents<'e, E: PgExecutor<'e>>(
    executor: E,
) -> Result<Vec<ControllerMonthVentCalculated>, sqlx::Error> {
    sqlx::query_as!(ControllerMonthVentCalculated,
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
        .fetch_all(executor)
        .await
}
