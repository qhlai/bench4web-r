use std::{net::IpAddr, num::ParseFloatError, time::Duration};
use structopt::StructOpt;
use serde::Deserialize;

#[derive(Deserialize)]
//#[derive(Debug, StructOpt)]
//#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opt {
    pub host: String,
    pub thread: Option<u32>,
    pub ports_per_thread: Option<u32>,
    pub duration: Option<u64>,
    pub timeout: Option<u16>,
}

#[derive(Deserialize)]
pub struct UDPConfig {
    pub config: Opt, 
    //key: Option<String>
}

//fn duration_try_from_str(s: &str) -> Result<Duration, ParseFloatError> {
//    Ok(Duration::from_secs_f64(s.parse()?))
//}
