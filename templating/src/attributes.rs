pub struct Attribute(String);

impl Attribute {
    pub fn to_string(&self) -> String {
        match self {
            Attribute(str) => str.clone()
        }
    }
}

pub fn attribute(key: String, value: String) -> Attribute {
    Attribute(std::format!(r#"{}={}"#, key, value))
}

pub fn style_list(input: Vec<(String, String)>) -> Attribute {
    let value = input
        .iter()
        .map(|(a, b)| format!("{}:{}", a, b))
        .collect::<Vec<String>>()
        .join(";");

    attribute(String::from("style"), value)
}

pub fn class_list(input: Vec<String>) -> Attribute {
    class(input.iter().map(|x| format!(".{}", x)).collect::<Vec<String>>().join(""))
}

pub fn class(name: String) -> Attribute {
    attribute(String::from("class"), name)
}

pub fn id(name: String) -> Attribute {
    attribute(String::from("id"), name)
}

pub fn title(name: String) -> Attribute {
    attribute(String::from("title"), name)
}

pub fn r#type(name: String) -> Attribute {
    attribute(String::from("type"), name)
}

pub fn value(name: String) -> Attribute {
    attribute(String::from("value"), name)
}

pub fn placeholder(name: String) -> Attribute {
    attribute(String::from("placeholder"), name)
}

pub fn selected(name: String) -> Attribute {
    attribute(String::from("placeholder"), name)
}