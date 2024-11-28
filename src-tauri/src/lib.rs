mod embeddings;
mod storage;
mod settings;
mod pdf;

use pdf::{DefaultPdfProcessor, PdfProcessor};
use settings::{Settings, SettingsManager};
use std::path::Path;
use tauri::generate_handler;

#[tauri::command]
async fn process_pdf(file_path: String) -> Result<String, String> {
    println!("Начало обработки PDF файла: {}", file_path);
    
    // Загружаем настройки
    let settings = SettingsManager::load().await?;
    
    // Создаем процессор с настройками
    let mut processor = DefaultPdfProcessor::new(&settings).await?;
    
    // Обрабатываем файл
    processor.process_file(Path::new(&file_path)).await
}

#[tauri::command]
async fn load_settings() -> Result<Settings, String> {
    SettingsManager::load().await
}

#[tauri::command]
async fn save_settings(settings: Settings) -> Result<(), String> {
    SettingsManager::save(settings).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Запуск приложения Tauri");
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(generate_handler![
            process_pdf,
            load_settings,
            save_settings
        ])
        .run(tauri::generate_context!())
        .expect("ошибка при запуске приложения tauri");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;

    #[tokio::test]
    async fn test_settings_load_save() {
        let test_settings = Settings {
            openai_api_key: "test_key".to_string(),
            pinecone_api_key: "test_key".to_string(),
            pinecone_index_host: "test_host".to_string(),
            pinecone_namespace: "test_namespace".to_string(),
        };

        // Тестируем сохранение
        assert!(save_settings(test_settings.clone()).await.is_ok());

        // Тестируем загрузку
        let loaded = load_settings().await.unwrap();
        assert_eq!(loaded.openai_api_key, test_settings.openai_api_key);
        assert_eq!(loaded.pinecone_api_key, test_settings.pinecone_api_key);
        assert_eq!(loaded.pinecone_index_host, test_settings.pinecone_index_host);
        assert_eq!(loaded.pinecone_namespace, test_settings.pinecone_namespace);
    }
}
