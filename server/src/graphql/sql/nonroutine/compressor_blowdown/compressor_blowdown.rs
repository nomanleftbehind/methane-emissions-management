use crate::{
    graphql::models::{
        input::MonthRangeInput,
        nonroutine::compressor_blowdown::{
            CompressorBlowdown, CompressorBlowdownInterim, CompressorBlowdownInterimNestedRows,
            CompressorBlowdownInterimUnnestedRows, MssqlCompressorBlowdownRows,
        },
    },
    MssqlFdcClient,
};
use sqlx::{query_as, query_file, Error, PgPool};
use uuid::Uuid;

pub async fn select_compressor_blowdowns(
    pool: &PgPool,
    compressor_id: Uuid,
) -> Result<Vec<CompressorBlowdown>, Error> {
    query_as!(
        CompressorBlowdown,
        "SELECT * FROM compressor_blowdown WHERE compressor_id = $1",
        compressor_id
    )
    .fetch_all(pool)
    .await
}

/// This function retrieves compressor blowdown volumes data from third party MSSQL database,
///
/// transforms data into insert friendly form, and inserts it into Postgres database.
pub async fn insert_compressor_blowdowns_from_fdc(
    pool: &PgPool,
    mssql_fdc_client: &mut MssqlFdcClient,
    user_id: Uuid,
    MonthRangeInput {
        from_month,
        to_month,
    }: &MonthRangeInput,
) -> Result<u64, anyhow::Error> {
    let stream = mssql_fdc_client.query(
        r#"
        DECLARE
        @start_month as date = @P1,
        @end_month as date = @P2;


        SELECT

        cb.IDREC as "fdc_rec_id",
        CAST(cb.DTTM as date) as "date",
        SUM(cb.RATE) as "gas_volume"

        FROM (
                SELECT

                c.IDREC,
                ume.DTTM,
                ume.RATE
                
                FROM pvCalcUnitsMetric.PVUNITMETERRATE um
                INNER JOIN pvCalcUnitsMetric.PVUNITEQUIP c ON c.SERIALNUM = um.SERIALNUM
                INNER JOIN pvCalcUnitsMetric.PVUNITMETERRATEENTRY ume ON ume.IDRECPARENT = um.IDREC AND ume.RATE IS NOT NULL AND ume.DTTM BETWEEN c.DTTMSTART AND ISNULL(c.DTTMEND, GETDATE()) AND DATEADD(D, 1, EOMONTH(ume.DTTM, -1)) BETWEEN @start_month AND @end_month

                UNION ALL

                SELECT

                c.IDREC,
                esde.DTTM,
                esde.RATE / COUNT(esde.IDREC) OVER (PARTITION BY esd.IDRECPARENT/*Partition by Unit (IDRECPARENT) because sometimes same compressor exist in multiple Units*/, esde.IDREC) as RATE

                FROM pvCalcUnitsMetric.PVUNITMETERRATE um
                INNER JOIN pvCalcUnitsMetric.PVUNITEQUIP c ON c.SERIALNUM = um.SERIALNUM
                INNER JOIN pvCalcUnitsMetric.PVUNITMETERRATE esd ON esd.IDRECPARENT = um.IDRECPARENT AND esd.NAME LIKE '%m3%' AND esd.NAME LIKE '%ESD%'
                INNER JOIN pvCalcUnitsMetric.PVUNITMETERRATEENTRY esde ON esde.IDRECPARENT = esd.IDREC AND esde.RATE IS NOT NULL AND esde.DTTM BETWEEN c.DTTMSTART AND ISNULL(c.DTTMEND, GETDATE()) AND DATEADD(D, 1, EOMONTH(esde.DTTM, -1)) BETWEEN @start_month AND @end_month
        ) cb

        GROUP BY
        cb.IDREC,
        cb.DTTM"#,
        &[from_month, to_month]).await?;

    let mssql_compressor_blowdown_rows = stream.into_first_result().await?;

    let mssql_compressor_blowdown_rows_struct = MssqlCompressorBlowdownRows {
        mssql_compressor_blowdown_rows,
    };

    let compressor_blowdown_interims_result: Result<
        Vec<CompressorBlowdownInterim>,
        tiberius::error::Error,
    > = mssql_compressor_blowdown_rows_struct.into();

    let CompressorBlowdownInterimNestedRows {
        id,
        fdc_rec_id,
        date,
        gas_volume,
        created_at,
    } = CompressorBlowdownInterimUnnestedRows(compressor_blowdown_interims_result?).into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/compressor_blowdown_insert.sql",
        &id,
        &fdc_rec_id,
        &date,
        &gas_volume,
        &created_at,
        user_id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
