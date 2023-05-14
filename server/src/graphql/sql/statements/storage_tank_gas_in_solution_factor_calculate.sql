SELECT
	stgisf.id as "storage_tank_id!",
	stgisf.from_date as "date!",
	stgisf.coeff_1 * stgisf.gas_gravity * POWER(
		/*kPa to psi*/
		stgisf.pressure / 6.89475729316836,
		stgisf.coeff_2
	) * EXP(
		coeff_3 * stgisf.api_density /(
			/*Celsius to Rankine*/
			stgisf.temperature * 1.8 + 491.67
		)
	) * (
		/*scf/bbl to m3/m3*/
		6.2898 / 35.3147
	) as "gis_factor!"
FROM
	(
		SELECT
			stc.id,
			GREATEST(stc.from_date, ga.from_date) as from_date,
			LEAST(stc.to_date, ga.to_date) as to_date,
			stc.api_density,
			stc.temperature,
			stc.pressure,
			COALESCE(gacp.gas_gravity, $3) as gas_gravity,
			CASE
				WHEN stc.api_density > 30 THEN 0.0178
				ELSE 0.0362
			END as coeff_1,
			CASE
				WHEN stc.api_density > 30 THEN 1.187
				ELSE 1.0937
			END as coeff_2,
			CASE
				WHEN stc.api_density > 30 THEN 23.931
				ELSE 25.724
			END as coeff_3
		FROM
			(
				SELECT
					st.id,
					s.facility_id,
					COALESCE(stc.from_date, st.start_date) as from_date,
					COALESCE(stc.to_date, st.end_date, CURRENT_DATE) as to_date,
					COALESCE(stc.api_density, 36.56) as api_density,
					COALESCE(stc.temperature, 45) as temperature,
					COALESCE(stc.pressure, 313) as pressure
				FROM
					(
						SELECT
							id,
							site_id,
							start_date,
							COALESCE(end_date, CURRENT_DATE) as end_date
						FROM
							storage_tank
						WHERE
							start_date <= $2
							AND COALESCE(end_date, CURRENT_DATE) >= $1
					) st
					LEFT OUTER JOIN site s ON s.id = st.site_id
					LEFT OUTER JOIN (
						SELECT
							storage_tank_id,
							date as from_date,
							COALESCE(
								LEAD(date) OVER (
									PARTITION BY storage_tank_id
									ORDER BY
										date
								) - INTERVAL '1 day',
								CURRENT_DATE
							)::date as to_date,
							api_density,
							temperature,
							pressure
						FROM
							storage_tank_change
					) stc ON stc.storage_tank_id = st.id
					AND stc.from_date <= st.end_date
					AND stc.to_date >= st.start_date
			) stc
			LEFT OUTER JOIN (
				SELECT
					id,
					facility_id,
					date as from_date,
					COALESCE(
						LEAD(date) OVER (
							PARTITION BY facility_id
							ORDER BY
								date
						) - INTERVAL '1 day',
						CURRENT_DATE
					)::date as to_date
				FROM
					gas_analysis
			) ga ON ga.facility_id = stc.facility_id
			AND ga.from_date <= stc.to_date
			AND ga.to_date >= stc.from_date
			LEFT OUTER JOIN gas_analysis_calculated_param gacp ON gacp.id = ga.id
	) stgisf
WHERE
	stgisf.from_date <= $2
	AND stgisf.to_date >= $1