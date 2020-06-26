use maud::{html, Markup};

pub trait BlockRenderer<T> {
    fn page_block(&self, children: &T, text: Option<T>) -> T;
    fn text_block(&self, children: &T, text: Option<T>) -> T;
    fn bulleted_list_block(&self, children: &T, text: Option<T>) -> T;
    fn numbered_list_block(&self, children: &T, text: Option<T>) -> T;
    fn toggle_block(&self, children: &T, text: Option<T>) -> T;
    fn empty(&self) -> T;
}

pub struct Blocks {}

impl BlockRenderer<Markup> for Blocks {
    fn page_block(&self, children: &Markup, text: Option<Markup>) -> Markup {
        html! {
            h1 class="notion-page-block" {
                @if let Some(text) = text {
                    (text)
                }
                div {
                    (children)
                }
            }
        }
    }
    fn text_block(&self, children: &Markup, text: Option<Markup>) -> Markup {
        html! {
            p class="notion-text-block" {
                @if let Some(text) = text {
                    (text)
                }
                div {
                    (children)
                }
            }
        }
    }
    fn bulleted_list_block(&self, children: &Markup, text: Option<Markup>) -> Markup {
        html! {
            li class="notion-bulleted_list-block" {
                @if let Some(text) = text {
                    (text)
                }
                div {
                    (children)
                }
            }
        }
    }
    fn numbered_list_block(&self, children: &Markup, text: Option<Markup>) -> Markup {
        html! {
            li class="notion-numbered_list-block" {
                @if let Some(text) = text {
                    (text)
                }
                div {
                    (children)
                }
            }
        }
    }
    fn toggle_block(&self, children: &Markup, text: Option<Markup>) -> Markup {
        html! {
            detail class="notion-toggle-block" {
                @if let Some(text) = text {
                    (text)
                }
                summary {
                    (children)
                }
            }
        }
    }
    fn empty(&self) -> Markup {
        html! {}
    }
}