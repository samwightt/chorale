use maud::{html, Markup};

pub trait WrapperRenderer<T> {
    fn bulleted_list_wrapper(&self, items: Vec<T>) -> T;
    fn numbered_list_wrapper(&self, items: Vec<T>) -> T;
    fn collect(&self, items: Vec<T>) -> T;
}

pub struct Wrapper {}

impl WrapperRenderer<Markup> for Wrapper {
    fn bulleted_list_wrapper(&self, items: Vec<Markup>) -> Markup {
        html! {
            ul class="notion-bulleted_list-wrapper" {
                @for item in items.iter() {
                    (item)
                }
            }
        }
    }

    fn numbered_list_wrapper(&self, items: Vec<Markup>) -> Markup {
        html! {
            ol class="notion-numbered_list-wrapper" {
                @for item in items.iter() {
                    (item)
                }
            }
        }
    }

    fn collect(&self, items: Vec<Markup>) -> Markup {
        html! {
            @for item in items.iter() {
                (item)
            }
        }
    }
}