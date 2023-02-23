SELECT
  tfvf.id as tank_farm_id,
  tfvf.from_date as date,
  tfvf.coeff_apply * tfvf.coeff_1 * tfvf.gas_gravity * POWER(
    /*kPa to psi*/
    tfvf.pressure / 6.89475729316836,
    tfvf.coeff_2
  ) * EXP(
    coeff_3 * tfvf.api_density /(
      /*Celsius to Rankine*/
      tfvf.temperature * 1.8 + 491.67
    )
  ) *(
    /*scf/bbl to m3/m3*/
    6.2898 / 35.3147
  ) as vent_factor
FROM
  (
    SELECT
      tf.id,
      f.type as facility_type,
      GREATEST(tfc.from_date, ga.from_date) as from_date,
      LEAST(tfc.to_date, ga.to_date) as to_date,
      tfc.api_density,
      tfc.temperature,
      tfc.pressure,
      tfc.ia,
      tfc.vru,
      COALESCE(gacp.gas_gravity, 0.7) as gas_gravity,
      CASE
        WHEN (
          f.type = 'GP'
          AND tfc.ia
        )
        OR (
          f.type <> 'GP'
          AND tfc.vru
        ) THEN 0
        ELSE 1
      END as coeff_apply,
      CASE
        WHEN tfc.api_density > 30 THEN 0.0178
        ELSE 0.0362
      END as coeff_1,
      CASE
        WHEN tfc.api_density > 30 THEN 1.187
        ELSE 1.0937
      END as coeff_2,
      CASE
        WHEN tfc.api_density > 30 THEN 23.931
        ELSE 25.724
      END as coeff_3
    FROM
      tank_farms tf
      LEFT OUTER JOIN facilities f ON f.id = tf.facility_id
      LEFT OUTER JOIN (
        SELECT
          tank_farm_id,
          date as from_date,
          COALESCE(
            LEAD(date) OVER (
              PARTITION BY tank_farm_id
              ORDER BY
                date
            ) - INTERVAL '1 day',
            CURRENT_DATE
          ) as to_date,
          ia,
          vru,
          api_density,
          temperature,
          pressure
        FROM
          tank_farm_changes
      ) tfc ON tfc.tank_farm_id = tf.id
      LEFT OUTER JOIN (
        SELECT
          id,
          facility_id,
          date as from_date,
          COALESCE(
            LEAD(date) OVER (
              PARTITION BY facility_id
              ORDER BY
                date
            ) - INTERVAL '1 day',
            CURRENT_DATE
          ) as to_date
        FROM
          gas_analyses
      ) ga ON ga.facility_id = tf.facility_id
      AND ga.from_date <= tfc.to_date
      AND ga.to_date >= tfc.from_date
      LEFT OUTER JOIN gas_analysis_calculated_params gacp ON gacp.id = ga.id
  ) tfvf