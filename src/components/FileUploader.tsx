import { useState, useEffect, useCallback } from 'react';
import { 
  Box,
  Button,
  Typography,
  LinearProgress,
  Paper,
  IconButton,
  Fade,
  CircularProgress
} from '@mui/material';
import CloudUploadIcon from '@mui/icons-material/CloudUpload';
import SettingsIcon from '@mui/icons-material/Settings';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { Settings, DEFAULT_SETTINGS } from '../types/settings';
import SettingsDialog from './SettingsDialog';
import { loadSettings, saveSettings } from '../utils/store';

export default function FileUploader() {
  const [file, setFile] = useState<string | null>(null);
  const [uploading, setUploading] = useState(false);
  const [settings, setSettings] = useState<Settings>(DEFAULT_SETTINGS);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [dragActive, setDragActive] = useState(false);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    loadSettings().then(setSettings);
  }, []);

  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }
  }, []);

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      const file = e.dataTransfer.files[0];
      if (file.type === 'application/pdf') {
        setFile(URL.createObjectURL(file));
      } else {
        alert('Пожалуйста, загрузите PDF файл');
      }
    }
  }, []);

  const handleFileSelect = async () => {
    try {
      const selected = await open({
        multiple: false,
        filters: [{
          name: 'PDF',
          extensions: ['pdf']
        }]
      });
      
      if (selected) {
        setFile(selected as string);
      }
    } catch (error) {
      console.error('Ошибка при выборе файла:', error);
      alert('Произошла ошибка при выборе файла');
    }
  };

  const handleUpload = async () => {
    if (!file) return;
    
    setUploading(true);
    setProgress(0);
    
    try {
      // Имитация прогресса загрузки
      const progressInterval = setInterval(() => {
        setProgress(prev => Math.min(prev + 5, 95));
      }, 500);

      const extractedText = await invoke('process_pdf', { filePath: file });
      
      clearInterval(progressInterval);
      setProgress(100);
      
      console.log('Текст извлечен:', extractedText);
      
      // Показываем 100% прогресс на короткое время
      setTimeout(() => {
        setUploading(false);
        setProgress(0);
        setFile(null);
      }, 1000);
      
    } catch (error) {
      console.error('Ошибка загрузки:', error);
      alert('Ошибка при обработке файла');
      setUploading(false);
      setProgress(0);
    }
  };

  return (
    <Paper 
      elevation={3} 
      sx={{ 
        p: 4, 
        maxWidth: 600, 
        mx: 'auto', 
        mt: 4,
        position: 'relative',
        overflow: 'hidden'
      }}
    >
      <Box sx={{ position: 'absolute', right: 16, top: 16 }}>
        <IconButton onClick={() => setSettingsOpen(true)}>
          <SettingsIcon />
        </IconButton>
      </Box>

      <Box
        onDragEnter={handleDrag}
        onDragLeave={handleDrag}
        onDragOver={handleDrag}
        onDrop={handleDrop}
        sx={{
          border: '2px dashed',
          borderColor: dragActive ? 'primary.main' : 'grey.300',
          borderRadius: 2,
          p: 4,
          textAlign: 'center',
          transition: 'all 0.3s ease',
          backgroundColor: dragActive ? 'action.hover' : 'background.paper',
          cursor: 'pointer',
          '&:hover': {
            borderColor: 'primary.main',
            backgroundColor: 'action.hover'
          }
        }}
        onClick={handleFileSelect}
      >
        <CloudUploadIcon sx={{ fontSize: 48, color: 'primary.main', mb: 2 }} />
        <Typography variant="h6" gutterBottom>
          Перетащите PDF файл сюда
        </Typography>
        <Typography variant="body2" color="textSecondary">
          или нажмите для выбора
        </Typography>
      </Box>

      <Fade in={!!file}>
        <Box sx={{ mt: 3, textAlign: 'center' }}>
          <Typography variant="body1" sx={{ mb: 2 }}>
            Выбран файл: {file}
          </Typography>
          <Button
            variant="contained"
            color="primary"
            onClick={handleUpload}
            disabled={uploading}
            startIcon={uploading ? <CircularProgress size={20} /> : null}
          >
            {uploading ? 'Обработка...' : 'Обработать файл'}
          </Button>
        </Box>
      </Fade>

      <Fade in={uploading}>
        <Box sx={{ mt: 3 }}>
          <LinearProgress variant="determinate" value={progress} />
          <Typography variant="body2" color="textSecondary" align="center" sx={{ mt: 1 }}>
            {progress}%
          </Typography>
        </Box>
      </Fade>

      <SettingsDialog
        open={settingsOpen}
        settings={settings}
        onClose={() => setSettingsOpen(false)}
        onSave={saveSettings}
      />
    </Paper>
  );
} 