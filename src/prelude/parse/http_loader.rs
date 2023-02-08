//! Module containing a loader where all the possible files are stored on an http server.

use super::loader::IncludeLoaderError;
use crate::prelude::parse::loader::IncludeLoader;
use std::collections::HashSet;
use std::io::ErrorKind;

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
/// # Example
/// ```rust
/// use mrml::prelude::parse::http_loader::HttpIncludeLoader;
/// use mrml::prelude::parse::ParserOptions;
/// use std::collections::HashSet;
/// use std::rc::Rc;
///
/// let resolver = HttpIncludeLoader::new_allow(HashSet::from(["http://localhost".to_string()]));
/// let opts = ParserOptions {
///     include_loader: Box::new(resolver),
/// };
/// let template = r#"<mjml>
///   <mj-body>
///     <mj-include path="http://localhost/partials/mj-body.mjml" />
///   </mj-body>
/// </mjml>"#;
/// match mrml::parse_with_options(template, Rc::new(opts)) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Couldn't parse template: {err:?}"),
/// }
/// ```
pub struct HttpIncludeLoader {
    origin: OriginList,
}

impl HttpIncludeLoader {
    /// Creates a new [`HttpIncludeLoader`](crate::prelude::parse::http_loader::HttpIncludeLoader) that allows all the origins.
    ///
    /// If you use this method, you should be careful, you could be loading some data from anywhere.
    pub fn allow_all() -> Self {
        Self {
            origin: OriginList::Deny(Default::default()),
        }
    }

    /// Creates a new instance with an allow list to filter the origins.
    ///
    /// ```rust
    /// use mrml::prelude::parse::http_loader::HttpIncludeLoader;
    /// use std::collections::HashSet;
    ///
    /// let resolver = HttpIncludeLoader::new_allow(HashSet::from(["http://localhost".to_string()]));
    /// ```
    pub fn new_allow(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Allow(origins),
        }
    }

    /// Creates a new instance with an dey list to filter the origins.
    ///
    /// ```rust
    /// use mrml::prelude::parse::http_loader::HttpIncludeLoader;
    /// use std::collections::HashSet;
    ///
    /// let resolver = HttpIncludeLoader::new_allow(HashSet::from(["http://somewhere.com".to_string()]));
    /// ```
    pub fn new_deny(origins: HashSet<String>) -> Self {
        Self {
            origin: OriginList::Deny(origins),
        }
    }

    /// Check that the given url provided by the `path` attribute in the `mj-include` complies with the filtering.
    fn check_url(&self, path: &str) -> Result<(), IncludeLoaderError> {
        let url = url::Url::parse(path).map_err(|err| {
            IncludeLoaderError::new(path, ErrorKind::InvalidInput).with_cause(Box::new(err))
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

impl IncludeLoader for HttpIncludeLoader {
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError> {
        self.check_url(path)?;
        ureq::get(path)
            .call()
            .map_err(|err| {
                IncludeLoaderError::new(path, ErrorKind::NotFound).with_cause(Box::new(err))
            })?
            .into_string()
            .map_err(|err| {
                IncludeLoaderError::new(path, ErrorKind::InvalidData).with_cause(Box::new(err))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpIncludeLoader, OriginList};
    use crate::prelude::parse::loader::IncludeLoader;
    use std::{collections::HashSet, io::ErrorKind};

    #[test]
    fn origin_list_is_allowed() {
        assert!(!OriginList::Allow(Default::default()).is_allowed("localhost"));
        assert!(OriginList::Allow(HashSet::from(["localhost".to_string()])).is_allowed("localhost"));
        assert!(OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("localhost"));
        assert!(!OriginList::Deny(HashSet::from(["somewhere".to_string()])).is_allowed("somewhere"));
        assert!(OriginList::Deny(HashSet::default()).is_allowed("somewhere"));
    }

    #[test]
    fn include_loader_should_validate_url() {
        // allow everything
        assert!(HttpIncludeLoader::allow_all()
            .check_url("http://localhost/partial.mjml")
            .is_ok());
        // allow nothing
        assert!(HttpIncludeLoader::new_allow(HashSet::default())
            .check_url("http://localhost/partial.mjml")
            .is_err());
        // only deny some domains
        let loader = HttpIncludeLoader::new_deny(HashSet::from(["http://somewhere".to_string()]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
        // only allow some domains
        let loader = HttpIncludeLoader::new_allow(HashSet::from([
            "http://localhost".to_string(),
            "https://somewhere".to_string(),
        ]));
        assert!(loader.check_url("http://localhost/partial.mjml").is_ok());
        assert!(loader.check_url("http://somewhere/partial.mjml").is_err());
        assert!(loader.check_url("https://somewhere/partial.mjml").is_ok());
    }

    #[test]
    fn include_loader_should_resolve_with_content() {
        let partial = "<mj-text>Hello World!</mj-text>";
        let m = mockito::mock("GET", "/partial.mjml")
            .with_status(200)
            .with_body("<mj-text>Hello World!</mj-text>")
            .create();
        let loader = HttpIncludeLoader::new_allow(HashSet::from([mockito::server_url()]));
        let resolved = loader
            .resolve(&format!("{}/partial.mjml", mockito::server_url()))
            .unwrap();
        assert_eq!(partial, resolved);
        m.assert();
    }

    #[test]
    fn include_loader_should_resolve_with_not_found() {
        let m = mockito::mock("GET", "/partial.mjml")
            .with_status(404)
            .with_body("Not Found")
            .create();
        let loader = HttpIncludeLoader::new_allow(HashSet::from([mockito::server_url()]));
        let err = loader
            .resolve(&format!("{}/partial.mjml", mockito::server_url()))
            .unwrap_err();
        assert_eq!(err.reason, ErrorKind::NotFound);
        m.assert();
    }
}
