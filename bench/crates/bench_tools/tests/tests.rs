extern crate rand;
//extern crate ping;

use std::time::Duration;
use bench_tools::*;
use rand::random;
use {

    log::LevelFilter,
    serde::{Deserialize, Serialize},
    serde_json::{Value},
    std::{
        fs::File,
        io::{self, Read},
        sync::Arc,
        net::{Ipv4Addr,Ipv6Addr},
        fmt::Write,
        num::ParseIntError,
        thread,
    },
    tokio::{
        fs::OpenOptions, 
        io::{AsyncWriteExt,AsyncBufReadExt,BufReader,
        },
        runtime::Builder,
    },
    pnet_datalink::MacAddr,
    crate::{
        url2domain::{slice_domain_from_url}
        //error::Error,
        //log::LevelFilter,
        //serde::Deserialize,
        //pub mod custom_rand;
    },
};
#[test]
fn basic() {

    let timeout = Duration::from_secs(1);
    //ping::ping(addr, Some(timeout), Some(166), Some(3), Some(5), Some(&random())).unwrap();
}
#[tokio::test]
async fn bench_http() {
    let url="http://192.168.2.140:3333/root/".to_string();
    let threads= 64;
    let duration=0;
    let run_mode="GET".to_string();
    let bench_mode="dynamic".to_string();
    let mut target_index=0;
    log::error!("target num:{}",target_index);
    
    //ping::ping(addr, Some(timeout), Some(166), Some(3), Some(5), Some(&random())).unwrap();
}
//http://cachefly.cachefly.net/10mb.test
#[tokio::test]
async fn bench_download() {
    //let url="http://cachefly.cachefly.net/10mb.test".to_string();
    //let url="http://192.168.2.140:3333/root/test/raw/branch/main/10mb.test".to_string();

    let threads= 4;
    let duration=0;
    let run_mode="GET".to_string();
    let bench_mode="download".to_string();
    let mut target_index=0;
    log::error!("target num:{}",target_index);
    
}

#[tokio::test]
async fn bench_slow() {
    let mut ssl=false;
    let default_port:u16 = if ssl { 443 } else { 80 };
    let port = default_port as u16;
    let port = 3333;
    let finalize =false;
    let cycles = 10;
    let timeout = 10 as u64;
    let repeat = true;
    let threads = 64 as usize;
    let cmd_mode="GET";
    let domain =false;
    let bench_url="192.168.2.140".to_string();
    
}
#[tokio::test]
async fn bench_tcp() {

    let dst_port=80 as u16;
    let destination_ip= Ipv4Addr::new(192, 168, 2, 140);
    let mac="50:EB:F6:5B:1A:50";
    let interface_mac=string_to_macaddr(mac).unwrap_or(MacAddr::new(255,255,255,255,255,255));
    
    let count= 16 as usize;
    let threads= 16 as usize;
    
    
}
#[tokio::test]
async fn bench_udp() {
    let target_index=0;
    let ip="192.168.2.140";
    let port=52081 as u32;
    let threads=4 as usize;
    let duration= 4 as u64;
    let  start_port= 30000  as u32;
    let  ports_per_thread= 1 as u32;
    log::info!("udp mode start");
}



