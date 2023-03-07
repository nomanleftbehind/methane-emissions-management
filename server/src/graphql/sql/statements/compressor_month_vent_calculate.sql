WITH allocate_month as (
	SELECT
		*
	FROM
		UNNEST($1::date[]) as month_beginning
)
SELECT
	cmv.id as "compressor_id!",
	cmv.month_beginning as "month!",
	SUM(cmv.gas_volume * cmv.percent) as "gas_volume!",
	SUM(cmv.gas_volume * cmv.c1 * cmv.percent) as "c1_volume!",
	SUM(cmv.gas_volume * cmv.co2 * cmv.percent) as "co2_volume!"
FROM
	(
		SELECT
			cmv.id,
			cmv.month_beginning,
			cmv.gas_volume,
			cmv.c1,
			cmv.co2,
			EXTRACT(
				DAY
				FROM
					(
						cmv.to_date + INTERVAL '1 day' - cmv.from_date
					)
			) / cmv.days_in_period as percent
		FROM
			(
				SELECT
					cmv.id,
					cmv.month_beginning,
					cmv.gas_volume,
					COALESCE(ga.c1, $2) as c1,
					COALESCE(ga.co2, $3) as co2,
					GREATEST(ga.from_date, cmv.from_date) as from_date,
					LEAST(ga.to_date, cmv.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(cmv.to_date + INTERVAL '1 day' - cmv.from_date)
					) as days_in_period
				FROM
					(
						SELECT
							cmv.*
						FROM
							(
								SELECT
									cmv.id,
									cmv.facility_id,
									cmv.month_beginning,
									cmv.from_date,
									cmv.to_date,
									cmv.gas_volume + cmv.blowdown_gas_volume as gas_volume
								FROM
									(
										SELECT
											cmv.id,
											cmv.facility_id,
											cmv.month_beginning,
											cmv.from_date,
											cmv.to_date,
											cmv.hours_on * cmv.rate * cmv.percent as gas_volume,
											SUM(COALESCE(cb.gas_volume, 0)) as blowdown_gas_volume
										FROM
											(
												SELECT
													cmv.id,
													cmv.facility_id,
													cmv.month_beginning,
													cmv.from_date,
													cmv.to_date,
													cmv.hours_on,
													cmv.rate,
													EXTRACT(
														DAY
														FROM
															(cmv.to_date + INTERVAL '1 day' - cmv.from_date)
													) / cmv.days_in_month as percent
												FROM
													(
														SELECT
															cmpr.id,
															cmpr.facility_id,
															cmpr.month_beginning,
															GREATEST(cc.from_date, cmpr.month_beginning) as from_date,
															LEAST(
																cc.to_date,
																cmpr.month_beginning + INTERVAL '1 month - 1 day'
															) as to_date,
															DATE_PART(
																'days',
																cmpr.month_beginning + INTERVAL '1 month - 1 day'
															) as days_in_month,
															COALESCE(cmh.pressurized_hours, 0) hours_on,
															COALESCE(cc.rate, 0) rate
														FROM
															(
																SELECT
																	id,
																	facility_id,
																	month_beginning
																FROM
																	compressors,
																	allocate_month
																WHERE
																	(id, month_beginning) NOT IN (
																		SELECT
																			cmvo.compressor_id,
																			cmvo.month
																		FROM
																			compressor_month_vent_override cmvo
																			INNER JOIN allocate_month am ON am.month_beginning = cmvo.month
																	)
															) cmpr
															LEFT OUTER JOIN compressor_month_hours cmh ON cmh.compressor_id = cmpr.id
															AND cmh.month = cmpr.month_beginning
															LEFT OUTER JOIN (
																SELECT
																	compressor_id,
																	DATE_TRUNC('month', date) as month_join_beginning,
																	DATE_TRUNC(
																		'month',
																		COALESCE(
																			LEAD(date) OVER (
																				PARTITION BY compressor_id
																				ORDER BY
																					date
																			) - INTERVAL '1 day',
																			CURRENT_DATE
																		)
																	) + INTERVAL '1 month - 1 day' as month_join_end,
																	date as from_date,
																	COALESCE(
																		LEAD(date) OVER (
																			PARTITION BY compressor_id
																			ORDER BY
																				date
																		) - INTERVAL '1 day',
																		CURRENT_DATE
																	) as to_date,
																	rate
																FROM
																	compressor_changes
															) cc ON cc.compressor_id = cmpr.id
															AND cmpr.month_beginning BETWEEN cc.month_join_beginning
															AND cc.month_join_end
													) cmv
											) cmv
											LEFT OUTER JOIN compressor_blowdown cb ON cb.compressor_id = cmv.id
											AND cb.date BETWEEN cmv.from_date
											AND cmv.to_date
										GROUP BY
											cmv.id,
											cmv.facility_id,
											cmv.month_beginning,
											cmv.from_date,
											cmv.to_date,
											cmv.hours_on,
											cmv.rate,
											cmv.percent
									) cmv
							) cmv
						UNION
						ALL
						SELECT
							cmv.*
						FROM
							(
								SELECT
									cmpr.id,
									cmpr.facility_id,
									cmvo.month as month_beginning,
									cmvo.month as from_date,
									cmvo.month + INTERVAL '1 month - 1 day' as to_date,
									cmvo.gas_volume
								FROM
									compressor_month_vent_override cmvo
									INNER JOIN compressors cmpr ON cmpr.id = cmvo.compressor_id
									AND cmvo.month IN (
										SELECT
											*
										FROM
											allocate_month
									)
							) cmv
					) cmv
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
							gas_analyses
					) ga ON ga.facility_id = cmv.facility_id
					AND ga.from_date <= cmv.to_date
					AND ga.to_date >= cmv.from_date
			) cmv
	) cmv
GROUP BY
	cmv.id,
	cmv.month_beginning