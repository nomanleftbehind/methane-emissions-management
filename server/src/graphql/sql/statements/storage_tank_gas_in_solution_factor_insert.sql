INSERT INTO
  storage_tank_gas_in_solution_factor_calculated AS stgisf (
    id,
    storage_tank_id,
    date,
    gis_factor,
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
    $2::uuid [],
    $3::date [],
    $4::double precision [],
    $5::timestamp without time zone [],
    $5::timestamp without time zone []
  ) ON CONFLICT (storage_tank_id, date) DO
UPDATE
SET
  gis_factor = EXCLUDED.gis_factor,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at
WHERE
  stgisf.gis_factor <> EXCLUDED.gis_factor;