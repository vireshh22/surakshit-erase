// DriveSelector.jsx
import React, { useState, useEffect } from "react";
import { 
    HardDrive, 
    RefreshCw, 
    Loader, 
    AlertCircle,
    ChevronRight,
    Database,
    Info
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import "../styles/DriveSelector.css";

const DriveSelector = ({ onDriveSelect, selectedDrive }) => {
    const [drives, setDrives] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        loadAvailableDrives();
    }, []);

    const loadAvailableDrives = async () => {
        setLoading(true);
        setError(null);
        
        try {
            console.log('Loading available drives...');
            const driveList = await invoke('get_available_drives');
            console.log('Received drives:', driveList);
            setDrives(driveList);
        } catch (error) {
            console.error('Failed to load drives:', error);
            setError(error.toString());
        } finally {
            setLoading(false);
        }
    };

    const handleDriveClick = (drive) => {
        onDriveSelect(drive);
    };

    const formatSize = (sizeStr) => {
        if (!sizeStr) return "Unknown";
        return sizeStr;
    };

    const getDriveIcon = (drive) => {
        if (drive.device_type === "usb") {
            return <Database size={24} className="drive-icon usb" />;
        }
        return <HardDrive size={24} className="drive-icon" />;
    };

    const getDriveTypeLabel = (drive) => {
        if (drive.removable) return "Removable Drive";
        if (drive.device_type === "usb") return "USB Drive";
        return "Fixed Drive";
    };

    if (loading) {
        return (
            <div className="drive-selector">
                <div className="selector-header">
                    <h2>Select Drive to Erase</h2>
                </div>
                <div className="loading-container">
                    <Loader className="loading-spinner" size={32} />
                    <span>Scanning for available drives...</span>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="drive-selector">
                <div className="selector-header">
                    <h2>Select Drive to Erase</h2>
                    <button
                        className="refresh-button"
                        onClick={loadAvailableDrives}
                        title="Retry"
                    >
                        <RefreshCw size={16} />
                        Retry
                    </button>
                </div>
                <div className="error-container">
                    <AlertCircle size={32} className="error-icon" />
                    <span className="error-message">
                        Failed to load drives: {error}
                    </span>
                </div>
            </div>
        );
    }

    return (
        <div className="drive-selector">
            <div className="selector-header">
                <h2>Select Drive to Erase</h2>
                <button
                    className="refresh-button"
                    onClick={loadAvailableDrives}
                    title="Refresh drives"
                >
                    <RefreshCw size={16} />
                    Refresh
                </button>
            </div>

            <div className="drives-container">
                {drives.length === 0 ? (
                    <div className="no-drives">
                        <HardDrive size={48} className="no-drives-icon" />
                        <h3>No Drives Found</h3>
                        <p>No storage drives were detected on this system.</p>
                    </div>
                ) : (
                    <div className="drives-grid">
                        {drives.map((drive, index) => (
                            <div
                                key={drive.name}
                                className={`drive-card ${
                                    selectedDrive?.name === drive.name ? 'selected' : ''
                                }`}
                                onClick={() => handleDriveClick(drive)}
                            >
                                <div className="drive-card-header">
                                    {getDriveIcon(drive)}
                                    <div className="drive-info">
                                        <div className="drive-name">{drive.name}</div>
                                        <div className="drive-type">
                                            {getDriveTypeLabel(drive)}
                                        </div>
                                    </div>
                                    <ChevronRight size={20} className="select-arrow" />
                                </div>

                                <div className="drive-details">
                                    <div className="detail-item">
                                        <span className="detail-label">Size:</span>
                                        <span className="detail-value">
                                            {formatSize(drive.size)}
                                        </span>
                                    </div>
                                    {drive.model && (
                                        <div className="detail-item">
                                            <span className="detail-label">Model:</span>
                                            <span className="detail-value">{drive.model}</span>
                                        </div>
                                    )}
                                    {drive.vendor && (
                                        <div className="detail-item">
                                            <span className="detail-label">Vendor:</span>
                                            <span className="detail-value">{drive.vendor}</span>
                                        </div>
                                    )}
                                    <div className="detail-item">
                                        <span className="detail-label">Path:</span>
                                        <span className="detail-value device-path">
                                            {drive.path}
                                        </span>
                                    </div>
                                </div>

                                {drive.partitions && drive.partitions.length > 0 && (
                                    <div className="partitions-info">
                                        <div className="partitions-header">
                                            <Info size={14} />
                                            <span>
                                                {drive.partitions.length} partition(s)
                                            </span>
                                        </div>
                                        {drive.partitions.slice(0, 2).map((partition, pIndex) => (
                                            <div key={pIndex} className="partition-item">
                                                <span className="partition-name">
                                                    {partition.name}
                                                </span>
                                                {partition.size && (
                                                    <span className="partition-size">
                                                        ({partition.size})
                                                    </span>
                                                )}
                                                {partition.filesystem && (
                                                    <span className="partition-fs">
                                                        {partition.filesystem}
                                                    </span>
                                                )}
                                            </div>
                                        ))}
                                        {drive.partitions.length > 2 && (
                                            <div className="more-partitions">
                                                +{drive.partitions.length - 2} more
                                            </div>
                                        )}
                                    </div>
                                )}
                            </div>
                        ))}
                    </div>
                )}
            </div>

            <div className="selector-footer">
                <div className="warning-message">
                    <AlertCircle size={16} className="warning-icon" />
                    <span>
                        ⚠️ Warning: All data on the selected drive will be permanently erased.
                    </span>
                </div>
            </div>
        </div>
    );
};

export default DriveSelector;