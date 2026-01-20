import { useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

interface UpdateInfo {
  version: string;
  date: string;
  body: string;
}

export function UpdateChecker() {
  const [checking, setChecking] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [downloading, setDownloading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);

    try {
      const update = await check();

      if (update) {
        setUpdateAvailable(true);
        setUpdateInfo({
          version: update.version,
          date: update.date || '',
          body: update.body || '暂无更新说明',
        });
      } else {
        setUpdateAvailable(false);
        setUpdateInfo(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : '检查更新失败');
      console.error('Update check failed:', err);
    } finally {
      setChecking(false);
    }
  };

  return { checking, updateAvailable, updateInfo, downloading, error, checkForUpdates };
}
