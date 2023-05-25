WITH calculate_dates as (
	SELECT
		date::date
	FROM
		generate_series(DATE_TRUNC('month', '2023-03-01'::date), '2023-04-01'::date + INTERVAL '1 month - 1 day', '1 day') as date
)

SELECT
	stmme.facility_id,
	stmme.site_id as "site_id!",
	'storage_tank'::methane_emission_source_table as "source_table!: _",
	stmme.id as "source_table_id!",
	stmme.category as "category!: _",
	stmme.source as "source!: _",
	stmme.month as "month!",
	SUM(stmme.gas_volume) as "gas_volume!",
	SUM(stmme.c1_volume) as "c1_volume!",
	SUM(stmme.co2_volume) as "co2_volume!"
FROM
	(

SELECT
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.month,
	COALESCE(stmme.category, 'FUGITIVE') as category,
	COALESCE(stmme.source, 'FUGITIVE') as source,
	CASE WHEN stmme.category IS NULL THEN stmme.emission_survey_gas_volume ELSE stmme.gas_volume + stmme.emission_survey_gas_volume END as gas_volume,
	CASE WHEN stmme.category IS NULL THEN stmme.emission_survey_c1_volume ELSE stmme.c1_volume + stmme.emission_survey_c1_volume END as c1_volume,
	CASE WHEN stmme.category IS NULL THEN stmme.emission_survey_co2_volume ELSE stmme.co2_volume + stmme.emission_survey_co2_volume END as co2_volume
	
	FROM (

SELECT
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	CASE
		WHEN stmme.control_device IS NULL THEN 'ROUTINE'::methane_emission_category
		ELSE CASE
		 -- AER Manual 015 section 4.2.2
		WHEN stmme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_category
		WHEN stmme.reason IN ('PLANNED_MAINTENANCE','UNPLANNED_MAINTENANCE') THEN 'NONROUTINE'::methane_emission_category
		END
	END as category,
	CASE
		WHEN stmme.control_device IS NULL THEN -- AER Manual 015 section 4.2.2
			'DEFINED_VENT_GAS'::methane_emission_source
		ELSE CASE
			-- AER Manual 015 section 4.2.2
			WHEN stmme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_source
			WHEN stmme.reason = 'PLANNED_MAINTENANCE' THEN 'PLANNED'::methane_emission_source
			WHEN stmme.reason = 'UNPLANNED_MAINTENANCE' THEN 'UNPLANNED'::methane_emission_source
		END
	END as source,
	stmme.month,
	stmme.gas_volume,
	stmme.c1_volume,
	stmme.co2_volume,
	stmme.emission_survey_gas_volume,
	stmme.emission_survey_c1_volume,
	stmme.emission_survey_co2_volume
FROM
	(

SELECT
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.month,
	stmme.control_device,
	stmme.reason,
	SUM(stmme.gas_volume) as gas_volume,
	SUM(stmme.c1_volume) as c1_volume,
	SUM(stmme.co2_volume) as co2_volume,
	SUM(stmme.emission_survey_gas_volume) as emission_survey_gas_volume,
	SUM(stmme.emission_survey_c1_volume) as emission_survey_c1_volume,
	SUM(stmme.emission_survey_co2_volume) as emission_survey_co2_volume
FROM
	(

SELECT
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.date,
	stmme.month,
	stmme.control_device,
	stmme.reason,
	stmme.gas_volume,
	stmme.gas_volume * stmme.c1 as c1_volume,
	stmme.gas_volume * stmme.co2 as co2_volume,
	stmme.emission_survey_c1_volume / stmme.c1 as emission_survey_gas_volume,
	stmme.emission_survey_c1_volume,
	(stmme.emission_survey_c1_volume / stmme.c1) * stmme.co2 as emission_survey_co2_volume

FROM (

SELECT

stmme.*,
COALESCE(ga.c1, 0.82) as c1,
COALESCE(ga.co2, 0.0067) as co2

FROM (

SELECT

s.facility_id,
stmme.*

FROM (

SELECT

stmme.*,
COALESCE(stes.c1_volume, 0) as emission_survey_c1_volume

FROM (

SELECT

stmme.site_id,
stmme.id,
stmme.date,
stmme.month,
stmme.gas_volume,
stmme.control_device,
stcdi.reason

FROM (

SELECT

stmme.*,
stcc.id as storage_tank_controlled_characterization_id,
stcc.control_device

FROM (

SELECT
	stmme.id,
	stmme.site_id,
	stmme.date,
	stmme.month,
	stmme.liquid_hydrocarbon_volume * stmme.gis_factor as gas_volume
FROM
	(
SELECT
	st.id,
	st.site_id,
	st.date,
	st.month,
	COALESCE(stmlhe.liquid_hydrocarbon_volume / COUNT(st.id) OVER (PARTITION BY st.id, st.month), 0) as liquid_hydrocarbon_volume,
	COALESCE(stgisf.gis_factor, 0.9475) as gis_factor

FROM
	(
	SELECT
		st.id,
		st.site_id,
		cd.date,
		DATE_TRUNC('month', cd.date)::date as month
	FROM
		storage_tank st
		INNER JOIN calculate_dates cd ON cd.date BETWEEN st.start_date
		AND COALESCE(st.end_date, CURRENT_DATE)
		AND (st.id, DATE_TRUNC('month', cd.date)) NOT IN (
			SELECT
				stmmeo.storage_tank_id,
				stmmeo.month
			FROM
				storage_tank_month_methane_emission_override stmmeo
		)
	--INNER JOIN site s ON s.id = st.site_id WHERE s.fdc_rec_id = '4D63F14CBC20470B8E7A4A18708DF47E'
		) st
		LEFT OUTER JOIN storage_tank_month_liquid_hydrocarbon_entering stmlhe ON stmlhe.storage_tank_id = st.id AND stmlhe.month = st.month
		LEFT OUTER JOIN (
			SELECT
				stgisf.storage_tank_id,
				stgisf.date as from_date,
				COALESCE((LEAD(stgisf.date) OVER (PARTITION BY stgisf.storage_tank_id
						ORDER BY stgisf.date) - INTERVAL '1 day')::date, CURRENT_DATE) as to_date,
				stgisf.gis_factor

			FROM storage_tank_gas_in_solution_factor_calculated stgisf
		) stgisf ON stgisf.storage_tank_id = st.id AND st.date BETWEEN stgisf.from_date AND stgisf.to_date
		) stmme

UNION ALL
		SELECT
			st.id,
			st.site_id,
			cd.date,
			stmmeo.month,
			stmmeo.gas_volume / COUNT(stmmeo.storage_tank_id) OVER (PARTITION BY stmmeo.storage_tank_id, stmmeo.month) as gas_volume
		FROM
			storage_tank_month_methane_emission_override stmmeo
			INNER JOIN calculate_dates cd ON DATE_TRUNC('month', cd.date) = stmmeo.month
			INNER JOIN storage_tank st ON st.id = stmmeo.storage_tank_id AND cd.date BETWEEN st.start_date AND COALESCE(st.end_date, CURRENT_DATE)
			--INNER JOIN site s ON s.id = st.site_id WHERE s.fdc_rec_id = '4D63F14CBC20470B8E7A4A18708DF47E'
	) stmme
LEFT OUTER JOIN (
		SELECT
		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		LEAST(stcc.end_date, stcc.max_end_date) as end_date,
		stcc.control_device

		FROM (

		SELECT

		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		stcc.end_date,
		stcc.control_device,
		COALESCE((LEAD(stcc.start_date) OVER (
					PARTITION BY stcc.storage_tank_id
					ORDER BY
						stcc.start_date
				) - INTERVAL '1 day')::date, CURRENT_DATE) as max_end_date

		FROM storage_tank_controlled_characterization stcc
		) stcc
	) stcc ON stcc.storage_tank_id = stmme.id AND stmme.date BETWEEN stcc.start_date AND stcc.end_date
	) stmme
	LEFT OUTER JOIN (
			SELECT
			stcdi.storage_tank_controlled_characterization_id,
			stcdi.start_date,
			LEAST(stcdi.end_date, stcdi.max_end_date) as end_date,
			stcdi.reason

			FROM (

			SELECT
				stcdi.storage_tank_controlled_characterization_id,
				stcdi.start_date,
				stcdi.end_date,
				stcdi.reason,
				COALESCE((LEAD(stcdi.start_date) OVER (
						PARTITION BY stcdi.storage_tank_controlled_characterization_id
						ORDER BY
							stcdi.start_date
					) - INTERVAL '1 day')::date, CURRENT_DATE) as max_end_date

			FROM
				storage_tank_control_device_inactivity stcdi

			) stcdi
	) stcdi ON stcdi.storage_tank_controlled_characterization_id = stmme.storage_tank_controlled_characterization_id AND stmme.date BETWEEN stcdi.start_date AND stcdi.end_date
	) stmme
	LEFT OUTER JOIN (
			SELECT

			stes.storage_tank_id,
			stes.date,
			SUM(stes.c1_volume) as c1_volume

			FROM (

			SELECT

			stes.storage_tank_id,
			stes.date,
			24 * stes.rate * stes.percent as c1_volume

			FROM (

			SELECT

			stes.storage_tank_id,
			stes.date,
			stes.rate,
			stes.leak_duration / stes.max_leak_duration as percent

			FROM (

			SELECT

			stes.storage_tank_id,
			stes.date,
			stes.rate,
			LEAST(stes.max_leak_duration, stes.leak_duration) as leak_duration,
			stes.max_leak_duration

			FROM (

			SELECT

			stes.storage_tank_id,
			stes.start_date,
			stes.end_date,
			stes.rate,
			stes.leak_duration,
			stes.min_start_date,
			stes.max_end_date,
			EXTRACT(DAY FROM (stes.end_date + INTERVAL '1 day' - stes.start_date))*24 as max_leak_duration,
			generate_series(stes.min_start_date, stes.max_end_date, '1 day')::date as date

			FROM (

				SELECT

				stes.storage_tank_id,
				stes.start_date,
				stes.end_date,
				stes.rate,
				stes.leak_duration,
				GREATEST(stes.start_date, stes.storage_tank_start_date, '2023-03-01') as min_start_date,
				LEAST(stes.end_date, stes.storage_tank_end_date, '2023-04-01'::date + INTERVAL '1 month - 1 day')::date as max_end_date

				FROM (
				SELECT

				stes.storage_tank_id,
				stes.start_date,
				COALESCE(stes.end_date, CURRENT_DATE) as end_date,
				st.start_date as storage_tank_start_date,
				COALESCE(st.end_date, CURRENT_DATE) as storage_tank_end_date,
				stes.rate,
				stes.leak_duration

				FROM storage_tank_emission_survey stes
				INNER JOIN storage_tank st ON st.id = stes.storage_tank_id
				AND stes.start_date <= LEAST(COALESCE(st.end_date, CURRENT_DATE), ('2023-04-01'::date + INTERVAL '1 month - 1 day'))
				AND COALESCE(stes.end_date, CURRENT_DATE) >= GREATEST(st.start_date, '2023-03-01')
			) stes
				) stes
				) stes
				) stes
				) stes
				) stes
			GROUP BY
			stes.storage_tank_id,
			stes.date

			ORDER BY stes.date
	
	) stes ON stes.storage_tank_id = stmme.id AND stes.date = stmme.date
	) stmme
	LEFT OUTER JOIN site s ON s.id = stmme.site_id
	) stmme
	LEFT OUTER JOIN (
		SELECT
		ga.facility_id,
		ga.date as from_date,
		COALESCE((LEAD(ga.date) OVER (PARTITION BY ga.facility_id
				ORDER BY ga.date) - INTERVAL '1 day')::date, CURRENT_DATE) as to_date,
		ga.c1,
		ga.co2
		FROM gas_analysis ga
	) ga ON ga.facility_id = stmme.facility_id AND stmme.date BETWEEN ga.from_date AND ga.to_date
	) stmme
	) stmme
	GROUP BY
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.month,
	stmme.control_device,
	stmme.reason
	) stmme
	) stmme
	) stmme
GROUP BY
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.category,
	stmme.source,
	stmme.month