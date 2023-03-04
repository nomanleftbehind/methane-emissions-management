INSERT INTO
  compressor_blowdown (
    id,
    compressor_id,
    date,
    gas_volume,
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
    $2::uuid[],
    $3::date[],
    $4::double precision[],
    $5::uuid[],
    $6::timestamp without time zone[],
    $7::uuid[],
    $8::timestamp without time zone[]
  ) ON CONFLICT (compressor_id, date) DO
UPDATE
SET
  gas_volume = EXCLUDED.gas_volume,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at;