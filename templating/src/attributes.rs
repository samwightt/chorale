pub struct Attribute(String);

impl Attribute {
    pub fn to_string(&self) -> String {
        match self {
            Attribute(str) => str.clone(),
        }
    }
}

pub fn attribute(key: &str, value: &str) -> Attribute {
    Attribute([key, "=\"", value, "\""].concat())
}

pub fn style_list(input: Vec<(&str, &str)>) -> Attribute {
    let value = input
        .iter()
        .map(|(a, b)| format!("{}:{}", a, b))
        .collect::<Vec<String>>()
        .join(";");

    attribute("style", &value)
}

pub fn class(name: &str) -> Attribute {
    attribute("class", name)
}

pub fn id(name: &str) -> Attribute {
    attribute("id", name)
}

pub fn title(name: &str) -> Attribute {
    attribute("title", name)
}

pub fn r#type(name: &str) -> Attribute {
    attribute("type", name)
}

pub fn value(name: &str) -> Attribute {
    attribute("value", name)
}

pub fn placeholder(name: &str) -> Attribute {
    attribute("placeholder", name)
}

pub fn selected(name: &str) -> Attribute {
    attribute("selected", name)
}

pub fn href(name: &str) -> Attribute {
    attribute("href", name)
}
