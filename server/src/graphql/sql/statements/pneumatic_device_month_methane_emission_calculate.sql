WITH allocate_month as (
	SELECT
		month_beginning::date
	FROM
		generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
	pdmme.facility_id,
	pdmme.site_id as "site_id!",
	'pneumatic_device'::methane_emission_source_table as "source_table!: _",
	pdmme.id as "source_table_id!",
	'ROUTINE'::methane_emission_category as "category!: _",
	'PNEUMATIC_DEVICE'::methane_emission_source as "source!: _",
	pdmme.month_beginning as "month!",
	SUM(pdmme.gas_volume * pdmme.percent) as "gas_volume!",
	SUM(pdmme.gas_volume * pdmme.c1 * pdmme.percent) as "c1_volume!",
	SUM(pdmme.gas_volume * pdmme.co2 * pdmme.percent) as "co2_volume!"
FROM
	(
		SELECT
			pdmme.facility_id,
			pdmme.site_id,
			pdmme.id,
			pdmme.month_beginning,
			pdmme.gas_volume,
			pdmme.c1,
			pdmme.co2,
			EXTRACT(
				DAY
				FROM
					(
						pdmme.to_date + INTERVAL '1 day' - pdmme.from_date
					)
			) / pdmme.days_in_period as percent
		FROM
			(
				SELECT
					s.facility_id,
					pdmme.site_id,
					pdmme.id,
					pdmme.month_beginning,
					pdmme.gas_volume,
					COALESCE(ga.c1, $3) as c1,
					COALESCE(ga.co2, $4) as co2,
					GREATEST(ga.from_date, pdmme.from_date) as from_date,
					LEAST(ga.to_date, pdmme.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(
								pdmme.to_date + INTERVAL '1 day' - pdmme.from_date
							)
					) as days_in_period
				FROM
					(
						SELECT
							pdmme.*
						FROM
							(
								SELECT
									pdmme.id,
									pdmme.site_id,
									pdmme.month_beginning,
									pdmme.from_date,
									pdmme.to_date,
									pdmme.hours_on * pdmme.rate * pdmme.percent as gas_volume
								FROM
									(
										SELECT
											pdmme.id,
											pdmme.site_id,
											pdmme.month_beginning,
											pdmme.from_date,
											pdmme.to_date,
											pdmme.hours_on,
											pdmme.rate,
											EXTRACT(
												DAY
												FROM
													(
														pdmme.to_date + INTERVAL '1 day' - pdmme.from_date
													)
											) / pdmme.days_in_month as percent
										FROM
											(
												SELECT
													pd.id,
													pd.site_id,
													pd.month_beginning,
													GREATEST(cc.from_date, pd.month_beginning) as from_date,
													LEAST(
														cc.to_date,
														pd.month_beginning + INTERVAL '1 month - 1 day'
													) as to_date,
													DATE_PART(
														'days',
														pd.month_beginning + INTERVAL '1 month - 1 day'
													) as days_in_month,
													COALESCE(cmh.hours_on, 0) hours_on,
													COALESCE(cc.rate, 0) rate
												FROM
													(
														SELECT
															pd.id,
															pd.site_id,
															am.month_beginning
														FROM
															pneumatic_device pd
															INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', pd.start_date)
															AND COALESCE(pd.end_date, CURRENT_DATE)
															AND (pd.id, am.month_beginning) NOT IN (
																SELECT
																	pdmmeo.pneumatic_device_id,
																	pdmmeo.month
																FROM
																	pneumatic_device_month_methane_emission_override pdmmeo
																	INNER JOIN allocate_month am ON am.month_beginning = pdmmeo.month
															)
													) pd
													LEFT OUTER JOIN pneumatic_device_month_hours cmh ON cmh.pneumatic_device_id = pd.id
													AND cmh.month = pd.month_beginning
													LEFT OUTER JOIN (
														SELECT
															pneumatic_device_id,
															DATE_TRUNC('month', date) as month_join_beginning,
															DATE_TRUNC(
																'month',
																COALESCE(
																	LEAD(date) OVER (
																		PARTITION BY pneumatic_device_id
																		ORDER BY
																			date
																	) - INTERVAL '1 day',
																	CURRENT_DATE
																)
															) + INTERVAL '1 month - 1 day' as month_join_end,
															-- If first device change, from_date has to be first of the month because there is no carryover from previous change.
															CASE
																WHEN ROW_NUMBER() OVER (
																	PARTITION BY pneumatic_device_id
																	ORDER BY
																		date
																) = 1 THEN DATE_TRUNC('month', date)::date
																ELSE date
															END as from_date,
															COALESCE(
																LEAD(date) OVER (
																	PARTITION BY pneumatic_device_id
																	ORDER BY
																		date
																) - INTERVAL '1 day',
																CURRENT_DATE
															) as to_date,
															rate
														FROM
															pneumatic_device_change
													) cc ON cc.pneumatic_device_id = pd.id
													AND pd.month_beginning BETWEEN cc.month_join_beginning
													AND cc.month_join_end
											) pdmme
									) pdmme
							) pdmme
						UNION
						ALL
						SELECT
							pdmme.*
						FROM
							(
								SELECT
									pd.id,
									pd.site_id,
									pdmmeo.month as month_beginning,
									pdmmeo.month as from_date,
									pdmmeo.month + INTERVAL '1 month - 1 day' as to_date,
									pdmmeo.gas_volume
								FROM
									pneumatic_device_month_methane_emission_override pdmmeo
									INNER JOIN pneumatic_device pd ON pd.id = pdmmeo.pneumatic_device_id
									AND pdmmeo.month BETWEEN DATE_TRUNC('month', pd.start_date)
									AND COALESCE(pd.end_date, CURRENT_DATE)
									AND pdmmeo.month IN (
										SELECT
											month_beginning
										FROM
											allocate_month
									)
							) pdmme
					) pdmme
					LEFT OUTER JOIN site s ON s.id = pdmme.site_id
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
					AND ga.from_date <= pdmme.to_date
					AND ga.to_date >= pdmme.from_date
			) pdmme
	) pdmme
GROUP BY
	pdmme.facility_id,
	pdmme.site_id,
	pdmme.id,
	pdmme.month_beginning