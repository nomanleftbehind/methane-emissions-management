INSERT INTO
	compressor_month_vent (
		id,
		month,
		gas_volume,
		c1_volume,
		co2_volume,
		compressor_id,
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
		$2::date[],
		$3::double precision[],
		$4::double precision[],
		$5::double precision[],
		$6::uuid[],
		$7::uuid[],
		$8::timestamp without time zone[],
		$9::uuid[],
		$10::timestamp without time zone[]
	) ON CONFLICT (compressor_id, month) DO
UPDATE
SET
	gas_volume = EXCLUDED.gas_volume,
	c1_volume = EXCLUDED.c1_volume,
	co2_volume = EXCLUDED.co2_volume,
	updated_by_id = EXCLUDED.updated_by_id,
	updated_at = EXCLUDED.updated_at;