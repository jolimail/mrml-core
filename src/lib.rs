//! This project is a reimplementation of the nice [MJML](https://mjml.io/) markup language in Rust.
//!
//! [![codecov](https://codecov.io/gh/jolimail/mrml-core/branch/main/graph/badge.svg?token=SIOPR0YWZA)](https://codecov.io/gh/jolimail/mrml-core)
//! [![.github/workflows/main.yml](https://github.com/jolimail/mrml-core/actions/workflows/main.yml/badge.svg)](https://github.com/jolimail/mrml-core/actions/workflows/main.yml)
//! [![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jolimail/mrml-core "Average time to resolve an issue")
//! [![Percentage of issues still open](http://isitmaintained.com/badge/open/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jolimail/mrml-core "Percentage of issues still open")
//! [![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jolimail/mrml-core/maintainability)
//!
//! To use it you can simply update your `Cargo.toml` by adding
//! ```toml
//! [dependencies]
//! mrml = "1.2"
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! And you can then just create a `main.rs` with the following code
//! ```rust
//! let root = mrml::parse("<mjml><mj-body></mj-body></mjml>").expect("parse template");
//! let opts = mrml::prelude::render::Options::default();
//! match root.render(&opts) {
//!     Ok(content) => println!("{}", content),
//!     Err(_) => println!("couldn't render mjml template"),
//! };
//! ```
//!
//! You can also use the `mj-include` component by specifying a [loader](crate::prelude::parse).
//! ```rust
//! use mrml::prelude::parse::ParserOptions;
//! use mrml::prelude::parse::memory_loader::MemoryIncludeLoader;
//! use std::rc::Rc;
//!
//! let loader = MemoryIncludeLoader::from(vec![("partial.mjml", "<mj-button>Hello</mj-button>")]);
//! let options = Rc::new(ParserOptions {
//!     include_loader: Box::new(loader),
//! });
//! match mrml::parse_with_options("<mjml><mj-head /><mj-body><mj-include path=\"partial.mjml\" /></mj-body></mjml>", options) {
//!     Ok(_) => println!("Success!"),
//!     Err(err) => eprintln!("Something went wrong: {err:?}"),
//! }
//! ```
//!
//! ### Why?
//!
//! A Node.js server rendering an MJML template takes around **20 MB** of RAM at startup and **130 MB** under stress test.
//! In Rust, less than **1.7 MB** at startup and a bit less that **3 MB** under stress test.
//! The Rust version can also handle twice as many requests per second.
//! You can perform the benchmarks by running `bash script/run-bench.sh`.
//!
//! Also, the JavaScript implementation cannot be run in the browser; the Rust one (and WebAssembly one) can be.

pub mod comment;
pub mod mj_accordion;
pub mod mj_accordion_element;
pub mod mj_accordion_text;
pub mod mj_accordion_title;
pub mod mj_attributes;
pub mod mj_attributes_all;
pub mod mj_attributes_class;
pub mod mj_attributes_element;
pub mod mj_body;
pub mod mj_breakpoint;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_carousel_image;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_font;
pub mod mj_group;
pub mod mj_head;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_include;
pub mod mj_navbar;
pub mod mj_navbar_link;
pub mod mj_preview;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_social_element;
pub mod mj_spacer;
pub mod mj_style;
pub mod mj_table;
pub mod mj_text;
pub mod mj_title;
pub mod mj_wrapper;
pub mod mjml;
pub mod node;
pub mod prelude;
pub mod text;

mod helper;
mod macros;

#[cfg(feature = "parse")]
/// Function to parse a raw mjml template with some parsing [options](crate::prelude::parse::ParserOptions).
/// This function is just an alias to [the `Mjml::parse_with_options` function](crate::mjml::Mjml).
///
/// You can specify the kind of loader mrml needs to use for loading the content of
/// [`mj-include`](crate::mj_include) elements.
///
/// You can take a look at the available loaders [here](crate::prelude::parse).
///
/// ```rust
/// use mrml::prelude::parse::ParserOptions;
/// use mrml::prelude::parse::memory_loader::MemoryIncludeLoader;
/// use std::rc::Rc;
///
/// let options = Rc::new(ParserOptions {
///     include_loader: Box::new(MemoryIncludeLoader::default()),
/// });
/// match mrml::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", options) {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// ```
pub fn parse_with_options<T: AsRef<str>>(
    input: T,
    opts: std::rc::Rc<crate::prelude::parse::ParserOptions>,
) -> Result<mjml::Mjml, prelude::parse::Error> {
    mjml::Mjml::parse_with_options(input, opts)
}

#[cfg(feature = "parse")]
/// Function to parse a raw mjml template using the default parsing [options](crate::prelude::parse::ParserOptions).
///
/// ```rust
/// match mrml::parse("<mjml><mj-head /><mj-body /></mjml>") {
///     Ok(_) => println!("Success!"),
///     Err(err) => eprintln!("Something went wrong: {err:?}"),
/// }
/// ```
pub fn parse<T: AsRef<str>>(input: T) -> Result<mjml::Mjml, prelude::parse::Error> {
    mjml::Mjml::parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_simple() {
        let _ = crate::parse("<mjml><mj-head /><mj-body /></mjml>");
    }

    #[test]
    fn parse_with_options() {
        use crate::prelude::parse::ParserOptions;
        use std::rc::Rc;

        let options = Rc::new(ParserOptions::default());
        let _ = crate::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", options);
    }
}
