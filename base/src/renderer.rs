use crate::parser::*;
use std::marker::PhantomData;

pub trait BlockRenderer<T> {
    fn page_block(&self, children: T, text: Option<T>) -> T;
    fn text_block(&self, children: T, text: Option<T>) -> T;
    fn bulleted_list_block(&self, children: T, text: Option<T>) -> T;
    fn numbered_list_block(&self, children: T, text: Option<T>) -> T;
    fn toggle_block(&self, children: T, text: Option<T>) -> T;
    fn quote_block(&self, children: T, text: Option<T>) -> T;
    fn header_block(&self, children: T, text: Option<T>) -> T;
    fn sub_header_block(&self, children: T, text: Option<T>) -> T;
    fn sub_sub_header_block(&self, children: T, text: Option<T>) -> T;
    fn divider_block(&self, children: T) -> T;
    fn empty(&self) -> T;
}

pub trait InlineRenderer<T> {
    fn text(&self, text: &str) -> T;
    fn bold(&self, acc: T) -> T;
    fn italic(&self, acc: T) -> T;
    fn underline(&self, acc: T) -> T;
    fn strike(&self, acc: T) -> T;
    fn link(&self, acc: T, link: &String) -> T;
    fn code(&self, acc: T) -> T;
    fn highlight(&self, acc: T, color: &ColorType) -> T;
}

pub trait WrapperRenderer<T> {
    fn bulleted_list_wrapper(&self, items: Vec<T>) -> T;
    fn numbered_list_wrapper(&self, items: Vec<T>) -> T;
    fn collect(&self, items: Vec<T>) -> T;
}

/// The main Chorale renderer. This takes care of all the Notion rendering logic
/// (eg. parsing the Notion JSON, iterating over the JSON, and building out the result
/// that will be returned). This piece of code is **not** responsible for actually implementing
/// the templating logic. Instead, it calls methods on three different structs that have the traits `BlockRenderer`,
/// `InlineRenderer`, and `WrapperRenderer`. By calling these in specific orders, it is able
/// to return a result without needing to ever manipulate the result directly, just by
/// calling functions to do so.
///
/// The renderer has ownership of all three structs and the block table that are passed in (using the `new` method),
/// but does not own the `BlockTableType` that may or may not be passed in at different points.
/// Templating logic and the block table are explicitly tied to the renderer instance, but the renderer is reusable, meaning
/// you can declare a single renderer and it to renderer multiple ids on a single block table.
///
/// This renderer uses the definitions provided by the `parser` module, which declares a number
/// of strict `serde_json` typings with graceful fallbacks. The renderer takes the same approach,
/// gracefully falling back if it encounters JSON that it doesn't necessarily understand. If a block
/// is invalid according to the latest version of its typings, it will skip rendering that block and
/// all of its children.
pub struct Renderer<'b, R, B, I, W> {
    /// Property that caches the block table, making it easier to pass it down during recursion.
    blocks: &'b BlockTableType,
    /// Templating definition for rendering blocks with the trait type of `BlockRenderer`.
    block_renderer: B,
    /// Templating definition for rendering inline markup with the trait type of `InlineRenderer`.
    inline_renderer: I,
    /// Templating definition for rendering wrappers with the trait type of `WrapperRenderer`.
    /// Blocks like bulleted lists or numbered lists need to be rendered inside of a `ul` tag (grouped together).
    /// The wrapper renderer takes care of this logic.
    wrapper_renderer: W,
    /// A phantom value to get the compiler to shut up about an unused parameter.
    p: PhantomData<R>,
}

impl<'b, R, B, I, W> Renderer<'b, R, B, I, W>
where
    B: BlockRenderer<R>,
    I: InlineRenderer<R>,
    W: WrapperRenderer<R>,
{
    /// Creates a new renderer. Requires a block table (the parsed Notion JSON document, only the blocks),
    /// a block renderer struct that implements the `BlockRenderer` trait for some `R`, an inline renderer
    /// struct that implements the `InlineRenderer` trait for some `R`, and a wrapper renderer that implements
    /// the `WrapperRenderer` trait for some `R`. Returns a renderer that can then be used to render individual
    /// blocks (and their children) on that block table.
    // TODO: Change blocks to be a string and handle all of the parsing.
    pub fn new(
        blocks: &'b BlockTableType,
        block_renderer: B,
        inline_renderer: I,
        wrapper_renderer: W,
    ) -> Self {
        Renderer {
            blocks,
            block_renderer,
            inline_renderer,
            wrapper_renderer,
            p: PhantomData,
        }
    }

    /// This is a helper function for determining if a specific `RootBlockType` needs
    /// grouping or not. This should probably not be defined on the renderer's struct and should
    /// probably be moved to the `parser` module.
    // TODO: Move this to the parser module.
    fn needs_grouping(&self, value: &RootBlockType) -> bool {
        match value {
            RootBlockType::BulletedList { properties: _ }
            | RootBlockType::NumberedList { properties: _ } => true,
            _ => false,
        }
    }

    /// Determines if a given `RootBlockType` value can be grouped with the other
    /// `BaseValueType`s found in the grouping accumulation vector. This only compares the type against
    /// the first item in the vector; it is assumed that the only items that are allowed to be added to
    /// the vector are items of the same block type. Therefore, we only need to check a single value,
    /// instead of checking all values in the vector.
    ///
    /// # Arguments
    /// - `value` - The value to compare against.
    /// - `vector` - The vector to compare the value against.
    // TODO: Rewrite the checking logic to use `needs_grouping`.
    fn can_be_grouped<'a>(&self, value: &RootBlockType, vector: &Vec<&'a BaseValueType>) -> bool {
        if vector.len() == 0 {
            false
        } else {
            let first = &vector[0];
            match (&first.block, value) {
                (
                    RootBlockType::BulletedList { properties: _ },
                    RootBlockType::BulletedList { properties: _ },
                )
                | (
                    RootBlockType::NumberedList { properties: _ },
                    RootBlockType::NumberedList { properties: _ },
                ) => true,
                _ => false,
            }
        }
    }

    /// Renders a wrapper around the items in the accumulation `vector`. If the vector is empty,
    /// returns an empty item, else, returns the wrapped elements.
    ///
    /// # Arguments
    /// - `grouping_set` - The grouping set (a vector of `BaseValueType`s) to wrap.
    fn render_wrapper<'a>(&self, grouping_set: &'a Vec<&'a BaseValueType>) -> R {
        if grouping_set.len() == 0 {
            return self.block_renderer.empty();
        }

        let first_block_value = grouping_set[0];
        let rendered_items = grouping_set
            .iter()
            .map(|x| self.render(&x.id))
            .collect::<Vec<_>>();

        match first_block_value.block {
            RootBlockType::BulletedList { properties: _ } => {
                self.wrapper_renderer.bulleted_list_wrapper(rendered_items)
            }
            RootBlockType::NumberedList { properties: _ } => {
                self.wrapper_renderer.numbered_list_wrapper(rendered_items)
            }
            _ => self.block_renderer.empty(),
        }
    }

    /// Renders an array of IDs (children). Calls `render` for each, wrapping them if necessary.
    /// Uses the `collect` method on the user-defined `WrapperRenderer` to collect
    /// all of the items after rendering them each.
    ///
    /// If an ID is not found in the block table, it will not be rendered.
    /// The `collect` method on the `WrapperRenderer` should handle cases where there are no elements
    /// (eg. simply calling an `empty` method on the `BlockRenderer`).
    ///
    /// # Arguments
    /// - `block_ids` - A string array of IDs to render.
    // TODO: If the resulting array is empty, don't call `collect` and instead call `empty` on the `BlockRenderer`.
    pub fn render_children<'a>(&self, block_ids: &Vec<String>) -> R {
        // Defines an accumaltor variable that is a tuple. The first item in the tuple
        // is an array of completely rendered items, and the second item is a temporary
        // accumulator array for grouping.
        // This would be a good candidate to replace with an enum that has more self-explanatory names.
        let accumulator: (Vec<R>, Vec<&'a BaseValueType>) = (vec![], vec![]);

        // Shadowing the variable here because it's still an accumulator.
        let accumulator = block_ids.iter().fold(
            accumulator,
            |(mut rendered_items, mut grouping_set), block_id| {
                let element = self.blocks.get(block_id);

                if let Some(block) = element {
                    if let Either::Left(block_value) = &block.value {
                        let rendered = self.render(&block_id);

                        let block_can_be_grouped = self.needs_grouping(&block_value.block)
                            && (self.can_be_grouped(&block_value.block, &grouping_set)
                                || grouping_set.len() == 0);

                        if block_can_be_grouped {
                            grouping_set.push(block_value);
                            return (rendered_items, grouping_set);
                        } else if self.needs_grouping(&block_value.block) {
                            grouping_set.push(&block_value);
                            rendered_items.push(self.render_wrapper(&grouping_set));
                            return (rendered_items, vec![]);
                        } else if grouping_set.len() > 0 {
                            rendered_items.push(self.render_wrapper(&grouping_set));
                            rendered_items.push(rendered);
                            return (rendered_items, vec![]);
                        } else {
                            rendered_items.push(rendered);
                            return (rendered_items, grouping_set);
                        }
                    }
                }

                return (rendered_items, grouping_set);
            },
        );

        let (mut rendered_items, grouping_set) = accumulator;
        if grouping_set.len() > 0 {
            rendered_items.push(self.render_wrapper(&grouping_set));
        }

        self.wrapper_renderer.collect(rendered_items)
    }

    /// Renders a piece of text (`FormattedText`), wrapping it with the appropriate inline styles.
    /// 
    /// Inline styles can either be context styles or no context styles. No context styles don't require
    /// any other information than the *type* of the inline style. Context inline styles, however, require
    /// some other piece of contex to render them (eg. a link requires an `href` attribute). Different methods 
    /// on the `InlineRenderer` passed in to the renderer are called based on this, with different numbers
    /// of attributes.
    ///
    /// This function iterates over all of the rendered text, then runs the `collect` method to concatenate
    /// the resulting array of `R` values into a single `R` value.
    ///
    /// # Arguments
    /// - text - A vector of `FormattedText` to render. Can have zero items if necessary.
    // TODO: Change `text.iter().fold` to use `text.iter().map` and `collect` at the end only once.
    pub fn render_text(&self, text: &Vec<FormattedText>) -> R {
        let empty_initial_value = self.block_renderer.empty();

        text.iter().fold(empty_initial_value, |acc, current_text| {
            if let Some(formatting) = &current_text.formatting {
                let initial_text = self.inline_renderer.text(&current_text.text);

                let wrapped_formatted_text = formatting.iter().fold(initial_text, |formatting_acc, current_format| {
                    return match current_format {
                        FormatType::NoContext(format_type) => match format_type {
                            NoContextFormat::Bold => self.inline_renderer.bold(formatting_acc),
                            NoContextFormat::Italic => self.inline_renderer.italic(formatting_acc),
                            NoContextFormat::Strike => self.inline_renderer.strike(formatting_acc),
                            NoContextFormat::Underline => self.inline_renderer.underline(formatting_acc),
                            NoContextFormat::Code => self.inline_renderer.code(formatting_acc),
                            _ => formatting_acc,
                        },
                        FormatType::Context(format_type) => match format_type {
                            ContextFormat::Link(href) => self.inline_renderer.link(formatting_acc, href),
                            ContextFormat::Highlight(highlight_color) => self.inline_renderer.highlight(formatting_acc, highlight_color),
                            _ => formatting_acc,
                        },
                    };
                });
                return self.wrapper_renderer.collect(vec![acc, wrapped_formatted_text]);
            }

            return self
                .wrapper_renderer
                .collect(vec![acc, self.inline_renderer.text(&current_text.text)]);
        })
    }

    /// Renders a block given the block ID. Renders the block's children if it has any by calling
    /// `render_children`. Renders the block content if it has any. If the block is empty or does not exist, 
    /// calls the `empty` method on the renderer's `BlockRenderer`.
    ///
    /// This method solely calls methods on the `BlockRenderer` based on the block's type, passing
    /// in the appropriate methods. It returns a single `R`, whatever the templating value is.
    ///
    /// # Arguments
    /// - `block_id` - The ID of the block to render.
    pub fn render(&self, block_id: &str) -> R {
        // We want to always return *something*, so this function doesn't deal with error cases
        if let Some(root_block) = self.blocks.get(block_id) {
            if let Either::Left(block_value) = &root_block.value {
                let default_child_ids: &Vec<String> = &vec![];
                let child_ids = block_value.content.as_ref().unwrap_or(default_child_ids);

                let children = self.render_children(&child_ids);

                return match &block_value.block {
                    RootBlockType::Page {
                        format: _,
                        file_ids: _,
                        properties,
                    } => self
                        .block_renderer
                        .page_block(children, Some(self.render_text(&properties.title))),
                    RootBlockType::Text { properties } => self.block_renderer.text_block(
                        children,
                        properties.as_ref().map(|x| self.render_text(&x.title)),
                    ),
                    RootBlockType::BulletedList { properties } => {
                        self.block_renderer.bulleted_list_block(
                            children,
                            properties.as_ref().map(|x| self.render_text(&x.title)),
                        )
                    }
                    RootBlockType::NumberedList { properties } => {
                        self.block_renderer.numbered_list_block(
                            children,
                            properties.as_ref().map(|x| self.render_text(&x.title)),
                        )
                    }
                    RootBlockType::Quote { properties } => self.block_renderer.quote_block(
                        children,
                        properties.as_ref().map(|x| self.render_text(&x.title)),
                    ),
                    RootBlockType::Header { properties } => self.block_renderer.header_block(
                        children,
                        properties.as_ref().map(|x| self.render_text(&x.title)),
                    ),
                    RootBlockType::SubHeader { properties } => {
                        self.block_renderer.sub_header_block(
                            children,
                            properties.as_ref().map(|x| self.render_text(&x.title)),
                        )
                    }
                    RootBlockType::SubSubHeader { properties } => {
                        self.block_renderer.sub_sub_header_block(
                            children,
                            properties.as_ref().map(|x| self.render_text(&x.title)),
                        )
                    }
                    RootBlockType::Toggle { properties } => self.block_renderer.toggle_block(
                        children,
                        properties.as_ref().map(|x| self.render_text(&x.title)),
                    ),
                    RootBlockType::Divider => self.block_renderer.divider_block(children),
                    _ => self.block_renderer.empty(),
                };
            }
        }

        self.block_renderer.empty()
    }
}
