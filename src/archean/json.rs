use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

impl Blueprint {
    pub fn components_with_hdd(&self) -> Vec<Component> {
        self.data.components.iter().filter(|c| c.has_data_storage()).cloned().collect()
    }
}

impl Component {
    pub fn name(&self) -> String {
        if self.alias.is_none() {
            self.module.to_string()
        } else {
            let has_alias = !self.alias.clone().unwrap().as_str().is_empty();
            if has_alias {
                format!("{}", self.alias.clone().unwrap())
            } else {
                self.module.to_string()
            }
        }
    }

    pub fn xc_files(&self) -> Vec<XcFileMeta> {
        if let Some(hdd) = &self.data.hdd {
            hdd.xc_files().iter().map(|f| XcFileMeta::new(self.name().to_string(), f.clone())).collect()
        } else {
            vec![]
        }
    }

    fn has_data_storage(&self) -> bool {
        self.data.has_hdd()
    }
}

impl ComponentData {
    fn has_hdd(&self) -> bool {
        self.hdd.is_some()
    }
}

impl Hdd {
    pub fn xc_files(&self) -> Vec<XcFile> {
        self.xc_files.clone()
    }
}

#[derive(Debug, Clone)]
pub struct XcFileMeta {
    component_name: String,
    inner: XcFile,
}

impl XcFileMeta {
    pub fn new(component_name: String, inner: XcFile) -> Self {
        XcFileMeta {
            component_name,
            inner,
        }
    }

    pub fn component(&self) -> &str {
        &self.component_name
    }

    pub fn file_name(&self) -> &str {
        &self.inner.name
    }

    pub fn file_content(&self) -> &str {
        &self.inner.plain_code
    }
}

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
    pub composite_builds: Vec<CompositeBuild>,
    pub doors: Vec<Value>,
    pub frames: Vec<Value>,
    pub labels: Vec<Value>,
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
    pub alias: Option<String>,
    pub colors: Vec<i64>,
    pub data: ComponentData,
    pub module: String,
    pub occupancies: Vec<Occupancy>,
    pub orientation: Orientation,
    pub position: Position,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComponentData {
    pub color: Option<Color>,
    #[serde(default)]
    pub elements: Vec<Element>,
    pub metallic: Option<i64>,
    pub roughness: Option<i64>,
    #[serde(rename = "size_x")]
    pub size_x: Option<i64>,
    #[serde(rename = "size_y")]
    pub size_y: Option<i64>,
    pub version: Option<i64>,
    pub charged: Option<bool>,
    pub angle: Option<f64>,
    pub max_power: Option<f64>,
    pub rgb: Option<Rgb>,
    pub smooth_controls: Option<bool>,
    pub grip: Option<f64>,
    pub mudguard: Option<bool>,
    pub suspension: Option<f64>,
    pub hdd: Option<Hdd>,
    pub program: Option<String>,
    pub blocks: Option<Vec<Block>>,
    pub components: Option<Vec<Component>>,
    #[serde(rename = "composite_builds")]
    #[serde(default)]
    pub composite_builds: Vec<Value>,
    #[serde(default)]
    pub doors: Vec<Value>,
    #[serde(default)]
    pub frames: Vec<Value>,
    #[serde(default)]
    pub labels: Vec<Value>,
    #[serde(default)]
    pub pipes: Vec<Value>,
    #[serde(rename = "symmetry_axis")]
    pub symmetry_axis: Option<i64>,
    pub acceleration: Option<f64>,
    pub input_value: Option<f64>,
    pub max_speed: Option<f64>,
    pub servo_mode: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    pub a: i64,
    pub b: i64,
    pub g: i64,
    pub r: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    pub base_color: BaseColor,
    pub base_metallic: i64,
    pub base_roughness: i64,
    pub main_color: MainColor,
    pub main_metallic: i64,
    pub main_roughness: i64,
    #[serde(rename = "pos_x")]
    pub pos_x: i64,
    #[serde(rename = "pos_y")]
    pub pos_y: i64,
    #[serde(rename = "size_x")]
    pub size_x: i64,
    #[serde(rename = "size_y")]
    pub size_y: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseColor {
    pub a: i64,
    pub b: i64,
    pub g: i64,
    pub r: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MainColor {
    pub a: i64,
    pub b: i64,
    pub g: i64,
    pub r: i64,
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
pub struct CompositeBuild {
    pub component: i64,
    pub slave_build_id: i64,
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