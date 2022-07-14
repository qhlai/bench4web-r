use actix_web::http::header;
use actix_web::{App, HttpServer};

use std::io::Read;
use crate::url2domain::{slice_domain_from_url};
use std::{collections::HashMap, string};
//use reqwest::header::HeaderMap;

extern crate rand;
use rand::Rng;
use rand::distributions::Uniform;
use reqwest::header::HeaderMap;
use std::net::{Ipv4Addr,Ipv6Addr};


pub fn rand_one<R>(rng: &mut R,gen_type:&str) -> String
where
    R: rand::Rng,
{
    let cha_dir=String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let num_dir=String::from("0123456789");
    let all_dir=String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    //let mut rng =rand::thread_rng();
    let mut use_dict:String;
    match gen_type{
        "num" =>{
            use_dict=num_dir;
            //let mut rng =rand::thread_rng();
            //let gen = rng.gen_range(0..10);
            //return num_dir[gen..gen+1].to_string();
        }
        "char" =>{
            use_dict=cha_dir;
            //let mut rng =rand::thread_rng();
            //let gen = rng.gen_range(0..26);
            //return cha_dir[gen..gen+1].to_string();
        }
        "all" =>{
            use_dict=all_dir;
            //let mut rng =rand::thread_rng();
            //let gen = rng.gen_range(0..all_dir.len());
            //return all_dir[gen..gen+1].to_string();
        }
        _ => {
            println!("Gen rand error");
            return "ERROR".to_string();
        }
    }
    //let mut rng =rand::thread_rng();
    let gen = rng.gen_range(0..use_dict.len());
    return use_dict[gen..gen+1].to_string();
}

pub fn rand_series<R>(rng: &mut R,num:i8,gen_type:&str)-> String
where
    R: rand::Rng,
{
    let mut str:String="".to_string();
    for i in 0..num{
        str += &rand_one(rng,gen_type);
    }
    return str;
}
/* 
pub fn gen_ipv4()->String{
    let mut rng = rand::thread_rng();
    let ip = Ipv4Addr::from(rng.gen::<u32>());
    return ip.to_string();
} 
pub fn gen_ipv6()->String{
    let mut rng = rand::thread_rng();
    let ip = Ipv6Addr::from(rng.gen::<u128>());
    return ip.to_string();
} */
pub fn gen_ipv4<R>(rng: &mut R)->Ipv4Addr
where
    R: rand::Rng,
{
    //let mut rng = rand::thread_rng();
    let ip = Ipv4Addr::from(rng.gen::<u32>());
    return ip;
} 
pub fn gen_ipv6<R>(rng: &mut R)->Ipv6Addr
where
    R: rand::Rng,
{
    //let mut rng = rand::thread_rng();
    let ip = Ipv6Addr::from(rng.gen::<u128>());
    return ip;
} 
pub fn gen_ipv4_global<R>(rng: &mut R)->String
where
    R: rand::Rng,
{
    let mut rng = rand::thread_rng();
    loop{
        let ip = Ipv4Addr::from(rng.gen::<u32>());
        //if ip.is_global() {
            return ip.to_string();
    }
    
    //return ip.to_string();
} 
pub fn gen_port<R>(min_port:u16,max_port:u16,rng: &mut R)->u16
where
    R: rand::Rng,
{
    let distribution = Uniform::new_inclusive(min_port, max_port);
    rng.sample(distribution)
} 

pub fn gen_user_agent<R>(rng: &mut R)->String
where
    R: rand::Rng,
{
    let ua=[
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:89.0) Gecko/20100101 Firefox/89.0",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36 OPR/26.0.1656.60",
        "Opera/8.0 (Windows NT 5.1; U; en)",
        "Mozilla/5.0 (Windows NT 5.1; U; en; rv:1.8.1) Gecko/20061208 Firefox/2.0.0 Opera 9.50",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; en) Opera 9.50",
        "Opera/9.80 (Macintosh; Intel Mac OS X 10.6.8; U; en) Presto/2.8.131 Version/11.11",
        "Opera/9.80 (Windows NT 6.1; U; en) Presto/2.8.131 Version/11.11",
        "Opera/9.80 (Android 2.3.4; Linux; Opera Mobi/build-1107180945; U; en-GB) Presto/2.8.149 Version/11.10",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:34.0) Gecko/20100101 Firefox/34.0",
        "Mozilla/5.0 (X11; U; Linux x86_64; zh-CN; rv:1.9.2.10) Gecko/20100922 Ubuntu/10.10 (maverick) Firefox/3.6.10",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.6; rv,2.0.1) Gecko/20100101 Firefox/4.0.1",
        "Mozilla/5.0 (Windows NT 6.1; rv,2.0.1) Gecko/20100101 Firefox/4.0.1",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/534.57.2 (KHTML, like Gecko) Version/5.1.7 Safari/534.57.2",
        "MAC：Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.122 Safari/537.36",
        "Windows：Mozilla/5.0 (Windows; U; Windows NT 6.1; en-us) AppleWebKit/534.50 (KHTML, like Gecko) Version/5.1 Safari/534.50",
        "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (iPad; U; CPU OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.11 (KHTML, like Gecko) Chrome/23.0.1271.64 Safari/537.11",
        "Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US) AppleWebKit/534.16 (KHTML, like Gecko) Chrome/10.0.648.133 Safari/534.16",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_0) AppleWebKit/535.11 (KHTML, like Gecko) Chrome/17.0.963.56 Safari/535.11",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/30.0.1599.101 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; 360SE)",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/536.11 (KHTML, like Gecko) Chrome/20.0.1132.11 TaoBrowser/2.0 Safari/536.11",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.1 (KHTML, like Gecko) Chrome/21.0.1180.71 Safari/537.1 LBBROWSER",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0; SLCC2; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30729; Media Center PC 6.0; .NET4.0C; .NET4.0E; LBBROWSER)",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; QQDownload 732; .NET4.0C; .NET4.0E; LBBROWSER)",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0; SLCC2; .NET CLR 2.0.50727; .NET CLR 3.5.30729; .NET CLR 3.0.30729; Media Center PC 6.0; .NET4.0C; .NET4.0E; QQBrowser/7.0.3698.400)",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1; SV1; QQDownload 732; .NET4.0C; .NET4.0E)",
        "Mozilla/5.0 (Windows NT 5.1) AppleWebKit/535.11 (KHTML, like Gecko) Chrome/17.0.963.84 Safari/535.11 SE 2.X MetaSr 1.0",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; SV1; QQDownload 732; .NET4.0C; .NET4.0E; SE 2.X MetaSr 1.0)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Trident/4.0; SE 2.X MetaSr 1.0; SE 2.X MetaSr 1.0; .NET CLR 2.0.50727; SE 2.X MetaSr 1.0)",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Maxthon/4.4.3.4000 Chrome/30.0.1599.101 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_7_0) AppleWebKit/535.11 (KHTML, like Gecko) Chrome/17.0.963.56 Safari/535.11",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/38.0.2125.122 UBrowser/4.0.3214.0 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2883.87 UBrowser/6.2.4094.1 Safari/537.36",
        "Mozilla/5.0 (iPhone; U; CPU iPhone OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (iPod; U; CPU iPhone OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (iPad; U; CPU OS 4_2_1 like Mac OS X; zh-cn) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8C148 Safari/6533.18.5",
        "Mozilla/5.0 (iPad; U; CPU OS 4_3_3 like Mac OS X; en-us) AppleWebKit/533.17.9 (KHTML, like Gecko) Version/5.0.2 Mobile/8J2 Safari/6533.18.5",
        "Mozilla/5.0 (Linux; U; Android 2.2.1; zh-cn; HTC_Wildfire_A3333 Build/FRG83D) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1",
        "Mozilla/5.0 (Linux; U; Android 2.3.7; en-us; Nexus One Build/FRF91) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1",
        "MQQBrowser/26 Mozilla/5.0 (Linux; U; Android 2.3.7; zh-cn; MB200 Build/GRJ22; CyanogenMod-7) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1",
        "Opera/9.80 (Android 2.3.4; Linux; Opera Mobi/build-1107180945; U; en-GB) Presto/2.8.149 Version/11.10",
        "Mozilla/5.0 (Linux; U; Android 3.0; en-us; Xoom Build/HRI39) AppleWebKit/534.13 (KHTML, like Gecko) Version/4.0 Safari/534.13",
        "Mozilla/5.0 (BlackBerry; U; BlackBerry 9800; en) AppleWebKit/534.1+ (KHTML, like Gecko) Version/6.0.0.337 Mobile Safari/534.1+",
        "Mozilla/5.0 (hp-tablet; Linux; hpwOS/3.0.0; U; en-US) AppleWebKit/534.6 (KHTML, like Gecko) wOSBrowser/233.70 Safari/534.6 TouchPad/1.0",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0;",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 6.0)",
        "Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.0; Trident/4.0)",
        "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; The World)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; TencentTraveler 4.0)",
        "Mozilla/4.0 (compatible; MSIE 7.0; Windows NT 5.1; Avant Browser)",
        "Mozilla/5.0 (Linux; U; Android 2.3.7; en-us; Nexus One Build/FRF91) AppleWebKit/533.1 (KHTML, like Gecko) Version/4.0 Mobile Safari/533.1",
        "Mozilla/5.0 (SymbianOS/9.4; Series60/5.0 NokiaN97-1/20.0.019; Profile/MIDP-2.1 Configuration/CLDC-1.1) AppleWebKit/525 (KHTML, like Gecko) BrowserNG/7.1.18124",
        "Mozilla/5.0 (compatible; MSIE 9.0; Windows Phone OS 7.5; Trident/5.0; IEMobile/9.0; HTC; Titan)",
        "UCWEB7.0.2.37/28/999",
        "NOKIA5700/ UCWEB7.0.2.37/28/999",
        "Openwave/ UCWEB7.0.2.37/28/999",
        "Openwave/ UCWEB7.0.2.37/28/999",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_6) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:49.0) Gecko/20100101 Firefox/49.0",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_1) AppleWebKit/602.2.14 (KHTML, like Gecko) Version/10.0.1 Safari/602.2.14",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12) AppleWebKit/602.1.50 (KHTML, like Gecko) Version/10.0 Safari/602.1.50",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2840.71 Safari/537.36",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
        "Mozilla/5.0 (Windows NT 6.3; rv:36.0) Gecko/20100101 Firefox/36.0",
        "Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2785.143 Safari/537.36",
        "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:49.0) Gecko/20100101 Firefox/49.0",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/79.0.3945.88 Safari/537.36",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1",
        "Mozilla/4.0 (compatible; MSIE 9.0; Windows NT 6.1)",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.141 Safari/537.36 Edg/87.0.664.75",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.18363",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.149 Safari/537.36",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 14_4 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Mobile/15E148 Safari/604.1",
        "Mozilla/5.0 (iPhone; CPU iPhone OS 13_1_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.1 Mobile/15E148 Safari/604.1",
        "Opera/9.80 (Windows NT 5.1; U; en) Presto/2.10.289 Version/12.01"
        ];
        //let mut rng =rand::thread_rng();
        let gen = rng.gen_range(0..ua.len());
        return ua[gen].to_string();
} 
pub fn gen_num<R>(rng: &mut R,start:i32,end:i32)->i32
where
    R: rand::Rng,
{
    //let mut rng =rand::thread_rng();
    let gen = rng.gen_range(start..end);
    return gen;
}
pub fn gen_cookie<R>(rng: &mut R,num:i8)->String
where
    R: rand::Rng,
{
    return rand_series(rng,num,"all")
}
pub fn gen_referer<R>(rng: &mut R)->String
where
    R: rand::Rng,
{
    let referer=[
        
        "https://www.qq.com",
        "https://www.sina.com",
        "https://www.weibo.com",
        "https://www.baidu.com",

        "https://www.bing.com",       
        "https://www.google.com",
        "https://www.yahoo.com",
        
        "https://www.facebook.com",
        "https://www.twitter.com",
        "https://www.quora.com",
        "https://www.ibm.com",
        "https://www.bbc.com",     
        ];
        //let mut rng =rand::thread_rng();
        let gen = rng.gen_range(0..referer.len());
        return referer[gen].to_string();
}
pub fn gen_header<R>(rng: &mut R,url:String,x_forward_for:bool)-> reqwest::header::HeaderMap
where
    R: rand::Rng,
{
    let mut headers = HeaderMap::new();
    headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;".parse().unwrap());
    headers.insert("Accept-Encoding", "gzip, deflate, br".parse().unwrap());
    headers.insert("Accept-Language", "en-US,en-UK,zh-HK,zh-TW,zh-SG,zh-CN;q=0.7,en;q=0.3".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Cache-Control", "max-age=0, no-cache".parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Content-Length", "0".parse().unwrap());
    let host=slice_domain_from_url(url).unwrap_or("ibm.com".to_string());
    headers.insert("Host", host.parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Upgrade-Insecure-Requests", "1".parse().unwrap());
    //headers.insert("Cookie", gen_cookie(12).parse().unwrap());
    headers.insert("User-Agent", gen_user_agent(rng).parse().unwrap());
    headers.insert("Referer", gen_referer(rng).parse().unwrap());  
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());  
    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap()); 
    headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap()); 
    
    //let cookie=format!("kYaj_2132_saltkey={}; kYaj_2132_lastvisit={}; kYaj_2132_sid={}; kYaj_2132_lastact={}%09search.php%09forum; kYaj_2132_seccode=12.{}",rand_series(8,"all"),rand_series(10,"num"),rand_series(5,"all"),rand_series(10,"num"),rand_series(18,"all"));
    let mut cookie=String::new();
    cookie+=&format!("kYaj_2132_saltkey={}; ",rand_series(rng,8,"all"));
    cookie+=&format!("kYaj_2132_lastvisit={}; ",rand_series(rng,10,"num"));
    cookie+=&format!("kYaj_2132_sid={}; ",rand_series(rng,5,"all"));
    cookie+=&format!("kYaj_2132_lastact={}%09search.php%09forum; ",rand_series(rng,10,"num"));
    cookie+=&format!("kYaj_2132_seccode=12.{}; ",rand_series(rng,18,"all"));
    cookie+=&format!("cf_chl_prog=a11; ");
    cookie+=&format!("cf_clearance=jaao0nrwjxkSpQeO4R0i2VN2nmLiqLleriONueIuqXQ-1629637273-0-250; ");
    cookie+=&format!("WHMCSy551iLvnhYt7=id64vkos8c7epbr287iggjd195; ");

    headers.insert("Cookie", cookie.parse().unwrap());
    /*
    if url.len() == 0 {
        headers.insert("Referer", url.parse().unwrap());  
    }else{
        headers.insert("Referer", gen_referer().parse().unwrap());  
    }
    */
    if x_forward_for==true {
        let fake_ip=gen_ipv4(rng);
        //HTTP_CF_CONNECTING_IP
        headers.insert("X-Forwarded-For", fake_ip.to_string().parse().unwrap());
        headers.insert("X-Real-IP", fake_ip.to_string().parse().unwrap());
        headers.insert("REMOTE-ADDR", fake_ip.to_string().parse().unwrap());
        //headers.insert("CF-Connecting-IP", fake_ip.parse().unwrap());
        //CF-Connecting-IP
    }

    return headers;
}