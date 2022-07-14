

use std::{collections::HashMap, string};
use reqwest::header::HeaderMap;
use serde_json::value::Value;
//#[macro_use]
//extern crate json;
use std::io::Read;
//#![deny(warnings)]
//#[cfg(not(target_arch = "wasm32"))]

#[tokio::main]
pub async fn post_min(url:String)->Result<reqwest::Response, reqwest::Error> {
   
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,en-US;q=0.7,en;q=0.3".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cache-Control", "max-age=0, no-cache".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Length", "0".parse().unwrap());
    headers.insert("Host", "bing.com".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("Cookie", "MUID=163A307441A0601E2F5C203040E36195; _EDGE_V=1; SRCHD=AF=UNKSBD; SRCHUID=V=2&GUID=D89881C6E47048BEA298D2393B57E3DD&dmnchg=1; SRCHUSR=DOB=20210527&T=1624248731000; SRCHHPGUSR=SRCHLANGV2=zh-Hans&BRW=M&BRH=S&CW=1280&CH=547&DPR=1.5&UTC=480&DM=0&WTS=63758756672&HV=1626270368&PLTL=867&PLTA=938&PLTN=75&EXLTT=31&SRCHLANG=zh-Hans&SW=1280&SH=720; ABDEF=V=13&ABDV=13&MRB=1626144651000&MRNB=0; BFBUSR=BAWAS=1&BAWFS=1; OID=AhBHBXx2pTtPIrD4vJNE5M3L_DAvjnBDA98VDVYLXQefr3WwCU-xK_jBvTgzbF9_xXadZnvf3j5cvnBDQO0qqMpae9vZ2MYbL1…bpbg4jU3nWBGgZR5GlH42pR-o_2D6B4KElI6S8wq6W-QP0KmOYsw43F45dsB0QIs7wOu1XsTZVrTKH4PXfl55rK0-30CwPcD3Gw0Vl42RpHxVJF6pZ7gdxX8MK8E0wH1zBULMxMxEA5sMCYawFA_bpKZuUYpmm_ESvT85Cvy_jsLGStO; MUIDB=163A307441A0601E2F5C203040E36195; _tarLang=default=zh-Hans; _TTSS_IN=hist=WyJlbiIsImF1dG8tZGV0ZWN0Il0=; _TTSS_OUT=hist=WyJ6aC1IYW5zIl0=; SNRHOP=I=&TS=; _EDGE_S=SID=329ADE4768BF67B71180CE3569F566F6&mkt=zh-cn; _SS=SID=329ADE4768BF67B71180CE3569F566F6&bIm=615983; ipv6=hit=1626270501833&t=4; OPF=X=1; ENSEARCH=BENVER=0; _FP=hta=on".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0".parse().unwrap());
    
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("format", "js");
    data.insert("idx", "0");
    data.insert("n", "1");
    data.insert("mkt", "zh-CN");

    let client = reqwest::Client::new();
    //client.get(url)
    //Ok(client.post(url).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
    let res =client.post(url).headers(headers).json(&data).send().await?;
    //let res = reqwest::get(url.to_string()).await?;
    print!("Status: {} ", res.status());
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(res)
    /* 
    if let Ok(res) = post::post_json(bing_url.to_string(),headers,data) {
        println!("{:#?}", res);
        
        let url = res.get("images").unwrap().clone().to_string();
        
        println!("url:{}", url);
        //res.values()
        //res.keys()
        println!("keys:{:?}", res.keys());
        println!("values:{:?}", res.values());
        return url.to_string();
    }
    else {
        println!("ERROR");
        return "ERROR".to_string();
    }
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(res)
    */
}

#[tokio::main]
pub async fn post_json(url:String,headers:HeaderMap,data:HashMap<&str, &str>) -> Result<HashMap<String, Value>, reqwest::Error>{
    // post 请求要创建client
    let client = reqwest::Client::new();
    // 发起post请求并返回
    Ok(client.post(url).headers(headers).json(&data).send().await?.json::<HashMap<String, Value>>().await?)
}
//#[tokio::main]
/*
pub fn post_output(url:String,headers:HeaderMap,data:HashMap<&str, &str>) -> String{
    if let Ok(res) = post_json(url.to_string(),headers,data) {
        println!("POST ERROR");
        println!("{:#?}", res);
        format!("{}:{}", "POST OK".to_string(),url)
    }
    else{
        println!("POST ERROR");
        format!("{}:{}", "POST ERROR".to_string(),url)
    }    
}
*/

