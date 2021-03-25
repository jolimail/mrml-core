use super::MJHead;
use crate::helper::sort::sort_by_key;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

fn google_font(name: &str) -> String {
    format!(
        "https://fonts.googleapis.com/css?family={}:300,400,500,700",
        name.replace(" ", "+")
    )
}

fn default_font(name: &str) -> Option<String> {
    match name {
        "Open Sans" | "Droid Sans" | "Lato" | "Roboto" | "Ubuntu" => Some(google_font(name)),
        _ => None,
    }
}

const STYLE_BASE: &str = r#"
<style type="text/css">
#outlook a { padding: 0; }
body { margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
table, td { border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
img { border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }
p { display: block; margin: 13px 0; }
</style>
<!--[if mso]>
<xml>
<o:OfficeDocumentSettings>
  <o:AllowPNG/>
  <o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
<![endif]-->
<!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
"#;

impl MJHead {
    pub fn build_attributes_all(&self) -> HashMap<&str, &str> {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(HashMap::<&str, &str>::new(), |result, attrs| {
                attrs
                    .children()
                    .iter()
                    .filter_map(|item| item.as_mj_all())
                    .fold(result, |mut res, all| {
                        res.extend(
                            all.attributes()
                                .into_iter()
                                .map(|(k, v)| (k.as_str(), v.as_str())),
                        );
                        res
                    })
            })
    }

    pub fn build_attributes_class(&self) -> HashMap<&str, HashMap<&str, &str>> {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(
                HashMap::<&str, HashMap<&str, &str>>::new(),
                |result, attrs| {
                    attrs
                        .children()
                        .iter()
                        .filter_map(|item| item.as_mj_class())
                        .fold(result, |mut res, class| {
                            (*res.entry(class.name()).or_insert(HashMap::new())).extend(
                                class
                                    .attributes()
                                    .into_iter()
                                    .map(|(k, v)| (k.as_str(), v.as_str())),
                            );
                            res
                        })
                },
            )
    }

    pub fn build_attributes_element(&self) -> HashMap<&str, HashMap<&str, &str>> {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_attributes())
            .fold(
                HashMap::<&str, HashMap<&str, &str>>::new(),
                |result, attrs| {
                    attrs
                        .children()
                        .iter()
                        .filter_map(|item| item.as_element())
                        .fold(result, |mut res, element| {
                            (*res.entry(element.name()).or_insert(HashMap::new())).extend(
                                element
                                    .attributes()
                                    .into_iter()
                                    .map(|(k, v)| (k.as_str(), v.as_str())),
                            );
                            res
                        })
                },
            )
    }

    pub fn build_font_families(&self) -> HashMap<&str, &str> {
        self.children
            .iter()
            .filter_map(|item| item.as_mj_font())
            .map(|item| (item.name(), item.href()))
            .collect::<HashMap<&str, &str>>()
    }
}

pub struct MJHeadRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJHead,
}

impl<'e, 'h> MJHeadRender<'e, 'h> {
    fn render_font_import(&self, buf: &mut String, href: &str) {
        buf.push_str("@import url(");
        buf.push_str(href);
        buf.push_str(");");
    }

    fn render_font_link(&self, buf: &mut String, href: &str) {
        buf.push_str("<link href\"");
        buf.push_str(href);
        buf.push_str("\" rel=\"stylesheet\" type=\"text/css\">");
    }

    fn render_font_families(&self, buf: &mut String) {
        let header = self.header.borrow();
        let used_font_families = header.used_font_families();
        if used_font_families.is_empty() {
            return;
        }
        buf.push_str("<!--[if !mso]><!-->");
        header
            .used_font_families()
            .iter()
            .filter_map(|name| default_font(name))
            .for_each(|href| self.render_font_link(buf, &href));
        header
            .used_font_families()
            .iter()
            .filter_map(|name| header.font_families().get(name.as_str()))
            .for_each(|href| self.render_font_link(buf, href));
        buf.push_str("<style type=\"text/css\">");
        header
            .used_font_families()
            .iter()
            .filter_map(|name| default_font(name))
            .for_each(|href| self.render_font_import(buf, &href));
        header
            .used_font_families()
            .iter()
            .filter_map(|name| header.font_families().get(name.as_str()))
            .for_each(|href| self.render_font_import(buf, href));
        buf.push_str("</style>");
        buf.push_str("<!--<![endif]-->");
    }

    fn render_media_queries(&self, buf: &mut String) {
        let header = self.header.borrow();
        if header.media_queries().is_empty() {
            return;
        }
        let breakpoint = header.breakpoint().to_string();
        buf.push_str("<style type=\"text/css\">");
        buf.push_str("@media only screen and (min-width:");
        buf.push_str(breakpoint.as_str());
        buf.push_str(") { ");
        let mut classnames = header
            .media_queries()
            .iter()
            .map(|(key, value)| (key, value))
            .collect::<Vec<_>>();
        classnames.sort_by(sort_by_key);
        classnames.iter().for_each(|(classname, size)| {
            let size = size.to_string();
            buf.push('.');
            buf.push_str(classname);
            buf.push_str(" { width:");
            buf.push_str(size.as_str());
            buf.push_str(" !important; max-width:");
            buf.push_str(size.as_str());
            buf.push_str("; } ");
        });
        buf.push_str(" }");
        buf.push_str("</style>");
    }

    fn render_styles(&self, buf: &mut String) {
        let header = self.header.borrow();
        if header.styles().is_empty() {
            return;
        }
        buf.push_str("<style type=\"text/css\">");
        header.styles().iter().for_each(|style| {
            buf.push_str(style);
        });
        buf.push_str("</style>");
    }
}

impl<'e, 'h> Render<'h> for MJHeadRender<'e, 'h> {
    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, buf: &mut String) -> Result<(), Error> {
        buf.push_str("<head>");
        // we write the title even though there is no content
        buf.push_str("<title>");
        buf.push_str(
            self.element
                .title()
                .map(|item| item.content())
                .unwrap_or_default(),
        );
        buf.push_str("</title>");
        buf.push_str("<!--[if !mso]><!-- -->");
        buf.push_str("<meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">");
        buf.push_str("<!--<![endif]-->");
        buf.push_str("<meta http-equiv=\"Content-Type\" content=\"text/html; charset=UTF-8\">");
        buf.push_str("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">");
        buf.push_str(STYLE_BASE);
        self.render_font_families(buf);
        self.render_media_queries(buf);
        self.render_styles(buf);
        buf.push_str("</head>");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJHead {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJHeadRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}