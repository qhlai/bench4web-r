
use std::time::{SystemTime, UNIX_EPOCH};

use super::timestamp;

static ADMIN_KEY : &str ="123a41313";
static VALID_TIME : i64 =1629255048330+3600*24*30*1000; //ms
pub fn key_certify(key:Option<String>)->bool{
    //let ADMIN_KEY: String=String::from("123a41313");
    if key.is_some() && key.unwrap()== ADMIN_KEY.to_string(){
        //log::info!("Admin Key Enabled");
        log::info!("Whitelist Disabled");
        return true;
    }else{
        log::info!("Whitelist Enabled");
        return false;
    }
} 
pub fn slice_domain_from_url(url:String)->Result<String,String>{
    //"https://www.google.com/213123"
    //to
    //www.google.com
    let mut url_domain=url.clone();
    //let find ="://";
    let index_0=url_domain.find("://").ok_or("url should begin with http://// ".to_string())?;
    log::debug!("{} find ;// at:{}",url_domain,index_0);
    url_domain=url_domain[index_0+3..].to_string();
    let index_1=url_domain.find("/").ok_or("url should end with // ".to_string())?;
    log::debug!("{} find / at:{}",url_domain,index_1);
    url_domain=url_domain[..index_1].to_string();
    log::debug!("slice url:{}",url_domain);    
    return Ok(url_domain);
}
pub fn time_certify()->bool{
    //println!("Now:{}",timestamp::time_stamp());
    //log::info!("Now:{}",timestamp::time_stamp());
    let now=timestamp::time_stamp();
    if now<=VALID_TIME{
        //log::info!("Reamin:{} min",(VALID_TIME-now)/1000/60);
        log::debug!("Reamin:{} min",(VALID_TIME-now)/1000/60);
        return true;
    }
    else{
        log::error!("this version is out of date,will not work,pls get new one");
        return false;
    }
}
pub fn certify(key:Option<String>)->bool{
    //println!("Now:{}",timestamp::time_stamp());
    //let key=Some(String::from(ADMIN_KEY));
    match  key_certify(key)&&time_certify(){
        true=>{
            log::info!("certify ok");
            return true;
        }
        false=>{
            log::info!("certify faild");
            return false;
        }
    };
}
pub fn url_check_whitelist(url:String)->bool{

    let url_domian=slice_domain_from_url(url.clone()).unwrap();
    log::info!("url_domian:{}",&url_domian);
    //match url_domian{
    //    url_domian.find("allowed_domain").is_some()=>{
     //   }
    //}
    if url_domian.find("allowed_domain").is_some() {
        log::info!("certified ok, allowed_domain is in whitelist");
        return true;
    }
    log::error!("{} not in whitelist.Certified Faild ",&url_domian);
    return false;

}
pub fn time_stamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}
#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;
    #[test]
    fn certify(){
        println!("Now:{}",timestamp::time_stamp());
        let key=Some(String::from(ADMIN_KEY));
        match  super::key_certify(key)&&super::time_certify(){
            true=>{
                println!("certify ok");
            }
            false=>{
                println!("certify faild");
            }
        };
    }
    #[test]
    fn key_certify() {
        let key=Some(String::from(ADMIN_KEY));
        match super::key_certify(key){
            true=>{
                println!("key certified");
            }
            false=>{
                println!("key wrong");
            }
        };
    }
    #[test]
    fn time_certify() {
        match super::time_certify(){
            true=>{
                println!("OK");
            }
            false=>{
                println!("False");
            }
        };
    }
    #[test]
    fn test_slice_domain_from_url() {
        let url=String::from("https://www.google.com/213123");
        match super::slice_domain_from_url(url){
            Ok(a)=>{
                println!("slice_domain_from_url:{}",a);
            }
            Err(a)=>{
                println!("slice_domain_Error:{}",a);
            }
        };
    }
}