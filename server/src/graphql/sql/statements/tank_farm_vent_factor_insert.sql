INSERT INTO
  tank_farm_vent_factors_calculated(
    id,
    tank_farm_id,
    date,
    vent_factor,
    created_by_id,
    created_at,
    updated_by_id,
    updated_at
  )
SELECT
  *
FROM
  UNNEST(
    $1::uuid [],
    $2::uuid [],
    $3::date [],
    $4::double precision [],
    $5::uuid [],
    $6::timestamp without time zone [],
    $7::uuid [],
    $8::timestamp without time zone []
  ) ON CONFLICT (tank_farm_id, date) DO
UPDATE
SET
  vent_factor = EXCLUDED.vent_factor,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at;