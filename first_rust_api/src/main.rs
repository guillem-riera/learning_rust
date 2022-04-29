use tokio;
use warp::Filter;
mod version;

// Use the tokio runtime
#[tokio::main]
async fn main() {
    let v = version::Version {
        version: "1.0.0".parse().unwrap(),
    };
    println!("Version: {v}");

    // Implementing an inline handler for the get_version
    let get_version_handler = warp::get()
        .and(warp::path("version"))
        .and(warp::path::end())
        .and_then(version::get_version);

    // Add the routes
    let routes = get_version_handler;

    // Start the server and serve requests
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
