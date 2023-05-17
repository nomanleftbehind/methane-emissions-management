WITH storage_tank_controlled_characterization_filtered as (
	SELECT
		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		stcc.end_date,
		stcc.control_device

	FROM
		storage_tank_controlled_characterization stcc
		INNER JOIN storage_tank st ON st.id = stcc.storage_tank_id
		AND st.start_date <= COALESCE(stcc.end_date, CURRENT_DATE)
		AND COALESCE(st.end_date, CURRENT_DATE) >= stcc.start_date
),

storage_tank_controlled_characterization_adjusted_dates as (
	SELECT 
		stcc.id,
		stcc.storage_tank_id,
		stcc.start_date,
		stcc.end_date,
		stcc.control_device,
		(stcc.end_date + INTERVAL '1 day')::date as start_date_fill,
		(LEAD(stcc.start_date) OVER (PARTITION BY stcc.storage_tank_id
									ORDER BY stcc.start_date
										) - INTERVAL '1 day')::date as end_date_fill
		
		
		FROM (
		SELECT
								stcc.id,
								stcc.storage_tank_id,
								stcc.start_date,
								GREATEST(LEAST(stcc.end_date, stcc.end_date_potential), stcc.start_date) as end_date,
								stcc.control_device

								FROM (
									
								SELECT
									
								stcc.*,
								COALESCE((LEAD(stcc.start_date) OVER (
											PARTITION BY stcc.storage_tank_id
											ORDER BY
												stcc.start_date
										) - INTERVAL '1 day')::date, CURRENT_DATE) as end_date_potential
									
								FROM (

								SELECT
									stcc.id,
									stcc.storage_tank_id,
									stcc.start_date,
									stcc.end_date,
									stcc.control_device

								FROM
									storage_tank_controlled_characterization_filtered stcc
								) stcc
								) stcc
								) stcc

		
	),
	
storage_tank_control_device_inactivity_adjusted_dates as (
	SELECT
	
	stcdi.id,
	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	(stcdi.end_date + INTERVAL '1 day')::date as start_date_fill,
	(LEAD(stcdi.start_date) OVER (PARTITION BY stcdi.storage_tank_controlled_characterization_id
									ORDER BY stcdi.start_date
										) - INTERVAL '1 day')::date as end_date_fill,
	stcdi.reason
	
	FROM (
	
	SELECT
	
	stcdi.id,
	stcdi.storage_tank_controlled_characterization_id,
	GREATEST(stcdi.start_date, stcc.start_date) as start_date,
	LEAST(stcdi.end_date, stcc.end_date) as end_date,
	stcdi.reason
	
	FROM storage_tank_controlled_characterization_adjusted_dates stcc
	INNER JOIN (
			SELECT
				stcdi.id,
				stcdi.storage_tank_controlled_characterization_id,
				stcdi.start_date,
				GREATEST(LEAST(stcdi.end_date, stcdi.end_date_potential), stcdi.start_date) as end_date,
				stcdi.reason

				FROM (

				SELECT
					stcdi.id,
					stcdi.storage_tank_controlled_characterization_id,
					stcdi.start_date,
					stcdi.end_date,
					stcdi.reason,
					COALESCE((LEAD(stcdi.start_date) OVER (
							PARTITION BY stcdi.storage_tank_controlled_characterization_id
							ORDER BY
								stcdi.start_date
						) - INTERVAL '1 day')::date, CURRENT_DATE) as end_date_potential

				FROM
					storage_tank_control_device_inactivity stcdi

				) stcdi
	) stcdi ON stcdi.storage_tank_controlled_characterization_id = stcc.id
		AND stcdi.start_date <= stcc.end_date
		AND stcdi.end_date >= stcc.start_date
		) stcdi
),

storage_tank_control_device_inactivity_filled_date_gaps as (

SELECT

stcdi.storage_tank_controlled_characterization_id,
stcdi.start_date,
stcdi.end_date,
stcdi.reason

FROM storage_tank_control_device_inactivity_adjusted_dates stcdi

UNION ALL

SELECT

stcdi.storage_tank_controlled_characterization_id,
stcdi.start_date_fill as start_date,
stcdi.end_date_fill as end_date,
NULL as reason

FROM storage_tank_control_device_inactivity_adjusted_dates stcdi

WHERE NOT stcdi.start_date_fill > stcdi.end_date_fill
	
UNION ALL
	
	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	NULL as reason

	FROM (

	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	MIN(stcc.start_date) as start_date,
	(MIN(stcdi.start_date) - INTERVAL '1 day')::date as end_date

	FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	INNER JOIN storage_tank_controlled_characterization_adjusted_dates stcc ON stcc.id = stcdi.storage_tank_controlled_characterization_id

	GROUP BY stcdi.storage_tank_controlled_characterization_id
		) stcdi
	WHERE (stcdi.storage_tank_controlled_characterization_id, stcdi.start_date) NOT IN (
		SELECT stcdi.storage_tank_controlled_characterization_id, stcdi.start_date
		FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	)
	
UNION ALL
	
	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	stcdi.start_date,
	stcdi.end_date,
	NULL as reason

	FROM (

	SELECT

	stcdi.storage_tank_controlled_characterization_id,
	(MAX(stcdi.end_date) + INTERVAL '1 day')::date as start_date,
	MAX(stcc.end_date) as end_date

	FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	INNER JOIN storage_tank_controlled_characterization_adjusted_dates stcc ON stcc.id = stcdi.storage_tank_controlled_characterization_id

	GROUP BY stcdi.storage_tank_controlled_characterization_id
		) stcdi
	WHERE (stcdi.storage_tank_controlled_characterization_id, stcdi.end_date) NOT IN (
		SELECT stcdi.storage_tank_controlled_characterization_id, stcdi.end_date
		FROM storage_tank_control_device_inactivity_adjusted_dates stcdi
	)

),

storage_tank_controlled_characterization_filled_date_gaps as (
	
	SELECT

	stcc.id,
	stcc.storage_tank_id,
	stcc.start_date,
	stcc.end_date,
	stcc.control_device

	FROM storage_tank_controlled_characterization_adjusted_dates stcc

	UNION ALL

	SELECT

	NULL as id,
	stcc.storage_tank_id,
	stcc.start_date_fill as start_date,
	stcc.end_date_fill as end_date,
	NULL as control_device

	FROM storage_tank_controlled_characterization_adjusted_dates stcc

	WHERE NOT stcc.start_date_fill > stcc.end_date_fill
	
UNION ALL
	
	SELECT

	NULL as id,
	stcc.storage_tank_id,
	stcc.start_date,
	stcc.end_date,
	NULL as control_device

	FROM (

	SELECT

	stcc.storage_tank_id,
	DATE_TRUNC('month', MIN(stcc.start_date))::date as start_date,
	(MIN(stcc.start_date) - INTERVAL '1 day')::date as end_date

	FROM
		storage_tank_controlled_characterization_adjusted_dates stcc

	GROUP BY stcc.storage_tank_id
		) stcc
	WHERE (stcc.storage_tank_id, stcc.start_date) NOT IN (
		SELECT stcc.storage_tank_id, stcc.start_date
		FROM storage_tank_controlled_characterization_adjusted_dates stcc
	)
)




SELECT

stcc.*

FROM (
	
SELECT

stcc.id,
stcc.storage_tank_id,
COALESCE(stcdi.start_date, stcc.start_date) as start_date,
COALESCE(stcdi.end_date, stcc.end_date) as end_date,
stcc.control_device,
stcdi.reason

FROM storage_tank_controlled_characterization_filled_date_gaps stcc
LEFT OUTER JOIN storage_tank_control_device_inactivity_filled_date_gaps stcdi ON stcdi.storage_tank_controlled_characterization_id = stcc.id

) stcc

ORDER BY stcc.storage_tank_id, stcc.start_date