use warp::Filter;
pub mod routes;

#[tokio::main]
async fn main() {
    // let page = routes::other::IndexPage {}.render();
    // println!("{}", page.unwrap())

    let index = warp::path::end().then(routes::index::page);
    warp::serve(index)
        .run(([127, 0, 0, 1], 3030))
        .await;
}