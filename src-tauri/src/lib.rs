mod library;

use std::fs::{read_to_string};
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
use log::{debug, info};

use library::utils::{read_csv_file, read_type_set, folder_files, logger_setting};

/// 讀取 CSV 檔案並返回記錄
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `filename`: CSV 檔案的名稱
/// ## 返回
/// - `String`: 成功時返回記錄的 JSON 字符串，失敗
#[tauri::command]
fn read_csv(app: AppHandle, filename: String) -> String {

    info!("Loading CSV file: {}", filename);
    debug!("Loading CSV file: {}", filename);

    let records  = match read_csv_file(app.clone(), filename) {
        Ok(records) => records,
        Err(error) => return serde_json::json!({ "error": error.to_string() }).to_string(),
    };

    serde_json::json!({ "result": records }).to_string()
}

/// 取得總Type的數值 => HashSet
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `filename`: CSV 檔案的名稱
/// ## 返回
/// - `String`: 成功時返回記錄的 JSON 字符串，失敗
#[tauri::command]
fn read_type(app: AppHandle, filename: String) -> String {

    let types  = match read_type_set(app.clone(), filename) {
        Ok(types) => types,
        Err(error) => return serde_json::json!({ "error": error.to_string() }).to_string(),
    };

    serde_json::json!({ "result": types }).to_string()
}

/// 讀取 CSV 檔案資料夾檔名列表
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// ## 返回
/// - `String`: 成功時返回記錄的 JSON 字符串，失敗
#[tauri::command]
fn csv_list(app: AppHandle) -> String {
    
    let list = match app.path().resolve("document", BaseDirectory::Resource) {
        Ok(path) => match folder_files(path) {
            Ok(array) => array,
            Err(error) => return serde_json::json!({ "error": error.to_string() }).to_string(),
        },
        Err(error) => return serde_json::json!({ "error": error.to_string() }).to_string(),
    };

    serde_json::json!({ "result": list }).to_string()
}

/// 讀取JSON檔案資料夾檔名列表
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// ## 返回
/// - `String`: 成功時返回記錄的 JSON 字符串，失敗
#[tauri::command]
fn read_json_file(app: AppHandle, filename: String) -> Result<String, String> {
    let file_path = app.path()
        .resolve("config", BaseDirectory::Resource)
        .map_err(|e| format!("無法獲取資源目錄: {}", e))?
        .join(filename);
    
    read_to_string(&file_path)
        .map_err(|e| format!("無法讀取檔案: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if let Err(error) = logger_setting(app) { eprintln!("Failed to setup logging: {}", error); }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_csv, csv_list, read_type, read_json_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
