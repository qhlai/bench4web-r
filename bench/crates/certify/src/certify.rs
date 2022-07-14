
use shadow_rs::shadow;
shadow!(build);

extern crate chrono; // 时间库
use chrono::prelude::*;
use std::time::{SystemTime,Duration};
use chrono::Duration as Durationx; //这样就可以编译通过了
use chrono::ParseResult;

static ADMIN_KEY : &str ="123a41313";
//static Complie_file: i64=time_tools::time_stamp();
static VALID_TIME : i64 =1657010198892+3600*24*1000*7; //ms
pub fn key_certify(key:&str)->bool{
    //let ADMIN_KEY: String=String::from("123a41313");
    //if key.is_some() && key.unwrap()== ADMIN_KEY.to_string(){
    if key== ADMIN_KEY{
        //log::info!("Admin Key Enabled");
        //log::info!("Whitelist Disabled");
        return true;
    }else{
        //log::info!("Whitelist Enabled");
        return false;
    }
} 

pub fn expire(valid_days:i64)-> (bool,String){
    let compiled_time: DateTime<FixedOffset> =
    DateTime::parse_from_str(build::BUILD_TIME_3339, "%Y-%m-%dT%H:%M:%S%z").unwrap();
    //println!("compiled_time({})", compiled_time);
    let valid_time=compiled_time+Durationx::days(valid_days);
    //println!("valid_time({})", valid_time);

    let local: DateTime<Local> = Local::now();
    let now: DateTime<FixedOffset>=local.try_into().unwrap();
    let expire=valid_time<now;
    let remain=valid_time-now;
    //println!("{}  {}",expire,remain);
    (expire,format!("expire:{}, valid_time:{} ,remain:{}",expire,valid_time,remain))
}
pub fn info()-> String{
    let info=format!("build time:{}, rust_version:{}, rust_channel:{}, {}", build::BUILD_TIME_3339,build::RUST_VERSION,build::RUST_CHANNEL,build::BUILD_RUST_CHANNEL);
    return info;
}