WITH allocate_month as (
  SELECT
    month_beginning::date
  FROM
    generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
  cbmme.facility_id,
  cbmme.site_id as "site_id!",
  'compressor_blowdown'::methane_emission_source as "source!: _",
  cbmme.id as "source_id!",
  'NONROUTINE'::methane_emission_category as "category!: _",
  cbmme.month_beginning as "month!",
  SUM(cbmme.gas_volume) as "gas_volume!",
  SUM(cbmme.gas_volume * cbmme.c1) as "c1_volume!",
  SUM(cbmme.gas_volume * cbmme.co2) as "co2_volume!"
FROM
  (
    SELECT
      s.facility_id,
      cbmme.site_id,
      cbmme.id,
      cbmme.month_beginning,
      cbmme.gas_volume,
      COALESCE(ga.c1, $3) as c1,
      COALESCE(ga.co2, $4) as co2
    FROM
      (
        SELECT
          cpr.id,
          cpr.site_id,
          am.month_beginning,
          COALESCE(cb.date, am.month_beginning) as date,
          COALESCE(cb.gas_volume, 0) as gas_volume
        FROM
          compressor cpr
          INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', cpr.install_date)
          AND COALESCE(cpr.remove_date, CURRENT_DATE)
          LEFT OUTER JOIN compressor_blowdown cb ON cb.compressor_id = cpr.id
          AND DATE_TRUNC('month', cb.date) = am.month_beginning
          AND cb.date BETWEEN cpr.install_date
          AND COALESCE(cpr.remove_date, CURRENT_DATE)
          AND (cpr.id, cb.date) NOT IN (
            SELECT
              cbo.compressor_id,
              cbo.date
            FROM
              compressor_blowdown_override cbo
          )
        UNION
        ALL
        SELECT
          cpr.id,
          cpr.site_id,
          DATE_TRUNC('month', cbo.date)::date as month_beginning,
          cbo.date,
          cbo.gas_volume
        FROM
          compressor_blowdown_override cbo
          INNER JOIN compressor cpr ON cpr.id = cbo.compressor_id
          AND cbo.date BETWEEN cpr.install_date
          AND COALESCE(cpr.remove_date, CURRENT_DATE)
          AND DATE_TRUNC('month', cbo.date) IN (
            SELECT
              month_beginning
            FROM
              allocate_month
          )
      ) cbmme
      LEFT OUTER JOIN site s ON s.id = cbmme.site_id
      LEFT OUTER JOIN (
        SELECT
          facility_id,
          date as from_date,
          COALESCE(
            LEAD(date) OVER (
              PARTITION BY facility_id
              ORDER BY
                date
            ) - INTERVAL '1 day',
            CURRENT_DATE
          ) as to_date,
          c1,
          co2
        FROM
          gas_analysis
      ) ga ON ga.facility_id = s.facility_id
      AND cbmme.date BETWEEN ga.from_date
      AND ga.to_date
  ) cbmme
GROUP BY
  cbmme.facility_id,
  cbmme.site_id,
  cbmme.id,
  cbmme.month_beginning