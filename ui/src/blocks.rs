use base::renderer::BlockRenderer;
use templating::attributes::*;
use templating::tags::*;

pub struct Blocks {}

fn wrapper(children: Tag, text: Option<Tag>, c: &str, root: TagType, wrapper: TagType) -> Tag {
    root(
        vec![class(c)],
        vec![option_include(text), wrapper(vec![], vec![children])],
    )
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
        wrapper(children, text, "notion-toggle-block", details, summary)
    }

    fn empty(&self) -> Tag {
        empty()
    }
}
