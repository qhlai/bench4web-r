
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::web::timestamp;
use crate::web::{post,get};

use std::{collections::HashMap, string};
use reqwest::header::HeaderMap;
//https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN
//const BING_IMG_URL_API:&str="https://bing.com/HPImageArchive.aspx";
//const BING_URL:&str="https://bing.com/";
#[derive(Deserialize)]
#[derive(Serialize)]
pub struct ClientRequest {
    pub id: i64,
    pub session: String,
    pub format: String,
    pub idx: String,
    pub n: String,
    pub mkt: String,
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct BingResponse<> {
    timestamp: i64,
    code: i32,
    msg: String,
    url: String,
    session: String
}
impl<> BingResponse<> {
    pub fn success(msg: String) -> Self{
         Self {
             timestamp: timestamp::time_stamp(),
             code: 200,
             msg,
             url: "123".to_string(),
             session: "123".to_string()
         }
    }
    pub fn success_with_string(msg: String) -> Self{
        Self {
            timestamp: timestamp::time_stamp(),
            code: 202,
            msg,
            url: "123".to_string(),
            session: "123".to_string()
        }
    }
    pub fn fail(msg: String) -> Self {
        Self {
            timestamp: timestamp::time_stamp(),
            msg,
            code: 200,
            url: "123".to_string(),
            session: "123".to_string()
        }
    }
}
pub fn bing_url() -> String{
    
    let url ="https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN";
    if let Ok(res) = get::get_json(url.to_string()){
        //println!("{:#?}", res);
        let ip = res.get("images").unwrap().clone().to_string();  
        println!("foreign ip:{}", ip);
        return ip;
    }
    else{
        return "foreign ERROR".to_string();
    }
}  

#[tokio::main]
pub async fn bing_url_1() -> String{

    let bing_url:&str="https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN";
    /* 
    if let Ok(res) = my_post::get(bing_url.to_string()).await {
        println!("{:#?}", res);
        let burl = res.get("url").unwrap().clone().to_string();
        println!("url:{}", burl);
        println!("keys:{:?}", res.keys());
        println!("values:{:?}", res.values());
        return burl.to_string();
    }
    else {
        println!("ERROR");
        return "ERROR".to_string();
    }
    */
    
    let mut bing_url="https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN";//"https://bing.com/HPImageArchive.aspx";
    println!("bing attemp post");
    // 组装header
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
    //headers.insert("Content-Type", "application/json".parse().unwrap());
    // 组装要提交的数据https://bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("format", "js");
    data.insert("idx", "0");
    data.insert("n", "1");
    data.insert("mkt", "zh-CN");

    //if let Ok(res) = post::post_json(bing_url.to_string(),headers,data).await {
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
    
}


#[post("/bing")]
pub async fn bing(bing_info: web::Json<ClientRequest>) -> impl Responder {
    println!("attemp bing");
    if bing_info.session =="123" {
        HttpResponse::Ok().json(BingResponse::<>::success("123".to_string()))
    } else {
        //println!("fail");
        HttpResponse::Forbidden().json(BingResponse::<>::fail("Wrong Session".to_string()))
    }
}
