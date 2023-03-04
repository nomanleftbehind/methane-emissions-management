use crate::{
    graphql::{
        models::{
            CompressorBlowdown, CompressorBlowdownDbCrossrefRows, CompressorBlowdownInterim,
            CompressorBlowdownInterimNestedRows, CompressorBlowdownInterimUnnestedRows,
        },
        queries::FromToMonthInput,
        sql::get_compressor_db_crossref,
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

pub async fn mutatation_insert_compressor_blowdowns_from_fdc(
    pool: &PgPool,
    mssql_fdc_client: &mut MssqlFdcClient,
    user_id: Uuid,
    FromToMonthInput {
        from_month,
        to_month,
    }: FromToMonthInput,
) -> Result<u64, anyhow::Error> {
    let compressor_db_crossref = get_compressor_db_crossref(pool).await?;

    let stream = mssql_fdc_client.query(
        r#"SELECT

        c.IDREC as "fdc_rec_id",
        CASE WHEN ume.DTTM = '2023-02-18' THEN NULL ELSE CAST(ume.DTTM as date) END as "date",
        SUM(ume.RATE) as "gas_volume"
        
        FROM pvCalcUnitsMetric.PVUNITMETERRATE um
        INNER JOIN pvCalcUnitsMetric.PVUNITEQUIP c ON c.SERIALNUM = um.SERIALNUM
        INNER JOIN pvCalcUnitsMetric.PVUNITMETERRATEENTRY ume ON ume.IDRECPARENT = um.IDREC AND DATEADD(D, 1, EOMONTH(ume.DTTM, -1)) BETWEEN @P1 AND @P2
        
        
        GROUP BY
        c.IDREC,
        ume.DTTM
        
        ORDER BY
        c.IDREC,
        ume.DTTM"#,
        &[&from_month, &to_month]).await?;

    let mssql_server_rows = stream.into_first_result().await?;

    let compressor_blowdown_db_crossref_rows = CompressorBlowdownDbCrossrefRows {
        crossref: &compressor_db_crossref,
        mssql_server_rows,
    };

    let compressor_blowdown_interims_result: Result<
        Vec<CompressorBlowdownInterim>,
        tiberius::error::Error,
    > = compressor_blowdown_db_crossref_rows.into();
    let compressor_blowdown_interims = compressor_blowdown_interims_result?;

    let CompressorBlowdownInterimNestedRows {
        id,
        compressor_id,
        date,
        gas_volume,
        created_by_id,
        created_at,
        updated_by_id,
        updated_at,
    } = CompressorBlowdownInterimUnnestedRows {
        user_id,
        compressor_blowdown_interims,
    }
    .into();

    let rows_inserted = query_file!(
        "src/graphql/sql/statements/compressor_blowdown_insert.sql",
        &id,
        &compressor_id,
        &date,
        &gas_volume,
        &created_by_id,
        &created_at,
        &updated_by_id,
        &updated_at
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_inserted)
}
