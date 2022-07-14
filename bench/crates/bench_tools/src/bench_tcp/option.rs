use std::{net::{Ipv4Addr,Ipv6Addr}, num::ParseFloatError, time::Duration};
use structopt::StructOpt;
use serde::Deserialize;
use pnet_datalink::MacAddr;

#[derive(Debug, StructOpt,Clone)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opt {
    pub dst_ip:Ipv4Addr,
    pub dst_port:u16,
    pub interface_mac:MacAddr,
    pub count:u16,
    pub threads:u16,
    //_PACKETS_SEND:
}

impl Opt {
    pub fn new() ->Opt{
        Opt {
            dst_ip: Ipv4Addr::new(127, 0, 0, 1),
            dst_port: 80,
            interface_mac: MacAddr::new(255,255,255,255,255,255),
            count: 1,
            threads: 1,
        }
    }
}
