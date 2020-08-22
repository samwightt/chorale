use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum YesOrNo {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoProperties {
    pub title: Value,
    pub checked: Vec<Vec<YesOrNo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ColumnFormat {
    pub column_ratio: f64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ImageProperties {
    pub source: Vec<Vec<String>>,
    pub caption: Option<Vec<Vec<String>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageFormat {
    pub block_width: i64,
    pub block_height: i64,
    pub display_source: Option<i64>,
    pub block_full_width: Option<bool>,
    pub block_page_width: Option<bool>,
    pub block_aspect_ratio: f64,
    pub block_preserve_scale: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageFormat {
    pub page_full_width: Option<bool>,
    pub page_small_text: Option<bool>,
    pub page_cover_position: Option<f64>,
    pub block_locked: Option<bool>,
    pub page_cover: Option<String>,
    pub page_icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagePermissions {
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FigmaProperties {
    pub source: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FigmaFormat {
    pub block_height: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextProperties {
    pub title: Vec<FormattedText>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageProperties {
    pub title: Vec<FormattedText>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RootBlockType {
    Text {
        properties: Option<TextProperties>,
    },
    BulletedList {
        properties: Option<TextProperties>,
    },
    NumberedList {
        properties: Option<TextProperties>,
    },
    Toggle {
        properties: Option<TextProperties>,
    },
    Header {
        properties: Option<TextProperties>,
    },
    SubHeader {
        properties: Option<TextProperties>,
    },
    SubSubHeader {
        properties: Option<TextProperties>,
    },
    Quote {
        properties: Option<TextProperties>,
    },
    ToDo {
        properties: Option<TodoProperties>,
    },
    Divider,
    ColumnList,
    Column {
        format: ColumnFormat,
    },
    Image {
        properties: ImageProperties,
        format: ImageFormat,
        file_ids: Vec<String>,
    },
    Page {
        format: Option<PageFormat>,
        file_ids: Option<Vec<String>>,
        properties: PageProperties,
    },
    Figma {
        properties: Option<FigmaProperties>,
        format: Option<FigmaFormat>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockFormatType {
    pub block_color: Option<ColorType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseValueType {
    pub id: String,
    pub version: i64,
    pub created_time: i64,
    pub last_edited_time: i64,
    pub parent_id: String,
    pub parent_table: String,
    pub alive: bool,
    pub created_by_table: String,
    pub created_by_id: String,
    pub last_edited_by_table: String,
    pub last_edited_by_id: String,
    pub shard_id: Option<i64>,
    pub space_id: Option<String>,
    pub content: Option<Vec<String>>,
    pub format: Option<BlockFormatType>,
    #[serde(flatten)]
    pub block: RootBlockType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ColorType {
    Gray,
    Brown,
    Orange,
    Yellow,
    Teal,
    Blue,
    Purple,
    Pink,
    Red,
    GrayBackground,
    BrownBackground,
    OrangeBackground,
    YellowBackground,
    TealBackground,
    BlueBackground,
    PurpleBackground,
    PinkBackground,
    RedBackground,
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum NoContextFormat {
    Bold,
    Italic,
    Strike,
    Underline,
    Code,
    None,
}

impl Serialize for NoContextFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            NoContextFormat::Bold => "b",
            NoContextFormat::Italic => "i",
            NoContextFormat::Strike => "s",
            NoContextFormat::Underline => "_",
            NoContextFormat::Code => "c",
            NoContextFormat::None => "",
        })
    }
}

impl<'de> Deserialize<'de> for NoContextFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "i" => Ok(NoContextFormat::Italic),
            "b" => Ok(NoContextFormat::Bold),
            "s" => Ok(NoContextFormat::Strike),
            "_" => Ok(NoContextFormat::Underline),
            "c" => Ok(NoContextFormat::Code),
            _ => Ok(NoContextFormat::None),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum IntermediaryFormatEnum {
    Main(Either<[NoContextFormat; 1], ContextFormat>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum IntermediaryContextFormattingRepresentation {
    Main((String, String)),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "IntermediaryContextFormattingRepresentation")]
pub enum ContextFormat {
    Link(String),
    Highlight(ColorType),
    None,
}

pub fn from_color(c: &str) -> ColorType {
    match c {
        "gray" => ColorType::Gray,
        "brown" => ColorType::Brown,
        "orange" => ColorType::Orange,
        "yellow" => ColorType::Yellow,
        "teal" => ColorType::Teal,
        "blue" => ColorType::Blue,
        "purple" => ColorType::Purple,
        "pink" => ColorType::Pink,
        "red" => ColorType::Red,
        "gray_background" => ColorType::GrayBackground,
        "brown_background" => ColorType::BrownBackground,
        "orange_background" => ColorType::OrangeBackground,
        "yellow_background" => ColorType::YellowBackground,
        "teal_background" => ColorType::TealBackground,
        "blue_background" => ColorType::BlueBackground,
        "purple_background" => ColorType::PurpleBackground,
        "pink_background" => ColorType::PinkBackground,
        "red_background" => ColorType::RedBackground,
        _ => ColorType::None,
    }
}

impl From<IntermediaryContextFormattingRepresentation> for ContextFormat {
    fn from(t: IntermediaryContextFormattingRepresentation) -> Self {
        let IntermediaryContextFormattingRepresentation::Main((a, b)) = t;
        match a.as_str() {
            "a" => ContextFormat::Link(b),
            "h" => ContextFormat::Highlight(from_color(&b)),
            _ => ContextFormat::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "IntermediaryFormatEnum")]
pub enum FormatType {
    NoContext(NoContextFormat),
    Context(ContextFormat),
}

impl From<IntermediaryFormatEnum> for FormatType {
    fn from(t: IntermediaryFormatEnum) -> Self {
        let IntermediaryFormatEnum::Main(s) = t;
        match s {
            Either::Left(l) => FormatType::NoContext(l[0]),
            Either::Right(l) => FormatType::Context(l),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum IntermediaryFormattingRepresentation {
    Main(Either<Vec<String>, (String, Vec<FormatType>)>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "IntermediaryFormattingRepresentation")]
pub struct FormattedText {
    pub text: String,
    pub formatting: Option<Vec<FormatType>>,
}

impl From<IntermediaryFormattingRepresentation> for FormattedText {
    fn from(text: IntermediaryFormattingRepresentation) -> Self {
        let IntermediaryFormattingRepresentation::Main(s) = text;
        match s {
            Either::Left(r) => FormattedText {
                text: r[0].clone(),
                formatting: None,
            },
            Either::Right((text, format)) => FormattedText {
                text,
                formatting: Some(format),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockType {
    pub role: String,
    pub value: Either<BaseValueType, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionUserValueType {
    pub id: String,
    pub version: i64,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub profile_photo: String,
    pub pubonboarding_complete: Option<bool>,
    pub mobile_onboarding_complete: Option<bool>,
    pub clipper_onboarding_complete: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotionUserType {
    pub role: String,
    pub value: NotionUserValueType,
}

pub type BlockTableType = HashMap<String, BlockType>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordMapType {
    pub block: HashMap<String, BlockType>,
    pub notion_user: HashMap<String, NotionUserType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadPageChunkData {
    pub record_map: RecordMapType,
}

pub fn parse(input: String) -> Result<LoadPageChunkData> {
    let result: LoadPageChunkData = serde_json::from_str(&input)?;
    Ok(result)
}
