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
        RootBlockType::NumberedList => {
            let count = vector.len();
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
        },
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
                else if !needs_grouping(&result.block) {
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

    let (results, _) = results;
    Ok(html! {
        div {
            @for result in results.iter() {
                (result)
            }
        }
    })
}

pub fn render(id: &String, blocks: &BlockTableType) -> Result<Markup> {
    let root = blocks.get(id);

    // We want to always return *something*, so this function doesn't deal with error cases
    if let Some(root) = root {
        let value = &root.value;

        if let Either::Left(value) = value { 
            if let Some(children) = &value.content {
                let children = render_children(&children, &blocks);
                return Ok(html! {
                    h1 {
                        "This works too!"
                        @if let Ok(children) = children {
                            (children)
                        }
                    }
                });
            }
            else {
                return Ok(html! {
                    h1 {
                        "This works!"
                    }
                });
            }
        }
    }

    Ok(html! {})
}