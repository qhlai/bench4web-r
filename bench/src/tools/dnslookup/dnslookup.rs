
use std::io;
use rand::Rng;
use std::cmp::Ordering;
const MOD_VERSION:str="1";
enum IpAddrKind{
    V4(u8,u8,u8,u8),
    V6(String)
}
enum Message{
    Quit,
    Move
}
struct IpAddrType{
    kind:IpAddrKind
}
fn ip{
    let home=IpAddrKind::V4(127.0.0.1);

}
fn guess(){
    let version="11";
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("secret:{}",secret_number);
    //let version:u32=version.trim().parse().expect("trim error");
    
    loop{

        let mut guess = String::new();
        io::stdio().read_line(&mut guess).expect("error");
        let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less=>println!("too low"),
            Ordering::Greater=>println!("too large"),
            Ordering::Equal=>println!("fit"),
        }
    }

}
fn ip_lookup(){
    println!("pls enter your domain");
    println!("1231");
    let tup:(i32,f64,u8)=(500,6.4,1);
    let ip_port:(u8,u8,u8,u8,u32) =(1,1,1,1,80);
    let (a,b,c)=tup;
    let a=[1,2,3];
    let a:[i32;3]=[1,2,3];
    //let mut domain = String::new();
    //io::stdio().read_line(&mut guess).expect("error");
}
