INSERT INTO
    month_methane_emission (
        id,
		facility_id,
		site_id,
		source,
		source_id,
		category,
        month,
        gas_volume,
        c1_volume,
        co2_volume,
        created_at,
        updated_at,
		created_by_id,
        updated_by_id
    )
SELECT
    *,
	$12::uuid,
	$12::uuid
FROM
    UNNEST(
        $1::uuid[],
		$2::uuid[],
		$3::uuid[],
		$4::methane_emission_source[],
		$5::uuid[],
		$6::methane_emission_category[],
        $7::date[],
        $8::double precision[],
        $9::double precision[],
        $10::double precision[],
        $11::timestamp without time zone[],
        $11::timestamp without time zone[]
    ) ON CONFLICT (source_id, category, month) DO
UPDATE
SET
    gas_volume = EXCLUDED.gas_volume,
    c1_volume = EXCLUDED.c1_volume,
    co2_volume = EXCLUDED.co2_volume,
    updated_by_id = EXCLUDED.updated_by_id,
    updated_at = EXCLUDED.updated_at;