use serde::{Deserialize, Deserializer};
#[derive(Deserialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(f64),
    String(String),
}
impl ToString for NumberOrString {
    fn to_string(&self) -> String {
        match self {
            Self::Number(v) => v.to_string(),
            Self::String(v) => v.clone(),
        }
    }
}
pub fn deserialize_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    NumberOrString::deserialize(deserializer).map(|v| v.to_string())
}
