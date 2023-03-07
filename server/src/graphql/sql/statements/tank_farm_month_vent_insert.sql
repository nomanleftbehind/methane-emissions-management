INSERT INTO
	tank_farm_month_vent (
		id,
		tank_farm_id,
		month,
		gas_volume,
		c1_volume,
		co2_volume,
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
		$5::double precision[],
		$6::double precision[],
		$7::uuid[],
		$8::timestamp without time zone[],
		$9::uuid[],
		$10::timestamp without time zone[]
	) ON CONFLICT (tank_farm_id, month) DO
UPDATE
SET
	gas_volume = EXCLUDED.gas_volume,
	c1_volume = EXCLUDED.c1_volume,
	co2_volume = EXCLUDED.co2_volume,
	updated_by_id = EXCLUDED.updated_by_id,
	updated_at = EXCLUDED.updated_at;