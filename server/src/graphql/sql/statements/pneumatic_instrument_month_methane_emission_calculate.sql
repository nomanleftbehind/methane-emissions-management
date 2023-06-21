WITH allocate_month as (
	SELECT
		month_beginning::date
	FROM
		generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
	nlcmme.facility_id,
	nlcmme.site_id as "site_id!",
	'pneumatic_instrument'::methane_emission_source_table as "source_table!: _",
	nlcmme.id as "source_table_id!",
	'ROUTINE'::methane_emission_category as "category!: _",
	'PNEUMATIC_DEVICE'::methane_emission_source as "source!: _",
	nlcmme.month_beginning as "month!",
	SUM(nlcmme.gas_volume * nlcmme.percent) as "gas_volume!",
	SUM(nlcmme.gas_volume * nlcmme.c1 * nlcmme.percent) as "c1_volume!",
	SUM(nlcmme.gas_volume * nlcmme.co2 * nlcmme.percent) as "co2_volume!"
FROM
	(
		SELECT
			nlcmme.facility_id,
			nlcmme.site_id,
			nlcmme.id,
			nlcmme.month_beginning,
			nlcmme.gas_volume,
			nlcmme.c1,
			nlcmme.co2,
			EXTRACT(
				DAY
				FROM
					(
						nlcmme.to_date + INTERVAL '1 day' - nlcmme.from_date
					)
			) / nlcmme.days_in_period as percent
		FROM
			(
				SELECT
					s.facility_id,
					nlcmme.site_id,
					nlcmme.id,
					nlcmme.month_beginning,
					nlcmme.gas_volume,
					COALESCE(ga.c1, $3) as c1,
					COALESCE(ga.co2, $4) as co2,
					GREATEST(ga.from_date, nlcmme.from_date) as from_date,
					LEAST(ga.to_date, nlcmme.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(
								nlcmme.to_date + INTERVAL '1 day' - nlcmme.from_date
							)
					) as days_in_period
				FROM
					(
						SELECT
							nlcmme.*
						FROM
							(
								SELECT
									nlcmme.id,
									nlcmme.site_id,
									nlcmme.month_beginning,
									nlcmme.from_date,
									nlcmme.to_date,
									nlcmme.hours_on * nlcmme.rate * nlcmme.percent as gas_volume
								FROM
									(
										SELECT
											nlcmme.id,
											nlcmme.site_id,
											nlcmme.month_beginning,
											nlcmme.from_date,
											nlcmme.to_date,
											nlcmme.hours_on,
											nlcmme.rate,
											EXTRACT(
												DAY
												FROM
													(
														nlcmme.to_date + INTERVAL '1 day' - nlcmme.from_date
													)
											) / nlcmme.days_in_month as percent
										FROM
											(
												SELECT
													nlc.id,
													nlc.site_id,
													nlc.month_beginning,
													GREATEST(nlcc.from_date, nlc.month_beginning) as from_date,
													LEAST(
														nlcc.to_date,
														nlc.month_beginning + INTERVAL '1 month - 1 day'
													) as to_date,
													DATE_PART(
														'days',
														nlc.month_beginning + INTERVAL '1 month - 1 day'
													) as days_in_month,
													COALESCE(cmh.hours_on, 0) as hours_on,
													COALESCE(nlcc.rate, 0) as rate
												FROM
													(
														SELECT
															nlc.id,
															nlc.site_id,
															am.month_beginning
														FROM
															pneumatic_instrument nlc
															INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', nlc.start_date)
															AND COALESCE(nlc.end_date, CURRENT_DATE)
															AND (nlc.id, am.month_beginning) NOT IN (
																SELECT
																	nlcmmeo.pneumatic_instrument_id,
																	nlcmmeo.month
																FROM
																	pneumatic_instrument_month_methane_emission_override nlcmmeo
																	INNER JOIN allocate_month am ON am.month_beginning = nlcmmeo.month
															)
													) nlc
													LEFT OUTER JOIN pneumatic_instrument_month_hours cmh ON cmh.pneumatic_instrument_id = nlc.id
													AND cmh.month = nlc.month_beginning
													LEFT OUTER JOIN (
														SELECT
															pneumatic_instrument_id,
															DATE_TRUNC('month', date) as month_join_beginning,
															DATE_TRUNC(
																'month',
																COALESCE(
																	LEAD(date) OVER (
																		PARTITION BY pneumatic_instrument_id
																		ORDER BY
																			date
																	) - INTERVAL '1 day',
																	CURRENT_DATE
																)
															) + INTERVAL '1 month - 1 day' as month_join_end,
															-- If first device change, from_date has to be first of the month because there is no carryover from previous change.
															CASE
																WHEN ROW_NUMBER() OVER (
																	PARTITION BY pneumatic_instrument_id
																	ORDER BY
																		date
																) = 1 THEN DATE_TRUNC('month', date)::date
																ELSE date
															END as from_date,
															COALESCE(
																LEAD(date) OVER (
																	PARTITION BY pneumatic_instrument_id
																	ORDER BY
																		date
																) - INTERVAL '1 day',
																CURRENT_DATE
															) as to_date,
															rate
														FROM
															pneumatic_instrument_change
													) nlcc ON nlcc.pneumatic_instrument_id = nlc.id
													AND nlc.month_beginning BETWEEN nlcc.month_join_beginning
													AND nlcc.month_join_end
											) nlcmme
									) nlcmme
							) nlcmme
						UNION
						ALL
						SELECT
							nlcmme.*
						FROM
							(
								SELECT
									nlc.id,
									nlc.site_id,
									nlcmmeo.month as month_beginning,
									nlcmmeo.month as from_date,
									nlcmmeo.month + INTERVAL '1 month - 1 day' as to_date,
									nlcmmeo.gas_volume
								FROM
									pneumatic_instrument_month_methane_emission_override nlcmmeo
									INNER JOIN pneumatic_instrument nlc ON nlc.id = nlcmmeo.pneumatic_instrument_id
									AND nlcmmeo.month BETWEEN DATE_TRUNC('month', nlc.start_date)
									AND COALESCE(nlc.end_date, CURRENT_DATE)
									AND nlcmmeo.month IN (
										SELECT
											month_beginning
										FROM
											allocate_month
									)
							) nlcmme
					) nlcmme
					LEFT OUTER JOIN site s ON s.id = nlcmme.site_id
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
					AND ga.from_date <= nlcmme.to_date
					AND ga.to_date >= nlcmme.from_date
			) nlcmme
	) nlcmme
GROUP BY
	nlcmme.facility_id,
	nlcmme.site_id,
	nlcmme.id,
	nlcmme.month_beginning