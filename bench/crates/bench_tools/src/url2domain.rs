#![allow(dead_code)]
#![forbid(unsafe_code)]


pub fn slice_domain_from_url_mini(url:String)->Result<String,String>{
    //"https://www.example.com/213123"
    //to
    //www.example.com
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
pub fn slice_domain_from_url(url:String)->Result<String,String>{
    //"https://www.example.com.uk:12345/213123"
    //to
    //www.example.com:12345
    let mut url_domain=url.clone();
    //let find ="://";
    let index_0=url_domain.find("://").unwrap_or(0);
    if 0 != index_0
    {
        url_domain=url_domain[index_0+3..].to_string();
    }

    //let index_1=url_domain.find(":").ok_or("url should end with // ".to_string())?;
    let index_1=url_domain.find("/").unwrap_or(url_domain.len()-1);
    url_domain=url_domain[..index_1].to_string();
    log::debug!("slice url:{}",url_domain);    
    return Ok(url_domain);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_slice_domain_from_url() {
        let urls=[
            //"www.example.com",
            //"https://www.example.com",
            //"https://www.example.com",
            "https://www.example.com/test/test",
            "http://www.example.com/test/test",
            "https://www.example.com:12345/test",
            "https://www.example.com.hk/test",
            "https://www.example.com.co.uk/test",
            "www.example.com/test",
            "https://www.example.com:12345"

        ];

        for i in 0..urls.len(){
            let _=match slice_domain_from_url(urls[i].to_string()){
                Ok(a)=>{
                    println!("{}, slice_domain_from_url:{}",urls[i],a);
                }
                Err(a)=>{
                    println!("{}, slice_domain_Error:{}",urls[i],a);
                }
            };      
        }
        
    }
}