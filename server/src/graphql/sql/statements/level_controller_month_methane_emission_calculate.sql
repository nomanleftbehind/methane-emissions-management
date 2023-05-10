WITH allocate_month as (
    SELECT
        month_beginning::date
    FROM
        generate_series($1::date, $2::date, '1 month') as month_beginning
)
SELECT
    lcmme.facility_id,
    lcmme.site_id as "site_id!",
    'level_controller'::methane_emission_source_table as "source_table!: _",
    lcmme.id as "source_table_id!",
    'ROUTINE'::methane_emission_category as "category!: _",
    'PNEUMATIC_DEVICE'::methane_emission_source as "source!: _",
    lcmme.month_beginning as "month!",
    SUM(lcmme.gas_volume * lcmme.percent) as "gas_volume!",
    SUM(lcmme.gas_volume * lcmme.c1 * lcmme.percent) as "c1_volume!",
    SUM(lcmme.gas_volume * lcmme.co2 * lcmme.percent) as "co2_volume!"
FROM
    (
        SELECT
            lcmme.facility_id,
            lcmme.site_id,
            lcmme.id,
            lcmme.month_beginning,
            lcmme.gas_volume,
            lcmme.c1,
            lcmme.co2,
            EXTRACT(
                DAY
                FROM
                    (
                        lcmme.to_date + INTERVAL '1 day' - lcmme.from_date
                    )
            ) / lcmme.days_in_period as percent
        FROM
            (
                SELECT
                    s.facility_id,
                    lcmme.site_id,
                    lcmme.id,
                    lcmme.month_beginning,
                    lcmme.gas_volume,
                    COALESCE(ga.c1, $3) as c1,
                    COALESCE(ga.co2, $4) as co2,
                    GREATEST(ga.from_date, lcmme.from_date) as from_date,
                    LEAST(ga.to_date, lcmme.to_date) as to_date,
                    EXTRACT(
                        DAY
                        FROM
                            (
                                lcmme.to_date + INTERVAL '1 day' - lcmme.from_date
                            )
                    ) as days_in_period
                FROM
                    (
                        SELECT
                            lcmme.*
                        FROM
                            (
                                SELECT
                                    lcmme.id,
                                    lcmme.site_id,
                                    lcmme.month_beginning,
                                    lcmme.from_date,
                                    lcmme.to_date,
                                    lcmme.hours_on * lcmme.rate * lcmme.percent as gas_volume
                                FROM
                                    (
                                        SELECT
                                            lcmme.id,
                                            lcmme.site_id,
                                            lcmme.month_beginning,
                                            lcmme.from_date,
                                            lcmme.to_date,
                                            lcmme.hours_on,
                                            lcmme.rate,
                                            EXTRACT(
                                                DAY
                                                FROM
                                                    (
                                                        lcmme.to_date + INTERVAL '1 day' - lcmme.from_date
                                                    )
                                            ) / lcmme.days_in_month as percent
                                        FROM
                                            (
                                                SELECT
                                                    lc.id,
                                                    lc.site_id,
                                                    lc.month_beginning,
                                                    GREATEST(lcc.from_date, lc.month_beginning) as from_date,
                                                    LEAST(
                                                        lcc.to_date,
                                                        lc.month_beginning + INTERVAL '1 month - 1 day'
                                                    ) as to_date,
                                                    DATE_PART(
                                                        'days',
                                                        lc.month_beginning + INTERVAL '1 month - 1 day'
                                                    ) as days_in_month,
                                                    COALESCE(cmh.hours_on, 0) hours_on,
                                                    COALESCE(lcc.rate, 0) rate
                                                FROM
                                                    (
                                                        SELECT
                                                            lc.id,
                                                            lc.site_id,
                                                            am.month_beginning
                                                        FROM
                                                            level_controller lc
                                                            INNER JOIN allocate_month am ON am.month_beginning BETWEEN DATE_TRUNC('month', lc.start_date)
                                                            AND COALESCE(lc.end_date, CURRENT_DATE)
                                                            AND (lc.id, am.month_beginning) NOT IN (
                                                                SELECT
                                                                    lcmmeo.level_controller_id,
                                                                    lcmmeo.month
                                                                FROM
                                                                    level_controller_month_methane_emission_override lcmmeo
                                                                    INNER JOIN allocate_month am ON am.month_beginning = lcmmeo.month
                                                            )
                                                    ) lc
                                                    LEFT OUTER JOIN level_controller_month_hours cmh ON cmh.level_controller_id = lc.id
                                                    AND cmh.month = lc.month_beginning
                                                    LEFT OUTER JOIN (
                                                        SELECT
                                                            level_controller_id,
                                                            DATE_TRUNC('month', date) as month_join_beginning,
                                                            DATE_TRUNC(
                                                                'month',
                                                                COALESCE(
                                                                    LEAD(date) OVER (
                                                                        PARTITION BY level_controller_id
                                                                        ORDER BY
                                                                            date
                                                                    ) - INTERVAL '1 day',
                                                                    CURRENT_DATE
                                                                )
                                                            ) + INTERVAL '1 month - 1 day' as month_join_end,
                                                            -- If first device change, from_date has to be first of the month because there is no carryover from previous change.
                                                            CASE
                                                                WHEN ROW_NUMBER() OVER (
                                                                    PARTITION BY level_controller_id
                                                                    ORDER BY
                                                                        date
                                                                ) = 1 THEN DATE_TRUNC('month', date)::date
                                                                ELSE date
                                                            END as from_date,
                                                            COALESCE(
                                                                LEAD(date) OVER (
                                                                    PARTITION BY level_controller_id
                                                                    ORDER BY
                                                                        date
                                                                ) - INTERVAL '1 day',
                                                                CURRENT_DATE
                                                            ) as to_date,
                                                            rate
                                                        FROM
                                                            level_controller_change
                                                    ) lcc ON lcc.level_controller_id = lc.id
                                                    AND lc.month_beginning BETWEEN lcc.month_join_beginning
                                                    AND lcc.month_join_end
                                            ) lcmme
                                    ) lcmme
                            ) lcmme
                        UNION
                        ALL
                        SELECT
                            lcmme.*
                        FROM
                            (
                                SELECT
                                    lc.id,
                                    lc.site_id,
                                    lcmmeo.month as month_beginning,
                                    lcmmeo.month as from_date,
                                    lcmmeo.month + INTERVAL '1 month - 1 day' as to_date,
                                    lcmmeo.gas_volume
                                FROM
                                    level_controller_month_methane_emission_override lcmmeo
                                    INNER JOIN level_controller lc ON lc.id = lcmmeo.level_controller_id
                                    AND lcmmeo.month BETWEEN DATE_TRUNC('month', lc.start_date)
                                    AND COALESCE(lc.end_date, CURRENT_DATE)
                                    AND lcmmeo.month IN (
                                        SELECT
                                            month_beginning
                                        FROM
                                            allocate_month
                                    )
                            ) lcmme
                    ) lcmme
                    LEFT OUTER JOIN site s ON s.id = lcmme.site_id
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
                    AND ga.from_date <= lcmme.to_date
                    AND ga.to_date >= lcmme.from_date
            ) lcmme
    ) lcmme
GROUP BY
    lcmme.facility_id,
    lcmme.site_id,
    lcmme.id,
    lcmme.month_beginning