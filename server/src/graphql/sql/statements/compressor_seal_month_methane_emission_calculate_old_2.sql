WITH calculate_dates as (
	SELECT
		date::date
	FROM
		generate_series(
			DATE_TRUNC('month', '2023-03-01'::date),
			'2023-04-01'::date + INTERVAL '1 month - 1 day',
			'1 day'
		) as date
)
SELECT
	csmme.facility_id,
	csmme.site_id as "site_id!",
	'compressor_seal'::methane_emission_source_table as "source_table!: _",
	csmme.id as "source_table_id!",
	csmme.category as "category!: _",
	csmme.source as "source!: _",
	csmme.month as "month!",
	SUM(csmme.gas_volume) as "gas_volume!",
	SUM(csmme.c1_volume) as "c1_volume!",
	SUM(csmme.co2_volume) as "co2_volume!"
FROM
	(
		SELECT
			csmme.facility_id,
			csmme.site_id,
			csmme.id,
			csmme.month,
			COALESCE(csmme.category, 'FUGITIVE') as category,
			COALESCE(csmme.source, 'FUGITIVE') as source,
			CASE
				WHEN csmme.category IS NULL THEN csmme.emission_survey_gas_volume
				ELSE csmme.gas_volume + csmme.emission_survey_gas_volume
			END as gas_volume,
			CASE
				WHEN csmme.category IS NULL THEN csmme.emission_survey_c1_volume
				ELSE csmme.c1_volume + csmme.emission_survey_c1_volume
			END as c1_volume,
			CASE
				WHEN csmme.category IS NULL THEN csmme.emission_survey_co2_volume
				ELSE csmme.co2_volume + csmme.emission_survey_co2_volume
			END as co2_volume
		FROM
			(
				SELECT
					csmme.facility_id,
					csmme.site_id,
					csmme.id,
					CASE
						WHEN csmme.control_device IS NULL THEN 'ROUTINE'::methane_emission_category
						ELSE CASE
							-- AER Manual 015 section 4.2.2
							WHEN csmme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_category
							WHEN csmme.reason IN ('PLANNED_MAINTENANCE', 'UNPLANNED_MAINTENANCE') THEN 'NONROUTINE'::methane_emission_category
						END
					END as category,
					CASE
						WHEN csmme.control_device IS NULL THEN -- AER Manual 015 section 4.2.2
						'COMPRESSOR_SEAL'::methane_emission_source
						ELSE CASE
							-- AER Manual 015 section 4.2.2
							WHEN csmme.reason = 'MALFUNCTION' THEN 'FUGITIVE'::methane_emission_source
							WHEN csmme.reason = 'PLANNED_MAINTENANCE' THEN 'PLANNED'::methane_emission_source
							WHEN csmme.reason = 'UNPLANNED_MAINTENANCE' THEN 'UNPLANNED'::methane_emission_source
						END
					END as source,
					csmme.month,
					csmme.gas_volume,
					csmme.c1_volume,
					csmme.co2_volume,
					csmme.emission_survey_gas_volume,
					csmme.emission_survey_c1_volume,
					csmme.emission_survey_co2_volume
				FROM
					(
						SELECT
							csmme.facility_id,
							csmme.site_id,
							csmme.id,
							csmme.month,
							csmme.control_device,
							csmme.reason,
							SUM(csmme.gas_volume) as gas_volume,
							SUM(csmme.c1_volume) as c1_volume,
							SUM(csmme.co2_volume) as co2_volume,
							SUM(csmme.emission_survey_gas_volume) as emission_survey_gas_volume,
							SUM(csmme.emission_survey_c1_volume) as emission_survey_c1_volume,
							SUM(csmme.emission_survey_co2_volume) as emission_survey_co2_volume
						FROM
							(
								SELECT
									csmme.facility_id,
									csmme.site_id,
									csmme.id,
									csmme.date,
									csmme.month,
									csmme.control_device,
									csmme.reason,
									csmme.gas_volume,
									csmme.gas_volume * csmme.c1 as c1_volume,
									csmme.gas_volume * csmme.co2 as co2_volume,
									csmme.emission_survey_c1_volume / csmme.c1 as emission_survey_gas_volume,
									csmme.emission_survey_c1_volume,
									(csmme.emission_survey_c1_volume / csmme.c1) * csmme.co2 as emission_survey_co2_volume
								FROM
									(
										SELECT
											csmme.*,
											COALESCE(ga.c1, 0.82) as c1,
											COALESCE(ga.co2, 0.0067) as co2
										FROM
											(
												SELECT
													s.facility_id,
													csmme.*
												FROM
													(
														SELECT
															csmme.*,
															COALESCE(ces.c1_volume, 0) as emission_survey_c1_volume
														FROM
															(
																SELECT
																	csmme.site_id,
																	csmme.id,
																	csmme.date,
																	csmme.month,
																	csmme.gas_volume,
																	csmme.control_device,
																	ccdi.reason
																FROM
																	(
																		SELECT
																			csmme.*,
																			ccc.id as compressor_controlled_characterization_id,
																			ccc.control_device
																		FROM
																			(
																				SELECT
																					csmme.id,
																					csmme.site_id,
																					csmme.date,
																					csmme.month,
																					csmme.pressurized_hours * csmme.rate as gas_volume
																				FROM
																					(
																						SELECT
																							csmme.id,
																							csmme.site_id,
																							csmme.date,
																							csmme.month,
																							COALESCE(
																								csmme.pressurized_hours / COUNT(csmme.id) OVER (PARTITION BY csmme.id, csmme.month),
																								0
																							) as pressurized_hours,
																							csmme.rate
																						FROM (

																						SELECT
																							csmme.id,
																							csmme.site_id,
																							csmme.date,
																							csmme.month,
																							LEAST(csmme.pressurized_hours, csmme.max_pressurized_hours) as pressurized_hours,
																							csmme.rate
																						FROM (

																						SELECT
																							csmme.id,
																							csmme.site_id,
																							csmme.date,
																							csmme.month,
																							COALESCE(csmme.pressurized_hours, 0) as pressurized_hours,
																							SUM(24) OVER (PARTITION BY csmme.id, csmme.month) as max_pressurized_hours,
																							COALESCE(csmme.rate, 0) as rate

																							FROM (
																							
																						SELECT
																						
																						cs.id,
																						cs.site_id,
																						cs.date,
																						cs.month,
																						cmh.pressurized_hours,
																						SUM(cst.rate) as rate

																						FROM
																							(
																								SELECT
																									cs.id,
																									cpr.site_id,
																									cd.date,
																									DATE_TRUNC('month', cd.date)::date as month
																								FROM
																									compressor_seal cs -- Compressor seal and compressor share same id, however compressor is joined here because it contains site_id, install_date and remove_date columns, which are used subsequently.
																									LEFT OUTER JOIN compressor cpr ON cpr.id = cs.id -- Despite sharing same id, existence of compressor doesn't guarantee existence of compressor seal, hence why joining calculation days on compressor is not done here.
																									INNER JOIN calculate_dates cd ON cd.date BETWEEN cpr.install_date
																									AND COALESCE(cpr.remove_date, CURRENT_DATE)
																									AND (cs.id, DATE_TRUNC('month', cd.date)) NOT IN (
																										SELECT
																											csmmeo.compressor_seal_id,
																											csmmeo.month
																										FROM
																											compressor_seal_month_methane_emission_override csmmeo
																									)
																								INNER JOIN site s ON s.id = cpr.site_id WHERE s.fdc_rec_id = '22BBCC95F9DD4E0D92F222ADF415B524'
																							) cs
																							LEFT OUTER JOIN compressor_month_hours cmh ON cmh.compressor_id = cs.id AND cmh.month = cs.month
																							LEFT OUTER JOIN (
																								SELECT

																								cst.compressor_seal_id,
																								cst.start_date,
																								LEAST(cst.end_date, cst.max_end_date) as end_date,
																								cst.rate,
																								cst.testing_point

																								FROM (

																								SELECT
																									cst.compressor_seal_id,
																									cst.start_date,
																									cst.end_date,
																									COALESCE(
																										(
																											LEAD(cst.start_date) OVER (
																												PARTITION BY cst.compressor_seal_id, cst.testing_point
																												ORDER BY
																													cst.start_date
																											) - INTERVAL '1 day'
																										)::date,
																										CURRENT_DATE
																									) as max_end_date,
																									cst.rate,
																									cst.testing_point
																								FROM compressor_seal_test cst
																									) cst
																							) cst ON cst.compressor_seal_id = cs.id AND cs.date BETWEEN cst.start_date
																							AND cst.end_date
																							GROUP BY
																							cs.id,
																							cs.site_id,
																							cs.date,
																							cs.month,
																							cmh.pressurized_hours
																							ORDER BY cs.id, cs.date
																							) csmme
																							) csmme
																							) csmme
																							
																					) csmme
																				UNION
																				ALL
																				SELECT
																					cpr.id,
																					cpr.site_id,
																					cd.date,
																					csmmeo.month,
																					csmmeo.gas_volume / COUNT(csmmeo.compressor_seal_id) OVER (
																						PARTITION BY csmmeo.compressor_seal_id,
																						csmmeo.month
																					) as gas_volume
																				FROM
																					compressor_seal_month_methane_emission_override csmmeo
																					INNER JOIN calculate_dates cd ON DATE_TRUNC('month', cd.date) = csmmeo.month
																					INNER JOIN compressor cpr ON cpr.id = csmmeo.compressor_seal_id
																					AND cd.date BETWEEN cpr.install_date
																					AND COALESCE(cpr.remove_date, CURRENT_DATE) --INNER JOIN site s ON s.id = st.site_id WHERE s.fdc_rec_id = '4D63F14CBC20470B8E7A4A18708DF47E'
																			) csmme
																			LEFT OUTER JOIN (
																				SELECT
																					ccc.id,
																					ccc.compressor_id,
																					ccc.start_date,
																					LEAST(ccc.end_date, ccc.max_end_date) as end_date,
																					ccc.control_device
																				FROM
																					(
																						SELECT
																							ccc.id,
																							ccc.compressor_id,
																							ccc.start_date,
																							ccc.end_date,
																							ccc.control_device,
																							COALESCE(
																								(
																									LEAD(ccc.start_date) OVER (
																										PARTITION BY ccc.compressor_id
																										ORDER BY
																											ccc.start_date
																									) - INTERVAL '1 day'
																								)::date,
																								CURRENT_DATE
																							) as max_end_date
																						FROM
																							compressor_controlled_characterization ccc
																					) ccc
																			) ccc ON ccc.compressor_id = csmme.id
																			AND csmme.date BETWEEN ccc.start_date
																			AND ccc.end_date
																	) csmme
																	LEFT OUTER JOIN (
																		SELECT
																			ccdi.compressor_controlled_characterization_id,
																			ccdi.start_date,
																			LEAST(ccdi.end_date, ccdi.max_end_date) as end_date,
																			ccdi.reason
																		FROM
																			(
																				SELECT
																					ccdi.compressor_controlled_characterization_id,
																					ccdi.start_date,
																					ccdi.end_date,
																					ccdi.reason,
																					COALESCE(
																						(
																							LEAD(ccdi.start_date) OVER (
																								PARTITION BY ccdi.compressor_controlled_characterization_id
																								ORDER BY
																									ccdi.start_date
																							) - INTERVAL '1 day'
																						)::date,
																						CURRENT_DATE
																					) as max_end_date
																				FROM
																					compressor_control_device_inactivity ccdi
																			) ccdi
																	) ccdi ON ccdi.compressor_controlled_characterization_id = csmme.compressor_controlled_characterization_id
																	AND csmme.date BETWEEN ccdi.start_date
																	AND ccdi.end_date
															) csmme
															LEFT OUTER JOIN (
																SELECT
																	ces.compressor_id,
																	ces.date,
																	SUM(ces.c1_volume) as c1_volume
																FROM
																	(
																		SELECT
																			ces.compressor_id,
																			ces.date,
																			24 * ces.rate * ces.percent as c1_volume
																		FROM
																			(
																				SELECT
																					ces.compressor_id,
																					ces.date,
																					ces.rate,
																					ces.leak_duration / ces.max_leak_duration as percent
																				FROM
																					(
																						SELECT
																							ces.compressor_id,
																							ces.date,
																							ces.rate,
																							LEAST(ces.max_leak_duration, ces.leak_duration) as leak_duration,
																							ces.max_leak_duration
																						FROM
																							(
																								SELECT
																									ces.compressor_id,
																									ces.start_date,
																									ces.end_date,
																									ces.rate,
																									ces.leak_duration,
																									ces.min_start_date,
																									ces.max_end_date,
																									EXTRACT(
																										DAY
																										FROM
																											(
																												ces.end_date + INTERVAL '1 day' - ces.start_date
																											)
																									) * 24 as max_leak_duration,
																									generate_series(ces.min_start_date, ces.max_end_date, '1 day')::date as date
																								FROM
																									(
																										SELECT
																											ces.compressor_id,
																											ces.start_date,
																											ces.end_date,
																											ces.rate,
																											ces.leak_duration,
																											GREATEST(
																												ces.start_date,
																												ces.compressor_start_date,
																												'2023-03-01'
																											) as min_start_date,
																											LEAST(
																												ces.end_date,
																												ces.compressor_end_date,
																												'2023-04-01'::date + INTERVAL '1 month - 1 day'
																											)::date as max_end_date
																										FROM
																											(
																												SELECT
																													ces.compressor_id,
																													ces.start_date,
																													COALESCE(ces.end_date, CURRENT_DATE) as end_date,
																													cpr.install_date as compressor_start_date,
																													COALESCE(cpr.remove_date, CURRENT_DATE) as compressor_end_date,
																													ces.rate,
																													ces.leak_duration
																												FROM
																													compressor_emission_survey ces
																													INNER JOIN compressor cpr ON cpr.id = ces.compressor_id
																													AND ces.start_date <= LEAST(
																														COALESCE(cpr.remove_date, CURRENT_DATE),
																														('2023-04-01'::date + INTERVAL '1 month - 1 day')
																													)
																													AND COALESCE(ces.end_date, CURRENT_DATE) >= GREATEST(cpr.install_date, '2023-03-01')
																											) ces
																									) ces
																							) ces
																					) ces
																			) ces
																	) ces
																GROUP BY
																	ces.compressor_id,
																	ces.date
																ORDER BY
																	ces.date
															) ces ON ces.compressor_id = csmme.id
															AND ces.date = csmme.date
													) csmme
													LEFT OUTER JOIN site s ON s.id = csmme.site_id
											) csmme
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
											) ga ON ga.facility_id = csmme.facility_id
											AND csmme.date BETWEEN ga.from_date
											AND ga.to_date
									) csmme
							) csmme
						GROUP BY
							csmme.facility_id,
							csmme.site_id,
							csmme.id,
							csmme.month,
							csmme.control_device,
							csmme.reason
					) csmme
			) csmme
	) csmme
GROUP BY
	csmme.facility_id,
	csmme.site_id,
	csmme.id,
	csmme.category,
	csmme.source,
	csmme.month