mod blocks;

use crate::parser::*;
use maud::{html, Markup};
use blocks::*;

type Accumulator<'a> = (Vec<Markup>, Vec<&'a BaseValueType>);

pub struct Renderer<'b> {
    blocks: &'b BlockTableType,
}

impl<'b> Renderer<'b> {
    pub fn new(blocks: &'b BlockTableType) -> Self {
        Renderer {
            blocks,
        }
    }

    fn needs_grouping(&self, value: &RootBlockType) -> bool {
        match value {
            RootBlockType::BulletedList {properties: _} |
            RootBlockType::NumberedList {properties: _} => true,
            _ => false
        }
    }

    fn can_be_grouped<'a>(&self, value: &RootBlockType, vector: &Vec<&'a BaseValueType>) -> bool {
        if vector.len() == 0 { false }
        else {
            let first = &vector[0];
            match (&first.block, value) {
                (RootBlockType::BulletedList { properties: _}, RootBlockType::BulletedList { properties: _}) |
                (RootBlockType::NumberedList { properties: _}, RootBlockType::NumberedList {properties: _}) => true,
                _ => false
            }
        }
    }


    fn render_wrapper<'a>(&self, vector: &'a Vec<&'a BaseValueType>) -> Markup {
        if vector.len() == 0 { return html! {} }
        let first = vector[0];
        match first.block {
            RootBlockType::BulletedList {properties: _} => render_bulleted_list_wrapper(&vector, &self),
            RootBlockType::NumberedList {properties: _} => render_numbered_list_wrapper(&vector, &self),
            _ => html! {}
        }
    }

    pub fn render_children<'a>(&self, ids: &Vec<String>) -> Markup {
        let acc: Accumulator<'a> = (vec![], vec![]);

        let results = ids.iter().fold(acc, |(mut a, mut b), x| {
            let element = self.blocks.get(x);
            if let Some(block) = element {
                if let Either::Left(result) = &block.value {
                    let rendered = self.render(&x);
                    if self.needs_grouping(&result.block) && (self.can_be_grouped(&result.block, &b) || b.len() == 0) {
                        b.push(result);
                        return (a, b);
                    }
                    else if self.needs_grouping(&result.block) {
                        b.push(&result);
                        let result = self.render_wrapper(&b);
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
            results.push(self.render_wrapper(&b));
        }

        html! {
            @for result in results.iter() {
                (result)
            }
        }
    }
    pub fn render_text(&self, text: &Vec<FormattedText>) -> Markup {
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
                                    b class="notion-bold" {
                                        (other)
                                    }
                                },
                                NoContextFormat::Italic => html! {
                                    em class="notion-italic" {
                                        (other)
                                    }
                                },
                                NoContextFormat::Strike => html! {
                                    strike class="notion-strike" {
                                        (other)
                                    }
                                },
                                NoContextFormat::Underline => html! {
                                    u class="notion-underline" {
                                        (other)
                                    }
                                },
                                NoContextFormat::Code => html! {
                                    code class="notion-code-inline" {
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

    pub fn render(&self, id: &String) -> Markup {
        let root = self.blocks.get(id);

        // We want to always return *something*, so this function doesn't deal with error cases
        if let Some(root) = root {
            let value = &root.value;

            if let Either::Left(value) = value { 
                return match &value.block {
                    RootBlockType::Page {format: _, file_ids: _, properties } => render_page(properties, &value.content, &self),
                    RootBlockType::Text {properties} => render_text_block(properties, &value.content, &self),
                    RootBlockType::BulletedList {properties} => render_bulleted_list(properties, &value.content, &self),
                    RootBlockType::NumberedList {properties} => render_numbered_list(properties, &value.content, &self),
                    RootBlockType::Toggle {properties} => render_toggle(properties, &value.content, &self),
                    _ => html! {
                        h1 {
                            "Could not render!"
                        }
                    }
                }
            }
        }

        html! {}
    }
}