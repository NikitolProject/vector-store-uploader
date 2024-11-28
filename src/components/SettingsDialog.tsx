import { useState, useEffect } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  Stack
} from '@mui/material';
import { Settings } from '../types/settings';
import { loadSettings } from '../utils/store';

interface SettingsDialogProps {
  open: boolean;
  settings: Settings;
  onClose: () => void;
  onSave: (settings: Settings) => Promise<void>;
}

export default function SettingsDialog({ open, settings, onClose, onSave }: SettingsDialogProps) {
  const [settingsState, setSettingsState] = useState<Settings>(settings);

  useEffect(() => {
    if (open) {
      loadSettings().then(setSettingsState);
    }
  }, [open]);

  const handleSave = () => {
    onSave(settingsState);
  };

  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>Настройки</DialogTitle>
      <DialogContent>
        <Stack spacing={2} sx={{ mt: 1 }}>
          <TextField
            label="OpenAI API Key"
            fullWidth
            value={settingsState.openai_api_key}
            onChange={(e) => setSettingsState({ ...settingsState, openai_api_key: e.target.value })}
          />
          <TextField
            label="Pinecone API Key"
            fullWidth
            value={settingsState.pinecone_api_key}
            onChange={(e) => setSettingsState({ ...settingsState, pinecone_api_key: e.target.value })}
          />
          <TextField
            label="Pinecone Index Host"
            fullWidth
            value={settingsState.pinecone_index_host}
            onChange={(e) => setSettingsState({ ...settingsState, pinecone_index_host: e.target.value })}
          />
          <TextField
            label="Pinecone Namespace"
            fullWidth
            value={settingsState.pinecone_namespace}
            onChange={(e) => setSettingsState({ ...settingsState, pinecone_namespace: e.target.value })}
          />
        </Stack>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Отмена</Button>
        <Button onClick={handleSave} variant="contained">Сохранить</Button>
      </DialogActions>
    </Dialog>
  );
}
