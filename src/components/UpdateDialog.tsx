import { useState } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import Modal from './Modal';

interface UpdateDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function UpdateDialog({ isOpen, onClose }: UpdateDialogProps) {
  const [checking, setChecking] = useState(false);
  const [updateAvailable, setUpdateAvailable] = useState(false);
  const [currentVersion, setCurrentVersion] = useState('');
  const [newVersion, setNewVersion] = useState('');
  const [releaseNotes, setReleaseNotes] = useState('');
  const [downloading, setDownloading] = useState(false);
  const [downloadProgress, setDownloadProgress] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const checkForUpdates = async () => {
    setChecking(true);
    setError(null);
    setUpdateAvailable(false);

    try {
      const update = await check();

      if (update) {
        setUpdateAvailable(true);
        setCurrentVersion(update.currentVersion);
        setNewVersion(update.version);
        setReleaseNotes(update.body || '暂无更新说明');
      } else {
        setError('当前已是最新版本');
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : '检查更新失败');
      console.error('Update check failed:', err);
    } finally {
      setChecking(false);
    }
  };

  const downloadAndInstall = async () => {
    setDownloading(true);
    setError(null);

    try {
      const update = await check();
      if (!update) {
        setError('未找到更新');
        return;
      }

      await update.downloadAndInstall((event) => {
        switch (event.event) {
          case 'Started':
            setDownloadProgress(0);
            break;
          case 'Progress':
            setDownloadProgress(event.data.chunkLength);
            break;
          case 'Finished':
            setDownloadProgress(100);
            break;
        }
      });

      await relaunch();
    } catch (err) {
      setError(err instanceof Error ? err.message : '下载更新失败');
      console.error('Update download failed:', err);
    } finally {
      setDownloading(false);
    }
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="检查更新">
      <div className="space-y-4">
        {/* 检查更新按钮 */}
        {!updateAvailable && !checking && (
          <button
            onClick={checkForUpdates}
            className="w-full px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            检查更新
          </button>
        )}

        {/* 检查中状态 */}
        {checking && (
          <div className="text-center py-4">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
            <p className="mt-2 text-gray-600">正在检查更新...</p>
          </div>
        )}

        {/* 错误信息 */}
        {error && (
          <div className="p-4 bg-red-50 border border-red-200 rounded-lg">
            <p className="text-red-600">{error}</p>
          </div>
        )}

        {/* 更新可用 */}
        {updateAvailable && !downloading && (
          <div className="space-y-4">
            <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
              <h3 className="font-semibold text-green-800 mb-2">发现新版本</h3>
              <div className="text-sm text-gray-600 space-y-1">
                <p>当前版本: {currentVersion}</p>
                <p>最新版本: {newVersion}</p>
              </div>
            </div>

            <div className="p-4 bg-gray-50 rounded-lg">
              <h4 className="font-medium text-gray-800 mb-2">更新说明</h4>
              <div className="text-sm text-gray-600 whitespace-pre-wrap">
                {releaseNotes}
              </div>
            </div>

            <button
              onClick={downloadAndInstall}
              className="w-full px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700"
            >
              下载并安装更新
            </button>
          </div>
        )}

        {/* 下载中状态 */}
        {downloading && (
          <div className="space-y-4">
            <div className="text-center py-4">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-green-600 mx-auto"></div>
              <p className="mt-2 text-gray-600">正在下载更新...</p>
              {downloadProgress > 0 && (
                <p className="text-sm text-gray-500 mt-1">
                  已下载: {(downloadProgress / 1024 / 1024).toFixed(2)} MB
                </p>
              )}
            </div>
          </div>
        )}
      </div>
    </Modal>
  );
}
