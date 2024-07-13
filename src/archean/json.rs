use std::io::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blueprint {
    pub author: String,
    #[serde(rename = "box_max")]
    pub box_max: BoxMax,
    #[serde(rename = "box_min")]
    pub box_min: BoxMin,
    #[serde(rename = "box_size")]
    pub box_size: BoxSize,
    pub data: RootData,
    pub datetime: String,
    pub mass: f64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxMax {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxMin {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxSize {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootData {
    pub blocks: Vec<Block>,
    pub components: Vec<Component>,
    #[serde(rename = "composite_builds")]
    pub composite_builds: Vec<Value>,
    pub doors: Vec<Value>,
    pub frames: Vec<Value>,
    pub labels: Vec<Label>,
    pub pipes: Vec<Pipe>,
    #[serde(rename = "symmetry_axis")]
    pub symmetry_axis: i64,
    pub version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub colors: Vec<i64>,
    pub extra: i64,
    #[serde(rename = "frame_x")]
    pub frame_x: i64,
    #[serde(rename = "frame_y")]
    pub frame_y: i64,
    #[serde(rename = "frame_z")]
    pub frame_z: i64,
    pub material: i64,
    #[serde(rename = "pos_x")]
    pub pos_x: i64,
    #[serde(rename = "pos_y")]
    pub pos_y: i64,
    #[serde(rename = "pos_z")]
    pub pos_z: i64,
    #[serde(rename = "size_x")]
    pub size_x: i64,
    #[serde(rename = "size_y")]
    pub size_y: i64,
    #[serde(rename = "size_z")]
    pub size_z: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Component {
    pub alias: String,
    pub colors: Vec<i64>,
    pub data: ComponentData,
    pub module: String,
    pub occupancies: Vec<Occupancy>,
    pub orientation: Orientation,
    pub position: Position,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Debug, Clone)]
struct JsonError;

impl Component {
    pub fn with_hdd(&self) -> Result<Hdd, JsonError> {
        if let Some(hdd) = &self.data.hdd {
            Ok(hdd.clone())
        } else {
            Err(JsonError)
        }
    }
    fn has_data_storage(&self) -> bool {
        self.data.has_hdd()
    }
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentData {
    pub angle: Option<f64>,
    pub max_power: Option<f64>,
    pub rgb: Option<Rgb>,
    pub hdd: Option<Hdd>,
    pub program: Option<String>,
    pub version: Option<i64>,
    pub charged: Option<bool>,
    pub contents: Option<Contents>,
    pub state: Option<bool>,
    pub receive_frequency: Option<String>,
    pub transmit_data: Option<String>,
    pub transmit_frequency: Option<String>,
}

impl ComponentData {
    fn has_hdd(&self) -> bool {
        self.hdd.is_some()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rgb {
    pub b: f64,
    pub g: f64,
    pub r: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hdd {
    pub capacity: i64,
    pub label: String,
    #[serde(rename = "xc_files")]
    pub xc_files: Vec<XcFile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XcFile {
    pub name: String,
    #[serde(rename = "plain_code")]
    pub plain_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Occupancy {
    #[serde(rename = "frame_x")]
    pub frame_x: i64,
    #[serde(rename = "frame_y")]
    pub frame_y: i64,
    #[serde(rename = "frame_z")]
    pub frame_z: i64,
    #[serde(rename = "pos_x")]
    pub pos_x: i64,
    #[serde(rename = "pos_y")]
    pub pos_y: i64,
    #[serde(rename = "pos_z")]
    pub pos_z: i64,
    #[serde(rename = "size_x")]
    pub size_x: i64,
    #[serde(rename = "size_y")]
    pub size_y: i64,
    #[serde(rename = "size_z")]
    pub size_z: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orientation {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    #[serde(rename = "align_center")]
    pub align_center: i64,
    #[serde(rename = "dir_x")]
    pub dir_x: i64,
    #[serde(rename = "dir_y")]
    pub dir_y: i64,
    #[serde(rename = "dir_z")]
    pub dir_z: i64,
    pub metallic: i64,
    #[serde(rename = "panel_color")]
    pub panel_color: PanelColor,
    pub position: Position2,
    pub roughness: i64,
    pub size: f64,
    pub text: String,
    #[serde(rename = "text_color")]
    pub text_color: TextColor,
    #[serde(rename = "up_x")]
    pub up_x: i64,
    #[serde(rename = "up_y")]
    pub up_y: i64,
    #[serde(rename = "up_z")]
    pub up_z: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanelColor {
    pub a: i64,
    pub b: i64,
    pub g: i64,
    pub r: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position2 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextColor {
    pub b: i64,
    pub g: i64,
    pub r: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pipe {
    #[serde(rename = "a_component")]
    pub a_component: i64,
    #[serde(rename = "a_port")]
    pub a_port: String,
    #[serde(rename = "b_component")]
    pub b_component: i64,
    #[serde(rename = "b_port")]
    pub b_port: String,
    pub radius: f64,
    pub segments: Vec<Segment>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    pub a: i64,
    pub b: i64,
    #[serde(rename = "box")]
    pub box_field: bool,
    pub chrome: bool,
    pub dir: i64,
    pub flexible: bool,
    pub g: i64,
    pub glossy: bool,
    pub length: f64,
    pub metal: bool,
    pub r: i64,
    #[serde(rename = "rounded_caps")]
    pub rounded_caps: bool,
    pub start: Start,
    pub striped: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Start {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
