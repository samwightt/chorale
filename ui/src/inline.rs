use base::renderer::InlineRenderer;
use templating::attributes::*;
use templating::tags::*;
use base::parser::ColorType;
use crate::utils::color_class_name;

pub struct Inline {}

impl InlineRenderer<Tag> for Inline {
    fn text(&self, t: &str) -> Tag {
        text(t)
    }

    fn bold(&self, acc: Tag) -> Tag {
        b(vec![class("notion-bold")], vec![acc])
    }

    fn italic(&self, acc: Tag) -> Tag {
        i(vec![class("notion-italic")], vec![acc])
    }

    fn underline(&self, acc: Tag) -> Tag {
        u(vec![class("notion-underline")], vec![acc])
    }

    fn strike(&self, acc: Tag) -> Tag {
        strike(vec![class("notion-strike")], vec![acc])
    }

    fn code(&self, acc: Tag) -> Tag {
        code(vec![class("notion-code-inline")], vec![acc])
    }

    fn link(&self, acc: Tag, link: &String) -> Tag {
        a(vec![class("notion-link"), href(link)], vec![acc])
    }

    fn highlight(&self, acc: Tag, color: &ColorType) -> Tag {
        span(vec![class(color_class_name(color))], vec![acc])
    }
}
