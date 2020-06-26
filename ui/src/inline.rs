use maud::{html, Markup};

pub trait InlineRenderer<T> {
    fn text(&self, text: &str) -> T;
    fn bold(&self, acc: &T) -> T;
    fn italic(&self, acc: &T) -> T;
    fn underline(&self, acc: &T) -> T;
    fn strike(&self, acc: &T) -> T;
    fn code(&self, acc: &T) -> T;
}


pub struct Inline {}

impl InlineRenderer<Markup> for Inline {
    fn text(&self, text: &str) -> Markup {
        html! { (text) }
    }
    fn bold(&self, acc: &Markup) -> Markup {
        html! {
            b class="notion-bold" {
                (acc)
            }
        }
    }
    fn italic(&self, acc: &Markup) -> Markup {
        html! {
            em class="notion-italic" {
                (acc)
            }
        }
    }
    fn underline(&self, acc: &Markup) -> Markup {
        html! {
            u class="notion-underline" {
                (acc)
            }
        }
    }
    fn strike(&self, acc: &Markup) -> Markup {
        html! {
            strike class="notion-strike" {
                (acc)
            }
        }
    }
    fn code(&self, acc: &Markup) -> Markup {
        html! {
            code class="notion-code-inline" {
                (acc)
            }
        }
    }
}