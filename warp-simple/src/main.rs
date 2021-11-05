use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::any()
        .map(|| "Hello, World!");

    warp::serve(route)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
