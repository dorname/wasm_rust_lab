
use warp::Filter;
use std::env;



#[tokio::main]
async fn main() {
    println!("Current directory: {:?}", env::current_dir().unwrap());
    warp::serve(warp::fs::dir("public"))
        .run(([127, 0, 0, 1], 8089))
        .await;
}
