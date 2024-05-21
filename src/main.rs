use futures::{future::join_all, join, StreamExt};
use std::{collections::HashMap, io::Bytes, result, str::from_utf8_mut};

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, HOST};
use log::{info, warn,debug};
use env_logger::Builder;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let tasks: Vec<_> = (1..=2).map(|n| query_chat_once_with_id("hello",n)).collect();
    let result= join_all(tasks).await;

    
    print!("{:?}",result);

    // let res=queryChatOnce("用rust写个topk",1).await;
    // print!("{:?}",res);
    Ok(())
}

async fn query_chat_once_with_id(content: &str,number:usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("role", "user");
    map.insert("content", content);
    // let mut map_outter = HashMap::new();
    // map_outter.insert("inputs", vec![map]);

    let info= json!(
        {
            "inputs": vec![map],
            "session_id" : number.to_string(),
        }
    );

    println!("{:?}",info);

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = reqwest::Client::new();
    let res = client
        .post("http://172.30.64.1:12345/infer")
        .headers(headers)
        .json(&info)
        .send()
        .await?;
    debug!("number {number} get response:{res:#?}");

    let mut stream = res.bytes_stream();

    let mut answer = Vec::new();
    while let Some(item) = stream.next().await {
        // print!("{:?}", item);
        let bytes_slice: &[u8] = &item?;
        let get_str=String::from_utf8_lossy(bytes_slice);
        debug!("number {number} receive : {get_str}");
        answer.append(&mut Vec::from(bytes_slice));
    }

    Ok(String::from_utf8_lossy(&answer).to_string())
}


async fn query_chat_once(content: &str,number:usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("role", "user");
    map.insert("content", content);
    let mut map_outter = HashMap::new();
    map_outter.insert("inputs", vec![map]);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    let client = reqwest::Client::new();
    let res = client
        .post("http://172.30.64.1:12345/infer")
        .headers(headers)
        .json(&map_outter)
        .send()
        .await?;
    debug!("number {number} get response:{res:#?}");

    let mut stream = res.bytes_stream();

    let mut answer = Vec::new();
    while let Some(item) = stream.next().await {
        // print!("{:?}", item);
        let bytes_slice: &[u8] = &item?;
        let get_str=String::from_utf8_lossy(bytes_slice);
        debug!("number {number} receive : {get_str}");
        answer.append(&mut Vec::from(bytes_slice));
    }

    Ok(String::from_utf8_lossy(&answer).to_string())
}
