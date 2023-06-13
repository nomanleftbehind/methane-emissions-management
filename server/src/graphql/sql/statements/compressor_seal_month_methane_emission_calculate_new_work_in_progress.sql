WITH calculate_dates as (
	SELECT
		date::date
	FROM
		generate_series(
			DATE_TRUNC('month', $1::date),
			$2::date + INTERVAL '1 month - 1 day',
			'1 day'
		) as date
),
compressor_seal_test_cleaned as (
	SELECT
		cst.compressor_seal_id,
		cst.start_date,
		LEAST(cst.end_date, cst.max_end_date) as end_date,
		cst.rate,
		cst.testing_point
	FROM
		(
			SELECT
				cst.compressor_seal_id,
				cst.start_date,
				cst.end_date,
				COALESCE(
					(
						LEAD(cst.start_date) OVER (
							PARTITION BY cst.compressor_seal_id,
							cst.testing_point
							ORDER BY
								cst.start_date
						) - INTERVAL '1 day'
					)::date,
					CURRENT_DATE
				) as max_end_date,
				-- scf/hr to m3/hr
				cst.rate / 35.3147 as rate,
				cst.testing_point
			FROM
				compressor_seal_test cst
		) cst
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
			CASE
				WHEN csmme.identity = 'EMISSION_SURVEY' THEN 'FUGITIVE'::methane_emission_category
				ELSE CASE
					-- Uncontrolled
					-- For uncontrolled reciprocating compressors, any gas emitted from the crankcase is vent gas from an RCS.
					-- Vent gas associated with other compressor types (screw, scroll, etc.) are included under DVG and categorized as other routine venting and not compressor seals.
					WHEN csmme.control_device IS NULL THEN 'ROUTINE'::methane_emission_category
					ELSE -- Controlled
					CASE
						WHEN csmme.reason IS NULL THEN 'FUGITIVE'::methane_emission_category
						ELSE CASE
							-- For controlled reciprocating compressors, crankcase vents are considered fugitive emissions.
							WHEN cpr.type = 'RECIPROCATING'
							AND csmme.identity = 'CRANKCASE' THEN 'FUGITIVE'::methane_emission_category -- When a compressor seal vent control is not operating, compressor seal vents are considered nonroutine vents.
							ELSE 'NONROUTINE'::methane_emission_category
						END
					END
				END
			END as category,
			CASE
				WHEN csmme.identity = 'EMISSION_SURVEY' THEN 'FUGITIVE'::methane_emission_source
				ELSE CASE
					-- AER Directive 060 section 8.6.2.2
					WHEN csmme.control_device IS NULL THEN -- Uncontrolled
					-- For uncontrolled reciprocating compressors, any gas emitted from the crankcase is vent gas from an RCS.
					CASE
						WHEN cpr.type IN ('RECIPROCATING', 'CENTRIFUGAL') THEN 'COMPRESSOR_SEAL'::methane_emission_source -- Vent gas associated with other compressor types (screw, scroll, etc.) are included under DVG and categorized as other routine venting and not compressor seals.
						ELSE 'DEFINED_VENT_GAS'::methane_emission_source
					END
					ELSE -- Controlled
					CASE
						WHEN csmme.reason IS NULL THEN 'FUGITIVE'::methane_emission_source
						ELSE CASE
							-- For controlled reciprocating compressors, crankcase vents are considered fugitive emissions.
							WHEN cpr.type = 'RECIPROCATING'
							AND csmme.identity = 'CRANKCASE' THEN 'FUGITIVE'::methane_emission_source
							ELSE -- When a compressor seal vent control is not operating, compressor seal vents are considered nonroutine vents.
							CASE
								WHEN csmme.reason = 'PLANNED_MAINTENANCE' THEN 'PLANNED'::methane_emission_source
								ELSE 'UNPLANNED'::methane_emission_source
							END
						END
					END
				END
			END as source,
			csmme.gas_volume,
			csmme.c1_volume,
			csmme.co2_volume
		FROM
			(
				SELECT
					csmme.facility_id,
					csmme.site_id,
					csmme.id,
					csmme.month,
					csmme.control_device,
					csmme.reason,
					UNNEST(
						ARRAY ['NONCRANKCASE', 'CRANKCASE', 'EMISSION_SURVEY']
					) as identity,
					UNNEST(
						ARRAY [csmme.gas_volume, csmme.gas_volume_crankcase, csmme.emission_survey_gas_volume]
					) as gas_volume,
					UNNEST(
						ARRAY [csmme.c1_volume, csmme.c1_volume_crankcase, csmme.emission_survey_c1_volume]
					) as c1_volume,
					UNNEST(
						ARRAY [csmme.co2_volume, csmme.co2_volume_crankcase, csmme.emission_survey_co2_volume]
					) as co2_volume
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
							SUM(csmme.gas_volume_crankcase) as gas_volume_crankcase,
							SUM(csmme.c1_volume_crankcase) as c1_volume_crankcase,
							SUM(csmme.co2_volume_crankcase) as co2_volume_crankcase,
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
									csmme.gas_volume_crankcase,
									csmme.gas_volume_crankcase * csmme.c1 as c1_volume_crankcase,
									csmme.gas_volume_crankcase * csmme.co2 as co2_volume_crankcase,
									csmme.emission_survey_c1_volume / csmme.c1 as emission_survey_gas_volume,
									csmme.emission_survey_c1_volume,
									(csmme.emission_survey_c1_volume / csmme.c1) * csmme.co2 as emission_survey_co2_volume
								FROM
									(
										SELECT
											csmme.*,
											COALESCE(ga.c1, $3) as c1,
											COALESCE(ga.co2, $4) as co2
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
																	csmme.gas_volume_crankcase,
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
																					csmme.pressurized_hours * csmme.rate as gas_volume,
																					csmme.pressurized_hours * csmme.rate_crankcase as gas_volume_crankcase
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
																							csmme.rate,
																							csmme.rate_crankcase
																						FROM
																							(
																								SELECT
																									csmme.id,
																									csmme.site_id,
																									csmme.date,
																									csmme.month,
																									LEAST(
																										csmme.pressurized_hours,
																										csmme.max_pressurized_hours
																									) as pressurized_hours,
																									csmme.rate,
																									csmme.rate_crankcase
																								FROM
																									(
																										SELECT
																											csmme.id,
																											csmme.site_id,
																											csmme.date,
																											csmme.month,
																											COALESCE(csmme.pressurized_hours, 0) as pressurized_hours,
																											SUM(24) OVER (PARTITION BY csmme.id, csmme.month) as max_pressurized_hours,
																											COALESCE(csmme.rate, 0) as rate,
																											COALESCE(csmme.rate_crankcase, 0) as rate_crankcase
																										FROM
																											(
																												SELECT
																													csmme.id,
																													csmme.site_id,
																													csmme.date,
																													csmme.month,
																													csmme.pressurized_hours,
																													csmme.rate,
																													cst_ckc.rate as rate_crankcase
																												FROM
																													(
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
																																	) --INNER JOIN site s ON s.id = cpr.site_id WHERE s.fdc_rec_id = '22BBCC95F9DD4E0D92F222ADF415B524'
																															) cs
																															LEFT OUTER JOIN compressor_month_hours cmh ON cmh.compressor_id = cs.id
																															AND cmh.month = cs.month
																															LEFT OUTER JOIN (
																																SELECT
																																	cst.compressor_seal_id,
																																	cst.start_date,
																																	cst.end_date,
																																	cst.rate,
																																	cst.testing_point
																																FROM
																																	compressor_seal_test_cleaned cst
																																WHERE
																																	cst.testing_point <> 'CRANKCASE'
																															) cst ON cst.compressor_seal_id = cs.id
																															AND cs.date BETWEEN cst.start_date
																															AND cst.end_date
																														GROUP BY
																															cs.id,
																															cs.site_id,
																															cs.date,
																															cs.month,
																															cmh.pressurized_hours
																														ORDER BY
																															cs.id,
																															cs.date
																													) csmme
																													LEFT OUTER JOIN (
																														SELECT
																															cst.compressor_seal_id,
																															cst.start_date,
																															cst.end_date,
																															cst.rate,
																															cst.testing_point
																														FROM
																															compressor_seal_test_cleaned cst
																														WHERE
																															cst.testing_point = 'CRANKCASE'
																													) cst_ckc ON cst_ckc.compressor_seal_id = csmme.id
																													AND csmme.date BETWEEN cst_ckc.start_date
																													AND cst_ckc.end_date
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
																					) as gas_volume,
																					0 as gas_volume_crankcase
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
																												$1
																											) as min_start_date,
																											LEAST(
																												ces.end_date,
																												ces.compressor_end_date,
																												$2::date + INTERVAL '1 month - 1 day'
																											)::date as max_end_date
																										FROM
																											(
																												SELECT
																													ces.compressor_id,
																													ces.start_date,
																													COALESCE(ces.end_date, CURRENT_DATE) as end_date,
																													cpr.install_date as compressor_start_date,
																													COALESCE(cpr.remove_date, CURRENT_DATE) as compressor_end_date,
																													-- scf/hr to m3/hr
																													ces.rate / 35.3147 as rate,
																													ces.leak_duration
																												FROM
																													compressor_emission_survey ces
																													INNER JOIN compressor cpr ON cpr.id = ces.compressor_id
																													AND ces.start_date <= LEAST(
																														COALESCE(cpr.remove_date, CURRENT_DATE),
																														($2::date + INTERVAL '1 month - 1 day')
																													)
																													AND COALESCE(ces.end_date, CURRENT_DATE) >= GREATEST(cpr.install_date, $1)
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
			INNER JOIN compressor cpr ON cpr.id = csmme.id
			AND NOT (
				/*controlled emission from non-crankcase testing point is destroyed or recovered so it's filtered out here*/
				csmme.control_device IS NOT NULL
				AND csmme.reason IS NULL
				AND csmme.identity = 'NONCRANKCASE'
			)
	) csmme
GROUP BY
	csmme.facility_id,
	csmme.site_id,
	csmme.id,
	csmme.category,
	csmme.source,
	csmme.month