use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Serialize, Deserialize, Queryable)]   //Queryable  json 형태로 출력해주고 나머지는 잘;;;
#[edgedb(json)]     //title,id 순서를 섞어도 error를 잡아줌
struct Movie {
  id: Uuid,
  title: String,
}
#[tokio::main]
async fn main() {
    let client = edgedb_tokio::create_client().await.unwrap();

    // let query_res:String = client.query_required_single("Select{'This }", &()).await.unwrap();
    // println!("{}",query_res);
    let movie_res:Movie = client
        .query_required_single(
            "select <json>(insert Movie {
            title := '웅남이',
            }) {
              title,
              id,
             };",
            &(),
        )
        .await
        .unwrap();
    
    println!("{movie_res:#?}")
}
