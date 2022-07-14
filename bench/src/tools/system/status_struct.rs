
use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
//use std::time::Duration;
use super::timestamp;
//用于表示请求传来的Json对象

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct StatusInfo {
    
    pub id: i64,
    pub session: String,
    pub username: String,
    pub password: String,
}
#[derive(Deserialize)]
#[derive(Serialize)]
struct StatusResult<T> {
    timestamp: i64,
    msg: String,
    code: i32,
    data: Option<Vec<T>>,
    session: String
}

impl<T> StatusResult<T> {
    pub fn success(data_opt: Option<Vec<T>>) -> Self{
         Self {
             timestamp: timestamp::time_stamp(),
             msg: "success".to_string(),
             code: 200,
             data: data_opt,
             session: "123".to_string()
         }
    }

    pub fn success_without_data() -> Self {
        Self::success(Option::None)
    }
    pub fn success_with_single(single: T) -> Self{
        Self {
            timestamp: timestamp::time_stamp(),
            msg:  "success".to_string(),
            code: 201,
            data: Option::Some(vec![single]),
            session: "123".to_string()
        }
    }
    pub fn success_with_string(msg: String) -> Self{
        Self {
            timestamp: timestamp::time_stamp(),
            msg,
            code: 202,
            data: None,
            session: "123".to_string()
        }
    }
    pub fn fail(msg: String) -> Self {
        Self {
            timestamp: timestamp::time_stamp(),
            msg,
            code: 200,
            data: None,
            session: "123".to_string()
        }
    }

}
