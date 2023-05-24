use edgedb_protocol::value::Value;

#[tokio::main]
async fn main() {
    let client = edgedb_tokio::create_client().await.unwrap();

    // let query_res:String = client.query_required_single("Select{'This }", &()).await.unwrap();
    // println!("{}",query_res);
    let movie_res: Value = client
        .query_required_single(
            "insert Movie {
            title := '웅남이',
            release_year := 2023
            };",
            &(),
        )
        .await
        .unwrap();

      println!("{movie_res:#?}")
}
