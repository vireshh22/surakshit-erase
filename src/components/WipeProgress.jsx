// import { useState, useEffect } from 'react';

// const WipeProgress = ({ method, onComplete }) => {
//   const [progress, setProgress] = useState(0);
//   const [status, setStatus] = useState('Starting wipe...');

//   useEffect(() => {
//     // Simulate progress with setTimeout
//     if (progress < 100) {
//       const timeoutId = setTimeout(() => {
//         const increment = Math.floor(Math.random() * 10) + 5;
//         const newProgress = Math.min(progress + increment, 100);
        
//         if (newProgress < 30) setStatus('Erasing visible files...');
//         else if (newProgress < 70) setStatus('Purging hidden HPA area...');
//         else setStatus('Clearing DCO partition...');
        
//         setProgress(newProgress);
//       }, 500); // Update every 500ms

//       return () => clearTimeout(timeoutId);
//     } else {
//       setStatus('Wipe complete!');
//       // Notify parent component after a short delay
//       setTimeout(onComplete, 1000);
//     }
//   }, [progress, onComplete]);

//   return (
//     <div className="wipe-progress-container">
//       <h3>Wiping Drive using '{method}' Method</h3>
//       <div className="progress-bar-container">
//         <div className="progress-bar" style={{ width: `${progress}%` }}>
//           {progress}%
//         </div>
//       </div>
//       <p>{status}</p>
//     </div>
//   );
// };

// export default WipeProgress;


import React, { useState, useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { 
    Play, 
    Pause, 
    Square, 
    FileX, 
    FolderX, 
    AlertCircle,
    CheckCircle,
    HardDrive,
    Clock,
    Database
} from 'lucide-react';

const WipeProgress = ({ method, selectedDrive, onComplete, onCancel }) => {
    const [isRunning, setIsRunning] = useState(false);
    const [isPaused, setIsPaused] = useState(false);
    const [progress, setProgress] = useState({
        current_file: '',
        current_path: '',
        file_size: null,
        files_processed: 0,
        total_size_processed: 0,
        percentage: 0,
        stage: 'Initializing...',
        is_complete: false
    });
    const [startTime, setStartTime] = useState(null);
    const [elapsedTime, setElapsedTime] = useState(0);
    const [recentFiles, setRecentFiles] = useState([]);
    const [error, setError] = useState(null);
    const [isComplete, setIsComplete] = useState(false);
    const [wipeResult, setWipeResult] = useState(null);
    
    const progressListener = useRef(null);
    const completeListener = useRef(null);
    const timerRef = useRef(null);

    useEffect(() => {
        startWipeProcess();
        
        return () => {
            cleanup();
        };
    }, []);

    useEffect(() => {
        if (isRunning && !isPaused) {
            timerRef.current = setInterval(() => {
                setElapsedTime(Date.now() - startTime);
            }, 1000);
        } else {
            clearInterval(timerRef.current);
        }

        return () => clearInterval(timerRef.current);
    }, [isRunning, isPaused, startTime]);

    const startWipeProcess = async () => {
        try {
            setIsRunning(true);
            setStartTime(Date.now());
            setProgress(prev => ({ ...prev, stage: 'Starting wipe operation...' }));

            // Listen for progress updates
            progressListener.current = await listen('wipe-progress', (event) => {
                const progressData = event.payload;
                setProgress(progressData);
                
                // Add to recent files list
                if (progressData.current_file && progressData.current_file !== '') {
                    setRecentFiles(prev => {
                        const newFile = {
                            name: progressData.current_file,
                            path: progressData.current_path,
                            size: progressData.file_size,
                            timestamp: Date.now(),
                            type: progressData.stage.includes('directory') ? 'directory' : 'file'
                        };
                        
                        // Keep only the last 10 files
                        const updated = [newFile, ...prev.slice(0, 9)];
                        return updated;
                    });
                }
            });

            // Listen for completion
            completeListener.current = await listen('wipe-complete', async () => {
                setIsRunning(false);
                setIsComplete(true);
                setProgress(prev => ({ ...prev, stage: 'Wipe operation completed!', is_complete: true }));
            });

            // Start the actual wipe operation
            const result = await invoke('start_wipe_operation', {
                drivePath: selectedDrive.path,
                method: method
            });

            setWipeResult(result);
            onComplete(result);

        } catch (err) {
            console.error('Wipe operation failed:', err);
            setError(err.toString());
            setIsRunning(false);
        }
    };

    const handleCancel = async () => {
        try {
            await invoke('cancel_wipe_operation');
            setIsRunning(false);
            setProgress(prev => ({ ...prev, stage: 'Cancelling operation...' }));
            
            setTimeout(() => {
                onCancel?.();
            }, 1000);
        } catch (err) {
            console.error('Failed to cancel wipe operation:', err);
            setError('Failed to cancel operation: ' + err.toString());
        }
    };

    const cleanup = () => {
        if (progressListener.current) {
            progressListener.current();
        }
        if (completeListener.current) {
            completeListener.current();
        }
        clearInterval(timerRef.current);
    };

    const formatTime = (milliseconds) => {
        const seconds = Math.floor(milliseconds / 1000);
        const minutes = Math.floor(seconds / 60);
        const hours = Math.floor(minutes / 60);

        if (hours > 0) {
            return `${hours}h ${minutes % 60}m ${seconds % 60}s`;
        } else if (minutes > 0) {
            return `${minutes}m ${seconds % 60}s`;
        } else {
            return `${seconds}s`;
        }
    };

    const formatBytes = (bytes) => {
        if (!bytes || bytes === 0) return '0 B';
        const k = 1024;
        const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    };

    const calculateSpeed = () => {
        if (!elapsedTime || elapsedTime === 0) return '0 B/s';
        const bytesPerSecond = (progress.total_size_processed * 1000) / elapsedTime;
        return formatBytes(bytesPerSecond) + '/s';
    };

    if (error) {
        return (
            <div className="wipe-progress-container error">
                <div className="error-header">
                    <AlertCircle size={32} className="error-icon" />
                    <h3>Wipe Operation Failed</h3>
                </div>
                <div className="error-message">
                    <p>{error}</p>
                </div>
                <div className="error-actions">
                    <button onClick={onCancel} className="cancel-button">
                        Close
                    </button>
                </div>
            </div>
        );
    }

    return (
        <div className="wipe-progress-container">
            <div className="progress-header">
                <div className="header-info">
                    <HardDrive size={24} className="drive-icon" />
                    <div>
                        <h3>Wiping Drive: {selectedDrive?.name}</h3>
                        <p>Method: <strong>{method}</strong> | Device: <strong>{selectedDrive?.path}</strong></p>
                    </div>
                </div>
                <div className="progress-status">
                    {isComplete ? (
                        <div className="status-complete">
                            <CheckCircle size={20} />
                            <span>Complete</span>
                        </div>
                    ) : isRunning ? (
                        <div className="status-running">
                            <div className="spinner" />
                            <span>Running</span>
                        </div>
                    ) : (
                        <div className="status-stopped">
                            <Square size={20} />
                            <span>Stopped</span>
                        </div>
                    )}
                </div>
            </div>

            <div className="progress-stats">
                <div className="stat-item">
                    <Clock size={16} />
                    <div>
                        <span className="stat-label">Elapsed Time</span>
                        <span className="stat-value">{formatTime(elapsedTime)}</span>
                    </div>
                </div>
                <div className="stat-item">
                    <FileX size={16} />
                    <div>
                        <span className="stat-label">Files Processed</span>
                        <span className="stat-value">{progress.files_processed.toLocaleString()}</span>
                    </div>
                </div>
                <div className="stat-item">
                    <Database size={16} />
                    <div>
                        <span className="stat-label">Data Processed</span>
                        <span className="stat-value">{formatBytes(progress.total_size_processed)}</span>
                    </div>
                </div>
                <div className="stat-item">
                    <div className="speed-indicator" />
                    <div>
                        <span className="stat-label">Processing Speed</span>
                        <span className="stat-value">{calculateSpeed()}</span>
                    </div>
                </div>
            </div>

            <div className="current-operation">
                <div className="operation-header">
                    <h4>Current Operation</h4>
                    <span className="operation-stage">{progress.stage}</span>
                </div>
                
                {progress.current_file && (
                    <div className="current-file">
                        <div className="file-info">
                            {progress.stage.includes('directory') ? (
                                <FolderX size={16} className="file-icon folder" />
                            ) : (
                                <FileX size={16} className="file-icon file" />
                            )}
                            <div className="file-details">
                                <div className="file-name" title={progress.current_file}>
                                    {progress.current_file}
                                </div>
                                <div className="file-path" title={progress.current_path}>
                                    {progress.current_path}
                                </div>
                            </div>
                            {progress.file_size && (
                                <div className="file-size">
                                    {progress.file_size}
                                </div>
                            )}
                        </div>
                    </div>
                )}
            </div>

            <div className="recent-files">
                <h4>Recently Processed Files</h4>
                <div className="files-list">
                    {recentFiles.length === 0 ? (
                        <div className="no-files">
                            <p>No files processed yet...</p>
                        </div>
                    ) : (
                        recentFiles.map((file, index) => (
                            <div key={`${file.path}-${file.timestamp}`} className={`file-item ${file.type}`}>
                                {file.type === 'directory' ? (
                                    <FolderX size={14} className="item-icon" />
                                ) : (
                                    <FileX size={14} className="item-icon" />
                                )}
                                <div className="item-info">
                                    <span className="item-name" title={file.name}>
                                        {file.name}
                                    </span>
                                    <span className="item-path" title={file.path}>
                                        {file.path}
                                    </span>
                                </div>
                                {file.size && (
                                    <span className="item-size">{file.size}</span>
                                )}
                            </div>
                        ))
                    )}
                </div>
            </div>

            <div className="progress-controls">
                {!isComplete && (
                    <button 
                        onClick={handleCancel}
                        className="cancel-button"
                        disabled={!isRunning}
                    >
                        <Square size={16} />
                        Cancel Operation
                    </button>
                )}
                {isComplete && (
                    <button 
                        onClick={() => onComplete(wipeResult)}
                        className="complete-button"
                    >
                        <CheckCircle size={16} />
                        Continue to Certificate
                    </button>
                )}
            </div>

            <style jsx>{`
                .wipe-progress-container {
                    width: 100%;
                    max-width: 1200px;
                    margin: 0 auto;
                    background: #1e1e1e;
                    border: 1px solid #333;
                    border-radius: 12px;
                    padding: 24px;
                    color: #cccccc;
                    font-family: "Consolas", "Monaco", "Courier New", monospace;
                }

                .wipe-progress-container.error {
                    border-color: #ff6b6b;
                    background: linear-gradient(135deg, rgba(255, 107, 107, 0.1) 0%, rgba(255, 107, 107, 0.05) 100%);
                }

                .progress-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 24px;
                    padding-bottom: 16px;
                    border-bottom: 1px solid #333;
                }

                .header-info {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                }

                .drive-icon {
                    color: #4fc3f7;
                }

                .header-info h3 {
                    margin: 0;
                    font-size: 18px;
                    color: #ffffff;
                }

                .header-info p {
                    margin: 4px 0 0 0;
                    font-size: 14px;
                    color: #999;
                }

                .progress-status {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                }

                .status-complete {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    color: #4caf50;
                    background: rgba(76, 175, 80, 0.1);
                    padding: 8px 12px;
                    border-radius: 6px;
                    border: 1px solid rgba(76, 175, 80, 0.3);
                }

                .status-running {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    color: #007acc;
                    background: rgba(0, 122, 204, 0.1);
                    padding: 8px 12px;
                    border-radius: 6px;
                    border: 1px solid rgba(0, 122, 204, 0.3);
                }

                .status-stopped {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    color: #999;
                    background: rgba(153, 153, 153, 0.1);
                    padding: 8px 12px;
                    border-radius: 6px;
                    border: 1px solid rgba(153, 153, 153, 0.3);
                }

                .spinner {
                    width: 16px;
                    height: 16px;
                    border: 2px solid rgba(0, 122, 204, 0.3);
                    border-top: 2px solid #007acc;
                    border-radius: 50%;
                    animation: spin 1s linear infinite;
                }

                @keyframes spin {
                    0% { transform: rotate(0deg); }
                    100% { transform: rotate(360deg); }
                }

                .progress-stats {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
                    gap: 16px;
                    margin-bottom: 24px;
                }

                .stat-item {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    background: #2d2d30;
                    padding: 16px;
                    border-radius: 8px;
                    border: 1px solid #444;
                }

                .stat-item svg {
                    color: #4fc3f7;
                }

                .speed-indicator {
                    width: 16px;
                    height: 16px;
                    background: linear-gradient(45deg, #4fc3f7, #007acc);
                    border-radius: 50%;
                }

                .stat-label {
                    display: block;
                    font-size: 12px;
                    color: #999;
                    margin-bottom: 2px;
                }

                .stat-value {
                    display: block;
                    font-size: 16px;
                    font-weight: 600;
                    color: #ffffff;
                }

                .current-operation {
                    margin-bottom: 24px;
                    background: #2d2d30;
                    border: 1px solid #444;
                    border-radius: 8px;
                    padding: 16px;
                }

                .operation-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    margin-bottom: 12px;
                }

                .operation-header h4 {
                    margin: 0;
                    color: #ffffff;
                }

                .operation-stage {
                    color: #4fc3f7;
                    font-size: 14px;
                    font-weight: 500;
                }

                .current-file {
                    background: #1e1e1e;
                    border: 1px solid #333;
                    border-radius: 6px;
                    padding: 12px;
                }

                .file-info {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                }

                .file-icon.folder {
                    color: #ffb74d;
                }

                .file-icon.file {
                    color: #81c784;
                }

                .file-details {
                    flex: 1;
                    min-width: 0;
                }

                .file-name {
                    font-size: 14px;
                    font-weight: 500;
                    color: #ffffff;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }

                .file-path {
                    font-size: 12px;
                    color: #999;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    margin-top: 2px;
                }

                .file-size {
                    color: #4fc3f7;
                    font-size: 12px;
                    font-weight: 500;
                }

                .recent-files {
                    margin-bottom: 24px;
                }

                .recent-files h4 {
                    margin: 0 0 12px 0;
                    color: #ffffff;
                }

                .files-list {
                    background: #2d2d30;
                    border: 1px solid #444;
                    border-radius: 8px;
                    max-height: 200px;
                    overflow-y: auto;
                }

                .no-files {
                    padding: 20px;
                    text-align: center;
                    color: #999;
                }

                .file-item {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 12px;
                    border-bottom: 1px solid #333;
                }

                .file-item:last-child {
                    border-bottom: none;
                }

                .file-item.directory .item-icon {
                    color: #ffb74d;
                }

                .file-item.file .item-icon {
                    color: #81c784;
                }

                .item-info {
                    flex: 1;
                    min-width: 0;
                }

                .item-name {
                    display: block;
                    font-size: 12px;
                    color: #ffffff;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }

                .item-path {
                    display: block;
                    font-size: 10px;
                    color: #666;
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                }

                .item-size {
                    color: #4fc3f7;
                    font-size: 10px;
                }

                .progress-controls {
                    display: flex;
                    justify-content: center;
                    gap: 16px;
                }

                .cancel-button, .complete-button {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 12px 24px;
                    border: none;
                    border-radius: 6px;
                    font-size: 14px;
                    font-weight: 500;
                    cursor: pointer;
                    transition: all 0.2s ease;
                }

                .cancel-button {
                    background: #ff6b6b;
                    color: white;
                }

                .cancel-button:hover:not(:disabled) {
                    background: #ff5252;
                }

                .cancel-button:disabled {
                    background: #666;
                    cursor: not-allowed;
                }

                .complete-button {
                    background: #4caf50;
                    color: white;
                }

                .complete-button:hover {
                    background: #45a049;
                }

                .error-header {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    margin-bottom: 16px;
                }

                .error-icon {
                    color: #ff6b6b;
                }

                .error-header h3 {
                    margin: 0;
                    color: #ff6b6b;
                }

                .error-message {
                    background: rgba(255, 107, 107, 0.1);
                    border: 1px solid rgba(255, 107, 107, 0.3);
                    border-radius: 6px;
                    padding: 16px;
                    margin-bottom: 16px;
                }

                .error-actions {
                    text-align: center;
                }

                .files-list::-webkit-scrollbar {
                    width: 6px;
                }

                .files-list::-webkit-scrollbar-track {
                    background: #1e1e1e;
                }

                .files-list::-webkit-scrollbar-thumb {
                    background: #555;
                    border-radius: 3px;
                }
            `}</style>
        </div>
    );
};

export default WipeProgress;