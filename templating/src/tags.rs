use crate::attributes::Attribute;
use std::format;
pub struct Tag(String);

impl Tag {
    pub fn to_string(&self) -> String {
        match self {
            Tag(str) => str.clone()
        }
    }
}

fn join_attributes(attributes: Vec<Attribute>) -> String {
    attributes
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn join_children(tags: Vec<Tag>) -> String {
    tags
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn tag(name: String, attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag {
    if attributes.len() != 0 && children.len() != 0 {
        let attributes = join_attributes(attributes);
        let children = join_children(children);
        return Tag(format!("<{} {}>{}</{}>", name, attributes, children, name));
    }
    if attributes.len() != 0 {
        let attributes = join_attributes(attributes);
        return Tag(format!("<{} {}/>", name, attributes));
    }
    if children.len() != 0 {
        let children = join_children(children);
        return Tag(format!("<{}>{}</{}>", name, children, name));
    }
    else {
        return Tag(format!("<{}/>", name));
    }
}

pub fn single_tag(name: String, attributes: Vec<Attribute>) -> Tag {
    if attributes.len() != 0 {
        let attributes = join_attributes(attributes);
        return Tag(format!("<{} {}/>", name, attributes));
    }
    else {
        return Tag(format!("<{}/>", name));
    }
}

pub fn h1(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h1"), attributes, children)
}

pub fn h2(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h2"), attributes, children)
}

pub fn h3(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h3"), attributes, children)
}

pub fn h4(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h4"), attributes, children)
}

pub fn h5(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h5"), attributes, children)
}

pub fn h6(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("h6"), attributes, children)
}

pub fn div(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("div"), attributes, children)
}

pub fn p(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("p"), attributes, children)
}

pub fn hr(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("hr"), attributes, children)
}

pub fn pre(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("pre"), attributes, children)
}

pub fn blockquote(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("blockquote"), attributes, children)
}

pub fn span(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("span"), attributes, children)
}

pub fn a(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("a"), attributes, children)
}

pub fn code(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("code"), attributes, children)
}

pub fn em(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("em"), attributes, children)
}

pub fn strong(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("strong"), attributes, children)
}

pub fn i(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("i"), attributes, children)
}

pub fn b(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("b"), attributes, children)
}

pub fn u(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("u"), attributes, children)
}

pub fn sub(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("sub"), attributes, children)
}

pub fn sup(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("sup"), attributes, children)
}

pub fn br(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("br"), attributes, children)
}

pub fn ol(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("ol"), attributes, children)
}

pub fn ul(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("ul"), attributes, children)
}

pub fn li(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("li"), attributes, children)
}

pub fn dl(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("dl"), attributes, children)
}

pub fn dt(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("dt"), attributes, children)
}

pub fn img(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("img"), attributes, children)
}

pub fn iframe(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("iframe"), attributes, children)
}

pub fn canvas(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("canvas"), attributes, children)
}

pub fn math(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("math"), attributes, children)
}

pub fn form(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("form"), attributes, children)
}

pub fn input(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("input"), attributes, children)
}

pub fn textarea(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("textarea"), attributes, children)
}

pub fn button(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("button"), attributes, children)
}

pub fn select(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("select"), attributes, children)
}

pub fn option(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("option"), attributes, children)
}

pub fn section(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("section"), attributes, children)
}

pub fn nav(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("nav"), attributes, children)
}

pub fn article(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("article"), attributes, children)
}

pub fn aside(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("aside"), attributes, children)
}

pub fn header(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("header"), attributes, children)
}

pub fn footer(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("footer"), attributes, children)
}

pub fn address(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("address"), attributes, children)
}

pub fn main(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("main"), attributes, children)
}


pub fn figure(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("figure"), attributes, children)
}

pub fn figcaption(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("figcaption"), attributes, children)
}

pub fn table(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("table"), attributes, children)
}

pub fn caption(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("caption"), attributes, children)
}

pub fn colgroup(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("colgroup"), attributes, children)
}

pub fn col(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("col"), attributes, children)
}

pub fn tbody(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("tbody"), attributes, children)
}

pub fn thead(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("thead"), attributes, children)
}

pub fn tfoot(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("tfoot"), attributes, children)
}

pub fn tr(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("tr"), attributes, children)
}

pub fn td(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("td"), attributes, children)
}

pub fn th(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("th"), attributes, children)
}

pub fn fieldset(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("fieldset"), attributes, children)
}

pub fn legend(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("legend"), attributes, children)
}

pub fn label(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("label"), attributes, children)
}

pub fn datalist(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("datalist"), attributes, children)
}

pub fn optgroup(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("optgroup"), attributes, children)
}

pub fn output(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("output"), attributes, children)
}

pub fn progress(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("progress"), attributes, children)
}

pub fn meter(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("meter"), attributes, children)
}

pub fn audio(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("audio"), attributes, children)
}

pub fn video(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("video"), attributes, children)
}

pub fn source(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("source"), attributes, children)
}

pub fn track(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("track"), attributes, children)
}

pub fn embed(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("embed"), attributes, children)
}

pub fn object(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("object"), attributes, children)
}

pub fn param(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("param"), attributes, children)
}

pub fn ins(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("ins"), attributes, children)
}

pub fn del(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("del"), attributes, children)
}

pub fn small(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("small"), attributes, children)
}

pub fn cite(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("cite"), attributes, children)
}

pub fn dfn(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("dfn"), attributes, children)
}

pub fn abbr(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("abbr"), attributes, children)
}

pub fn time(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("time"), attributes, children)
}

pub fn var(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("var"), attributes, children)
}
pub fn samp(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("samp"), attributes, children)
}

pub fn kbd(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("kbd"), attributes, children)
}

pub fn s(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("s"), attributes, children)
}

pub fn q(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("q"), attributes, children)
}

pub fn mark(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("mark"), attributes, children)
}

pub fn ruby(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("ruby"), attributes, children)
}

pub fn rt(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("rt"), attributes, children)
}

pub fn rp(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("rp"), attributes, children)
}

pub fn bdi(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("bdi"), attributes, children)
}

pub fn bdo(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("bdo"), attributes, children)
}

pub fn wbr(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("wbr"), attributes, children)
}

pub fn details(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("details"), attributes, children)
}

pub fn summary(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("summary"), attributes, children)
}

pub fn menuitem(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("menuitem"), attributes, children)
}

pub fn menu(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag{
    tag(String::from("menu"), attributes, children)
}