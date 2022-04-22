use std::fmt;
use tokio;
use warp::Filter;
use serde::Serialize;

// Use the tokio runtime
#[tokio::main]
async fn main() {
    let version = Version { version: "1.0.0".parse().unwrap() };
    println!("Version: {version}");

    // Implementing an inline handler for the get_version
    let get_version_handler = warp::get()
        .and(warp::path("version"))
        .and(warp::path::end())
        .and_then(get_version);

    // Add the routes
    let routes = get_version_handler;

    // Start the server and serve requests
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Debug, Serialize, Clone)]
struct Version {
    version: String
}

impl Version {
    fn new(version: String) -> Self {
        Version {
            version
        }
    }
}

impl fmt::Display for Version {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.version)
    }
}

// get version function returns a JSON reply wrapped in an OK Result
async fn get_version() -> Result<impl warp::Reply, warp::Rejection> {
    let v = Version { version: "1.0.0".parse().unwrap() };
    Ok(warp::reply::json(
        &v
    ))
}
