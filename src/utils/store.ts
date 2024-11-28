import { invoke } from '@tauri-apps/api/core';
import { Settings, DEFAULT_SETTINGS } from '../types/settings';

export async function loadSettings(): Promise<Settings> {
  try {
    const settings = await invoke<Settings>('load_settings');
    return settings;
  } catch (error) {
    console.error('Ошибка загрузки настроек:', error);
    return DEFAULT_SETTINGS;
  }
}

export async function saveSettings(settings: Settings): Promise<void> {
  try {
    await invoke('save_settings', { settings });
  } catch (error) {
    console.error('Ошибка сохранения настроек:', error);
    throw error;
  }
}
