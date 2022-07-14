use std::env;
use std::net::UdpSocket;
use std::process::exit;
use std::thread;
use std::time;
use super::*;
use rand::Rng;
use crate::time_tools;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize,Ordering};
static _PACKETS_SEND: AtomicUsize = AtomicUsize::new(0);
fn new_socket(host: &str, port: u32) -> UdpSocket {
    let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).unwrap();
    //println!("{:?}",host);
    //let host=format!("0.0.0.0:{}", port);
    socket.connect(host).unwrap();
    return socket;
}
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
pub fn run(opt:option::Opt){
  
    
    log::info!("Starting threads...");
    //let mut start_port: u32 = 40000;  
    //let port_step=30;

    let opt_arc = Arc::new(opt);    
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
    print!("\rCreate Timer Thread {}", opt_arc.threads+1);
    let t=thread::spawn(move || { 
        time_tools::timer(opt_arc.threads+1,&_PACKETS_SEND,5000) });
    threads_handle.push(t);

    log::info!("udp mode start");
    log::info!("All Create Thread Num:{}",threads_handle.len());
    for t in threads_handle {
        t.join().unwrap();
    }
}

pub fn  _run(thread_num:u16,opt:Arc<option::Opt>)  -> Result<String,String>{
    
    log::info!("Starting simulated bench on thread {}...", thread_num);
    let host =format!("{}:{}", opt.dst_ip,opt.dst_port);
    let mut src_port = opt.start_port.clone() + opt.ports_per_thread*opt.threads;
    let mut socket_list = Vec::new();
    for _ in 1..opt.ports_per_thread {
        src_port = src_port.clone() + 1;
        //let host =format!("{}:{}", ip,port);

        let socket = new_socket(&host[..], src_port as u32);
        socket_list.push(socket);
    }

    let msg = rand::thread_rng().gen::<[u8; 32]>();
    loop {
        for socket in &socket_list {
            socket.send(&msg).unwrap();
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
        }
    };
    //log::error!("thread died");
    //return Err(String::from("thread died"));    
}