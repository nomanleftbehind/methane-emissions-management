SELECT

cmv.controller_id,
cmv.month::date as "month!",
SUM(cmv.hours_on * cmv.rate * cmv.percent) as "gas_volume!",
0::double precision as "c1_volume!",
0::double precision as "co2_volume!"

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
      WHERE month = $1
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
    ) cmv

GROUP BY
cmv.controller_id,
cmv.month