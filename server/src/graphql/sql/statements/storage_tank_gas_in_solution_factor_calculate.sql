SELECT
	tfvf.id as "storage_tank_id!",
	tfvf.from_date as "date!",
	tfvf.coeff_1 * tfvf.gas_gravity * POWER(
		/*kPa to psi*/
		tfvf.pressure / 6.89475729316836,
		tfvf.coeff_2
	) * EXP(
		coeff_3 * tfvf.api_density /(
			/*Celsius to Rankine*/
			tfvf.temperature * 1.8 + 491.67
		)
	) *(
		/*scf/bbl to m3/m3*/
		6.2898 / 35.3147
	) as "gis_factor!"
FROM
	(
		SELECT
			tf.id,
			GREATEST(tfc.from_date, ga.from_date) as from_date,
			LEAST(tfc.to_date, ga.to_date) as to_date,
			tfc.api_density,
			tfc.temperature,
			tfc.pressure,
			tfc.ia,
			COALESCE(gacp.gas_gravity, $1) as gas_gravity,
			CASE
				WHEN tfc.api_density > 30 THEN 0.0178
				ELSE 0.0362
			END as coeff_1,
			CASE
				WHEN tfc.api_density > 30 THEN 1.187
				ELSE 1.0937
			END as coeff_2,
			CASE
				WHEN tfc.api_density > 30 THEN 23.931
				ELSE 25.724
			END as coeff_3
		FROM
			storage_tank tf
			LEFT OUTER JOIN site s ON s.id = tf.site_id
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
					) as to_date,
					ia,
					api_density,
					temperature,
					pressure
				FROM
					storage_tank_change
			) tfc ON tfc.storage_tank_id = tf.id
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
					) as to_date
				FROM
					gas_analysis
			) ga ON ga.facility_id = s.facility_id
			AND ga.from_date <= tfc.to_date
			AND ga.to_date >= tfc.from_date
			LEFT OUTER JOIN gas_analysis_calculated_param gacp ON gacp.id = ga.id
	) tfvf