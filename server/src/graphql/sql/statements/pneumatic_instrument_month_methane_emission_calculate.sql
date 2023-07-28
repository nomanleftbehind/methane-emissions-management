WITH calculate_dates as (
	SELECT
		date::date
	FROM
		generate_series(
			DATE_TRUNC('month', $1::date),
			DATE_TRUNC('month', $2::date) + INTERVAL '1 month - 1 day',
			'1 day'
		) as date
)
SELECT
	pimme.facility_id,
	pimme.site_id as "site_id!",
	'pneumatic_instrument'::methane_emission_source_table as "source_table!: _",
	pimme.id as "source_table_id!",
	pimme.category as "category!: _",
	pimme.source as "source!: _",
	pimme.month as "month!",
	SUM(pimme.gas_volume) as "gas_volume!",
	SUM(pimme.c1_volume) as "c1_volume!",
	SUM(pimme.co2_volume) as "co2_volume!"
FROM
	(
		SELECT
			pimme.facility_id,
			pimme.site_id,
			pimme.id,
			CASE
				WHEN pimme.reason IS NULL THEN 'ROUTINE'::methane_emission_category
				ELSE CASE
					-- AER Manual 015 section 1.1.2
					WHEN pimme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_category
					WHEN pimme.reason IN ('PLANNED_MAINTENANCE', 'UNPLANNED_MAINTENANCE') THEN 'NONROUTINE'::methane_emission_category
				END
			END as category,
			CASE
				WHEN pimme.reason IS NULL THEN 'PNEUMATIC_DEVICE'::methane_emission_source
				ELSE CASE
					-- AER Manual 015 section 1.1.2
					WHEN pimme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_source
					WHEN pimme.reason = 'PLANNED_MAINTENANCE' THEN 'PLANNED'::methane_emission_source
					WHEN pimme.reason = 'UNPLANNED_MAINTENANCE' THEN 'UNPLANNED'::methane_emission_source
				END
			END as source,
			pimme.month,
			pimme.gas_volume,
			pimme.c1_volume,
			pimme.co2_volume
		FROM
			(
				SELECT
					pimme.facility_id,
					pimme.site_id,
					pimme.id,
					pimme.month,
					pimme.control_device,
					pimme.reason,
					SUM(pimme.gas_volume) as gas_volume,
					SUM(pimme.c1_volume) as c1_volume,
					SUM(pimme.co2_volume) as co2_volume
				FROM
					(
						SELECT
							pimme.facility_id,
							pimme.site_id,
							pimme.id,
							pimme.date,
							pimme.month,
							pimme.control_device,
							pimme.reason,
							pimme.gas_volume,
							pimme.gas_volume * pimme.c1 as c1_volume,
							pimme.gas_volume * pimme.co2 as co2_volume
						FROM
							(
								SELECT
									pimme.*,
									COALESCE(ga.c1, $3) as c1,
									COALESCE(ga.co2, $4) as co2
								FROM
									(
										SELECT
											s.facility_id,
											pimme.*
										FROM
											(
												SELECT
													pimme.site_id,
													pimme.id,
													pimme.date,
													pimme.month,
													pimme.gas_volume,
													pimme.control_device,
													picdi.reason
												FROM
													(
														SELECT
															pimme.*,
															picc.id as pneumatic_instrument_controlled_characterization_id,
															picc.control_device
														FROM
															(
																SELECT
																	pimme.id,
																	pimme.site_id,
																	pimme.date,
																	pimme.month,
																	pimme.hours_on * pimme.rate as gas_volume
																FROM
																	(
																		SELECT
																			pimme.id,
																			pimme.site_id,
																			pimme.date,
																			pimme.month,
																			-- If month hours are larger than theoretical, use theoretical.
																			LEAST(pimme.hours_on, 24) as hours_on,
																			pimme.rate
																		FROM
																			(
																				SELECT
																					pi.id,
																					pi.site_id,
																					pi.date,
																					pi.month,
																					COALESCE(
																						pimh.hours_on / COUNT(pi.id) OVER (PARTITION BY pi.id, pi.month),
																						0
																					) as hours_on,
																					COALESCE(pier.rate, 0) as rate
																				FROM
																					(
																						SELECT
																							pi.id,
																							pi.site_id,
																							cd.date,
																							DATE_TRUNC('month', cd.date)::date as month
																						FROM
																							pneumatic_instrument pi
																							INNER JOIN calculate_dates cd ON cd.date BETWEEN pi.start_date
																							AND COALESCE(pi.end_date, CURRENT_DATE)
																							AND (pi.id, DATE_TRUNC('month', cd.date)) NOT IN (
																								SELECT
																									pimmeo.pneumatic_instrument_id,
																									pimmeo.month
																								FROM
																									pneumatic_instrument_month_methane_emission_override pimmeo
																							)
																						-- 	INNER JOIN site s ON s.id = pi.site_id
																						-- WHERE
																						-- 	s.fdc_rec_id = '01695F3482624E4A946AA9E144C3B719'
																						-- 	AND pi.serial_number = 'PP37405'
																					) pi
																					LEFT OUTER JOIN pneumatic_instrument_month_hours pimh ON pimh.pneumatic_instrument_id = pi.id
																					AND pimh.month = pi.month
																					LEFT OUTER JOIN (
																						SELECT
																							pier.pneumatic_instrument_id,
																							pier.date as from_date,
																							COALESCE(
																								(
																									LEAD(pier.date) OVER (
																										PARTITION BY pier.pneumatic_instrument_id
																										ORDER BY
																											pier.date
																									) - INTERVAL '1 day'
																								)::date,
																								CURRENT_DATE
																							) as to_date,
																							pier.rate
																						FROM
																							pneumatic_instrument_emission_rate pier
																					) pier ON pier.pneumatic_instrument_id = pi.id
																					AND pi.date BETWEEN pier.from_date
																					AND pier.to_date
																			) pimme
																	) pimme
																UNION
																ALL
																SELECT
																	pi.id,
																	pi.site_id,
																	cd.date,
																	pimmeo.month,
																	pimmeo.gas_volume / COUNT(pimmeo.pneumatic_instrument_id) OVER (
																		PARTITION BY pimmeo.pneumatic_instrument_id,
																		pimmeo.month
																	) as gas_volume
																FROM
																	pneumatic_instrument_month_methane_emission_override pimmeo
																	INNER JOIN calculate_dates cd ON DATE_TRUNC('month', cd.date) = pimmeo.month
																	INNER JOIN pneumatic_instrument pi ON pi.id = pimmeo.pneumatic_instrument_id
																	AND cd.date BETWEEN pi.start_date
																	AND COALESCE(pi.end_date, CURRENT_DATE)
																-- 	INNER JOIN site s ON s.id = pi.site_id
																-- WHERE
																-- 	s.fdc_rec_id = '01695F3482624E4A946AA9E144C3B719'
															) pimme
															LEFT OUTER JOIN (
																SELECT
																	picc.id,
																	picc.pneumatic_instrument_id,
																	picc.start_date,
																	LEAST(picc.end_date, picc.max_end_date) as end_date,
																	picc.control_device
																FROM
																	(
																		SELECT
																			picc.id,
																			picc.pneumatic_instrument_id,
																			picc.start_date,
																			picc.end_date,
																			picc.control_device,
																			COALESCE(
																				(
																					LEAD(picc.start_date) OVER (
																						PARTITION BY picc.pneumatic_instrument_id
																						ORDER BY
																							picc.start_date
																					) - INTERVAL '1 day'
																				)::date,
																				CURRENT_DATE
																			) as max_end_date
																		FROM
																			pneumatic_instrument_controlled_characterization picc
																	) picc
															) picc ON picc.pneumatic_instrument_id = pimme.id
															AND pimme.date BETWEEN picc.start_date
															AND picc.end_date
													) pimme
													LEFT OUTER JOIN (
														SELECT
															picdi.pneumatic_instrument_controlled_characterization_id,
															picdi.start_date,
															LEAST(picdi.end_date, picdi.max_end_date) as end_date,
															picdi.reason
														FROM
															(
																SELECT
																	picdi.pneumatic_instrument_controlled_characterization_id,
																	picdi.start_date,
																	picdi.end_date,
																	picdi.reason,
																	COALESCE(
																		(
																			LEAD(picdi.start_date) OVER (
																				PARTITION BY picdi.pneumatic_instrument_controlled_characterization_id
																				ORDER BY
																					picdi.start_date
																			) - INTERVAL '1 day'
																		)::date,
																		CURRENT_DATE
																	) as max_end_date
																FROM
																	pneumatic_instrument_control_device_inactivity picdi
															) picdi
													) picdi ON picdi.pneumatic_instrument_controlled_characterization_id = pimme.pneumatic_instrument_controlled_characterization_id
													AND pimme.date BETWEEN picdi.start_date
													AND picdi.end_date
											) pimme
											INNER JOIN site s ON s.id = pimme.site_id
											AND NOT (
												-- Controlled emission is destroyed or recovered, so it's filtered out here.
												pimme.control_device IS NOT NULL
												AND pimme.reason IS NULL
											)
									) pimme
									LEFT OUTER JOIN (
										SELECT
											ga.facility_id,
											ga.date as from_date,
											COALESCE(
												(
													LEAD(ga.date) OVER (
														PARTITION BY ga.facility_id
														ORDER BY
															ga.date
													) - INTERVAL '1 day'
												)::date,
												CURRENT_DATE
											) as to_date,
											ga.c1,
											ga.co2
										FROM
											gas_analysis ga
									) ga ON ga.facility_id = pimme.facility_id
									AND pimme.date BETWEEN ga.from_date
									AND ga.to_date
							) pimme
					) pimme
				GROUP BY
					pimme.facility_id,
					pimme.site_id,
					pimme.id,
					pimme.month,
					pimme.control_device,
					pimme.reason
			) pimme
	) pimme
GROUP BY
	pimme.facility_id,
	pimme.site_id,
	pimme.id,
	pimme.category,
	pimme.source,
	pimme.month