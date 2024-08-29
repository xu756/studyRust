use axum::Json;
use serde::Serialize;
use serde_json::{json, Value};

// 响应结构体
#[derive(Serialize)]
pub struct Response<T> {
    success: bool,
    err_code: u32,
    err_msg: String,
    data: T, // 使用Option来表示数据可能不存在
}

// 成功响应
pub fn success<T: Serialize>(data: T) -> Json<Value> {
    let res = Response {
        success: true,
        err_code: 0,
        err_msg: "".to_string(),
        data,
    };
    Json(json!(res))
}


// 失败响应
pub fn error<T>(err_code: u32, err_msg: &str) -> Json<Value> {
    let res = Response {
        success: false,
        err_code,
        err_msg: err_msg.to_string(),
        data: None,
    };
    Json(json!(res))
}