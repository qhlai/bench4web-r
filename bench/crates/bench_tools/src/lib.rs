
#![allow(dead_code)]
#![forbid(unsafe_code)]
#![allow(unused_imports)]
//pub mod login;

//pub mod bing;


use crate::{
    bench_http::{
        bench_over_http, 
        //custom_rand
    }
};


//pub mod bench_get;
//pub mod post;
//pub mod get;
//pub mod bench_download;
//pub mod custom_rand_bak;

//pub mod super::bench_http;
pub mod bench_http;
pub mod bench_udp;
pub mod bench_tcp;
pub mod bench_slow;
pub mod check;

mod time_tools;
pub mod gen_random;
pub mod url2domain;


//pub mod certify;


//pub mod dns_resolve;
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
    certify,
};

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub mode: String,
    pub log_level: Option<String>,
    //config:
    pub key: Option<String>,
}


fn str2_to_u8(s:&str)->Result<u8,String>{
    println!("len:{}",s.len());
    //match 
    let a=s.chars().nth(0).unwrap_or('0') as u8;

    return Ok(a);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn string_to_macaddr(s:&str)->Result<MacAddr,String>{
    let mut mac:[u8;6]=[0; 6];
    //Ipv4Addr::from(ip);
    //let mut index:usize=0;
    //let a=s.into_bytes();
    //println!("{}",a);
    println!("{}",s);
    let str=s.clone();
    let mut str=str.split(":");
    for i in 0..6{
        
        let tmp=str.next().unwrap_or("0");//.parse::<u8>().unwrap();//.parse::<u8>().unwrap();
        //println!("{}",tmp);
        mac[i]=decode_hex(&tmp).unwrap().pop().unwrap_or(0);
    }
    Ok(MacAddr::from(mac))

}
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}
//#[no_std]
//#[feature(str_split_as_str)]
pub  fn string_to_ipv4addr( s:&str)->Result<Ipv4Addr,String>{
    let mut ip:[u8;4]=[0,0,0,0];
    let str=s.clone();
    let mut str=str.split(".");

    for i in 0..4{
        ip[i]=str.next().unwrap_or("0").parse::<u8>().unwrap_or(0);
    }
    Ok(Ipv4Addr::from(ip))

}
fn get_loglevel(name: &str) -> LevelFilter {
    match name {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => {
            println!("log level default:INFO");
            LevelFilter::Warn
        }
    }
}


pub async fn launch_from_config_filename_json(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut config_string = String::new();
    file.read_to_string(&mut config_string)?;
    launch_from_config_string_json(&config_string[..]).await
}
pub async fn launch_from_config_string_json(config_string: &str) -> io::Result<()> {
    
    let config: Value = serde_json::from_str(config_string).expect("config understading failed");

    //let log_level = config["log"]["loglevel"] ;
    let level=get_loglevel(config["log"]["loglevel"].as_str().unwrap_or("None")); 
    let _ = env_logger::builder().filter_level(level).try_init();

    log::info!("log level:{}",level); 
    log::info!("{}",certify::certify::info());
    //let mut key_flag= certify::certify(config["key"].as_str().unwrap_or("0"));

    #[cfg(feature = "certify")]
    {
        let (expire,remain)=certify::certify::expire(3);
        log::info!("{}",remain);
        let key_cert=certify::certify::key_certify(config["key"].as_str().unwrap_or("0"));
        
        if expire && !key_cert {
            log::warn!("Expire");
            if key_cert{
                log::warn!("Admin mode ignore Expire");
            }
            else {
                panic!("Expire");
            }
        }
        if !key_cert {
            log::info!("User mode");
        }
        else{
            log::info!("Admin mode");
        }
    }


    let run_index=config["select"].as_u64().unwrap_or(0) as usize;

    if config["run"][run_index]["enable"].as_bool().unwrap_or(true)
    {

        log::info!("run mode:{},select:{} enabled",config["run"][run_index]["mode"].as_str().unwrap_or("None"),run_index); 
        match config["run"][run_index]["mode"].as_str().unwrap() {
            
        #[cfg(feature = "http_mode")]
        "http" => {
            log::info!("http mode");
            log::info!("http mode start");
                let mut opt:bench_http::option::Opt=bench_http::option::Opt::new();
                opt.url=config["run"][run_index]["targets"][0]["url"].as_str().unwrap_or({
                    log::info!("default duration(0)");
                    "example.com"}).to_string();
                opt.threads = config["run"][run_index]["threads"].as_u64().unwrap_or({
                    log::info!("default thread(8)");
                    8}) as u16;
                opt.duration=config["run"][run_index]["duration"].as_u64().unwrap_or({
                    log::info!("default duration(0)");
                    0});
                opt.run_mode=config["run"][run_index]["setting"]["mode"].as_str().unwrap_or({
                    log::info!("default run_mode(MIX)");
                    "GET,POST"}).to_string();
                
                opt.bench_mode=config["run"][run_index]["setting"]["bench_mode"].as_str().unwrap_or({
                    log::info!("default bench_mode(dynamic)");
                    "dynamic"}).to_string();
                bench_http::bench_over_http::run(opt).await;
            }
        #[cfg(feature = "download_mode")]
        "download" => {
            log::info!("download mode");

            log::info!("download mode start");
                let mut opt:bench_http::option::Opt=bench_http::option::Opt::new();
                opt.url=config["run"][run_index]["targets"][0]["url"].as_str().unwrap_or({
                    log::info!("default duration(0)");
                    "example.com"}).to_string();
                opt.threads = config["run"][run_index]["threads"].as_u64().unwrap_or({
                    log::info!("default thread(8)");
                    8}) as u16;
                opt.duration=config["run"][run_index]["duration"].as_u64().unwrap_or({
                    log::info!("default duration(0)");
                    0});
                opt.run_mode=config["run"][run_index]["setting"]["mode"].as_str().unwrap_or({
                    log::info!("default run_mode(MIX)");
                    "GET,POST"}).to_string();
                
                opt.bench_mode=config["run"][run_index]["setting"]["bench_mode"].as_str().unwrap_or({
                    log::info!("default bench_mode(dynamic)");
                    "dynamic"}).to_string();
                bench_http::bench_download::run(opt).await;
                
            }            
        #[cfg(feature = "tcp_mode")]
        "tcp" => {
            log::info!("tcp mode");
           
            log::info!("tcp mode start");
            let mut opt:bench_tcp::option::Opt=bench_tcp::option::Opt::new();
            let target_index=0;
            
            let ip=config["run"][run_index]["targets"][target_index]["ip"].as_str().unwrap_or({
                log::info!("default ip");
                "127.0.0.1"
            });
            opt.dst_ip=string_to_ipv4addr(ip).unwrap_or({
                log::info!("default port");
                Ipv4Addr::new(127, 0, 0, 1)
            });
            opt.dst_port=config["run"][run_index]["targets"][target_index]["dst_port"].as_u64().unwrap_or({
                log::info!("default port");
                80
            }) as u16;
            log::info!("ip:{}",ip);

            

            let mac=config["run"][run_index]["setting"]["mac"].as_str().unwrap_or({
                log::info!("default ip");
                "00:00:00:00:00:00"
            });
            opt.interface_mac=string_to_macaddr(mac).unwrap_or(MacAddr::new(255,255,255,255,255,255));
            opt.count=config["run"][run_index]["setting"]["count"].as_u64().unwrap_or({
                log::info!("default");
                4}) as u16;

            opt.threads=config["run"]["threads"].as_u64().unwrap_or({
                log::info!("default");
                4}) as u16;
            
            bench_tcp::tcp_flood::run(opt).await;//thread_num, destination_ip, interface_mac, count).await;
            
        }
        #[cfg(feature = "udp_mode")]
        "udp" => {
            log::info!("udp mode");
            //let _config: bench_udp::option::UDPConfig = toml::from_str(&config_string)?;
           
            let mut opt:bench_udp::option::Opt=bench_udp::option::Opt::new();

            let target_index=0;
            let ip=config["run"][run_index]["targets"][target_index]["ip"].as_str().unwrap_or({
                log::info!("default ip");
                "127.0.0.1"
            });
            opt.dst_ip=string_to_ipv4addr(ip).unwrap_or({
                log::info!("default port");
                Ipv4Addr::new(127, 0, 0, 1)
            });
            opt.dst_port=config["run"][run_index]["targets"][target_index]["port"].as_u64().unwrap_or({
                log::info!("default port 8081");
                8081}) as u16;
            opt.threads=config["run"][run_index]["threads"].as_u64().unwrap_or({
                log::info!("default threads 4");
                4}) as u16;
            opt.duration=config["run"][run_index]["setting"]["duration"].as_u64().unwrap_or({
                log::info!("default duration");
                4}) as u64;
            opt.start_port=config["run"][run_index]["setting"]["start_port"].as_u64().unwrap_or({
                log::info!("default ports_per_thread(30)");
                40000}) as u16;
            opt.ports_per_thread=config["run"][run_index]["setting"]["ports_per_thread"].as_u64().unwrap_or({
                    log::info!("default ports_per_thread(30)");
                    30}) as u16;
            log::info!("udp mode start");
            bench_udp::udp_flood_socket::run(opt);
        }
        #[cfg(feature = "slow_mode")]
        "slow" => {
            log::info!("slow mode");
            
            let mut opt:bench_slow::option::Opt=bench_slow::option::Opt::new();

            let target_index=0;
            let bench_host=config["run"][run_index]["targets"][target_index]["url"].as_str().unwrap_or({
                log::info!("default ssl");
                "http://example.com"}).to_string();
            opt.ssl=config["run"][run_index]["setting"]["ssl"].as_bool().unwrap_or({
                log::info!("default ssl");
                false});
            if bench_host.find("https://").is_some() && bench_host.find("https://").unwrap()==0{
                opt.url=bench_host[8..].to_string();
                opt.ssl=true;
                log::info!("https://+{}",opt.url);
            }else if bench_host.find("http://").is_some() && bench_host.find("http://").unwrap()==0{
                
                opt.url=bench_host[7..].to_string();
                opt.ssl=false;
                log::info!("http://+{}",opt.url);
            }else{
                opt.url=bench_host;
            }
            //let mut bench_host=opt.host.clone();
            // The default port is 80, but for SSL it's 443.
            //let mut ssl = opt.ssl.unwrap_or(false);
            opt.ssl=config["run"][run_index]["setting"]["ssl"].as_bool().unwrap_or({
                log::info!("default ssl");
                false});
            let default_port:u64 = if opt.ssl { 443 } else { 80 };
            opt.port = config["run"][run_index]["targets"][target_index]["port"].as_u64().unwrap_or({
                log::info!("default port");
                default_port}) as u16;
            opt.finalize = config["run"][run_index]["setting"]["finalize"].as_bool().unwrap_or({
                log::info!("default finalize");
                false});
            opt.cycles = config["run"][run_index]["setting"]["cycles"].as_u64().unwrap_or({
                log::info!("default finalize");
                10}) as u32;
            opt.timeout = config["run"][run_index]["setting"]["timeout"].as_u64().unwrap_or({
                log::info!("default timeout");
                3}) as u16;
            opt.repeat = config["run"][run_index]["setting"]["repeat"].as_bool().unwrap_or({
                log::info!("default finalize");
                true});
            opt.threads = config["run"][run_index]["threads"].as_u64().unwrap_or({
                log::info!("default finalize");
                4}) as u16;
            opt.cmd_mode=config["run"][run_index]["setting"]["cmd_mode"].as_str().unwrap_or({
                log::info!("default cmd_mode");
                "GET"}).to_string();
            opt.domain =config["run"][run_index]["setting"]["domain"].as_bool().unwrap_or({
                log::info!("default domain");
                opt.ssl});
            opt.update();
            bench_slow::slowloris_bench::slowloris_bench(opt).await;

            }
            _ => {
                log::error!("invalid mode");
            }
        }
    }
    else{
        println!("Disabled");
    }
    
    
    Ok(())
}
pub async fn run(config: &str)->io::Result<()>{
    let _config: Value = serde_json::from_str(config).expect("config understading failed");
    
    Ok(())
}
//pub mod get_ip;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_analysis_1() {
        let _config=r#"{
            "key":"12a3",
            "log" : {
                "msg": "./msg.log",
                "error": "./error.log",
                "loglevel": "warning"
            },
            "run": [
                {    
                    "mode": "http",
                    "enable":false,
                    "threads":1,
                    "targets":[
                        {
                            "url":"example1.com",
                            "ip":"127.0.0.1",
                            "port": 80
                        },
                        {
                            "url":"example2.com",
                            "ip":"127.0.0.1",
                            "port": 80
                        }
                    ],
                    http_setting:{
                        "dynamic":true,        
                        "request_mode":"MIX",
                        "network": "tcp,udp"
                    }

                }
            ]
        }"#;
        //launch_from_config_string_json(config).await;
    }
}