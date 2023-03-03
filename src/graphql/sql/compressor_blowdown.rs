use crate::graphql::models::CompressorBlowdownInterim;
use crate::FdcClient;
use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

pub async fn select_compressor_blowdowns_interim(
    _fdc_client: &FdcClient,
) -> Result<Vec<CompressorBlowdownInterim>, anyhow::Error> {
    let mut config = Config::new();
    config.host("host");
    config.database("port");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("user", "password"));

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;

    let mut client = Client::connect(config, tcp.compat_write()).await?;

    let stream = client.query(r#"SELECT

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
  ume.DTTM"#, &[&"2022-01-01", &"2022-02-01"]).await?;

    let v = stream.into_first_result().await?;

    let row = v
        .into_iter()
        .map(|d| CompressorBlowdownInterim {
            fdc_rec_id: d.get("fdc_rec_id").map(str::to_string).unwrap(),
            date: d.get("date").unwrap(),
            gas_volume: d.get("gas_volume").unwrap(),
        })
        .collect::<Vec<_>>();

    client.close().await?;

    Ok(row)
}
