use std::{collections::HashMap, string};
use reqwest::header::HeaderMap;
use crate::tools::post;
/* 
#[tokio::main]
pub async fn get_min(url:String) -> Result<(), reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    println!("Status: {}", res.status());
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(())
}
*/
#[tokio::main]
pub async fn get_min(url:String) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,en-US;q=0.7,en;q=0.3".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cache-Control", "max-age=0, no-cache".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Length", "0".parse().unwrap());
    //headers.insert("Host", "flios.top".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    headers.insert("Cookie", "MUID=163A307441A0601E2F5C203040E36195; _EDGE_V=1; SRCHD=AF=UNKSBD; SRCHUID=V=2&GUID=D89881C6E47048BEA298D2393B57E3DD&dmnchg=1; SRCHUSR=DOB=20210527&T=1624248731000; SRCHHPGUSR=SRCHLANGV2=zh-Hans&BRW=M&BRH=S&CW=1280&CH=547&DPR=1.5&UTC=480&DM=0&WTS=63758756672&HV=1626270368&PLTL=867&PLTA=938&PLTN=75&EXLTT=31&SRCHLANG=zh-Hans&SW=1280&SH=720; ABDEF=V=13&ABDV=13&MRB=1626144651000&MRNB=0; BFBUSR=BAWAS=1&BAWFS=1; OID=AhBHBXx2pTtPIrD4vJNE5M3L_DAvjnBDA98VDVYLXQefr3WwCU-xK_jBvTgzbF9_xXadZnvf3j5cvnBDQO0qqMpae9vZ2MYbL1…bpbg4jU3nWBGgZR5GlH42pR-o_2D6B4KElI6S8wq6W-QP0KmOYsw43F45dsB0QIs7wOu1XsTZVrTKH4PXfl55rK0-30CwPcD3Gw0Vl42RpHxVJF6pZ7gdxX8MK8E0wH1zBULMxMxEA5sMCYawFA_bpKZuUYpmm_ESvT85Cvy_jsLGStO; MUIDB=163A307441A0601E2F5C203040E36195; _tarLang=default=zh-Hans; _TTSS_IN=hist=WyJlbiIsImF1dG8tZGV0ZWN0Il0=; _TTSS_OUT=hist=WyJ6aC1IYW5zIl0=; SNRHOP=I=&TS=; _EDGE_S=SID=329ADE4768BF67B71180CE3569F566F6&mkt=zh-cn; _SS=SID=329ADE4768BF67B71180CE3569F566F6&bIm=615983; ipv6=hit=1626270501833&t=4; OPF=X=1; ENSEARCH=BENVER=0; _FP=hta=on".parse().unwrap());
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0".parse().unwrap());
    
    let res =client.get(url).headers(headers).send().await?;
    //let res = reqwest::get(url.to_string()).await?;
    print!("Status: {} ", res.status());
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(res)
}
#[tokio::main]
pub async fn get_medium(url:String) -> Result<reqwest::Response, reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    println!("Status: {}", res.status());
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(res)
}
#[tokio::main]
pub async fn get_norm(url:String) -> Result<(), reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}
#[tokio::main]
pub async fn get_str(url:String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    println!("Status: {}", res.status());
    let body = res.text().await?;
    //println!("Body:\n{}", body);
    return Ok(body);
}
//get::get_str_show(url.to_string(),None)
//pub async fn get_str_show(url:String,show:Option<bool>) -> Result<String, reqwest::Error>
#[tokio::main]
pub async fn get_str_show(url:String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    print!("Status: {} ", res.status());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    return Ok(body);
}

#[tokio::main]
pub async fn get_json(url:String) -> Result<HashMap<String, String>, reqwest::Error>{
    let res = reqwest::get(url).await?;
    //println!("Status: {}", res.status());
    let body = res.json::<HashMap<String, String>>().await?;
    //println!("Body:\n{}", body);
    
    return Ok(body);
    //Ok(reqwest::get(url).await?.json::<HashMap<String, String>>().await?)
}
#[tokio::main]
pub async fn get_json_show(url:String) -> Result<HashMap<String, String>, reqwest::Error>{
    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    let body = res.json::<HashMap<String, String>>().await?;
    //println!("Body:\n{}", body);
    return Ok(body);
    //Ok(reqwest::get(url).await?.json::<HashMap<String, String>>().await?)
}
/*
#[tokio::main]
pub async fn get_output(url:String) -> String{
    //let res = get(url.to_string());
    //println!("{:#?}", res);
    if let Ok(res) = get_json(url.to_string()).await {
        println!("GET OK");
        println!("{:#?}", res);
        return format!("{}:{}", "GET OK".to_string(),url);
    }
    else{
        //println!("{:#?}", res);
        println!("GET ERROR");
        return format!("{}:{}", "GET ERROR".to_string(), url);//.to_string();
    }

}
*/
//#[tokio::main]
//async fn get(url:String) -> Result<HashMap<String, String>, reqwest::Error>{
//    Ok(reqwest::get(url).await?.json::<HashMap<String, String>>().await?)
//} 
/*
    if let Ok(res) = get::get_str_show(url.to_string())
    {
        println!("{:#?}", res);
    }
    if let Ok(res) = get::get_json(url.to_string())
    {
        println!("{:#?}", res);
    }
*/
//#[tokio::main]
pub fn test() -> u8{
    println!("get test");
    let url="https://httpbin.org/ip";
    if let Ok(res) = get_json(url.to_string()){
        println!("{:#?}", res);
    }

    println!("post test");
    let url="https://httpbin.org/post";
    // 组装header
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    // 组装要提交的数据
    let mut data: HashMap<&str, &str> = HashMap::new();
    data.insert("user", "zhangsan");
    data.insert("password", "https://docs.rs/serde_json/1.0.59/serde_json/");

    if let Ok(res) = post::post_json(url.to_string(),headers,data){
        //println!("{:#?}", res);
        let ip = res.get("origin").unwrap().clone().to_string();  
        println!("ip:{}", ip);
        //res.values()
    }
  
    return 0;
}
