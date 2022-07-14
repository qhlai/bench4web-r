

//pub mod login;

//pub mod bing;
#![allow(unused_imports)]

use crate::attack::{attack_http::{attack_over_http::attack_http, custom_rand}, attack_tcp::custom_random::gen_ipv4};


//pub mod attack_get;
//pub mod post;
//pub mod get;
//pub mod attack_download;
//pub mod custom_rand_bak;
mod timestamp;
//pub mod super::attack_http;
pub mod attack_http;
pub mod attack_tcp;
pub mod attack_udp;
pub mod dnslookup;
pub mod attack_slow;
pub mod certify;

use pnet::datalink::MacAddr;

//pub mod dns_resolve;
use {
    super::*,
    log::LevelFilter,
    serde::Deserialize,
    std::{
        fs::File,
        io::{self, Read},
        sync::Arc,
        net::{Ipv4Addr,Ipv6Addr},
        fmt::Write,
        num::ParseIntError,
    },
    crate::{
        error::Error,
        //log::LevelFilter,
        //serde::Deserialize,
        //pub mod custom_rand;
        attack::{
            //custom_rand,
            //attack_http::{http_attack},
        },
    },
};


#[cfg(target_os = "macos")]
pub static DISCARD_PATH: &str = "/dev/null";
#[cfg(target_os = "linux")]
pub static DISCARD_PATH: &str = "/dev/null";
#[cfg(target_os = "windows")]
pub static DISCARD_PATH: &str = "nul";
//pub static ADMIN_KEY: &str="12341313";


#[derive(Deserialize)]
pub struct GlobalConfig {
    pub mode: String,
    pub log_level: Option<String>,
    //config:
    pub key: Option<String>,
}
/*
#[derive(Deserialize)]
pub struct AdminKey {
    admin_key:Option<String>,
}
#[derive(Deserialize)]
pub struct NormConfig {
    url: String,
    thread_num: Option<u32>,
    run_mode: Option<String>,
    attack_mode: Option<String>,
    duration: Option<u64>,
    timeout: Option<u16>,
}
#[derive(Deserialize)]
struct HttpConfig {
    config: attack_http::option::Opt, 
    //key: Option<String>
}
#[derive(Deserialize)]
struct UDPConfig {
    config: attack_udp::option::Opt, 
    //key: Option<String>
}
#[derive(Deserialize)]
struct TCPConfig {
    config: attack_tcp::option::Opt, 
    //key: Option<String>
}

#[derive(Deserialize)]
struct SlowConfig {
    config: attack_slow::option::Opt, 
    //key: Option<String>
}
*/
/*
fn string_find(string:String,find:&str)->Result<usize,()>{
    
    let mut url_domain=url.clone();
    //let find ="://";
    let index_0=url_domain.find("://").unwrap_or(0);
    log::debug!("{} find ;// at:{}",url_domain,index_0);
    url_domain=url_domain[index_0+3..].to_string();
    let index_1=url_domain.find("/").unwrap_or(0);
    log::debug!("{} find / at:{}",url_domain,index_1);
    url_domain=url_domain[..index_1].to_string();
    log::debug!("slice url:{}",url_domain);    
    return url_domain;
}
*/


fn str2_to_u8(s:&str)->Result<u8,String>{
    println!("len:{}",s.len());
    let Str=s.to_string();
    //match 
    //let a=s.chars()[0];

    return Ok(1);
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
pub fn slice_domain_from_url(url:String)->Result<String,String>{
    //"https://www.google.com/213123"
    //to
    //www.google.com
    let mut url_domain=url.clone();
    //let find ="://";
    let index_0=url_domain.find("://").ok_or("url should begin with http://// ".to_string())?;
    log::debug!("{} find ;// at:{}",url_domain,index_0);
    url_domain=url_domain[index_0+3..].to_string();
    let index_1=url_domain.find("/").ok_or("url should end with // ".to_string())?;
    log::debug!("{} find / at:{}",url_domain,index_1);
    url_domain=url_domain[..index_1].to_string();
    log::debug!("slice url:{}",url_domain);    
    return Ok(url_domain);
}
fn get_level(name: &str) -> LevelFilter {
    match name {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => {
            panic!("no such level {}", name)
        }
    }
}


pub async fn launch_from_config_filename(filename: String) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut config_string = String::new();
    file.read_to_string(&mut config_string)?;
    launch_from_config_string(config_string).await
}
pub async fn launch_from_config_string(config_string: String) -> io::Result<()> {
    let config: GlobalConfig = toml::from_str(&config_string)?;
    let mut key_flag=false;
    let ADMIN_KEY: String=String::from("123a41313");

    if let Some(log_level) = config.log_level {
        let level = match log_level.as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                return Err(Error::new("invalid log_level").into());
            }
        };
        let _ = env_logger::builder().filter_level(level).try_init();
    } else {
        let _ = env_logger::builder()
            .filter_level(LevelFilter::Debug)
            .try_init();
    }
    key_flag= certify::certify(config.key);

    
    match config.mode.as_str() {
        
        #[cfg(feature = "http_mode")]
        "http" => {
            log::info!("http mode");
            let _config: attack_http::option::HttpConfig = toml::from_str(&config_string)?;
            //let find=url.find('g')
            if !key_flag {
                if !certify::url_check_whitelist(_config.config.host.clone()){
                    return Ok(())
                }
            }
            log::info!("http mode start");

            self::attack_http::attack_over_http::attack_http(&_config.config).await;//thread_num, run_mode, url, attack_mode).await;
            //self::attack_http::attack_use_http::attack_http_(thread_num, run_mode, url, attack_mode);
        }
        #[cfg(feature = "tcp_mode")]
        "tcp" => {
            log::info!("tcp mode");
            let mut _config: attack_tcp::option::TCPConfig = toml::from_str(&config_string)?;
            //let url:String=_config.config.destination.clone();

            #[cfg(feature = "app_limit")]
            if !key_flag {
                //if !url_check_whitelist(url.clone()){
               //     return Ok(())
                //}
            }
            //let mut rng = rand::thread_rng();
            //_config.config.destination=slice_domain_from_url(_config.config.destination).unwrap_or("127.0.0.1:8080".to_string());
            log::info!("tcp mode start");
            //.unwrap_or(Ipv4Addr::new(127, 0, 0, 1))
            //let destination_ip=Ipv4Addr::from_str(string_to_static_str(_config.config.target));
            //log::info!("{}",string_to_ipv4addr(_config.config.target).unwrap_or(Ipv4Addr::from(28673));


            attack_tcp::tcp_flood::tcp_flood(&_config.config).await;//thread_num, destination_ip, interface_mac, count).await;
            
        }
        #[cfg(feature = "udp_mode")]
        "udp" => {
            log::info!("udp mode");
            let _config: attack_udp::option::UDPConfig = toml::from_str(&config_string)?;
            #[cfg(feature = "app_limit")]
            if !key_flag {
                if !certify::url_check_whitelist(_config.config.host.clone()){
                    return Ok(())
                }
            }
            log::info!("udp mode start");
            attack_udp::udp_flood_socket::udp_flood(&_config.config);
        }
        #[cfg(feature = "slow_mode")]
        "slow" => {
            log::info!("slow mode");
            let mut _config: attack_slow::option::SlowConfig = toml::from_str(&config_string)?;
            #[cfg(feature = "app_limit")]
            if !key_flag {
                let host:String=_config.config.host.clone();
                if !certify::url_check_whitelist(host.clone()){
                    return Ok(())
                }
            }
            attack_slow::slowloris_attack::slowloris_attack(&_config.config).await;

        }
        _ => {
            log::error!("invalid mode: {}", config.mode.as_str());
        }
    }
    Ok(())
}
//pub mod get_ip;
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    #[test]
    fn test_slice_domain_from_url() {
        let url=String::from("https://www.google.com/213123");
        let url_domian=match slice_domain_from_url(url){
            Ok(a)=>{
                println!("slice_domain_from_url:{}",a);
            }
            Err(a)=>{
                println!("slice_domain_Error:{}",a);
            }
        };
    }

}