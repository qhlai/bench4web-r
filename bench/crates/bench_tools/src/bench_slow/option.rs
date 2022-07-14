use std::{net::IpAddr, num::ParseFloatError, time::Duration};
use structopt::StructOpt;
use serde::Deserialize;

#[derive(Deserialize)]
//#[derive(Debug, StructOpt)]
//#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opt {
    pub url: String,
    pub domain: bool,
    pub ssl: bool,
    pub port: u16,
    pub ip_port: String,
    pub cmd_mode: String,  
    pub cycles: u32,
    pub threads:u16,
    pub timeout: u16,
    pub repeat: bool,  
    pub finalize: bool,
}
impl Opt {
    pub fn new() ->Opt{
        Opt {
            url: String::from("http://127.0.0.1"),
            domain: false,
            ssl: false,
            port: 80,
            ip_port:format!("{}:{}", "127.0.0.1", 80),
            cmd_mode: String::from("GET"),
            cycles: 10,
            threads:2,
            timeout: 0,
            repeat: true,  
            finalize: false,
        }
    }
    pub fn update(&mut self){
        self.ip_port=format!("{}:{}", self.url, self.port);
    }
}
#[derive(Debug, Clone)]
pub struct Target {
    domain: String,
    designator: String,
}

impl Target {
    pub fn new(target: String, port: u16) -> Self {
        Target {
            domain: format!("{}", target),
            designator: format!("{}:{}", target, port),
        }
    }
    pub fn get_designator(&self) -> &str {
        &self.designator
    }
    pub fn get_domain(&self) -> &str {
        &self.domain
    }
    pub fn set_domain(&mut self, domain: &str) {
        self.domain = domain.into();
    }
}
fn duration_try_from_str(s: &str) -> Result<Duration, ParseFloatError> {
    Ok(Duration::from_secs_f64(s.parse()?))
}
