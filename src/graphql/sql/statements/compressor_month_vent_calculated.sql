WITH allocate_month (month_beginning) as (
	VALUES
		($1::date)
)
SELECT
	cmv.id as compressor_id,
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
					cmv.gas_volume + cmv.blowdown_gas_volume as gas_volume,
					COALESCE(ga.c1, $2) as c1,
					COALESCE(ga.co2, $3) as co2,
					GREATEST(ga.date, cmv.from_date) as from_date,
					LEAST(ga.date_end, cmv.to_date) as to_date,
					EXTRACT(
						DAY
						FROM
							(cmv.to_date + INTERVAL '1 day' - cmv.from_date)
					) as days_in_period
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
											GREATEST(cc.date, cmpr.month_beginning) as from_date,
											LEAST(
												cc.date_end,
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
													*
												FROM
													compressors,
													allocate_month
											) cmpr
											LEFT OUTER JOIN compressor_month_hours cmh ON cmh.compressor_id = cmpr.id
											AND cmh.month = cmpr.month_beginning
											LEFT OUTER JOIN (
												SELECT
													compressor_id,
													DATE_TRUNC('month', date) month_join_beginning,
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
													) + INTERVAL '1 month - 1 day' month_join_end,
													date,
													COALESCE(
														LEAD(date) OVER (
															PARTITION BY compressor_id
															ORDER BY
																date
														) - INTERVAL '1 day',
														CURRENT_DATE
													) date_end,
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
					LEFT OUTER JOIN (
						SELECT
							facility_id,
							date,
							COALESCE(
								LEAD(date) OVER (
									PARTITION BY facility_id
									ORDER BY
										date
								) - INTERVAL '1 day',
								CURRENT_DATE
							) date_end,
							c1,
							co2
						FROM
							gas_analyses
					) ga ON ga.facility_id = cmv.facility_id
					AND ga.date <= cmv.to_date
					AND ga.date_end >= cmv.from_date
				ORDER BY
					cmv.from_date,
					ga.date
			) cmv
	) cmv
GROUP BY
	cmv.id,
	cmv.month_beginning