use std::{net::IpAddr, sync::atomic::{AtomicUsize,Ordering},num::ParseFloatError, time::Duration, str::FromStr};
use structopt::StructOpt;
use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opt {
    pub url: String,
    //#[structopt(default_value = "32")]
    pub threads: u16,
    pub run_mode: String,
    pub bench_mode: String,
    pub duration: u64,
    pub timeout: u16,
    pub display_interval:u64, //ms,
    
}
impl Opt {
    pub fn new() ->Opt{
        Opt {
            url: String::from("s"),
            //#[structopt(default_value = "32")]
            threads: 12,
            run_mode: String::from("s"),
            bench_mode: String::from("s"),
            duration: 1,
            timeout: 1,
            display_interval:5000,
        }
    }
}
#[derive(Deserialize)]
pub struct HttpConfig {
    pub config: Opt, 
    //key: Option<String>
}
fn duration_try_from_str(s: &str) -> Result<Duration, ParseFloatError> {
    Ok(Duration::from_secs_f64(s.parse()?))
}
