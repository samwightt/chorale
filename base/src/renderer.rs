use crate::parser::*;
use std::marker::PhantomData;

pub trait BlockRenderer<T> {
    fn page_block(&self, children: &T, text: Option<T>) -> T;
    fn text_block(&self, children: &T, text: Option<T>) -> T;
    fn bulleted_list_block(&self, children: &T, text: Option<T>) -> T;
    fn numbered_list_block(&self, children: &T, text: Option<T>) -> T;
    fn toggle_block(&self, children: &T, text: Option<T>) -> T;
    fn empty(&self) -> T;
}

pub trait InlineRenderer<T> {
    type R;
    fn text(&self, text: &str) -> T;
    fn bold(&self, acc: &T) -> T;
    fn italic(&self, acc: &T) -> T;
    fn underline(&self, acc: &T) -> T;
    fn strike(&self, acc: &T) -> T;
    fn code(&self, acc: &T) -> T;
}

pub trait WrapperRenderer<T> {
    fn bulleted_list_wrapper(&self, items: Vec<T>) -> T;
    fn numbered_list_wrapper(&self, items: Vec<T>) -> T;
    fn collect(&self, items: Vec<T>) -> T;
}

pub struct Renderer<'b, R, B: BlockRenderer<R>, I: InlineRenderer<R>, W: WrapperRenderer<R>> {
    blocks: &'b BlockTableType,
    block_renderer: B,
    inline_renderer: I,
    wrapper_renderer: W,
    p: PhantomData<R>
}

impl<'b, R, B: BlockRenderer<R>, I: InlineRenderer<R>, W: WrapperRenderer<R>> Renderer<'b, R, B, I, W> {
    pub fn new(blocks: &'b BlockTableType, block_renderer: B, inline_renderer: I, wrapper_renderer: W) -> Self {
        Renderer {
            blocks,
            block_renderer,
            inline_renderer,
            wrapper_renderer,
            p: PhantomData
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

    fn render_wrapper<'a>(&self, vector: &'a Vec<&'a BaseValueType>) -> R {
        if vector.len() == 0 { return self.block_renderer.empty() }
        let first = vector[0];
        let rendered = vector.iter().map(|x| self.render(&x.id)).collect::<Vec<_>>();

        match first.block {
            RootBlockType::BulletedList {properties: _} => self.wrapper_renderer.bulleted_list_wrapper(rendered),
            RootBlockType::NumberedList {properties: _} => self.wrapper_renderer.numbered_list_wrapper(rendered),
            _ => self.block_renderer.empty()
        }
    }

    pub fn render_children<'a>(&self, ids: &Vec<String>) -> R {
        let acc: (Vec<R>, Vec<&'a BaseValueType>) = (vec![], vec![]);

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
                    else if b.len() > 0 {
                        a.push(rendered);
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

        self.wrapper_renderer.collect(results)
    }

    pub fn render_text(&self, text: &Vec<FormattedText>) -> R {
        text.iter().fold(self.block_renderer.empty(), |acc, x| {
            if let Some(formatting) = &x.formatting {
                let initial = self.inline_renderer.text(&x.text);
                let resulting = formatting.iter().fold(initial, |other, y| {
                    return match y {
                        FormatType::NoContext(f) => {
                            match f {
                                NoContextFormat::Bold => self.inline_renderer.bold(&other),
                                NoContextFormat::Italic => self.inline_renderer.italic(&other),
                                NoContextFormat::Strike => self.inline_renderer.strike(&other),
                                NoContextFormat::Underline => self.inline_renderer.underline(&other),
                                NoContextFormat::Code => self.inline_renderer.code(&other),
                                _ => other
                            }
                        },
                        _ => other
                    }
                });
                return self.wrapper_renderer.collect(vec![acc, resulting]);
            }

            return self.wrapper_renderer.collect(vec![acc, self.inline_renderer.text(&x.text)]);
        })
    }

    pub fn render(&self, id: &String) -> R {
        let root = self.blocks.get(id);

        // We want to always return *something*, so this function doesn't deal with error cases
        if let Some(root) = root {
            let value = &root.value;

            if let Either::Left(value) = value { 
                let temp: &Vec<String> = &vec![];
                let child_ids = value.content.as_ref().unwrap_or(temp);
                let children = self.render_children(&child_ids);
                return match &value.block {
                    RootBlockType::Page {format: _, file_ids: _, properties } => self.block_renderer.page_block(&children, Some(self.render_text(&properties.title))),
                    RootBlockType::Text {properties} => self.block_renderer.text_block(&children, properties.as_ref().map(|x| self.render_text(&x.title))),
                    RootBlockType::BulletedList {properties } => self.block_renderer.bulleted_list_block(&children, properties.as_ref().map(|x| self.render_text(&x.title))),
                    RootBlockType::NumberedList {properties} => self.block_renderer.numbered_list_block(&children, properties.as_ref().map(|x| self.render_text(&x.title))),
                    RootBlockType::Toggle {properties} => self.block_renderer.toggle_block(&children, properties.as_ref().map(|x| self.render_text(&x.title))),
                    _ => self.block_renderer.empty()
                }
            }
        }

        self.block_renderer.empty()
    }
}