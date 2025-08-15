use std::fmt::Debug;
use std::fs::{read_dir, File, OpenOptions, create_dir_all};
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::collections::HashSet;

use csv::Reader;
use serde::de::{DeserializeOwned};
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use env_logger::Env;
use env_logger::{fmt::Color, Builder};
use chrono::Local;
use colored::Colorize;

use crate::library::models::CsvRecord;
use crate::ww_print;

/// 從 CSV 檔案讀取記錄
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `filename`: CSV 檔案的名稱
/// ## 返回
/// - `Result<Vec<CsvRecord>, Error>`: 成功時返回記錄的向量，失敗時返回錯誤
pub fn read_csv_file(app: AppHandle, filename: String) -> Result<Vec<CsvRecord>, Error> {
    let resource_path = _csv_file_path(&app, filename)?;
    let records: Vec<CsvRecord> = _parse_csv_file(resource_path.to_string_lossy().to_string())?;

    Ok(records)
}

/// 取得總Type的數值 => HashSet
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `filename`: CSV 檔案的名稱
/// ## 返回
/// - `Result<HashSet<String>, Error>`: 成功時返回記錄的向量，失敗時返回錯誤
pub fn read_type_set(app: AppHandle, filename: String) -> Result<HashSet<String>, Error> {

    let records = match read_csv_file(app, filename) {
        Ok(records) => records,
        Err(error) => return Err(error)
    };

    let mut type_set: HashSet<String> = HashSet::new();
    for record in records.iter() {
        for r#type in record.r#type.clone() { type_set.insert(r#type.clone()); }
    }

    return Ok(type_set);
}

/// 取得路徑資料夾內的檔案名稱列表 (排序)
/// ## 參數
/// - `path`: 資料夾完整路徑
/// ## 返回
/// - `Result<Vec<String>, Error>`: 檔案名稱列表
pub fn folder_files(path: PathBuf) -> Result<Vec<String>, Error> {
    
    let mut file_names = Vec::new();
    
    match read_dir(&path) {
        Err(error) => Err(error),
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(name) = entry.file_name().to_str() {
                        file_names.push(name.to_string());
                    }
                }
            }

            file_names.sort_by(|name1, name2| name1.to_lowercase().cmp(&name2.to_lowercase()));
            Ok(file_names)
        }
    }
}

/// 初始化日誌系統 (會在應用程序資源目錄中創建 logs 目錄)
/// ## 參數
/// - `app`: Tauri 應用程式的 handle
pub fn logger_setting(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {

    let log_dir = app.path().resolve("logs", BaseDirectory::Resource)?;
    create_dir_all(&log_dir)?;

    let log_file_name = format!("{}.log", Local::now().format("%Y%m%d"));
    let log_file_path = log_dir.join(log_file_name);

    ww_print!(format!("Log file location: {:?}", log_file_path));

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    Builder::from_env(Env::default().default_filter_or("debug"))
        .format(|buffer, record| {
            let mut style = buffer.style();
            let level_color = match record.level() {
                log::Level::Error => Color::Red,
                log::Level::Warn => Color::Yellow,
                log::Level::Info => Color::Green,
                log::Level::Debug => Color::Blue,
                log::Level::Trace => Color::Cyan,
            };

            writeln!(
                buffer,
                "{} [{}] {} - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                style.set_color(level_color).value(record.level()),
                record.target(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(log_file)))
        .init();

    Ok(())
}

/// 取得 CSV 檔案的完整路徑
/// ## 參數
/// - `app`: Tauri 應用的 AppHandle
/// - `filename`: CSV 檔案的名稱
/// ## 返回
/// - `Result<PathBuf, Error>`: 成功時返回檔案的完整路徑，失敗時返回錯誤
fn _csv_file_path(app: &AppHandle, filename: String) -> Result<PathBuf, Error> {
    if filename.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Filename cannot be empty",
        ));
    }

    let resource_path = match app.path().resolve("document", BaseDirectory::Resource) {
        Ok(path) => path,
        Err(error) => return Err(Error::new(ErrorKind::NotFound, error.to_string())),
    };

    Ok(resource_path.as_path().join(filename))
}

/// 解析 CSV 檔案並返回記錄
/// ## 參數
/// - `resource_path`: CSV 檔案的完整路徑
/// ## 返回
/// - `Result<Vec<T>, Error>`: 成功時返回記錄的向量
fn _parse_csv_file<T>(resource_path: String) -> Result<Vec<T>, Error> where T: DeserializeOwned + Debug {
    if resource_path.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Resource path cannot be empty",
        ));
    }

    let mut records: Vec<T> = Vec::new();
    let opened_file = File::open(&resource_path)?;
    let mut reader = Reader::from_reader(opened_file);

    for result in reader.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(error) => return Err(Error::new(ErrorKind::InvalidData, error.to_string())),
        }
    }

    Ok(records)
}

