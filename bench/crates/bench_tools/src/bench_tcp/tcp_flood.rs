
use {
    super::*,
    pnet_datalink::*,
    log::LevelFilter,
    serde::Deserialize,
    std::{
        fs::File,
        thread,
        time::Duration,
        io::{self, Read},
        sync::Arc,
        sync::atomic::{AtomicUsize,Ordering},
        net::{IpAddr,Ipv4Addr,Ipv6Addr},
        fmt::Write,
        num::ParseIntError,
    },
    crate::{
        time_tools
    },
};



static _PACKETS_SEND: AtomicUsize = AtomicUsize::new(0);

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}



//#[no_std]
//#[feature(str_split_as_str)]
pub  fn string_to_ipv4addr( s:&String)->Result<Ipv4Addr,String>{
    let mut ip:[u8;4]=[0,0,0,0];
    let str=s.clone();
    let mut str=str.split(".");
    //Ipv4Addr::from(ip);
    //let mut index:usize=0;
    //println!("{}",s);
    for i in 0..4{
        ip[i]=str.next().unwrap_or("0").parse::<u8>().unwrap();
        //if i<3{
            //index=str.find(".").ok_or("string to ip faild ".to_string())?;
        //}
        //ip[i]=a.parse::<u8>().unwrap();
        //ip[i]=s[0..index].parse::<u8>().unwrap();
        //println!("{}",ip[i]);
        //str=str[index+1..].to_string();
        //println!("remain {}",s);
        
        
    }
    //ip[3]=str.parse::<u8>().unwrap();

    Ok(Ipv4Addr::from(ip))

}
pub async fn run(opt:option::Opt){//thread_num:u32,destination_ip: Ipv4Addr, interface_mac: MacAddr, count: u32){
    //pub async fn bench_http(opt:Opt){    
        //let 

        println!("{}",format!("bench Start"));
        println!("{}",format!("Thread:{}  interface_mac:{} count:{}",opt.threads,opt.interface_mac,opt.count));
        println!("host:{} ",opt.dst_ip.to_string()); 
        //println!("{}",format!("bench Start Mode:{}  URL:{}bench Mode:{}",run_mode, url,bench_mode));
        println!("------Only For Test.Start in 3 sec------");
    
    
        thread::sleep(Duration::from_secs(3));
    
    
        //let url=string_to_static_str(url);
        //l//et run_mode=string_to_static_str(run_mode);
 
        let opt_arc = Arc::new(opt);
        // 循环中创建多个线程
        let mut threads_handle = vec![];
        println!("");
        for i in 0..opt_arc.threads {
            println!(" Create New Thread {}", i);
            let opt_tmp=opt_arc.clone();
            let t=thread::spawn(move || { tcp::packet::send_tcp_packets_flood(i,opt_tmp,&_PACKETS_SEND) });//.join().unwrap();
            threads_handle.push(t);
            
        }
        //timer
        print!("\rCreate Timer Thread {}", opt_arc.threads+1);
        let t=thread::spawn(move || { 
            time_tools::timer(opt_arc.threads+1,&_PACKETS_SEND,5000) });//.join().unwrap();
        threads_handle.push(t);
        
        //let thread_time=thread::spawn(thread_time);
        //threads.push(thread_time);
        log::info!("tcp mode start");
        log::info!("All Create Thread Num:{}",threads_handle.len());
        for t in threads_handle {
            //println!("join");
            t.join().unwrap();
            //log::debug!("http mode start");
        }
    
    }