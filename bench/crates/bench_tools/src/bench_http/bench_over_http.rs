use {
    std::{
        io::Write,
        sync::{Arc, Mutex},
        thread,
        time::Duration,
        fs,
        sync::atomic::{AtomicUsize,Ordering}
    },
    super::{http_request,bench_download,option},
    crate::{time_tools,gen_random},
    reqwest::header::HeaderMap,
    tokio::{
        fs::OpenOptions, 
        io::{AsyncWriteExt,AsyncBufReadExt,BufReader,
        },
        runtime::Builder,
    },
    serde::{Deserialize, Serialize},
    serde_json::{Value},
};

static _PACKETS_SEND: AtomicUsize = AtomicUsize::new(0);
/* 
use reqwest::header::HeaderMap;
#[tokio::main]
pub async fn get_min(url:String) -> Result<reqwest::Response, reqwest::Error> {
    let res = reqwest::get(url.to_string()).await?;
    print!("Status: {} ", res.status());
    //let body = res.text().await?;
    //println!("Body:\n{}", body);
    Ok(res)
}
*/
//use anyhow::{anyhow, Result};

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
async fn save(filename: &str, response: &mut reqwest::Response) -> Result<(),std::io::Error> {
    let mut options = OpenOptions::new();
    let mut file = options
        .append(true)
        .create(true)
        .read(true)
        .open(filename)
        .await?;
    println!("file open success");
    while let Some(chunk) = &response.chunk().await.expect("Failed") {
        match file.write_all(&chunk).await {
            Ok(_) => {}
            Err(e) => {
                println!("File {} save error: {}", filename, e);
                return Err(e);
            }
        }
    }
    Ok(())
}
//pub fn http_type()-> &str{
//    let a="s";
//    return a;
//
//}



pub async fn run(opt:option::Opt){//thread_num:u32,run_mode:String,url:String,bench_mode:String){

    println!("{}",format!("bench Start"));
    println!("{}",format!("Thread:{} Mode:{} bench Mode:{}",opt.threads,opt.run_mode,opt.bench_mode));
    println!("target url:{} ",opt.url); 
    //println!("{}",format!("bench Start Mode:{}  URL:{}bench Mode:{}",run_mode, url,bench_mode));
    println!("------Only For Test.Start in 5 sec------");

    thread::sleep(Duration::from_secs(5));
    //lt

    //print!("url:{} ",url);
    let opt_arc = Arc::new(opt);


    // 循环中创建多个线程
    println!("");
    let mut threads_handle = vec![];
    for i in 0..opt_arc.threads {
        print!("\rCreate New Thread {}", i);
        let opt_tmp=opt_arc.clone();
        let t=thread::spawn(move || { 
            _run(i,opt_tmp) });//.join().unwrap();
        //let t=thread::spawn(move || { thread_fn(i) });//.join().unwrap();
        threads_handle.push(t);
        
    }
    //timer
    print!("\rCreate timer Thread {}", opt_arc.threads+1);
    let t=thread::spawn(move || { 
        time_tools::timer(opt_arc.threads+1,&_PACKETS_SEND,5000) });//.join().unwrap();
    threads_handle.push(t);

    //let thread_time=thread::spawn(thread_time);
    //threads.push(thread_time);
    log::info!("http mode start");
    log::info!("All Create Thread Num:{}",threads_handle.len());
    for t in threads_handle {
        //println!("join");
        t.join().unwrap();
        //log::debug!("http mode start");
    }

}
pub fn  _run(thread_num:u16,opt:Arc<option::Opt>)  -> Result<String,String>{

    let mut target_url:String;
    //let mut data=" ".to_string();
    let mut data_gen_flag=false;
    let mut http_block_flag=false;
    let mut download_flag=false;
    let mut http_mix_flag=false;
    //let count=0;
    let mut rng =rand::thread_rng();
    //let start_time=time_tools::time_stamp();
    match &opt.bench_mode[..]{
        "static"=>{
            log::debug!("static");
            target_url=opt.url.to_string();
            //data=" ".to_string();
            //let dynamic_url=format!("{}{}{}", url,"/login.php?username=admin&passwd=", gen_random::rand_series(8,2));
        }
        "dynamic"=>{
            //target_url=format!("{}{}{}", url,"/login.php?username=admin&passwd=", gen_random::rand_series(8,"all"));
            log::debug!("dynamic");
            target_url=opt.url.to_string();
            data_gen_flag=true;
            //data=gen_random::rand_series(8,"all");
        }
        _=>{
            target_url=opt.url.to_string();
            log::warn!("Wrong bench mode,use default(static)");      
        }
    };
    
    //let mut http_origin_type:&str;
    match &opt.run_mode[..]{
        "GET,POST" =>{ 
            if download_flag==false{
                http_mix_flag=true;
            }else{
                //http_origin_type="GET";
            }
            
        }
        "GET" =>{
            //http_origin_type="GET";
        }
        "POST" =>{
            //http_origin_type="POST";
        }
        _=>{
         //   http_origin_type="GET";            
            log::warn!("Wrong http_tmp_type");


        }
    }

    let mut run_times:u32=0;
    
    let header=gen_random::gen_header(&mut rng,target_url.clone(), true);
    loop {
        run_times+=1;
        let mut data="".to_string();
        if data_gen_flag{
            target_url=format!("{}{}{}", opt.url,"?", gen_random::rand_series(&mut rng,8,"all"));
            data=gen_random::rand_series(&mut rng,8,"all");
        }
        //let mut http_tmp_type:&str;
        let http_tmp_type:&str;
        if http_mix_flag{
            match gen_random::gen_num(&mut rng,0, 2){
                0=>{
                    http_tmp_type="GET";
                }
                1=>{
                    http_tmp_type="POST";
                }
                _=>{
                    http_tmp_type="GET";
                    println!("Wrong http_tmp_type,use default(GET)");   
                }
            }
        }
        else if download_flag {
                http_tmp_type="GET";
        }
        else{
            http_tmp_type=&opt.run_mode[..];
        }

        if http_block_flag==false{
           if let Ok(_) = http_request::http_request_text(http_tmp_type, target_url.clone(), header.clone(), data,0){
                //println!("Get OK");
                log::debug!("\rNow Thread {}:run_times {} ", thread_num,run_times);  
            }
            else{
                print!("\rError Thread {}:run number {} ", thread_num, run_times);  
            }
        }
        else{
             if let Ok(_res) = http_request::http_request_text_block(http_tmp_type, target_url.clone(), header.clone(), data,0){
                log::debug!("block http");
                log::debug!("Now Thread {}:run_times {} ", thread_num,run_times);  
            }
            else{
                println!("Error Thread {}:run number {} ", thread_num, run_times);  
            }
            
        }
        _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);

    };
   // return Ok(String::from("arrive to count"));    
}


#[cfg(test)]
mod tests {
    use super::*;
    //let target_url="http://cachefly.cachefly.net/10mb.test";
    #[test]
    fn test_download() {
        //_run(0,"MIX","http://192.168.191.31:62013/10mb.test","download").unwrap();
        //download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
    #[test]
    fn test_mix() {
        //_run(0,"MIX","http://192.168.1.1/","dynamic").unwrap();
        //download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
}