use gloo_utils::document;
use reqwest::Url;
use std::collections::HashMap;

/// A set of init parameters
pub struct InitParams {
    pub application: Option<String>,
    pub device: Option<String>,
    pub password: Option<String>,
    pub url: Option<String>,
}

impl InitParams {
    /// Read the init parameters from the document's location.
    pub fn from_location() -> Self {
        let url = document().url().ok().and_then(|url| Url::parse(&url).ok());
        let mut query = url
            .map(|url| url.query_pairs().into_owned().collect::<HashMap<_, _>>())
            .unwrap_or_default();
        Self {
            application: query.remove("application"),
            device: query.remove("device"),
            password: query.remove("password"),
            url: query.remove("url"),
        }
    }
}
