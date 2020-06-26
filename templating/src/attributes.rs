pub struct Attribute(String);

pub fn attribute(key: String, value: String) -> Attribute {
    Attribute(std::format!(r#"{}={}"#, key, value))
}

pub fn class(name: String) -> Attribute {
    attribute(String::from("class"), name)
}

pub fn id(name: String) -> Attribute {
    attribute(String::from("id"), name)
}