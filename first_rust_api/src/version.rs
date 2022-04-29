use serde::Serialize;
use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, Serialize, Clone)]
pub struct Version {
    pub(crate) version: String,
}

impl Version {
    fn new(version: String) -> Self {
        Version { version }
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
pub async fn get_version() -> Result<impl warp::Reply, warp::Rejection> {
    let v = Version::new("1.0.0".to_string());
    Ok(warp::reply::json(&v))
}
