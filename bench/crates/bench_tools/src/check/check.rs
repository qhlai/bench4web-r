use std::cmp;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::{Duration, Instant};
use  log;


fn benchmark_connection(conn_str: &str, timeout: u8) -> Result<Duration, Error> {
    let now = Instant::now();
    let mut stream = TcpStream::connect(&conn_str)?;
    stream.write_all("GET / HTTP/1.1\r\n\r\n".as_bytes())?;
    let mut buffer = Vec::new();
    stream.set_read_timeout(Some(Duration::from_secs(timeout as u64)))?;
    stream.read_to_end(&mut buffer)?;
    Ok(now.elapsed())
}
fn check_web(){
    colog::init();

    let ip="192.168.3.1";
    let port=80;
    let  timeout=1;
    let conn_str = format!("{}:{}", ip, port);
    if let Ok(dur) = benchmark_connection(&conn_str, timeout) {
        log::info!("Server is up. Connected in {}s ({} ns).", dur.as_secs(), dur.as_nanos());
    } else {
        log::error!("Connection failed. Is the server up?");
        panic!();
    }


    loop {
        match benchmark_connection(&conn_str, 1) {
            Ok(dur) => log::info!("Server response in {}s ({} ns).", dur.as_secs(), dur.as_nanos()),
            Err(_) => log::warn!("Failed to benchmark. Is the server choking?"),
        }
        sleep(Duration::from_secs(1 as u64));
    }
}
