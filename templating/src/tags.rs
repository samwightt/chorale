use crate::attributes::Attribute;
use regex::{Regex, Captures};

#[derive(Debug)]
pub struct Tag(String);

pub type TagType = fn(a: Vec<Attribute>, b: Vec<Tag>) -> Tag;

impl Tag {
    pub fn to_string(&self) -> String {
        match self {
            Tag(str) => str.clone(),
        }
    }
}

pub fn collect(t: Vec<Tag>) -> Tag {
    Tag(t
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(""))
}

fn escape(t: &str) -> String {
    let re = Regex::new("(&|<|>|\"|')").unwrap();
    let result = re.replace_all(t, |cap: &Captures| {
        match &cap[0] {
            "&" => "&amp;",
            "<" => "&lt;",
            ">" => "&gt;",
            "\"" => "&quot;",
            "'" => "&apos;",
            _ => ""
        }
    }).to_string();
    return result;
}

pub fn text(t: &str) -> Tag {
    Tag(escape(t))
}

pub fn empty() -> Tag {
    Tag(String::from(""))
}

pub fn option_include(t: Option<Tag>) -> Tag {
    match t {
        Some(t) => t,
        None => empty(),
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
    tags.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn tag(name: &str, attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag {
    let attributes = join_attributes(attributes);
    let children = join_children(children);

    if attributes.len() > 0 && children.len() > 0 {
        return Tag(["<", &name, " ", &attributes, ">", &children, "</", &name, ">"].concat())
    }
    if attributes.len() > 0 {
        return Tag(["<", &name, " ", &attributes, "/>"].concat());
    }
    if children.len() > 0 {
        return Tag(["<", &name, ">", &children, "</", &name, ">"].concat());
    } 
    return empty();
}

macro_rules! t {
    ($x:ident) => {
        pub fn $x(attributes: Vec<Attribute>, children: Vec<Tag>) -> Tag {
            tag(stringify!($x), attributes, children)
        }
    }
}

t!(h1);
t!(h2);
t!(h3);
t!(h4);
t!(h5);
t!(h6);
t!(div);
t!(p);
t!(hr);
t!(pre);
t!(blockquote);
t!(span);
t!(a);
t!(code);
t!(em);
t!(strong);
t!(i);
t!(b);
t!(u);
t!(sub);
t!(br);
t!(ol);
t!(ul);
t!(li);
t!(dl);
t!(dt);

pub fn img(attributes: Vec<Attribute>) -> Tag {
    tag("img", attributes, vec![])
}

t!(iframe);
t!(canvas);
t!(math);
t!(form);
t!(input);
t!(textarea);
t!(button);
t!(select);
t!(option);
t!(section);
t!(nav);
t!(article);
t!(aside);
t!(header);
t!(footer);
t!(address);
t!(main);
t!(figure);
t!(figcaption);
t!(table);
t!(caption);
t!(colgroup);
t!(col);
t!(tbody);
t!(thead);
t!(tfoot);
t!(tr);
t!(td);
t!(th);
t!(fieldset);
t!(legend);
t!(label);
t!(datalist);
t!(optgroup);
t!(output);
t!(progress);
t!(meter);
t!(audio);
t!(video);
t!(source);
t!(track);
t!(embed);
t!(object);
t!(param);
t!(ins);
t!(del);
t!(small);
t!(cite);
t!(dfn);
t!(abbr);
t!(time);
t!(var);
t!(samp);
t!(kbd);
t!(s);
t!(mark);
t!(q);
t!(ruby);
t!(rt);
t!(rp);
t!(bd);
t!(bdo);
t!(wbr);
t!(details);
t!(summary);
t!(menu);
t!(strike);