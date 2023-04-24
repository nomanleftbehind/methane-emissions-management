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
  cb.id,
  cpr.id as compressor_id,
  cb.date,
  cb.gas_volume,
  cb.created_by_id,
  cb.created_at,
  cb.updated_by_id,
  cb.updated_at
FROM
  UNNEST(
    $1::uuid[],
    $2::varchar(32)[],
    $3::date[],
    $4::double precision[],
    $5::uuid[],
    $6::timestamp without time zone[],
    $7::uuid[],
    $8::timestamp without time zone[]
  ) as cb(
    id,
    fdc_rec_id,
    date,
    gas_volume,
    created_by_id,
    created_at,
    updated_by_id,
    updated_at
  )
  INNER JOIN compressor cpr ON cpr.fdc_rec_id = cb.fdc_rec_id ON CONFLICT (compressor_id, date) DO
UPDATE
SET
  gas_volume = EXCLUDED.gas_volume,
  updated_by_id = EXCLUDED.updated_by_id,
  updated_at = EXCLUDED.updated_at;