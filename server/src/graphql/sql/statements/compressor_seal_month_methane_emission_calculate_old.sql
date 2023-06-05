WITH allocate_month as (
	SELECT
		month_beginning::date
	FROM
		generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
	csmme.facility_id,
	csmme.site_id as "site_id!",
	'compressor_seal'::methane_emission_source_table as "source_table!: _",
	csmme.id as "source_table_id!",
	csmme.category as "category!: _",
	csmme.source as "source!: _",
	csmme.month_beginning as "month!",
	SUM(csmme.gas_volume) as "gas_volume!",
	SUM(csmme.c1_volume) as "c1_volume!",
	SUM(csmme.co2_volume) as "co2_volume!"
FROM
	(
		SELECT
			csmme.facility_id,
			csmme.site_id,
			csmme.id,
			CASE
				WHEN csmme.controlled_characterization = 'UNCONTROLLED' THEN 'ROUTINE'::methane_emission_category
				ELSE CASE
					WHEN cpr.type = 'RECIPROCATING'
					AND csmme.testing_point = 'CRANKCASE' THEN -- AER Directive 060 section 8.6.2.1 and AER Manual 015 section 4.5
					'FUGITIVE'::methane_emission_category
					ELSE 'NONROUTINE'::methane_emission_category
				END
			END as category,
			CASE
				WHEN csmme.controlled_characterization = 'UNCONTROLLED' THEN CASE
					WHEN cpr.type IN ('RECIPROCATING', 'CENTRIFUGAL') THEN 'COMPRESSOR_SEAL'::methane_emission_source
					ELSE -- AER Manual 015 section 4.5
					'DEFINED_VENT_GAS'::methane_emission_source
				END
				ELSE CASE
					WHEN cpr.type = 'RECIPROCATING'
					AND csmme.testing_point = 'CRANKCASE' THEN -- AER Directive 060 section 8.6.2.1 and AER Manual 015 section 4.5
					'FUGITIVE'::methane_emission_source
					ELSE 'PLANNED'::methane_emission_source
				END
			END as source,
			csmme.month_beginning,
			csmme.gas_volume,
			csmme.c1_volume,
			csmme.co2_volume
		FROM
			(
				SELECT
					csmme.facility_id,
					csmme.site_id,
					csmme.id,
					csmme.month_beginning,
					csmme.testing_point,
					csmme.controlled_characterization,
					SUM(csmme.gas_volume * csmme.percent) as gas_volume,
					SUM(csmme.c1_volume * csmme.percent) as c1_volume,
					SUM(csmme.gas_volume * csmme.co2 * csmme.percent) as co2_volume
				FROM
					(
						SELECT
							csmme.facility_id,
							csmme.site_id,
							csmme.id,
							csmme.month_beginning,
							csmme.from_date,
							csmme.to_date,
							COALESCE(csmme.gas_volume, csmme.c1_volume / csmme.c1) as gas_volume,
							COALESCE(csmme.c1_volume, csmme.gas_volume * csmme.c1) as c1_volume,
							csmme.co2,
							csmme.testing_point,
							csmme.controlled_characterization,
							EXTRACT(
								DAY
								FROM
									(
										csmme.to_date + INTERVAL '1 day' - csmme.from_date
									)
							) / csmme.days_in_period as percent
						FROM
							(
								SELECT
									s.facility_id,
									csmme.site_id,
									csmme.id,
									csmme.month_beginning,
									csmme.gas_volume,
									csmme.c1_volume,
									COALESCE(ga.c1, $3) as c1,
									COALESCE(ga.co2, $4) as co2,
									csmme.testing_point,
									csmme.controlled_characterization,
									GREATEST(ga.from_date, csmme.from_date) as from_date,
									LEAST(ga.to_date, csmme.to_date) as to_date,
									EXTRACT(
										DAY
										FROM
											(
												csmme.to_date + INTERVAL '1 day' - csmme.from_date
											)
									) as days_in_period
								FROM
									(
										SELECT
											csmme.id,
											csmme.site_id,
											csmme.month_beginning,
											csmme.from_date,
											csmme.to_date,
											csmme.gas_volume * csmme.percent as gas_volume,
											csmme.c1_volume * csmme.percent as c1_volume,
											csmme.testing_point,
											csmme.controlled_characterization
										FROM
											(
												SELECT
													csmme.id,
													csmme.site_id,
													csmme.month_beginning,
													csmme.from_date,
													csmme.to_date,
													csmme.gas_volume,
													csmme.c1_volume,
													csmme.testing_point,
													csmme.controlled_characterization,
													EXTRACT(
														DAY
														FROM
															(
																csmme.to_date + INTERVAL '1 day' - csmme.from_date
															)
													) / csmme.days_in_period as percent
												FROM
													(
														SELECT
															csmme.id,
															csmme.site_id,
															csmme.month_beginning,
															csmme.gas_volume,
															csmme.c1_volume,
															csmme.testing_point,
															COALESCE(ccc.controlled_characterization, 'UNCONTROLLED') as controlled_characterization,
															GREATEST(ccc.from_date, csmme.from_date) as from_date,
															LEAST(ccc.to_date, csmme.to_date) as to_date,
															EXTRACT(
																DAY
																FROM
																	(
																		csmme.to_date + INTERVAL '1 day' - csmme.from_date
																	)
															) as days_in_period
														FROM
															(
																SELECT
																	csmme.*
																FROM
																	(
																		SELECT
																			csmme.id,
																			csmme.site_id,
																			csmme.month_beginning,
																			csmme.from_date,
																			csmme.to_date,
																			NULL::double precision as gas_volume,
																			csmme.hours_on * csmme.rate * csmme.percent as c1_volume,
																			csmme.testing_point
																		FROM
																			(
																				SELECT
																					csmme.id,
																					csmme.site_id,
																					csmme.month_beginning,
																					csmme.from_date,
																					csmme.to_date,
																					csmme.hours_on,
																					csmme.rate,
																					csmme.testing_point,
																					EXTRACT(
																						DAY
																						FROM
																							(
																								csmme.to_date + INTERVAL '1 day' - csmme.from_date
																							)
																					) / csmme.days_in_month as percent
																				FROM
																					(
																						SELECT
																							cmpr.id,
																							cmpr.site_id,
																							cmpr.month_beginning,
																							GREATEST(cst.from_date, cmpr.month_beginning) as from_date,
																							LEAST(
																								cst.to_date,
																								cmpr.month_beginning + INTERVAL '1 month - 1 day'
																							) as to_date,
																							DATE_PART(
																								'days',
																								cmpr.month_beginning + INTERVAL '1 month - 1 day'
																							) as days_in_month,
																							COALESCE(cmh.pressurized_hours, 0) hours_on,
																							COALESCE(cst.rate, 0) rate,
																							cst.testing_point
																						FROM
																							(
																								SELECT
																									cs.id,
																									cpr.site_id,
																									am.month_beginning
																								FROM
																									compressor_seal cs -- Compressor seal and compressor share same id, however compressor is joined here because it contains site_id, install_date and remove_date columns, which are used subsequently.
																									LEFT OUTER JOIN compressor cpr ON cpr.id = cs.id -- Despite sharing same id, existence of compressor doesn't guarantee existence of compressor seal, hence why joining allocation months on compressor is not done here.
																									INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', cpr.install_date)
																									AND COALESCE(cpr.remove_date, CURRENT_DATE)
																									AND (cs.id, am.month_beginning) NOT IN (
																										SELECT
																											csmmeo.compressor_seal_id,
																											csmmeo.month
																										FROM
																											compressor_seal_month_methane_emission_override csmmeo
																											INNER JOIN allocate_month am ON am.month_beginning = csmmeo.month
																									)
																							) cmpr
																							LEFT OUTER JOIN compressor_month_hours cmh ON cmh.compressor_id = cmpr.id
																							AND cmh.month = cmpr.month_beginning
																							LEFT OUTER JOIN (
																								SELECT
																									compressor_seal_id,
																									DATE_TRUNC('month', date) as month_join_beginning,
																									DATE_TRUNC(
																										'month',
																										COALESCE(
																											LEAD(date) OVER (
																												PARTITION BY compressor_seal_id,
																												testing_point
																												ORDER BY
																													date
																											) - INTERVAL '1 day',
																											CURRENT_DATE
																										)
																									) + INTERVAL '1 month - 1 day' as month_join_end,
																									-- If first seal test, from_date has to be first of the month because there is no carryover from previous test.
																									CASE
																										WHEN ROW_NUMBER() OVER (
																											PARTITION BY compressor_seal_id,
																											testing_point
																											ORDER BY
																												date
																										) = 1 THEN DATE_TRUNC('month', date)::date
																										ELSE date
																									END as from_date,
																									COALESCE(
																										LEAD(date) OVER (
																											PARTITION BY compressor_seal_id,
																											testing_point
																											ORDER BY
																												date
																										) - INTERVAL '1 day',
																										CURRENT_DATE
																									) as to_date,
																									rate,
																									testing_point
																								FROM
																									compressor_seal_test
																							) cst ON cst.compressor_seal_id = cmpr.id
																							AND cmpr.month_beginning BETWEEN cst.month_join_beginning
																							AND cst.month_join_end
																					) csmme
																			) csmme
																	) csmme
																UNION
																ALL
																SELECT
																	csmme.*
																FROM
																	(
																		SELECT
																			cpr.id,
																			cpr.site_id,
																			csmmeo.month as month_beginning,
																			csmmeo.month as from_date,
																			csmmeo.month + INTERVAL '1 month - 1 day' as to_date,
																			csmmeo.gas_volume,
																			NULL::double precision as c1_volume,
																			NULL::compressor_seal_testing_point as testing_point
																		FROM
																			compressor_seal_month_methane_emission_override csmmeo -- Unlike earlier example where override was joined on compressor seal, where it could have seemingly been joined on compressor, here it is joined on compressor because existence of override entry guarantees existence of compressor seal.
																			INNER JOIN compressor cpr ON cpr.id = csmmeo.compressor_seal_id
																			AND csmmeo.month BETWEEN DATE_TRUNC('month', cpr.install_date)
																			AND COALESCE(cpr.remove_date, CURRENT_DATE)
																			AND csmmeo.month IN (
																				SELECT
																					month_beginning
																				FROM
																					allocate_month
																			)
																	) csmme
															) csmme
															LEFT OUTER JOIN (
																SELECT
																	compressor_id,
																	-- If first controlled characterization, from_date has to be first of the month because there is no carryover from previous controlled characterization.
																	CASE
																		WHEN ROW_NUMBER() OVER (
																			PARTITION BY compressor_id
																			ORDER BY
																				date
																		) = 1 THEN DATE_TRUNC('month', date)::date
																		ELSE date
																	END as from_date,
																	COALESCE(
																		LEAD(date) OVER (
																			PARTITION BY compressor_id
																			ORDER BY
																				date
																		) - INTERVAL '1 day',
																		CURRENT_DATE
																	) as to_date,
																	controlled_characterization
																FROM
																	compressor_controlled_characterization
															) ccc ON ccc.compressor_id = csmme.id
															AND ccc.from_date <= csmme.to_date
															AND ccc.to_date >= csmme.from_date
													) csmme
											) csmme
									) csmme
									LEFT OUTER JOIN site s ON s.id = csmme.site_id
									LEFT OUTER JOIN (
										SELECT
											facility_id,
											-- If first gas analysis, from_date has to be first of the month because there is no carryover from previous analysis.
											CASE
												WHEN ROW_NUMBER() OVER (
													PARTITION BY facility_id
													ORDER BY
														date
												) = 1 THEN DATE_TRUNC('month', date)::date
												ELSE date
											END as from_date,
											COALESCE(
												LEAD(date) OVER (
													PARTITION BY facility_id
													ORDER BY
														date
												) - INTERVAL '1 day',
												CURRENT_DATE
											) as to_date,
											c1,
											co2
										FROM
											gas_analysis
									) ga ON ga.facility_id = s.facility_id
									AND ga.from_date <= csmme.to_date
									AND ga.to_date >= csmme.from_date
							) csmme
					) csmme
				GROUP BY
					csmme.facility_id,
					csmme.site_id,
					csmme.id,
					csmme.month_beginning,
					csmme.testing_point,
					csmme.controlled_characterization
			) csmme
			LEFT OUTER JOIN compressor cpr ON cpr.id = csmme.id
	) csmme
GROUP BY
	csmme.facility_id,
	csmme.site_id,
	csmme.id,
	csmme.category,
	csmme.source,
	csmme.month_beginning