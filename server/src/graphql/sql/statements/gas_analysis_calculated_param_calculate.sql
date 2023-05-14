SELECT
  ga.id,
  (
    ga.h2 * 2.016 + ga.he * 4.002602 + ga.n2 * 28.0134 + ga.co2 * 44.01 + ga.h2s * 34.076 + ga.c1 * 16.043 + ga.c2 * 30.07 + ga.c3 * 44.097 + ga.c4_i * 58.124 + ga.c4_n * 58.124 + ga.c5_i * 72.151 + ga.c5_n * 72.151 + ga.c6 * 86.178 + ga.c7_plus * 100.205
  ) / 28.9647 as "gas_gravity!",
  (
    ga.h2 * 12.77982 + ga.c1 * 37.66879 + ga.c2 * 66.4327 + ga.c3 * 95.83001 + ga.c4_i * 120.1601 + ga.c4_n * 120.1601 + ga.c5_i * 148.3279 + ga.c5_n * 148.3279 + ga.c6 * 173.8875 + ga.c7_plus * 216.6736
  ) as "higher_heating_value!",
  (
    ga.co2 + ga.c1 + ga.c2 * 2 + ga.c3 * 3 + ga.c4_i * 4 + ga.c4_n * 4 + ga.c5_i * 5 + ga.c5_n * 5 + ga.c6 * 6 + ga.c7_plus * 7
  ) * 12.01 / 23.645 as "carbon_content!"
FROM
  (
    SELECT
      id,
      date as from_date,
      COALESCE(
        LEAD(date) OVER (
          PARTITION BY facility_id
          ORDER BY
            date
        ) - INTERVAL '1 day',
        CURRENT_DATE
      )::date as to_date,
      h2,
      he,
      n2,
      co2,
      h2s,
      c1,
      c2,
      c3,
      c4_i,
      c4_n,
      c5_i,
      c5_n,
      c6,
      c7_plus
    FROM
      gas_analysis
  ) ga
WHERE
  ga.from_date <= $2
  AND ga.to_date >= $1