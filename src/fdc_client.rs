use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;

pub type FdcClient = Client<Compat<TcpStream>>;
