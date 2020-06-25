use crate::renderer::Renderer;
use crate::parser::*;
use maud::{html, Markup};

pub fn render_numbered_list_wrapper<'a>(vector: &'a Vec<&'a BaseValueType>, renderer: &Renderer) -> Markup {
    html! {
        ol class="notion-numbered_list-wrapper" {
            @for item in vector.iter() {
                (renderer.render(&item.id))
            }
        }
    }
}

pub fn render_bulleted_list_wrapper<'a>(vector: &'a Vec<&'a BaseValueType>, renderer: &Renderer) -> Markup {
    html! {
        ul class="notion-bulleted_list-wrapper" {
            @for item in vector.iter() {
                (renderer.render(&item.id))
            }
        }
    }
}

pub fn render_page(properties: &PageProperties, children: &Option<Vec<String>>, renderer: &Renderer) -> Markup {
    let mut rendered_children = html! {};
    if let Some(children) = children {
        rendered_children = renderer.render_children(&children);
    }
    html! {
        h1 class="notion-page-block" {
            (renderer.render_text(&properties.title))
        }
        (rendered_children)
    }
}

pub fn render_text_block(properties: &Option<TextProperties>, children: &Option<Vec<String>>, renderer: &Renderer) -> Markup {
    let mut rendered_children = html! {};
    if let Some(children) = children {
        rendered_children = renderer.render_children(&children);
    }
    html! {
        p class="notion-text-block" {
            @if let Some(properties) = properties {
                (renderer.render_text(&properties.title))
            }
            (rendered_children)
        }
    }
}

pub fn render_bulleted_list(properties: &Option<TextProperties>, children: &Option<Vec<String>>, renderer: &Renderer) -> Markup {
    let mut rendered_children = html! {};
    if let Some(children) = children {
        rendered_children = renderer.render_children(&children);
    }
    html! { 
        li class="notion-bulleted_list-block" {
            @if let Some(properties) = properties {
                (renderer.render_text(&properties.title))
            }
            div {
                (rendered_children)
            }
        }
    }
}

pub fn render_numbered_list(properties: &Option<TextProperties>, children: &Option<Vec<String>>, renderer: &Renderer) -> Markup {
    let mut rendered_children = html! {};
    if let Some(children) = children {
        rendered_children = renderer.render_children(&children);
    }
    html! { 
        li class="notion-numbered_list-block" {
            @if let Some(properties) = properties {
                (renderer.render_text(&properties.title))
            }
            (rendered_children)
        }
    }
}

pub fn render_toggle(properties: &Option<TextProperties>, children: &Option<Vec<String>>, renderer: &Renderer) -> Markup {
    let mut rendered_children = html! {};
    if let Some(children) = children {
        rendered_children = renderer.render_children(&children);
    }
    html! { 
        detail {
            @if let Some(properties) = properties {
                (renderer.render_text(&properties.title))
            }
            summary {
                (rendered_children)
            }
        }
    }
}