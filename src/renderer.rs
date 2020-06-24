use crate::parser::*;
use anyhow::{Result, anyhow};
use std::rc::Rc;
use horrorshow::{Render, box_html};
use maud::{DOCTYPE, html, Markup};

type Accumulator<'a> = (Vec<Markup>, Vec<&'a BaseValueType>);

pub fn needs_grouping(value: &RootBlockType) -> bool {
    match value {
        RootBlockType::BulletedList |
        RootBlockType::NumberedList => true,
        _ => false
    }
}

pub fn can_be_grouped<'a>(value: &RootBlockType, vector: &Vec<&'a BaseValueType>) -> bool {
    if vector.len() == 0 { false }
    else {
        let first = &vector[0];
        match (&first.block, value) {
            (RootBlockType::BulletedList, RootBlockType::BulletedList) |
            (RootBlockType::NumberedList, RootBlockType::NumberedList) => true,
            _ => false
        }
    }
}

pub fn render_wrapper<'a>(vector: &'a Vec<&'a BaseValueType>, blocks: &BlockTableType) -> Markup {
    if vector.len() == 0 { return html! {} }
    let first = vector[0];
    match first.block {
        RootBlockType::NumberedList |
        RootBlockType::BulletedList => {
            html! {
                ul {
                    @for item in vector.iter() {
                        li {
                            (match render(&item.id, &blocks) {
                                Ok(a) => a,
                                Err(_) => html! {}
                            })
                        }
                    }
                }
            }
        }
        _ => html! {}
    }
}

pub fn render_children<'a>(ids: &Vec<String>, blocks: &BlockTableType) -> Result<Markup> {
    let acc: Accumulator<'a> = (vec![], vec![]);

    let results = ids.iter().fold(acc, |(mut a, mut b), x| {
        let element = blocks.get(x);
        if let Some(block) = element {
            if let Either::Left(result) = &block.value {
                let rendered = render(&x, &blocks).unwrap_or(html! {});
                if needs_grouping(&result.block) && can_be_grouped(&result.block, &b) {
                    b.push(result);
                    return (a, b);
                }
                else if needs_grouping(&result.block) {
                    let result = render_wrapper(&b, &blocks);
                    a.push(result);
                    return (a, vec![]);
                }
                else {
                    a.push(rendered);
                    return (a, b);
                }
            }
        }

        return (a, b);
    });

    let (mut results, b) = results;
    if b.len() > 0 {
        results.push(render_wrapper(&b, &blocks));
    }
    Ok(html! {
        div {
            @for result in results.iter() {
                (result)
            }
        }
    })
}

pub fn render_text(text: &Vec<FormattedText>) -> Markup {
    text.iter().fold(html! {}, |acc, x| {
        if let Some(formatting) = &x.formatting {
            let initial = html! {
                (x.text)
            };
            let resulting = formatting.iter().fold(initial, |other, y| {
                return match y {
                    FormatType::NoContext(f) => {
                        match f {
                            NoContextFormat::Bold => html! {
                                b {
                                    (other)
                                }
                            },
                            NoContextFormat::Italic => html! {
                                em {
                                    (other)
                                }
                            },
                            _ => other
                        }
                    },
                    _ => other
                }
            });
            return html! {
                (acc)
                (resulting)
            };
        }
        return html! {
            (acc)
            (x.text)
        };
    })
}

pub fn render_page(properties: &PageProperties) -> Markup {
    html! {
        h1 {
            (render_text(&properties.title))
        }
    }
}

pub fn render_text_block(properties: &TextProperties) -> Markup {
    html! {
        p {
            (render_text(&properties.title))
        }
    }
}

fn render_block(block: &RootBlockType) -> Markup {
    match block {
        RootBlockType::Page {format, file_ids, properties } => render_page(properties),
        RootBlockType::Text {properties} => render_text_block(properties),
        _ => html! {
            h1 {
                "Could not render!"
            }
        }
    }
}

pub fn render(id: &String, blocks: &BlockTableType) -> Result<Markup> {
    let root = blocks.get(id);

    println!("We're hitting this weirdly.");

    // We want to always return *something*, so this function doesn't deal with error cases
    if let Some(root) = root {
        let value = &root.value;

        if let Either::Left(value) = value { 
            if let Some(children) = &value.content {
                let children = render_children(&children, &blocks);
                return Ok(html! {
                    div {
                        (render_block(&value.block))
                        @if let Ok(children) = children {
                            (children)
                        }
                    }
                });
            }
            else {
                return Ok(html! {
                    div {
                        (render_block(&value.block))
                    }
                });
            }
        }
    }
    println!("We're hitting here!");

    Ok(html! {})
}