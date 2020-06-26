use base::renderer::WrapperRenderer;
use templating::attributes::*;
use templating::tags::*;

pub struct Wrapper {}

pub fn wrapper(items: Vec<Tag>, c: &str, elem: TagType) -> Tag {
    elem(vec![class(c)], items)
}

impl WrapperRenderer<Tag> for Wrapper {
    fn bulleted_list_wrapper(&self, items: Vec<Tag>) -> Tag {
        wrapper(items, "notion-bulleted_list-wrapper", ul)
    }

    fn numbered_list_wrapper(&self, items: Vec<Tag>) -> Tag {
        wrapper(items, "notion-numbered_list-wrapper", ol)
    }

    fn collect(&self, items: Vec<Tag>) -> Tag {
        collect(items)
    }
}
