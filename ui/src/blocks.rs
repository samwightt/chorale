use base::renderer::BlockRenderer;
use templating::attributes::*;
use templating::tags::*;

pub struct Blocks {}

fn wrapper(children: Tag, text: Option<Tag>, c: &str, root: TagType, wrapper: TagType) -> Tag {
    collect(vec![
        root(
            vec![class(c)],
            vec![option_include(text)]
        ),
        wrapper(vec![], vec![children]),
    ])
}

impl BlockRenderer<Tag> for Blocks {
    fn page_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-page-block", h1, div)
    }

    fn text_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-text", p, div)
    }

    fn bulleted_list_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-bulleted_list-block", li, div)
    }

    fn numbered_list_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-bulleted_list-block", li, div)
    }

    fn toggle_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-toggle-block", |a, b| { details(a, vec![summary(vec![], b)]) }, summary)
    }

    fn quote_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-quote-block", q, div)
    }

    fn header_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-header-block", h1, div)
    }

    fn sub_header_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-sub_header-block", h2, div)
    }

    fn sub_sub_header_block(&self, children: Tag, text: Option<Tag>) -> Tag {
        wrapper(children, text, "notion-sub_sub_header-block", h3, div)
    }

    fn divider_block(&self, children: Tag) -> Tag {
        hr(vec![class("notion-divider-block")], vec![children])
    }

    fn empty(&self) -> Tag {
        empty()
    }
}
