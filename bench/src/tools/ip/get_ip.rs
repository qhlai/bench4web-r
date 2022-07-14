use std::{collections::HashMap, string};
use reqwest::header::HeaderMap;
use crate::web::post;
use crate::web::get;

// get::get_norm("https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN".to_string());
    //get::get_json("https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN".to_string());
    //get::get_str("https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN".to_string());
    //info!("{}",get::get_output("https://httpbin.org/ip".to_string()));
    //info!("{}",get::get_output("https://ipinfo.io/json".to_string()));
    //get::get_norm("https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN".to_string());
    //my_post::get_str("https://myip.ipip.net/json".to_string());

pub fn get_ip_China() -> String{
    let url ="https://httpbin.org/ip";
    if let Ok(res) = get::get_json(url.to_string()){
        //println!("{:#?}", res);
        let ip = res.get("origin").unwrap().clone().to_string();  
        println!("China ip:{}", ip);
        return ip;
    }
    else{
        return "ERROR".to_string();
    }
}
/*
{
  "ret": "ok",
  "data": {
    "ip": "111.32.111.111",
    "location": [
      "中国",
      "四川",
      "成都",
      "",
      "移动"
    ]
  }
}
*/
pub fn get_ip_China1() -> String{
    let url ="https://myip.ipip.net/json";
    if let Ok(res) = get::get_json(url.to_string()){
        //println!("{:#?}", res);
        //let data = res.get("data").clone();
        //let ip = data.get("ip").unwrap().clone().to_string(); 
        let ip = res.get("data").unwrap().clone().to_string();  
        //.get("ip").unwrap().clone()
        println!("China ip:{}", ip);
        return ip;
    }
    else{
        return "ERROR".to_string();
    }
}
/*
{
  "ip": "35.238.15.8",
  "hostname": "li1442-28.members.linode.com",
  "city": "Singapore",
  "region": "Singapore",
  "country": "SG",
  "loc": "1.2897,103.8501",
  "org": "AS63949 Linode, LLC",
  "postal": "048508",
  "timezone": "Asia/Singapore",
  "readme": "https://ipinfo.io/missingauth"
}
*/
pub fn get_ip_foreign() -> String{
    
    let url ="https://ipinfo.io/json";
    if let Ok(res) = get::get_json(url.to_string()){
        //println!("{:#?}", res);
        let ip = res.get("ip").unwrap().clone().to_string();  
        println!("foreign ip:{}", ip);
        return ip;
    }
    else{
        return "foreign ERROR".to_string();
    }
}  
//#[tokio::main]

pub fn get_ip() -> String{
    let china=get_ip_China();
    let china1=get_ip_China1();
    let foreign=get_ip_foreign();
    //format!("{}\n{}\n{}",China,China1,foreign);
    return format!("\nChina:{}\nChina1:{}\nforeign:{}",china,china1,foreign);
}
