use crate::{
    graphql::{models::CompressorBlowdownInterim, queries::FromToDateInput},
    FdcClient,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn select_compressor_blowdowns_interim(
    atomic_fdc_client: &Arc<Mutex<FdcClient>>,
    FromToDateInput { from_date, to_date }: FromToDateInput,
) -> Result<Vec<CompressorBlowdownInterim>, anyhow::Error> {
    let ac = atomic_fdc_client.clone();
    let mut mg = ac.lock().await;
    let fdc_client = &mut *mg;

    let stream = fdc_client.query(r#"SELECT

  c.IDREC as "fdc_rec_id",
  CAST(ume.DTTM as date) as "date",
  SUM(ume.RATE) as "gas_volume"
  
  FROM pvCalcUnitsMetric.PVUNITMETERRATE um
  INNER JOIN pvCalcUnitsMetric.PVUNITEQUIP c ON c.SERIALNUM = um.SERIALNUM
  INNER JOIN pvCalcUnitsMetric.PVUNITMETERRATEENTRY ume ON ume.IDRECPARENT = um.IDREC AND DATEADD(D, 1, EOMONTH(ume.DTTM, -1)) BETWEEN @P1 AND @P2
  
  
  GROUP BY
  c.IDREC,
  ume.DTTM
  
  ORDER BY
  c.IDREC,
  ume.DTTM"#, &[&from_date, &to_date]).await?;

    let v = stream.into_first_result().await?;

    let row = v
        .into_iter()
        .map(|d| CompressorBlowdownInterim {
            fdc_rec_id: d.get("fdc_rec_id").map(str::to_string).unwrap(),
            date: d.get("date").unwrap(),
            gas_volume: d.get("gas_volume").unwrap(),
        })
        .collect::<Vec<_>>();

    Ok(row)
}
