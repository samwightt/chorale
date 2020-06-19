use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::{Value};
use std::collections::HashMap;
use anyhow::Result;

#[derive(Serialize, Deserialize)]
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
    pub content: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum YesOrNo {
    Yes,
    No
}

#[derive(Serialize, Deserialize)]
pub struct TodoProperties {
    pub title: Value,
    pub checked: Vec<Vec<YesOrNo>>
}

#[derive(Serialize, Deserialize)]
pub struct ColumnFormat {
    pub column_ratio: f64
}
#[derive(Serialize, Deserialize)]
pub struct ImageProperties {
    pub source: Vec<Vec<String>>,
    pub caption: Option<Vec<Vec<String>>>
}

#[derive(Serialize, Deserialize)]
pub struct ImageFormat {
    pub block_width: i64,
    pub block_height: i64,
    pub display_source: Option<i64>,
    pub block_full_width: Option<bool>,
    pub block_page_width: Option<bool>,
    pub block_aspect_ratio: f64,
    pub block_preserve_scale: bool
}

#[derive(Serialize, Deserialize)]
pub struct PageFormat {
    pub page_full_width: Option<bool>,
    pub page_small_text: Option<bool>,
    pub page_cover_position: Option<f64>,
    pub block_locked: Option<bool>,
    pub page_cover: Option<String>,
    pub page_icon: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct PagePermissions {
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct FigmaProperties {
    pub source: Option<Value>
}

#[derive(Serialize, Deserialize)]
pub struct FigmaFormat {
    pub block_height: Option<i64>
}

#[derive(Serialize, Deserialize)]
pub struct TextProperties {
    pub title: Vec<FormattedText>
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RootBlockType {
    Text { 
        #[serde(flatten)]
        data: BaseValueType,
        properties: Option<TextProperties>
    },
    BulletedList {
        #[serde(flatten)]
        data: BaseValueType
    },
    NumberedList {
        #[serde(flatten)]
        data: BaseValueType
    },
    Header {
        #[serde(flatten)]
        data: BaseValueType
    },
    SubHeader {
        #[serde(flatten)]
        data: BaseValueType
    },
    SubSubheader {
        #[serde(flatten)]
        data: BaseValueType
    },
    Quote {
        #[serde(flatten)]
        data: BaseValueType
    },
    ToDo {
        #[serde(flatten)]
        data: BaseValueType,
        properties: TodoProperties
    },
    Divider {
        #[serde(flatten)]
        data: BaseValueType
    },
    ColumnList {
        #[serde(flatten)]
        data: BaseValueType
    },
    Column {
        #[serde(flatten)]
        data: BaseValueType,
        format: ColumnFormat
    },
    Image {
        #[serde(flatten)]
        data: BaseValueType,
        properties: ImageProperties,
        format: ImageFormat,
        file_ids: Vec<String>
    },
    Page {
        #[serde(flatten)]
        data: BaseValueType,
        format: Option<PageFormat>,
        file_ids: Option<Vec<String>>
    },
    Figma {
        #[serde(flatten)]
        data: BaseValueType,
        properties: Option<FigmaProperties>,
        format: Option<FigmaFormat>
    }
}

#[derive(Serialize, Deserialize)]
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
    RedBackground
}



#[derive(Copy, Clone)]
pub enum NoContextFormat {
    Bold,
    Italic,
    None
}

impl Serialize for NoContextFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match *self {
            NoContextFormat::Bold => "b",
            NoContextFormat::Italic => "i",
            NoContextFormat::None => ""
        })
    }
}

impl<'de> Deserialize<'de> for NoContextFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "i" => Ok(NoContextFormat::Italic),
            "b" => Ok(NoContextFormat::Bold),
            _ => Ok(NoContextFormat::None)
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum IntermediaryFormatEnum {
    Main(Either<[NoContextFormat; 1], (String, String)>)
}

#[derive(Serialize, Deserialize)]
#[serde(from = "IntermediaryFormatEnum")]
enum FormatType {
    NoContext(NoContextFormat),
    Context(String, String)
}

impl From<IntermediaryFormatEnum> for FormatType {
    fn from(t: IntermediaryFormatEnum) -> Self {
        let IntermediaryFormatEnum::Main(s) = t;
        match s {
            Either::Left(l) => FormatType::NoContext(l[0]),
            Either::Right((l, r)) => FormatType::Context(l, r)
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum IntermediaryFormattingRepresentation {
    Main(Either<Vec<String>, (String, Vec<Vec<String>>)>)
}

#[derive(Serialize, Deserialize)]
#[serde(from = "IntermediaryFormattingRepresentation")]
pub struct FormattedText {
    text: String,
    formatting: Option<Vec<Vec<String>>>
}

impl From<IntermediaryFormattingRepresentation> for FormattedText {
    fn from(text: IntermediaryFormattingRepresentation) -> Self {
        let IntermediaryFormattingRepresentation::Main(s) = text;
        match s {
            Either::Left(r) => {
                FormattedText {
                    text: r[0].clone(),
                    formatting: None
                }
            }
            Either::Right((text, format)) => {
                FormattedText {
                    text,
                    formatting: Some(format)
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R)
}

#[derive(Serialize, Deserialize)]
pub struct BlockType {
    pub role: String,
    pub value: Either<RootBlockType, Value>
}

#[derive(Serialize, Deserialize)]
pub struct NotionUserValueType {
    pub id: String,
    pub version: i64,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub profile_photo: String,
    pub pubonboarding_complete: Option<bool>,
    pub mobile_onboarding_complete: Option<bool>,
    pub clipper_onboarding_complete: Option<bool>
}

#[derive(Serialize, Deserialize)]
pub struct NotionUserType {
    pub role: String,
    pub value: NotionUserValueType
}

#[derive(Serialize, Deserialize)]
pub struct RecordMapType {
    pub block: HashMap<String, BlockType>,
    pub notion_user: HashMap<String, NotionUserType>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadPageChunkData {
    pub record_map: RecordMapType,
}

pub fn parse(input: String) -> Result<LoadPageChunkData> {
    let result: LoadPageChunkData = serde_json::from_str(&input)?;
    Ok(result)
}