

#![forbid(unsafe_code)]
use std::time::{SystemTime, UNIX_EPOCH};
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
    }

};

pub fn time_stamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
pub fn timer(thread_num:u16,operate_num: &AtomicUsize,display_interval_ms:u64)  -> Result<String,String>{

    let start_time=time_tools::time_stamp();

    //log::info!("timer thread start");
     
    loop {
        std::thread::sleep(Duration::from_millis(display_interval_ms as u64));

        //operate_num.fetch_add(1, Ordering::SeqCst)
        let operate_run_num=operate_num.load(Ordering::Relaxed);
        let run_ms=(time_tools::time_stamp()-start_time) as usize ;
        //log::info!("Sent {:?} packets,{}packets/sec, in {} sec", &operate_run_num,&operate_run_num/run_ms*1000 , run_ms/1000);
    };
    return Ok(String::from("arrive to count"));    
}