//include!("login.rs");

use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
//use lazy_static::lazy_static;
//extern crate systemstat;
use super::status_struct;

use systemstat::{System, Platform, saturating_sub_bytes};
use super::timestamp;

//use systemstat::{System, Platform, saturating_sub_bytes};


use std::net::UdpSocket;
pub fn get_my_ip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("223.5.5.5:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct CPULoadAggregate<> {
    user: String,
    nice: String,
    system: String,
    interrupt: String,
    idle: String,
    //cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0
}

impl<> CPULoadAggregate<> { 
    pub fn read() -> Self{
        let sys = System::new();
        //thread::sleep(Duration::from_secs(1));
        let loadall=sys.cpu_load_aggregate().unwrap().done().unwrap();
        Self {//one, loadavg.five, loadavg.fifteen),
            user: (loadall.user*100.0).to_string(),
            nice: (loadall.nice*100.0).to_string(),
            system: (loadall.system*100.0).to_string(),
            interrupt: (loadall.interrupt*100.0).to_string(),
            idle: (loadall.idle*100.0).to_string()
        }
    }
    pub fn fail() -> Self{
        Self {
            user: "".to_string(),
            nice: "".to_string(),
            system: "".to_string(),
            interrupt: "".to_string(),
            idle: "".to_string()
        }
    }
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct CPULoadAverage<> {
    one: String,
    five: String,
    fifteen: String
}

impl<> CPULoadAverage<> { 
    pub fn read() -> Self{
        let sys = System::new();
        let loadavg=sys.load_average().unwrap();
        //println!("\nMemory: error: {}",sys.load_average().unwrap().fifteen.to_string());
        Self {
            one: loadavg.one.to_string(),
            five:  loadavg.five.to_string(),
            fifteen: loadavg.fifteen.to_string()
        }
    }
    pub fn fail() -> Self{
        Self {
            one: "".to_string(),
            five: "".to_string(),
            fifteen: "".to_string()
        }
    }
    pub fn expect() -> Self{
        Self {
            one: "".to_string(),
            five: "".to_string(),
            fifteen: "".to_string()
        }
    }
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct CPULoad<> {
    avg: CPULoadAverage,
    agg: CPULoadAggregate,
    //cpu.user * 100.0, cpu.nice * 100.0, cpu.system * 100.0, cpu.interrupt * 100.0, cpu.idle * 100.0
}

impl<> CPULoad<> { 
    pub fn read() -> Self{
        Self {
            avg:CPULoadAverage::<>::fail(),//.to_string()
            //agg: CPULoadAggregate::<>::read(),
            agg: CPULoadAggregate::<>::fail()//无法实现
        }
    }
    pub fn fail() -> Self{
        Self {
            avg:CPULoadAverage::<>::fail(),//.to_string()
            agg: CPULoadAggregate::<>::fail()
        }
    }
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct CPUState<> {
    load: CPULoad,
    temp: String
}

impl<> CPUState<> { 
    pub fn read() -> Self{
        let sys = System::new();
        Self {
            load: CPULoad::<>::read(),//.to_string()
            temp:"sys.cpu_temp().unwrap()".to_string()
        }
    }
    pub fn fail() -> Self{
        let sys = System::new();
        Self {
            load: CPULoad::<>::fail(),
            temp: "".to_string()
        }
    }
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct MemState<> {
    total: String,
    free: String
}
impl<> MemState<> { 

    pub fn read() -> Self{
        let sys = System::new();
        Self {
            total:sys.memory().unwrap().total.to_string(),
            free: sys.memory().unwrap().free.to_string()
        }
    }
    pub fn fail() -> Self{
        let sys = System::new();
        Self {
            total:"".to_string(),
            free: "".to_string()
        }
    }
}
#[derive(Deserialize)]
#[derive(Serialize)]//use systemstat::{System, Platform, saturating_sub_bytes};
//#[derive(systemstat)]
struct SeverState<> {
    timestamp:i64,
    msg:String,
    code: i32,
    uptime:String,
    servertime:String,
    severip:String,
    cpustate:CPUState,
    memstate:MemState

}
//static N: i32 = 5;
//static NAME: &'static str = "Steve";
lazy_static! {
    pub static ref  SERVER_IP: String = get_my_ip().unwrap();
}
impl<> SeverState<> { 

    pub fn success_with_string(msg: String) -> Self{
        let sys = System::new();
        let uptime_tmp:String ="1".to_string();

        Self {
            timestamp: timestamp::time_stamp(),
            msg,
            code: 200,
            uptime:"sys.uptime.unwarp()".to_string(),
            servertime:timestamp::time_stamp().to_string(),
            severip:SERVER_IP.to_string(),
            cpustate:CPUState::<>::read(),
            memstate:MemState::<>::read()//.to_string()
        }
    }
    pub fn fail(msg: String) -> Self {
        Self {
            timestamp: timestamp::time_stamp(),
            msg,
            code: 400,
            uptime:"".to_string(),
            servertime:"".to_string(),
            severip:"".to_string(),
            cpustate:CPUState::<>::fail(),
            memstate:MemState::<>::fail()//.to_string()
        }
    }

}

//curl -v -H "Content-Type:application/json" -X POST --data '{"username":"tianlang", "password":"tianlang"}' http://127.0.0.1:8088/iot/update
#[post("/status")]
async fn status(status_info: web::Json<status_struct::StatusInfo>) -> impl Responder {
    println!("attemp post");
    let sys = System::new();
    match sys.socket_stats() {
        Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string())
    }
    if status_info.username == status_info.password {
        /*uptime:sys.uptime().to_string(),
        servertime:time_stamp().to_string(),
        severip:SERVER_IP.to_string(),
        cpuload:sys.load_average().to_string(),
        cputmp:sys.cpu_temp().to_string()*/
        HttpResponse::Ok().json(SeverState::<>::success_with_string("str,1324".to_string()))
    } else {
        HttpResponse::Forbidden().json(SeverState::<>::fail("Wrong Password".to_string()))
    }
}