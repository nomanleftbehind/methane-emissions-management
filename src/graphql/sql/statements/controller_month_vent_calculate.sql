WITH allocate_month as (
	SELECT
		*
	FROM
		UNNEST($1::date[]) as month_beginning
)
SELECT
	cmv.id as "controller_id!",
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
									cmv.hours_on * cmv.rate * cmv.percent as gas_volume
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
													ctr.id,
													ctr.facility_id,
													ctr.month_beginning,
													GREATEST(cc.date, ctr.month_beginning) as from_date,
													LEAST(
														cc.date_end,
														ctr.month_beginning + INTERVAL '1 month - 1 day'
													) as to_date,
													DATE_PART(
														'days',
														ctr.month_beginning + INTERVAL '1 month - 1 day'
													) as days_in_month,
													COALESCE(cmh.hours_on, 0) hours_on,
													COALESCE(cc.rate, 0) rate
												FROM
													(
														SELECT
															id,
															facility_id,
															month_beginning
														FROM
															controllers,
															allocate_month
														WHERE
															(id, month_beginning) NOT IN (
																SELECT
																	cmvo.controller_id,
																	cmvo.month
																FROM
																	controller_month_vent_override cmvo
																	INNER JOIN allocate_month am ON am.month_beginning = cmvo.month
															)
													) ctr
													LEFT OUTER JOIN controller_month_hours cmh ON cmh.controller_id = ctr.id
													AND cmh.month = ctr.month_beginning
													LEFT OUTER JOIN (
														SELECT
															controller_id,
															DATE_TRUNC('month', date) month_join_beginning,
															DATE_TRUNC(
																'month',
																COALESCE(
																	LEAD(date) OVER (
																		PARTITION BY controller_id
																		ORDER BY
																			date
																	) - INTERVAL '1 day',
																	CURRENT_DATE
																)
															) + INTERVAL '1 month - 1 day' month_join_end,
															date,
															COALESCE(
																LEAD(date) OVER (
																	PARTITION BY controller_id
																	ORDER BY
																		date
																) - INTERVAL '1 day',
																CURRENT_DATE
															) date_end,
															rate
														FROM
															controller_changes
													) cc ON cc.controller_id = ctr.id
													AND ctr.month_beginning BETWEEN cc.month_join_beginning
													AND cc.month_join_end
											) cmv
									) cmv
							) cmv
						UNION
						ALL
						SELECT
							cmv.*
						FROM
							(
								SELECT
									ctr.id,
									ctr.facility_id,
									cmvo.month as month_beginning,
									cmvo.month as from_date,
									cmvo.month + INTERVAL '1 month - 1 day' as to_date,
									cmvo.gas_volume
								FROM
									controller_month_vent_override cmvo
									INNER JOIN controllers ctr ON ctr.id = cmvo.controller_id
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