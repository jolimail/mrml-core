//! Module containing a loader where all the possible files are stored on an http server.

use super::loader::IncludeLoaderError;
use crate::prelude::parse::loader::IncludeLoader;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::io::ErrorKind;

pub trait HttpFetcher: Default + Debug {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError>;
}

#[cfg(feature = "http-loader-reqwest")]
#[derive(Debug, Default)]
pub struct ReqwestFetcher(reqwest::blocking::Client);

#[cfg(feature = "http-loader-reqwest")]
impl HttpFetcher for ReqwestFetcher {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError> {
        let req = self.0.get(url);
        let req = headers
            .iter()
            .fold(req, |r, (key, value)| r.header(key, value));
        let res = req.send().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Box::new(err))
        })?;
        let res = res.error_for_status().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::NotFound)
                .with_message("unable to fetch template")
                .with_cause(Box::new(err))
        })?;
        res.text().map_err(|err| {
            IncludeLoaderError::new(url, ErrorKind::InvalidData)
                .with_message("unable to convert remote template as string")
                .with_cause(Box::new(err))
        })
    }
}

#[cfg(feature = "http-loader-ureq")]
#[derive(Debug, Default)]
pub struct UreqFetcher;

#[cfg(feature = "http-loader-ureq")]
impl HttpFetcher for UreqFetcher {
    fn fetch(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<String, IncludeLoaderError> {
        let req = ureq::get(url);
        let req = headers
            .iter()
            .fold(req, |r, (key, value)| r.set(key.as_str(), value.as_str()));
        req.call()
            .map_err(|err| {
                IncludeLoaderError::new(url, ErrorKind::NotFound)
                    .with_message("unable to fetch template")
                    .with_cause(Box::new(err))
            })?
            .into_string()
            .map_err(|err| {
                IncludeLoaderError::new(url, ErrorKind::InvalidData)
                    .with_message("unable to convert remote template as string")
                    .with_cause(Box::new(err))
            })
    }
}

#[derive(Debug)]
/// This enum is a representation of the origin filtering strategy.
///
/// This enum implements the `Default` trait by denying everything by default for security reason.
pub enum OriginList {
    Allow(HashSet<String>),
    Deny(HashSet<String>),
}

impl Default for OriginList {
    fn default() -> Self {
        // The default implementation will allow nothing, for security reasons.
        // If you need to allow everything, you'll have to specify it.
        Self::Allow(HashSet::new())
    }
}

impl OriginList {
    fn is_allowed(&self, origin: &str) -> bool {
        match self {
            Self::Allow(list) => list.contains(origin),
            Self::Deny(list) => !list.contains(origin),
        }
    }
}

#[derive(Debug, Default)]
/// This struct is an [`IncludeLoader`](crate::prelude::parse::loader::IncludeLoader) where
/// you can read a template from an http server and be able to use it with [`mj-include`](crate::mj_include).
///
/// # Example with `reqwest`
/// ```rust
/// #[cfg(feature = "http-loader-reqwest")]
/// {
///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, ReqwestFetcher};
///     use mrml::prelude::parse::ParserOptions;
///     use std::collections::HashSet;
///     use std::rc::Rc;
///
///     let resolver = HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
///     let opts = ParserOptions {
///         include_loader: Box::new(resolver),
///     };
///     let template = r#"<mjml>
///       <mj-body>
///         <mj-include path="http://localhost/partials/mj-body.mjml" />
///       </mj-body>
///     </mjml>"#;
///     match mrml::parse_with_options(template, Rc::new(opts)) {
///         Ok(_) => println!("Success!"),
///         Err(err) => eprintln!("Couldn't parse template: {err:?}"),
///     }
/// }
/// ```
///
/// # Example with `ureq`
/// ```rust
/// #[cfg(feature = "http-loader-ureq")]
/// {
///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, UreqFetcher};
///     use mrml::prelude::parse::ParserOptions;
///     use std::collections::HashSet;
///     use std::rc::Rc;
///
///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
///     let opts = ParserOptions {
///         include_loader: Box::new(resolver),
///     };
///     let template = r#"<mjml>
///       <mj-body>
///         <mj-include path="http://localhost/partials/mj-body.mjml" />
///       </mj-body>
///     </mjml>"#;
///     match mrml::parse_with_options(template, Rc::new(opts)) {
///         Ok(_) => println!("Success!"),
///         Err(err) => eprintln!("Couldn't parse template: {err:?}"),
///     }
/// }
/// ```
pub struct HttpIncludeLoader<F> {
    origin: OriginList,
    headers: HashMap<String, String>,
    fetcher: F,
}

impl<F: HttpFetcher> HttpIncludeLoader<F> {
    /// Creates a new [`HttpIncludeLoader`](crate::prelude::parse::http_loader::HttpIncludeLoader) that allows all the origins.
    ///
    /// If you use this method, you should be careful, you could be loading some data from anywhere.
    pub fn allow_all() -> Self {
        Self {
            origin: OriginList::Deny(Default::default()),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    /// Creates a new instance with an allow list to filter the origins.
    ///
    /// # Example with `reqwest`
    /// ```rust
    /// #[cfg(feature = "http-loader-reqwest")]
    /// {
    ///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, ReqwestFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
    /// }
    /// ```
    ///
    /// # Example with `ureq`
    /// ```
    /// #[cfg(feature = "http-loader-ureq")]
    /// {
    ///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, UreqFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
    /// }
    /// ```
    pub fn new_allow(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Allow(origins),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    /// Creates a new instance with an dey list to filter the origins.
    ///
    /// # Example with `reqwest`
    /// ```rust
    /// #[cfg(feature = "http-loader-reqwest")]
    /// {
    ///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, ReqwestFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from(["http://somewhere.com".to_string()]));
    /// }
    /// ```
    ///
    /// # Example with `ureq`
    /// ```rust
    /// #[cfg(feature = "http-loader-ureq")]
    /// {
    ///     use mrml::prelude::parse::http_loader::{HttpIncludeLoader, UreqFetcher};
    ///     use std::collections::HashSet;
    ///
    ///     let resolver = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from(["http://somewhere.com".to_string()]));
    /// }
    /// ```
    pub fn new_deny(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Deny(origins),
            headers: HashMap::default(),
            fetcher: F::default(),
        }
    }

    pub fn with_header<K: ToString, V: ToString>(mut self, name: K, value: V) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn set_header<K: ToString, V: ToString>(&mut self, name: K, value: V) {
        self.headers.insert(name.to_string(), value.to_string());
    }

    pub fn set_headers(&mut self, headers: HashMap<String, String>) {
        self.headers = headers;
    }

    /// Check that the given url provided by the `path` attribute in the `mj-include` complies with the filtering.
    fn check_url(&self, path: &str) -> Result<(), IncludeLoaderError> {
        let url = url::Url::parse(path).map_err(|err| {
            IncludeLoaderError::new(path, ErrorKind::InvalidInput)
                .with_message("unable to parse the provided url")
                .with_cause(Box::new(err))
        })?;
        let origin = url.origin().ascii_serialization();
        if self.origin.is_allowed(&origin) {
            Ok(())
        } else {
            Err(IncludeLoaderError::new(path, ErrorKind::InvalidInput)
                .with_message("the path is not allowed by the defined list of domains"))
        }
    }
}

impl<F: HttpFetcher> IncludeLoader for HttpIncludeLoader<F> {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.check_url(path)?;
        self.fetcher.fetch(path, &self.headers)
    }
}

#[cfg(test)]
mod common_tests {
    use super::OriginList;
    use std::collections::HashSet;

    #[test]
    fn origin_list_is_allowed() {
        assert!(!OriginList::Allow(Default::default()).is_allowed("localhost"));
        assert!(OriginList::Allow(HashSet::from(["localhost".to_string()])).is_allowed("localhost"));
        assert!(OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("localhost"));
        assert!(!OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("somewhere"));
        assert!(OriginList::Deny(HashSet::default()).is_allowed("somewhere"));
    }
}

#[cfg(all(test, feature = "http-loader-ureq"))]
mod ureq_tests {
    use super::{HttpIncludeLoader, UreqFetcher};
    use crate::prelude::parse::loader::IncludeLoader;
    use std::collections::{HashMap, HashSet};
    use std::io::ErrorKind;

    #[test]
    fn include_loader_should_implement_debug() {
        let _ = format!("{:?}", HttpIncludeLoader::<UreqFetcher>::default());
    }

    #[test]
    fn include_loader_should_validate_url() {
        // allow everything
        assert!(HttpIncludeLoader::<UreqFetcher>::allow_all()
            .check_url("http://localhost/partial.mjml")
            .is_ok());
        // allow nothing
        assert!(
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::default())
                .check_url("http://localhost/partial.mjml")
                .is_err()
        );
        assert!(HttpIncludeLoader::<UreqFetcher>::default()
            .check_url("http://localhost/partial.mjml")
            .is_err());
        // only deny some domains
        let loader = HttpIncludeLoader::<UreqFetcher>::new_deny(HashSet::from([
            "http://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // only allow some domains
        let loader = HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([
            "http://localhost".to_string(),
            "https://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // invalid urls
        assert_eq!(
            loader.check_url("").unwrap_err().message.unwrap(),
            "unable to parse the provided url"
        );
    }

    #[test]
    fn include_loader_should_resolve_with_content() {
        let partial = "<mj-text>Hello World!</mj-text>";
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(200)
            .with_body("<mj-text>Hello World!</mj-text>")
            .create();
        let mut loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]));
        loader.set_header("foo", "bar");
        loader.set_headers(Default::default());
        let resolved = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap();
        assert_eq!(partial, resolved);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_not_found() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_headers() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .match_header("user-agent", "mrml-test")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<UreqFetcher>::new_allow(HashSet::from([mock_server.url()]))
                .with_header("user-agent", "invalid")
                .with_headers(HashMap::from([(
                    "user-agent".to_string(),
                    "mrml-test".to_string(),
                )]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }
}

#[cfg(all(test, feature = "http-loader-reqwest"))]
mod reqwest_tests {
    use super::{HttpIncludeLoader, ReqwestFetcher};
    use crate::prelude::parse::loader::IncludeLoader;
    use std::collections::{HashMap, HashSet};
    use std::io::ErrorKind;

    #[test]
    fn include_loader_should_implement_debug() {
        let _ = format!("{:?}", HttpIncludeLoader::<ReqwestFetcher>::default());
    }

    #[test]
    fn include_loader_should_validate_url() {
        // allow everything
        assert!(HttpIncludeLoader::<ReqwestFetcher>::allow_all()
            .check_url("http://localhost/partial.mjml")
            .is_ok());
        // allow nothing
        assert!(
            HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::default())
                .check_url("http://localhost/partial.mjml")
                .is_err()
        );
        assert!(HttpIncludeLoader::<ReqwestFetcher>::default()
            .check_url("http://localhost/partial.mjml")
            .is_err());
        // only deny some domains
        let loader = HttpIncludeLoader::<ReqwestFetcher>::new_deny(HashSet::from([
            "http://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // only allow some domains
        let loader = HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from([
            "http://localhost".to_string(),
            "https://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // invalid urls
        assert_eq!(
            loader.check_url("").unwrap_err().message.unwrap(),
            "unable to parse the provided url"
        );
    }

    #[test]
    fn include_loader_should_resolve_with_content() {
        let partial = "<mj-text>Hello World!</mj-text>";
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(200)
            .with_body("<mj-text>Hello World!</mj-text>")
            .create();
        let mut loader =
            HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from([mock_server.url()]));
        loader.set_header("foo", "bar");
        loader.set_headers(Default::default());
        let resolved = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap();
        assert_eq!(partial, resolved);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_not_found() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from([mock_server.url()]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_headers() {
        let mut mock_server = mockito::Server::new();
        let m = mock_server
            .mock("GET", "/partial.mjml")
            .match_header("user-agent", "mrml-test")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader =
            HttpIncludeLoader::<ReqwestFetcher>::new_allow(HashSet::from([mock_server.url()]))
                .with_header("user-agent", "invalid")
                .with_headers(HashMap::from([(
                    "user-agent".to_string(),
                    "mrml-test".to_string(),
                )]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mock_server.url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }
}
