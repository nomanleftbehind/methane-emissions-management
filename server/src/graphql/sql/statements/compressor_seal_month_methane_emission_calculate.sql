WITH allocate_month as (
	SELECT
		month_beginning::date
	FROM
		generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
	csmme.facility_id,
	csmme.site_id as "site_id!",
	'compressor_seal'::methane_emission_source as "source!: _",
	csmme.id as "source_id!",
	'ROUTINE'::methane_emission_category as "category!: _",
	csmme.month_beginning as "month!",
	SUM(csmme.gas_volume * csmme.percent) as "gas_volume!",
	SUM(csmme.gas_volume * csmme.c1 * csmme.percent) as "c1_volume!",
	SUM(csmme.gas_volume * csmme.co2 * csmme.percent) as "co2_volume!"
FROM
	(
		SELECT
			csmme.facility_id,
			csmme.site_id,
			csmme.id,
			csmme.month_beginning,
			csmme.gas_volume,
			csmme.c1,
			csmme.co2,
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
					COALESCE(ga.c1, $3) as c1,
					COALESCE(ga.co2, $4) as co2,
					GREATEST(ga.from_date, csmme.from_date) as from_date,
					LEAST(ga.to_date, csmme.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(csmme.to_date + INTERVAL '1 day' - csmme.from_date)
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
									csmme.hours_on * csmme.rate * csmme.percent as gas_volume
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
											EXTRACT(
												DAY
												FROM
													(csmme.to_date + INTERVAL '1 day' - csmme.from_date)
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
													COALESCE(cst.rate, 0) rate
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
																		PARTITION BY compressor_seal_id
																		ORDER BY
																			date
																	) - INTERVAL '1 day',
																	CURRENT_DATE
																)
															) + INTERVAL '1 month - 1 day' as month_join_end,
															-- If first seal test, from_date has to be first of the month because there is no carryover from previous test.
															CASE
																WHEN ROW_NUMBER() OVER (
																	PARTITION BY compressor_seal_id
																	ORDER BY
																		date
																) = 1 THEN DATE_TRUNC('month', date)::date
																ELSE date
															END as from_date,
															COALESCE(
																LEAD(date) OVER (
																	PARTITION BY compressor_seal_id
																	ORDER BY
																		date
																) - INTERVAL '1 day',
																CURRENT_DATE
															) as to_date,
															rate
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
									csmmeo.gas_volume
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
					LEFT OUTER JOIN site s ON s.id = csmme.site_id
					LEFT OUTER JOIN (
						SELECT
							facility_id,
							date as from_date,
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
	csmme.month_beginning