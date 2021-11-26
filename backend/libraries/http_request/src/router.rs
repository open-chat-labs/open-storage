use std::str::FromStr;
use types::{BlobId, TimestampMillis};

pub enum Route {
    Blob(u128),
    Logs(Option<TimestampMillis>),
    Traces(Option<TimestampMillis>),
    Metrics,
    Other,
}

pub fn extract_route(path: &str) -> Route {
    let path = path.trim_start_matches('/').trim_end_matches('/').to_lowercase();

    if path.is_empty() {
        return Route::Other;
    }
    let parts: Vec<_> = path.split('/').collect();

    match parts[0] {
        "blobs" if parts.len() > 1 => {
            if let Ok(blob_id) = BlobId::from_str(parts[1]) {
                Route::Blob(blob_id)
            } else {
                Route::Other
            }
        }
        "logs" => {
            let since = parts.get(1).map(|p| TimestampMillis::from_str(p).ok()).flatten();
            Route::Logs(since)
        }
        "trace" => {
            let since = parts.get(1).map(|p| TimestampMillis::from_str(p).ok()).flatten();
            Route::Traces(since)
        }
        "metrics" => Route::Metrics,
        _ => Route::Other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blob() {
        assert!(matches!(extract_route("/blobs/78278371289379212398"), Route::Blob(_)));
    }

    #[test]
    fn logs() {
        assert!(matches!(extract_route("/logs/1633649663014109000"), Route::Logs(_)));
    }

    #[test]
    fn other() {
        assert!(matches!(extract_route("blah"), Route::Other));
    }
}
