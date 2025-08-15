use serde::{Serialize, Deserialize};
use serde::de::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CsvRecord {
    pub name: String,
    pub notes: String,

    #[serde(rename = "URL")]
    pub url: String,
    pub level: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,

    #[serde(deserialize_with = "deserialize_platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub platform: Vec<String>,

    #[serde(deserialize_with = "deserialize_platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<String>,

    #[serde(rename = "OS")]
    #[serde(deserialize_with = "deserialize_platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<String>,

    #[serde(deserialize_with = "deserialize_platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub language: Vec<String>,

    #[serde(deserialize_with = "deserialize_platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub category: Vec<String>,
}

/// 把字串轉換成平台列表
/// - 例如: "Windows, Linux, macOS" 會轉換成 ["Windows", "Linux", "macOS"]
/// # 參數
/// - `deserializer`: 用於反序列化的 Deserializer
/// # 返回
/// - `Result<Vec<String>, D::Error>`: 成功時返回平台列表，失敗時返回錯誤
fn deserialize_platform<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error> where D: Deserializer<'de> {
    let str = String::deserialize(deserializer)?;
    Ok(str.split(',').map(|str| str.trim().to_string()).collect())
}
