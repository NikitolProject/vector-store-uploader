use super::Settings;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct SettingsManager;

impl SettingsManager {
    pub fn get_settings_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "open-source-solutions", "vectore-store-uploader")
            .map(|proj_dirs| proj_dirs.config_dir().join("settings.json"))
    }

    pub async fn load() -> Result<Settings, String> {
        let settings_path = Self::get_settings_path()
            .ok_or_else(|| "Не удалось определить путь для настроек".to_string())?;

        if !settings_path.exists() {
            return Ok(Settings::default());
        }

        let content = fs::read_to_string(&settings_path)
            .map_err(|e| format!("Ошибка чтения файла настроек: {}", e))?;

        serde_json::from_str(&content)
            .map_err(|e| format!("Ошибка парсинга настроек: {}", e))
    }

    pub async fn save(settings: Settings) -> Result<(), String> {
        let settings_path = Self::get_settings_path()
            .ok_or_else(|| "Не удалось определить путь для настроек".to_string())?;

        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Ошибка создания директории настроек: {}", e))?;
        }

        let content = serde_json::to_string_pretty(&settings)
            .map_err(|e| format!("Ошибка сериализации настроек: {}", e))?;

        fs::write(&settings_path, content)
            .map_err(|e| format!("Ошибка сохранения настроек: {}", e))
    }
}
