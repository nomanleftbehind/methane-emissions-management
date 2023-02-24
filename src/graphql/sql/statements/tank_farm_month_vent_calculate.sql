WITH allocate_month (month_beginning) as (
	VALUES
		('2022-06-01'::date)
)

SELECT
	tmv.id as tank_farm_id,
	tmv.month_beginning as "month!",
	SUM(tmv.gas_volume * tmv.percent) as "gas_volume!",
	SUM(tmv.gas_volume * tmv.c1 * tmv.percent) as "c1_volume!",
	SUM(tmv.gas_volume * tmv.co2 * tmv.percent) as "co2_volume!"
FROM
	(
		SELECT
			tmv.id,
			tmv.month_beginning,
			tmv.gas_volume,
			tmv.c1,
			tmv.co2,
			EXTRACT(
				DAY
				FROM
					(
						tmv.to_date + INTERVAL '1 day' - tmv.from_date
					)
			) / tmv.days_in_period as percent
		FROM
			(
				SELECT
					tmv.id,
					tmv.month_beginning,
					tmv.oil * tmv.vent_factor * tmv.percent as gas_volume,
					COALESCE(ga.c1, 0.82) as c1,
					COALESCE(ga.co2, 0.0067) as co2,
					GREATEST(ga.date, tmv.from_date) as from_date,
					LEAST(ga.date_end, tmv.to_date) as to_date,
					EXTRACT(DAY FROM (tmv.to_date + INTERVAL '1 day' - tmv.from_date)) as days_in_period
				FROM
					(
						SELECT
							tmv.id,
							tmv.facility_id,
							tmv.month_beginning,
							tmv.from_date,
							tmv.to_date,
							tmv.oil,
							tmv.vent_factor,
							EXTRACT(DAY FROM (tmv.to_date + INTERVAL '1 day' - tmv.from_date)) / tmv.days_in_month as percent
						FROM
							(
								SELECT
									tf.id,
									tf.facility_id,
									tf.month_beginning,
									GREATEST(tfvf.from_date, tf.month_beginning) as from_date,
									LEAST(tfvf.to_date, tf.month_beginning + INTERVAL '1 month - 1 day') as to_date,
									DATE_PART('days', tf.month_beginning + INTERVAL '1 month - 1 day') as days_in_month,
									COALESCE(tfmof.oil, 0) as oil,
									COALESCE(tfvf.vent_factor, 0) as vent_factor
								FROM
									(
										SELECT
										id,
										facility_id,
										month_beginning
										
										FROM tank_farms, allocate_month) tf
									LEFT OUTER JOIN tank_farm_month_oil_flow tfmof ON tfmof.tank_farm_id = tf.id AND tfmof.month = tf.month_beginning
									LEFT OUTER JOIN (
										SELECT
										tank_farm_id,
										DATE_TRUNC('month', date) month_join_beginning,
										DATE_TRUNC('month', COALESCE(LEAD(date) OVER (PARTITION BY tank_farm_id ORDER BY date) - INTERVAL '1 day', CURRENT_DATE)) + INTERVAL '1 month - 1 day' month_join_end,
										date as from_date,
										COALESCE(LEAD(date) OVER (PARTITION BY tank_farm_id ORDER BY date) - INTERVAL '1 day', CURRENT_DATE) as to_date,
										vent_factor
										FROM tank_farm_vent_factors_calculated
									) tfvf ON tfvf.tank_farm_id = tf.id	AND tf.month_beginning BETWEEN tfvf.month_join_beginning AND tfvf.month_join_end
							) tmv
					) tmv
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
					) ga ON ga.facility_id = tmv.facility_id
					AND ga.date <= tmv.to_date
					AND ga.date_end >= tmv.from_date
				ORDER BY
					tmv.from_date,
					ga.date
			) tmv
	) tmv
GROUP BY
	tmv.id,
	tmv.month_beginning