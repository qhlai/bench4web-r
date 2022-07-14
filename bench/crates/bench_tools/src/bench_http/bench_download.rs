#![forbid(unsafe_code)]
use {
    std::{
        io,
        io::{Write,BufWriter,prelude::*},
        sync::{Arc, Mutex},
        thread,
        process,
        time::Duration,
        fs::File,
        path::Path,
        sync::atomic::{AtomicUsize,Ordering}
    },
    super::{http_request,bench_download,option},
    crate::{gen_random,time_tools},
    reqwest::header::{HeaderMap,HeaderValue},
    reqwest::header,
    tokio::{
        fs::OpenOptions, 
        io::{AsyncWriteExt,AsyncBufReadExt,BufReader,
        },
        runtime::Builder,
    },
    serde::{Deserialize, Serialize},
    serde_json::{Value},
    pbr::{ProgressBar, Units},
    data_encoding::HEXLOWER,
    clap::{App, Arg},
    hyper::Uri,
    digest::Digest,
    sha1::Sha1,
    sha2::Sha256,


};

use super::http_request::http_request_text_block;

const EXIT_URL_FAILURE: i32 = 1;
const EXIT_OUTPUT_FAILURE: i32 = 2;
const DOWNLOAD_BUF_SIZE:usize=8096*5;//u8 5MB

static _PACKETS_SEND: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_os = "macos")]
pub static DISCARD_PATH: &str = "/dev/null";
#[cfg(target_os = "linux")]
pub static DISCARD_PATH: &str = "/dev/null";
#[cfg(target_os = "windows")]
pub static DISCARD_PATH: &str = "nul";
//pub static ADMIN_KEY: &str="12341313";

struct DownloadResult {
    bytes_written: u64,
    sha1: String,
    sha256: String,
}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
fn get_filename(url: &str) -> Option<&str> {
    url.rsplit('/').next()
}

fn write_status(writer: &mut dyn Write, resp: &reqwest::blocking::Response) {
    let _ = writeln!(writer, "{:?} {}", resp.version(), resp.status());
}

fn write_headers(writer: &mut dyn Write, resp: &reqwest::blocking::Response) {
    for (key, value) in resp.headers().iter() {
        let _ = writeln!(writer, "{}: {}", key, value.to_str().unwrap_or(""));
    }
}

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
fn download_min<'a, R: ?Sized, W: ?Sized>(
    reader: &mut R,
    writer: &mut W,
) -> io::Result<u64>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DOWNLOAD_BUF_SIZE];
    let mut written = 0;
    loop {
    let len = match reader.read(&mut buf){
        Ok(0) => {
            return Ok(written);
        }
        Ok(len) => len,
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
        Err(e) => return Err(e),
    };
        // write buf to writer
        writer.write_all(&buf[..len])?;

        // add buf to hash digests
        written += len as u64;
    }
}
fn download_with_progress<'a, R: ?Sized, W: ?Sized>(
    reader: &mut R,
    writer: &mut W,
    progress: &mut ProgressBar<io::Stdout>,
) -> io::Result<DownloadResult>
where
    R: Read,
    W: Write,
{
    let mut buf = [0; DOWNLOAD_BUF_SIZE];
    let mut written = 0;
    let mut sha1_hasher = Sha1::new();
    let mut sha256_hasher = Sha256::new();
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => {
                return Ok(DownloadResult {
                    bytes_written: written,
                    sha1: HEXLOWER.encode(sha1_hasher.finalize().as_slice()),
                    sha256: HEXLOWER.encode(sha256_hasher.finalize().as_slice()),
                })
            }
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        
        // write buf to writer
        writer.write_all(&buf[..len])?;

        // add buf to hash digests
        sha1_hasher.update(&buf[..len]);
        sha256_hasher.update(&buf[..len]);

        // increment progress and bytes written
        progress.add(len as u64);
        written += len as u64;
        //buf.clear();
    }
}
//./target/debug/download.exe  -o rustup.sh https://sh.rustup.rs
//./target/debug/download.exe  -o nul http://cachefly.cachefly.net/10mb.test

pub fn download(url:&str,output:&str,user_agent:&str,max_redirects:usize,show_level:u8) {

    //let url = url;
    //let output = output;
    //let user_agent = user_agent;
    //let max_redirects = max_redirects;
    //let show_level = show_level;//show return header

    // determine an output filename; if none are set then send to stdout
    let uri = url.parse::<Uri>().unwrap();
    let output_path = {
        if output.len()==0 {//output to a file using the same name as the remote
            get_filename(uri.path())
                .and_then(|filename| Some(Path::new(filename)))
        } else {
            get_filename(output)
            .and_then(|path| Some(Path::new(path)))
        }
    };

    // setup client for downloading and send request
    let mut resp = http_download(url, user_agent, max_redirects).unwrap_or_else(|e| {
        let _ = writeln!(&mut io::stderr(), "{}", e);
        process::exit(EXIT_URL_FAILURE);
    });
    match show_level{
        0=>{
            //write_status(&mut io::stdout(), &resp);
            //write_headers(&mut io::stdout(), &resp);
        }
        1=>{
            write_status(&mut io::stdout(), &resp);
            write_headers(&mut io::stdout(), &resp);
        }
        _=>{
    
        }
    }
    // process response
    if let Some(file_path) = output_path {
        let _ = File::create(file_path)
            .and_then(|output_file| {
                let mut writer = BufWriter::new(output_file);

                // setup progress bar based on content-length
                let n_bytes: u64 = resp.headers()
                    .get(header::CONTENT_LENGTH)
                    .and_then(|content_len| content_len.to_str().ok())
                    .and_then(|content_len| content_len.parse().ok())
                    .unwrap_or(0);
                let mut pb = ProgressBar::new(n_bytes);
                pb.set_units(Units::Bytes);

                // copy file with progress updates
                let result = download_with_progress(&mut resp, &mut writer, &mut pb)?;
                writer.flush()?;

                if show_level > 1{
                        println!("sha1({}) = {}",file_path.display(),result.sha1);
                        println!("sha256({}) = {}",file_path.display(),result.sha256);        
                    }

                pb.finish_print("Done.");

                Ok(())
            })
            .map_err(|e| {
                let _ = writeln!(&mut io::stderr(), "{}", e);
                process::exit(EXIT_OUTPUT_FAILURE);
            });
    } 
    else {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut writer = BufWriter::new(lock);

        io::copy(&mut resp, &mut writer).unwrap_or_else(|e| {
            let _ = writeln!(&mut io::stderr(), "{}", e);
            process::exit(EXIT_OUTPUT_FAILURE);
        });
        }

    
    //()
}
pub fn download_min1(url:&str,output:&str,user_agent:&str,max_redirects:usize) {

    //let url = url;
    //let output = output;
    //let user_agent = user_agent;
    //let max_redirects = max_redirects;
    //let show_level = show_level;//show return header

    // determine an output filename; if none are set then send to stdout
    let uri = url.parse::<Uri>().unwrap();
    let output_path = {
        if output.len()==0 {//output to a file using the same name as the remote
            get_filename(uri.path())
                .and_then(|filename| Some(Path::new(filename)))
        } else {
            get_filename(output)
            .and_then(|path| Some(Path::new(path)))
            //get_filename(output)
            //    .and_then(|path| Some(Path::new(path)))
        }
    };
    
    // setup client for downloading and send request
    let mut resp = http_download(url, user_agent, max_redirects).unwrap_or_else(|e| {
        let _ = writeln!(&mut io::stderr(), "{}", e);
        process::exit(EXIT_URL_FAILURE);
    });

    // process response
    if let Some(file_path) = output_path {
        let _ = File::create(file_path)
            .and_then(|output_file| {
                

                // setup progress bar based on content-length
                let n_bytes: u64 = resp.headers()
                    .get(header::CONTENT_LENGTH)
                    .and_then(|content_len| content_len.to_str().ok())
                    .and_then(|content_len| content_len.parse().ok())
                    .unwrap_or(0);

                let mut writer = BufWriter::new(output_file);
                // copy file with progress updates
                let result = download_min(&mut resp, &mut writer)?;
                writer.flush()?;
                log::info!("download bytes:{}",result);
                Ok(())
            })
            .map_err(|e| {
                let _ = writeln!(&mut io::stderr(), "{}", e);
                process::exit(EXIT_OUTPUT_FAILURE);
            });
    } 
    else {
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut writer = BufWriter::new(lock);

        io::copy(&mut resp, &mut writer).unwrap_or_else(|e| {
            let _ = writeln!(&mut io::stderr(), "{}", e);
            process::exit(EXIT_OUTPUT_FAILURE);
        });
        }

}

pub fn download_from_respond(output:&str,mut resp:reqwest::blocking::Response){
    //let uri = url.parse::<Uri>().unwrap();
    let output_path = {
            get_filename(output)
            .and_then(|path| Some(Path::new(path)))
            //get_filename(output)
            //    .and_then(|path| Some(Path::new(path)))
    };

    //let mut resp = super::http_request::http_request_text_block("", target_url.clone(), crate::gen_random::gen_header(target_url, true), "data".to_string(),0).unwrap_or_else(|e| {
    //    let _ = writeln!(&mut io::stderr(), "{}", e);
    //    process::exit(EXIT_URL_FAILURE);
    //});
    //let mut resp=resp;
        //    let _ = writeln!(&mut io::stderr(), "{}", e);
    //let resp=http_request::http_request_text_block("", target_url.clone(), crate::gen_random::gen_header(target_url, true), "data".to_string(),0);
    if let Some(file_path) = output_path {
    let _ = File::create(file_path)
            .and_then(|output_file| {

                let mut writer = BufWriter::new(output_file);
                // copy file with progress updates
                let result = download_min(&mut resp, &mut writer)?;
                writer.flush()?;
                log::info!("download bytes:{}",result);
                Ok(())
            })
            .map_err(|e| {
                let _ = writeln!(&mut io::stderr(), "{}", e);
                process::exit(EXIT_OUTPUT_FAILURE);
            });
    }
}

pub fn download_from_url(output:&str,url:&str){
    let mut rng =rand::thread_rng();
    //let uri = url.parse::<Uri>().unwrap();
    let output_path = {
            get_filename(output)
            .and_then(|path| Some(Path::new(path)))
            //get_filename(output)
            //    .and_then(|path| Some(Path::new(path)))
    };
    let target_url=url.to_string();

    //let mut resp = http_download(url, DEFAULT_USER_AGENT, 0).unwrap_or_else(|e| {
    //    let _ = writeln!(&mut io::stderr(), "{}", e);
    //    process::exit(EXIT_URL_FAILURE);
    //});
    let mut resp = super::http_request::http_request_text_block("", target_url.clone(), gen_random::gen_header(&mut rng ,target_url, true), "data".to_string(),0).unwrap_or_else(|e| {
        let _ = writeln!(&mut io::stderr(), "{}", e);
        process::exit(EXIT_URL_FAILURE);
    });
    //let mut resp = super::http_request::http_request_text_block("", target_url.clone(), crate::gen_random::gen_header(target_url, true), "data".to_string(),0).unwrap_or_else(|e| {
    //    let _ = writeln!(&mut io::stderr(), "{}", e);
    //    process::exit(EXIT_URL_FAILURE);
    //});
    //let resp=super::http_request::http_request_text_block("", target_url.clone(), crate::gen_random::gen_header(target_url, true), "data".to_string(),0);
    if let Some(file_path) = output_path {
    let _ = File::create(file_path)
            .and_then(|output_file| {

                let mut writer = BufWriter::new(output_file);
                // copy file with progress updates
                let result = download_min(&mut resp, &mut writer)?;
                writer.flush()?;
                log::info!("download bytes:{}",result);
                Ok(())
            })
            .map_err(|e| {
                let _ = writeln!(&mut io::stderr(), "{}", e);
                process::exit(EXIT_OUTPUT_FAILURE);
            });
    }
}
//Opt
pub async fn run(opt:option::Opt){

        println!("{}",format!("bench Start"));
        println!("{}",format!("Thread:{} Mode:{} bench Mode:{}",opt.threads,opt.run_mode,opt.bench_mode));
        println!("target url:{} ",opt.url); 
        //println!("{}",format!("bench Start Mode:{}  URL:{}bench Mode:{}",run_mode, url,bench_mode));
        println!("------Only For Test.Start in 5 sec------");
    
        if opt.bench_mode=="download" &&opt.run_mode=="MIX"{
            println!("Download Only Support \"GET\" Method");
        }
    
        std::thread::sleep(Duration::from_secs(5));
    
        let opt_arc = Arc::new(opt);        
        // 循环中创建多个线程
        let mut threads_handle = vec![];
        println!("");
        for i in 0..opt_arc.threads {
            print!("\rCreate New Thread {}", i);
            let opt_tmp=opt_arc.clone();
            let t=thread::spawn(move || { 
                _run(i,opt_tmp) });//.join().unwrap();
            //let t=thread::spawn(move || { thread_fn(i) });//.join().unwrap();
            threads_handle.push(t);
            
        }
        //timer
        print!("\rCreate Timer Thread {}", opt_arc.threads+1);
        let t=thread::spawn(move || { 
            time_tools::timer(opt_arc.threads+1,&_PACKETS_SEND,5000) });//.join().unwrap();
        threads_handle.push(t);


        log::info!("download mode start");
        log::info!("All Create Thread Num:{}",threads_handle.len());
        for t in threads_handle {
            //println!("join");
            t.join().unwrap();
            //log::debug!("http mode start");
        }
    
    }
    
    pub fn  _run(thread_num:u16,opt:Arc<option::Opt>) -> Result<String,String>{
    
        
        let mut target_url:String;
        //let mut data=" ".to_string();
        let data_gen_flag:bool;
        let mut rng =rand::thread_rng();
        data_gen_flag=false;
        log::info!("download at {}",thread_num);
        
        target_url=opt.url.to_string();
        
        loop {
            let mut data="".to_string();
            if data_gen_flag{
                target_url=format!("{}{}{}", opt.url,"?", gen_random::rand_series(&mut rng,8,"all"));
                data=gen_random::rand_series(&mut rng,8,"all");
            }
    
            let user_agent=gen_random::gen_user_agent(&mut rng);
            download_min1(&target_url[..], DISCARD_PATH,&user_agent[..],0);
            //download("https://app.com", DISCARD_PATH,"Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",1,0);
            //bench_download::download_from_respond( super::super::DISCARD_PATH, res);
            /* 
            if let Ok(res) = http_request::http_request_text_block(http_tmp_type, target_url.clone(), header.clone(), data,0){
            log::debug!("block http");
            log::debug!("Now Thread {}:run_times {} ", thread_num,run_times);  
            if download_flag{
                //bench_download::download_from_respond( super::super::DISCARD_PATH, res);
                
            }
           }
            else{
            println!("Error Thread {}:run number {} ", thread_num, run_times);  
            }
             */   
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst); 
        };
        //return Ok(String::from("arrive to count"));    
    }

    
  

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    static DEFAULT_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    #[test]
    fn test_0() {
        download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,0);
    }
    #[test]
    fn test_1() {
        download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,1);

    }
    #[test]
    fn test_2() {
        download("http://cachefly.cachefly.net/10mb.test","1.txt",DEFAULT_USER_AGENT,0,2);

    }
    #[test]
    fn test_3() {
        let mut rng =rand::thread_rng();
        let target_url="http://cachefly.cachefly.net/10mb.test".to_string();
        if let Ok(res) = http_request::http_request_text_block("", target_url.clone(), gen_random::gen_header(&mut rng,target_url, true), "data".to_string(),0){
            println!("Get OK");
           //log::debug!("Now Thread {}:run_times {} ", thread_num,run_times);  
            //download_from_respond("nul", res);
        }
        else{
            //println!("Error Thread {}:run number {} ", thread_num, run_times);  
        }
    
    }
    #[test]
    fn test_4() {
        let target_url="http://cachefly.cachefly.net/10mb.test";
        download_from_url("2.txt",target_url);
    
    }
}
