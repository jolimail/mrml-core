use super::{MjInclude, MjIncludeChild};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::convert::TryFrom;
use std::rc::Rc;

impl MjIncludeChild {
    pub fn as_renderable<'r, 'e: 'r, 'h: 'r>(&'e self) -> &'e (dyn Renderable<'r, 'e, 'h> + 'e) {
        match self {
            Self::MjAccordion(elt) => elt,
            Self::MjButton(elt) => elt,
            Self::MjCarousel(elt) => elt,
            Self::MjColumn(elt) => elt,
            Self::MjDivider(elt) => elt,
            Self::MjGroup(elt) => elt,
            Self::MjHero(elt) => elt,
            Self::MjImage(elt) => elt,
            Self::MjNavbar(elt) => elt,
            Self::MjRaw(elt) => elt,
            Self::MjSection(elt) => elt,
            Self::MjSocial(elt) => elt,
            Self::MjSpacer(elt) => elt,
            Self::MjTable(elt) => elt,
            Self::MjText(elt) => elt,
            Self::MjWrapper(elt) => elt,
            Self::Node(elt) => elt,
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjIncludeChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        self.as_renderable().renderer(header)
    }
}

struct MjIncludeRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjInclude,
}

impl<'e, 'h> Render<'h> for MjIncludeRender<'e, 'h> {
    fn attributes(&self) -> Option<&Map<String, String>> {
        None
    }

    fn default_attribute(&self, _key: &str) -> Option<&str> {
        None
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
        // let element_width = self.get_width();
        let mut children = String::default();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            // renderer.set_container_width(element_width.clone());
            renderer.set_index(index);
            renderer.set_siblings(self.element.children.len());
            children.push_str(&renderer.render(opts)?);
        }
        Ok(children)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjInclude {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjIncludeRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::mj_head::MjHead;
    use crate::mj_include::MjInclude;
    use crate::mj_text::MjText;
    use crate::prelude::render::{Header, Options, Renderable};

    #[test]
    fn simple() {
        let opts = Options::default();
        let mj_head = Some(MjHead::default());
        let expected = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));
            let elt = MjText::default();
            let renderer = elt.renderer(header);
            renderer.render(&opts).unwrap()
        };
        let result = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));
            let mut elt = MjInclude::default();
            elt.attributes.path = "memory:foo.mjml".to_string();
            elt.children
                .push(crate::mj_include::MjIncludeChild::MjText(MjText::default()));
            let renderer = elt.renderer(header);
            renderer.render(&opts).unwrap()
        };
        assert_eq!(expected, result);
    }
}
