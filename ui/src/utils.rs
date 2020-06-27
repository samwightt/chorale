use base::parser::ColorType;

pub fn color_class_name(c: &ColorType) -> &str {
    match c {
        ColorType::Gray => "notion-gray",
        ColorType::Brown => "notion-brown",
        ColorType::Orange => "notion-orange",
        ColorType::Yellow => "notion-yellow",
        ColorType::Teal => "notion-teal",
        ColorType::Blue => "notion-blue",
        ColorType::Purple => "notion-purple",
        ColorType::Pink => "notion-pink",
        ColorType::Red => "notion-red",
        ColorType::GrayBackground => "notion-gray_background",
        ColorType::BrownBackground => "notion-brown_background",
        ColorType::OrangeBackground => "notion-orange_background",
        ColorType::YellowBackground => "notion-yellow_background",
        ColorType::TealBackground => "notion-teal_background",
        ColorType::BlueBackground => "notion-blue_background",
        ColorType::PurpleBackground => "notion-purple_background",
        ColorType::PinkBackground => "notion-pink_background",
        ColorType::RedBackground => "notion-red_background",
        ColorType::None => ""
    }
}