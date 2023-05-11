INSERT INTO
  gas_analysis_calculated_param AS gacp (
    id,
    gas_gravity,
    higher_heating_value,
    carbon_content,
    created_at,
    updated_at,
    created_by_id,
    updated_by_id
  )
SELECT
  *,
  $6,
  $6
FROM
  UNNEST(
    $1::uuid [],
    $2::double precision [],
    $3::double precision [],
    $4::double precision [],
    $5::timestamp without time zone [],
    $5::timestamp without time zone []
  ) ON CONFLICT (id) DO
UPDATE
SET
  gas_gravity = EXCLUDED.gas_gravity,
  higher_heating_value = EXCLUDED.higher_heating_value,
  carbon_content = EXCLUDED.carbon_content,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at
WHERE
  gacp.gas_gravity <> EXCLUDED.gas_gravity
  OR gacp.higher_heating_value <> EXCLUDED.higher_heating_value
  OR gacp.carbon_content <> EXCLUDED.carbon_content;