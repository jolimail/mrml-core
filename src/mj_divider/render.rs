use super::{MjDivider, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::Pixel;
use crate::helper::size::Size;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct MjDividerRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjDivider,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MjDividerRender<'e, 'h> {
    fn set_style_p_without_width(&self, tag: Tag) -> Tag {
        tag.add_style(
            "border-top",
            format!(
                "{} {} {}",
                self.attribute("border-style").unwrap(),
                self.attribute("border-width").unwrap(),
                self.attribute("border-color").unwrap()
            ),
        )
        .add_style("font-size", "1px")
        .add_style("margin", "0px auto")
    }
    fn set_style_p(&self, tag: Tag) -> Tag {
        self.set_style_p_without_width(tag)
            .maybe_add_style("width", self.attribute("width"))
    }

    fn set_style_outlook(&self, tag: Tag) -> Tag {
        self.set_style_p_without_width(tag)
            .add_style("width", self.get_outlook_width().to_string())
    }

    fn get_outlook_width(&self) -> Pixel {
        let container_width = self.container_width.as_ref().unwrap();
        let padding_horizontal = self.get_padding_horizontal();
        let width = self
            .attribute_as_size("width")
            .unwrap_or_else(|| Size::percent(100.0));
        match width {
            Size::Percent(value) => {
                let effective = container_width.value() - padding_horizontal.value();
                let multiplier = value.value() / 100.0;
                Pixel::new(effective * multiplier)
            }
            Size::Pixel(value) => value,
            _ => Pixel::new(container_width.value() - padding_horizontal.value()),
        }
    }

    fn render_after(&self) -> String {
        let table = self
            .set_style_outlook(Tag::table_presentation())
            .add_attribute("align", "center")
            .add_attribute("width", self.get_outlook_width().to_string());
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("height", "0")
            .add_style("line-height", "0");
        conditional_tag(table.render(tr.render(td.render("&nbsp;"))))
    }
}

impl<'e, 'h> Render<'h> for MjDividerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "align" => Some("center"),
            "border-color" => Some("#000000"),
            "border-style" => Some("solid"),
            "border-width" => Some("4px"),
            "padding" => Some("10px 25px"),
            "width" => Some("100%"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, _opts: &Options) -> Result<String, Error> {
        Ok(self.set_style_p(Tag::new("p")).render("") + &self.render_after())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjDivider {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjDividerRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::mjml::Mjml;
    use crate::prelude::render::Options;

    #[test]
    fn basic() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-divider.mjml");
        let expected = include_str!("../../resources/compare/success/mj-divider.html");
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        html_compare::assert_similar(expected, result.as_str());
    }

    #[test]
    fn class() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-divider-class.mjml");
        let expected = include_str!("../../resources/compare/success/mj-divider-class.html");
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        html_compare::assert_similar(expected, result.as_str());
    }

    #[test]
    fn container_background_color() {
        let opts = Options::default();
        let template = include_str!(
            "../../resources/compare/success/mj-divider-container-background-color.mjml"
        );
        let expected = include_str!(
            "../../resources/compare/success/mj-divider-container-background-color.html"
        );
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        html_compare::assert_similar(expected, result.as_str());
    }

    #[test]
    fn padding() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-divider-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-divider-padding.html");
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        html_compare::assert_similar(expected, result.as_str());
    }

    #[test]
    fn width() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-divider-width.mjml");
        let expected = include_str!("../../resources/compare/success/mj-divider-width.html");
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        html_compare::assert_similar(expected, result.as_str());
    }
}
