use std::{collections::HashMap, string};
use hyper::http::response;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
//use crate::custom_rand_bak;
//use super::{custom_rand};
use crate::{gen_random};
use reqwest::RequestBuilder;


#[tokio::main]
pub async fn http_request_text(http_type:&str,url:String,headers:reqwest::header::HeaderMap,data:String,max_redirects:usize) -> Result<reqwest::Response, reqwest::Error> {
    let client =reqwest::Client::builder()
    .redirect(reqwest::redirect::Policy::limited(max_redirects))
    .build()?;

    let mut http_res_build=client.get("");
    //let mut http_res_build=reqwest::RequestBuilder::new(client,Result);
    match http_type{
        "GET"=>{
            http_res_build =client.get(url.clone());
        }
        "POST"=>{
            http_res_build =client.post(url.clone());
        }
        _=>{
            print!("No HTTP Type Named:{} ",http_type);

        }
    }
    let res=http_res_build.headers(headers).body(data).send().await?;
    println!("Status: {} url:{}", res.status(),url);
    Ok(res)
}
#[tokio::main]
pub async fn http_request_json(http_type:&str,url:String,headers:reqwest::header::HeaderMap,data:HashMap<&str, &str>) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let mut http_res_build:reqwest::RequestBuilder=client.get(url.clone());
    match http_type{
        "GET"=>{
            http_res_build =client.get(url.clone());
        }
        "POST"=>{
            http_res_build =client.post(url.clone());
        }
        _=>{
            print!("No HTTP Type Named:{} ",http_type);
        }
    }
    
    let res=http_res_build.headers(headers).json(&data).send().await?;
    print!("Status: {} ", res.status());

    Ok(res)
}


pub fn http_request_text_block(http_type:&str,url:String,headers:reqwest::header::HeaderMap,data:String,max_redirects:usize) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(max_redirects))
        .build()?;
    let mut http_res_build=client.get("");
    match http_type{
        "GET"=>{
            http_res_build =client.get(url.clone());
        }
        "POST"=>{
            http_res_build =client.post(url.clone());
        }
        _=>{
            print!("No HTTP Type Named:{} ",http_type);
        }
    }

    let res=http_res_build.headers(headers).body(data).send()?;

    log::info!("Status: {} url:{}", res.status(),url);
    //println!("testa");
    Ok(res)
}

use reqwest::header;
fn http_download(
    url: &str,
    user_agent: &str,
    max_redirects: usize,
) -> reqwest::Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(max_redirects))
        .build()?;

    let ua_header = header::HeaderValue::from_str(user_agent).unwrap();
    let resp = client
        .get(url)
        .header(header::USER_AGENT, ua_header)
        .send()?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_0() {
        let mut rng =rand::thread_rng();
        let target_url="http://cachefly.cachefly.net/10mb.test".to_string();
        let http_tmp_type="GET";
        http_request_text(http_tmp_type, target_url.clone(), gen_random::gen_header(&mut rng,target_url.clone(), true), "data".to_string(),0).unwrap();
        //download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
    #[test]
    fn test_1() {
        let mut rng =rand::thread_rng();
        let target_url="http://cachefly.cachefly.net/10mb.test".to_string();
        let http_tmp_type="GET";
        http_request_text_block(http_tmp_type, target_url.clone(), gen_random::gen_header(&mut rng,target_url.clone(), true), "data".to_string(),0).unwrap();
        //download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
    #[test]
    fn test_2() {
        let mut rng =rand::thread_rng();
        let target_url="http://cachefly.cachefly.net".to_string();
        let http_tmp_type="GET";
        http_request_text(http_tmp_type, target_url.clone(), gen_random::gen_header(&mut rng,target_url.clone(), true), "data".to_string(),0).unwrap();
        //download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
}