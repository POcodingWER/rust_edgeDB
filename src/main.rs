use std::fmt::format;

use edgedb_derive::Queryable;
use edgedb_protocol::{value::Value, model::BigInt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Queryable)] //Queryable  json 형태로 출력해주고 나머지는 잘;;;
#[edgedb(json)] //title,id 순서를 섞어도 error를 잡아줌
struct Movie {
    id: Uuid,
    title: String,
}
#[tokio::main]
async fn main() {
    let client = edgedb_tokio::create_client().await.unwrap();
    //1. 값불러오기
    let movie_res: Movie = client
        .query_required_single(
            "select <json>(insert Movie {title := '웅남이',}) 
          { 
            title,
            id,
          };",
            &(),
        )
        .await
        .unwrap();
    println!("{movie_res:#?}");

    //2.쿼리에 값 넣는법
    let query = format!("select{{('{}', {})}}", "Hello there", 10);
    let res: Value = client.query_required_single(&query, &()).await.unwrap();
    println!("{res:?}");

    //3.arguments 쿼리 넣는법
    let query = "select {(<str>$0, <bigint>$1)}";
    let my_big_number = BigInt::from(1000000);
    let res: Value = client.query_required_single(&query, &("Hello there", my_big_number)).await.unwrap();
    println!("{res:?}");
}
