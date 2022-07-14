#![forbid(unsafe_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[macro_use]
extern crate log;

extern crate rand;
//https://dev.to/basman/lookup-txt-records-using-rust-trust-dns-resolver-1mhk
use {
    std::{
        io,env,
        vec::Vec,
        collections::HashMap,
        string,
        thread,
        time::Duration,
        sync::{Mutex, Arc},
    },
    rand::Rng,
    clap::{load_yaml, value_t, value_t_or_exit, App, Arg, ArgMatches},
    rust_web_bench::{
        //time_full,
        pause,
        //tools::{
        //    get,post,
        //    debug::logger,
        //    //system::{my_shell,status},
        //},
    },
    bench_tools::*
};

//cargo build --bin main --release
#[tokio::main]
async fn main() {

    let args_def = load_yaml!("web_bench_r.yaml");
    let args = App::from_yaml(args_def).get_matches();

    let filename = args.value_of("config").unwrap_or("./default.json");
    if let Err(e) = bench_tools::launch_from_config_filename_json(filename).await {
        println!("failed to launch bench: {}", e);
        //pause::pause();
    }
    pause::pause();
    //thread::sleep(Duration::from_secs(60));
}

mod tests {
    use futures::TryFutureExt;

    use super::*;
    //use super::super::*;
    #[test]
     fn http() {
         let filename="../../src/config_example/json/1.json";
         bench_tools::launch_from_config_filename_json(filename); 
    }
}
