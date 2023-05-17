WITH allocate_month as (
	SELECT
		month_beginning::date
	FROM
		generate_series('2023-03-01'::date, '2023-04-01'::date, '1 month') as month_beginning
),

storage_tank_controlled_characterization_filtered as (
	SELECT
		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		stcc.end_date,
		stcc.control_device

	FROM
		storage_tank_controlled_characterization stcc
		INNER JOIN storage_tank st ON st.id = stcc.storage_tank_id
		AND st.start_date <= COALESCE(stcc.end_date, CURRENT_DATE)
		AND COALESCE(st.end_date, CURRENT_DATE) >= stcc.start_date
),

storage_tank_controlled_characterization_adjusted_dates as (
	SELECT 
		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		stcc.end_date,
		stcc.control_device,
		(stcc.end_date + INTERVAL '1 day')::date as start_date_fill,
		(LEAD(stcc.start_date) OVER (PARTITION BY stcc.storage_tank_id
									ORDER BY stcc.start_date
										) - INTERVAL '1 day')::date as end_date_fill
		
		
		FROM (
		SELECT
								stcc.id,
								stcc.storage_tank_id,
								stcc.start_date,
								GREATEST(LEAST(stcc.end_date, stcc.end_date_potential), stcc.start_date) as end_date,
								stcc.control_device

								FROM (
									
								SELECT
									
								stcc.*,
								COALESCE((LEAD(stcc.start_date) OVER (
											PARTITION BY stcc.storage_tank_id
											ORDER BY
												stcc.start_date
										) - INTERVAL '1 day')::date, CURRENT_DATE) as end_date_potential
									
								FROM (

								SELECT
									stcc.id,
									stcc.storage_tank_id,
									stcc.start_date,
									stcc.end_date,
									stcc.control_device

								FROM
									storage_tank_controlled_characterization_filtered stcc
								) stcc
								) stcc
								) stcc

		
	),
	
storage_tank_control_device_inactivity_adjusted_dates as (
	SELECT
	
	stcdi.id,
	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	(stcdi.end_date + INTERVAL '1 day')::date as start_date_fill,
	(LEAD(stcdi.start_date) OVER (PARTITION BY stcdi.storage_tank_controlled_characterization_id
									ORDER BY stcdi.start_date
										) - INTERVAL '1 day')::date as end_date_fill,
	stcdi.reason
	
	FROM (
	
	SELECT
	
	stcdi.id,
	stcdi.storage_tank_controlled_characterization_id,
	GREATEST(stcdi.start_date, stcc.start_date) as start_date,
	LEAST(stcdi.end_date, stcc.end_date) as end_date,
	stcdi.reason
	
	FROM storage_tank_controlled_characterization_adjusted_dates stcc
	INNER JOIN (
			SELECT
				stcdi.id,
				stcdi.storage_tank_controlled_characterization_id,
				stcdi.start_date,
				GREATEST(LEAST(stcdi.end_date, stcdi.end_date_potential), stcdi.start_date) as end_date,
				stcdi.reason

				FROM (

				SELECT
					stcdi.id,
					stcdi.storage_tank_controlled_characterization_id,
					stcdi.start_date,
					stcdi.end_date,
					stcdi.reason,
					COALESCE((LEAD(stcdi.start_date) OVER (
							PARTITION BY stcdi.storage_tank_controlled_characterization_id
							ORDER BY
								stcdi.start_date
						) - INTERVAL '1 day')::date, CURRENT_DATE) as end_date_potential

				FROM
					storage_tank_control_device_inactivity stcdi

				) stcdi
	) stcdi ON stcdi.storage_tank_controlled_characterization_id = stcc.id
		AND stcdi.start_date <= stcc.end_date
		AND stcdi.end_date >= stcc.start_date
		) stcdi
),

storage_tank_control_device_inactivity_filled_date_gaps as (

SELECT

stcdi.storage_tank_controlled_characterization_id,
stcdi.start_date,
stcdi.end_date,
stcdi.reason

FROM storage_tank_control_device_inactivity_adjusted_dates stcdi

UNION ALL

SELECT

stcdi.storage_tank_controlled_characterization_id,
stcdi.start_date_fill as start_date,
stcdi.end_date_fill as end_date,
NULL as reason

FROM storage_tank_control_device_inactivity_adjusted_dates stcdi

WHERE NOT stcdi.start_date_fill > stcdi.end_date_fill
	
UNION ALL
	
	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	NULL as reason

	FROM (

	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	MIN(stcc.start_date) as start_date,
	(MIN(stcdi.start_date) - INTERVAL '1 day')::date as end_date

	FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	INNER JOIN storage_tank_controlled_characterization_adjusted_dates stcc ON stcc.id = stcdi.storage_tank_controlled_characterization_id

	GROUP BY stcdi.storage_tank_controlled_characterization_id
		) stcdi
	WHERE (stcdi.storage_tank_controlled_characterization_id, stcdi.start_date) NOT IN (
		SELECT stcdi.storage_tank_controlled_characterization_id, stcdi.start_date
		FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	)
	
UNION ALL
	
	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	NULL as reason

	FROM (

	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	(MAX(stcdi.end_date) + INTERVAL '1 day')::date as start_date,
	MAX(stcc.end_date) as end_date

	FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	INNER JOIN storage_tank_controlled_characterization_adjusted_dates stcc ON stcc.id = stcdi.storage_tank_controlled_characterization_id

	GROUP BY stcdi.storage_tank_controlled_characterization_id
		) stcdi
	WHERE (stcdi.storage_tank_controlled_characterization_id, stcdi.end_date) NOT IN (
		SELECT stcdi.storage_tank_controlled_characterization_id, stcdi.end_date
		FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	)

),

storage_tank_controlled_characterization_filled_date_gaps as (
	
	SELECT

	stcc.id,
	stcc.storage_tank_id,
	stcc.start_date,
	stcc.end_date,
	stcc.control_device

	FROM storage_tank_controlled_characterization_adjusted_dates stcc

	UNION ALL

	SELECT

	NULL as id,
	stcc.storage_tank_id,
	stcc.start_date_fill as start_date,
	stcc.end_date_fill as end_date,
	NULL as control_device

	FROM storage_tank_controlled_characterization_adjusted_dates stcc

	WHERE NOT stcc.start_date_fill > stcc.end_date_fill
	
UNION ALL
	
	SELECT

	NULL as id,
	stcc.storage_tank_id,
	stcc.start_date,
	stcc.end_date,
	NULL as control_device

	FROM (

	SELECT

	stcc.storage_tank_id,
	DATE_TRUNC('month', MIN(stcc.start_date))::date as start_date,
	(MIN(stcc.start_date) - INTERVAL '1 day')::date as end_date

	FROM
		storage_tank_controlled_characterization_adjusted_dates stcc

	GROUP BY stcc.storage_tank_id
		) stcc
	WHERE (stcc.storage_tank_id, stcc.start_date) NOT IN (
		SELECT stcc.storage_tank_id, stcc.start_date
		FROM storage_tank_controlled_characterization_adjusted_dates stcc
	)
),

gas_analysis_filled_date_gaps as (
	SELECT
	ga.facility_id,
	ga.date as from_date,
	COALESCE((LEAD(ga.date) OVER (PARTITION BY ga.facility_id
			ORDER BY ga.date) - INTERVAL '1 day')::date, CURRENT_DATE) as to_date,
	ga.c1,
	ga.co2
FROM gas_analysis ga
	
UNION ALL
	
SELECT

	ga.facility_id,
	ga.from_date,
	ga.to_date,
	NULL as c1,
	NULL as co2

	FROM (
	
SELECT
	ga.facility_id,
	DATE_TRUNC('month', MIN(ga.date))::date as from_date,
	(MIN(ga.date) - INTERVAL '1 day')::date as to_date
	
	FROM gas_analysis ga
	GROUP BY ga.facility_id
		) ga
	WHERE (ga.facility_id, ga.from_date) NOT IN (
		SELECT ga.facility_id, ga.date
		FROM gas_analysis ga
		)
	)


SELECT
	stmme.facility_id,
	stmme.site_id as "site_id!",
	'storage_tank'::methane_emission_source_table as "source_table!: _",
	stmme.id as "source_table_id!",
	stmme.category as "category!: _",
	stmme.source as "source!: _",
	stmme.month_beginning as "month!",
	SUM(stmme.gas_volume) as "gas_volume!",
	SUM(stmme.c1_volume) as "c1_volume!",
	SUM(stmme.co2_volume) as "co2_volume!"
FROM
	(

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
	stmme.month_beginning,
	stmme.gas_volume,
	stmme.c1_volume,
	stmme.co2_volume
FROM
	(


SELECT
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.month_beginning,
	stmme.control_device,
	stmme.reason,
	SUM(stmme.gas_volume * stmme.percent) as gas_volume,
	SUM(stmme.gas_volume * stmme.c1 * stmme.percent) as c1_volume,
	SUM(stmme.gas_volume * stmme.co2 * stmme.percent) as co2_volume
FROM
	(
		SELECT
			stmme.facility_id,
			stmme.site_id,
			stmme.id,
			stmme.month_beginning,
			stmme.gas_volume,
			stmme.c1,
			stmme.co2,
			stmme.control_device,
			stmme.reason,
			EXTRACT(
				DAY
				FROM
					(
						stmme.to_date + INTERVAL '1 day' - stmme.from_date
					)
			) / stmme.days_in_period as percent
		FROM
			(
				SELECT
					s.facility_id,
					stmme.site_id,
					stmme.id,
					stmme.month_beginning,
					stmme.gas_volume,
					COALESCE(ga.c1, 0.82) as c1,
					COALESCE(ga.co2, 0.0067) as co2,
					stmme.control_device,
					stmme.reason,
					GREATEST(ga.from_date, stmme.from_date) as from_date,
					LEAST(ga.to_date, stmme.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(stmme.to_date + INTERVAL '1 day' - stmme.from_date)
					) as days_in_period
				FROM
					(
						SELECT
							stmme.id,
							stmme.site_id,
							stmme.month_beginning,
							stmme.from_date,
							stmme.to_date,
							stmme.gas_volume * stmme.percent as gas_volume,
							stmme.control_device,
							stmme.reason

						FROM
							(

						SELECT
							stmme.id,
							stmme.site_id,
							stmme.month_beginning,
							stmme.from_date,
							stmme.to_date,
							stmme.gas_volume,
							stmme.control_device,
							stmme.reason,
							EXTRACT(
								DAY
								FROM
									(
										stmme.to_date + INTERVAL '1 day' - stmme.from_date
									)
							) / stmme.days_in_period as percent
						FROM
							(

						SELECT
							stmme.id,
							stmme.site_id,
							stmme.month_beginning,
							stmme.gas_volume,
							stcc.control_device,
							stcc.reason,
							GREATEST(stcc.start_date, stmme.from_date) as from_date,
							LEAST(stcc.end_date, stmme.to_date) as to_date,
							EXTRACT(
								DAY
								FROM
									(
										stmme.to_date + INTERVAL '1 day' - stmme.from_date
									)
							) as days_in_period
						FROM
							(
						SELECT
							stmme.*
						FROM
							(
								SELECT
									stmme.id,
									stmme.site_id,
									stmme.month_beginning,
									stmme.from_date,
									stmme.to_date,
									stmme.liquid_hydrocarbon_volume * stmme.gis_factor * stmme.percent as gas_volume
								FROM
									(
										SELECT
											stmme.id,
											stmme.site_id,
											stmme.month_beginning,
											stmme.from_date,
											stmme.to_date,
											stmme.liquid_hydrocarbon_volume,
											stmme.gis_factor,
											EXTRACT(
												DAY
												FROM
													(stmme.to_date + INTERVAL '1 day' - stmme.from_date)
											) / stmme.days_in_month as percent
										FROM
											(
												SELECT
													st.id,
													st.site_id,
													st.month_beginning,
													GREATEST(stgisf.from_date, st.month_beginning) as from_date,
													LEAST(
														stgisf.to_date,
														st.month_beginning + INTERVAL '1 month - 1 day'
													)::date as to_date,
													DATE_PART(
														'days',
														st.month_beginning + INTERVAL '1 month - 1 day'
													) as days_in_month,
													COALESCE(stmlhe.liquid_hydrocarbon_volume, 0) as liquid_hydrocarbon_volume,
													COALESCE(stgisf.gis_factor, 0) as gis_factor
												FROM
													(
														SELECT
															st.id,
															st.site_id,
															am.month_beginning
														FROM
															storage_tank st
															INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', st.start_date)
															AND COALESCE(st.end_date, CURRENT_DATE)
															AND (st.id, am.month_beginning) NOT IN (
																SELECT
																	stmmeo.storage_tank_id,
																	stmmeo.month
																FROM
																	storage_tank_month_methane_emission_override stmmeo
																	INNER JOIN allocate_month am ON am.month_beginning = stmmeo.month
															)
														INNER JOIN site s ON s.id = st.site_id
														WHERE s.fdc_rec_id = '4D63F14CBC20470B8E7A4A18708DF47E'
													) st
													LEFT OUTER JOIN storage_tank_month_liquid_hydrocarbon_entering stmlhe ON stmlhe.storage_tank_id = st.id
													AND stmlhe.month = st.month_beginning
													LEFT OUTER JOIN (
														SELECT
															storage_tank_id,
															DATE_TRUNC('month', date) as month_join_beginning,
															DATE_TRUNC(
																'month',
																COALESCE(
																	LEAD(date) OVER (
																		PARTITION BY storage_tank_id
																		ORDER BY
																			date
																	) - INTERVAL '1 day',
																	CURRENT_DATE
																)
															) + INTERVAL '1 month - 1 day' as month_join_end,
															-- If first GIS factor, from_date has to be first of the month because there is no carryover from previous GIS factor.
															CASE
																WHEN ROW_NUMBER() OVER (
																	PARTITION BY storage_tank_id
																	ORDER BY
																		date
																) = 1 THEN DATE_TRUNC('month', date)::date
																ELSE date
															END as from_date,
															COALESCE(
																LEAD(date) OVER (
																	PARTITION BY storage_tank_id
																	ORDER BY
																		date
																) - INTERVAL '1 day',
																CURRENT_DATE
															)::date as to_date,
															gis_factor
														FROM
															storage_tank_gas_in_solution_factor_calculated
													) stgisf ON stgisf.storage_tank_id = st.id
													AND st.month_beginning BETWEEN stgisf.month_join_beginning
													AND stgisf.month_join_end
											) stmme
									) stmme
							) stmme
						UNION
						ALL
						SELECT
							stmme.*
						FROM
							(
								SELECT
									st.id,
									st.site_id,
									stmmeo.month as month_beginning,
									stmmeo.month as from_date,
									(stmmeo.month + INTERVAL '1 month - 1 day')::date as to_date,
									stmmeo.gas_volume
								FROM
									storage_tank_month_methane_emission_override stmmeo
									INNER JOIN storage_tank st ON st.id = stmmeo.storage_tank_id
									AND stmmeo.month IN (
										SELECT
											*
										FROM
											allocate_month
									)
									INNER JOIN site s ON s.id = st.site_id
									WHERE s.fdc_rec_id = '4D63F14CBC20470B8E7A4A18708DF47E'
							) stmme
							) stmme
						LEFT OUTER JOIN (
						SELECT

						stcc.storage_tank_id,
						COALESCE(stcdi.start_date, stcc.start_date) as start_date,
						COALESCE(stcdi.end_date, stcc.end_date) as end_date,
						stcc.control_device,
						stcdi.reason

						FROM storage_tank_controlled_characterization_filled_date_gaps stcc
						LEFT OUTER JOIN storage_tank_control_device_inactivity_filled_date_gaps stcdi ON stcdi.storage_tank_controlled_characterization_id = stcc.id

						) stcc ON stcc.storage_tank_id = stmme.id
						AND stcc.start_date <= stmme.to_date
						AND stcc.end_date >= stmme.from_date
					) stmme
					) stmme
					) stmme
					LEFT OUTER JOIN site s ON s.id = stmme.site_id
					LEFT OUTER JOIN gas_analysis_filled_date_gaps ga ON ga.facility_id = s.facility_id
					AND ga.from_date <= stmme.to_date
					AND ga.to_date >= stmme.from_date
					ORDER BY stmme.id, stmme.from_date, ga.from_date
			) stmme
	) stmme
GROUP BY
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.month_beginning,
	stmme.control_device,
	stmme.reason
	) stmme
) stmme
GROUP BY
	stmme.facility_id,
	stmme.site_id,
	stmme.id,
	stmme.category,
	stmme.source,
	stmme.month_beginning