INSERT INTO
  gas_analysis_calculated_params(
    id,
    gas_gravity,
    higher_heating_value,
    carbon_content,
    created_by_id,
    created_at,
    updated_by_id,
    updated_at
  )
SELECT
  *
FROM
  UNNEST(
    $1::uuid[],
    $2::double precision[],
    $3::double precision[],
    $4::double precision[],
    $5::uuid[],
    $6::timestamp without time zone[],
    $7::uuid[],
    $8::timestamp without time zone[]
  ) ON CONFLICT (id) DO
UPDATE
SET
  gas_gravity = EXCLUDED.gas_gravity,
  higher_heating_value = EXCLUDED.higher_heating_value,
  carbon_content = EXCLUDED.carbon_content,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at;