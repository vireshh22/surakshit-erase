// import React, { useState, useCallback, useEffect } from "react";
// import {
//     ChevronRight,
//     ChevronDown,
//     Folder,
//     FolderOpen,
//     File,
//     HardDrive,
//     Eye,
//     EyeOff,
//     Database,
//     Settings,
//     Lock,
//     Shield,
//     RefreshCw,
//     Loader,
// } from "lucide-react";
// import { invoke } from "@tauri-apps/api/core";
// import "../styles/DriveTree.css";

// const DriveTree = ({ data }) => {
//     const [fileSystemData, setFileSystemData] = useState(null);
//     const [expandedNodes, setExpandedNodes] = useState(
//         new Set(["root", "sda", "sda1", "visible-data"])
//     );
//     const [selectedNode, setSelectedNode] = useState(null);
//     const [showHidden, setShowHidden] = useState(true);
//     const [loading, setLoading] = useState(true);
//     const [error, setError] = useState(null);

//     // Load real file system data on component mount
//     useEffect(() => {
//         loadFileSystemData();
//     }, []);

//     const loadFileSystemData = async () => {
//         setLoading(true);
//         setError(null);
        
//         try {
//             console.log('Loading file system data...');
//             const data = await invoke('get_file_system_data');
//             console.log('Received data:', data);
//             setFileSystemData(data);
            
//             // Auto-expand first drive if available
//             if (data.children && data.children.length > 0) {
//                 const firstDrive = data.children[0];
//                 const driveId = getNodeId(firstDrive, "");
//                 setExpandedNodes(prev => new Set([...prev, driveId]));
//             }
//         } catch (error) {
//             console.error('Failed to load file system data:', error);
//             setError(error.toString());
//             // Fallback to mock data if Tauri call fails
//             setFileSystemData(getMockData());
//         } finally {
//             setLoading(false);
//         }
//     };

//     // Fallback mock data (your original data structure)
//     const getMockData = () => ({
//         name: "System Drives",
//         type: "root",
//         children: [
//             {
//                 name: "sda (50GB VBOX HARDDISK)",
//                 type: "drive",
//                 icon: "drive",
//                 size: "50GB",
//                 children: [
//                     {
//                         name: "sda1 (45GB)",
//                         type: "partition",
//                         icon: "partition",
//                         size: "45GB",
//                         children: [
//                             {
//                                 name: "Visible User Data (/mnt/data)",
//                                 type: "folder",
//                                 icon: "folder",
//                                 visible: true,
//                                 children: [
//                                     {
//                                         name: "Documents",
//                                         type: "folder",
//                                         icon: "folder",
//                                         children: [
//                                             {
//                                                 name: "sample.txt",
//                                                 type: "file",
//                                                 size: "100MB",
//                                                 icon: "file",
//                                             },
//                                             {
//                                                 name: "report.docx",
//                                                 type: "file",
//                                                 size: "2.5MB",
//                                                 icon: "file",
//                                             },
//                                         ],
//                                     },
//                                     {
//                                         name: "Media",
//                                         type: "folder",
//                                         icon: "folder",
//                                         children: [
//                                             {
//                                                 name: "photo1.jpg",
//                                                 type: "file",
//                                                 size: "3.2MB",
//                                                 icon: "file",
//                                             },
//                                             {
//                                                 name: "video.mp4",
//                                                 type: "file",
//                                                 size: "250MB",
//                                                 icon: "file",
//                                             },
//                                         ],
//                                     },
//                                 ],
//                             },
//                             {
//                                 name: "Hidden HPA Region (/mnt/hpa)",
//                                 type: "folder",
//                                 icon: "hidden",
//                                 hidden: true,
//                                 children: [
//                                     {
//                                         name: "HiddenBackup",
//                                         type: "folder",
//                                         icon: "folder",
//                                         hidden: true,
//                                         children: [
//                                             {
//                                                 name: "secret.db",
//                                                 type: "file",
//                                                 size: "50MB",
//                                                 icon: "database",
//                                                 hidden: true,
//                                             },
//                                             {
//                                                 name: "backup.img",
//                                                 type: "file",
//                                                 size: "2GB",
//                                                 icon: "file",
//                                                 hidden: true,
//                                             },
//                                         ],
//                                     },
//                                 ],
//                             },
//                             {
//                                 name: "Hidden DCO Region (/mnt/dco)",
//                                 type: "folder",
//                                 icon: "system",
//                                 hidden: true,
//                                 children: [
//                                     {
//                                         name: "ManufacturerData",
//                                         type: "folder",
//                                         icon: "folder",
//                                         hidden: true,
//                                         children: [
//                                             {
//                                                 name: "config.bin",
//                                                 type: "file",
//                                                 size: "20MB",
//                                                 icon: "settings",
//                                                 hidden: true,
//                                             },
//                                             {
//                                                 name: "firmware.rom",
//                                                 type: "file",
//                                                 size: "8MB",
//                                                 icon: "file",
//                                                 hidden: true,
//                                             },
//                                         ],
//                                     },
//                                 ],
//                             },
//                         ],
//                     },
//                 ],
//             },
//         ],
//     });

//     const getNodeId = useCallback((node, parentPath = "") => {
//         const path = parentPath ? `${parentPath}/${node.name}` : node.name;
//         return path.replace(/[^a-zA-Z0-9]/g, "-").toLowerCase();
//     }, []);

//     const toggleExpanded = useCallback((nodeId) => {
//         setExpandedNodes((prev) => {
//             const newSet = new Set(prev);
//             if (newSet.has(nodeId)) {
//                 newSet.delete(nodeId);
//             } else {
//                 newSet.add(nodeId);
//             }
//             return newSet;
//         });
//     }, []);

//     const getIcon = (node, isExpanded) => {
//         const iconProps = { size: 16, className: "icon" };

//         switch (node.icon) {
//             case "drive":
//                 return <HardDrive {...iconProps} className="icon drive-icon" />;
//             case "partition":
//                 return (
//                     <Database {...iconProps} className="icon partition-icon" />
//                 );
//             case "folder":
//                 return isExpanded ? (
//                     <FolderOpen {...iconProps} />
//                 ) : (
//                     <Folder {...iconProps} />
//                 );
//             case "hidden":
//                 return <Lock {...iconProps} className="icon hidden-icon" />;
//             case "system":
//                 return <Shield {...iconProps} className="icon system-icon" />;
//             case "database":
//                 return (
//                     <Database {...iconProps} className="icon database-icon" />
//                 );
//             case "settings":
//                 return (
//                     <Settings {...iconProps} className="icon settings-icon" />
//                 );
//             default:
//                 return <File {...iconProps} />;
//         }
//     };

//     const renderNode = (node, level = 0, parentPath = "") => {
//         if (node.hidden && !showHidden) return null;

//         const nodeId = getNodeId(node, parentPath);
//         const hasChildren = node.children && node.children.length > 0;
//         const isExpanded = expandedNodes.has(nodeId);
//         const isSelected = selectedNode === nodeId;

//         const handleClick = () => {
//             setSelectedNode(nodeId);
//             if (hasChildren) {
//                 toggleExpanded(nodeId);
//             }
//         };

//         const getNodeClass = () => {
//             let baseClass = "tree-node";
//             if (isSelected) baseClass += " selected";
//             if (node.hidden) baseClass += " hidden-node";
//             // Handle both 'type' and 'node_type' properties from different sources
//             const nodeType = node.type || node.node_type;
//             if (nodeType === "drive") baseClass += " drive-node";
//             if (nodeType === "partition") baseClass += " partition-node";
//             return baseClass;
//         };

//         return (
//             <div key={nodeId} className="tree-item">
//                 <div
//                     className={getNodeClass()}
//                     style={{ paddingLeft: `${level * 20 + 8}px` }}
//                     onClick={handleClick}
//                 >
//                     <span className="node-expand">
//                         {hasChildren ? (
//                             isExpanded ? (
//                                 <ChevronDown size={16} />
//                             ) : (
//                                 <ChevronRight size={16} />
//                             )
//                         ) : (
//                             <span
//                                 style={{ width: 16, display: "inline-block" }}
//                             />
//                         )}
//                     </span>

//                     <span className="node-icon">
//                         {getIcon(node, isExpanded)}
//                     </span>

//                     <span className="node-name">
//                         {node.name}
//                         {node.hidden && (
//                             <EyeOff size={12} className="hidden-indicator" />
//                         )}
//                     </span>

//                     {node.size && (
//                         <span className="node-size">{node.size}</span>
//                     )}
//                 </div>

//                 {hasChildren && isExpanded && (
//                     <div className="children">
//                         {node.children.map((child, index) =>
//                             renderNode(
//                                 child,
//                                 level + 1,
//                                 `${parentPath}/${node.name}`
//                             )
//                         )}
//                     </div>
//                 )}
//             </div>
//         );
//     };

//     // Use real data if available, otherwise fall back to mock data
//     const dataToRender = fileSystemData || getMockData();

//     if (loading) {
//         return (
//             <div className="file-explorer">
//                 <div className="explorer-header">
//                     <h2>File System Explorer</h2>
//                 </div>
//                 <div className="loading-container">
//                     <Loader className="loading-spinner" size={24} />
//                     <span>Loading file system data...</span>
//                 </div>
//             </div>
//         );
//     }

//     if (error && !fileSystemData) {
//         return (
//             <div className="file-explorer">
//                 <div className="explorer-header">
//                     <h2>File System Explorer</h2>
//                     <div className="explorer-controls">
//                         <button
//                             className="refresh-button"
//                             onClick={loadFileSystemData}
//                             title="Retry loading"
//                         >
//                             <RefreshCw size={16} />
//                             Retry
//                         </button>
//                     </div>
//                 </div>
//                 <div className="error-container">
//                     <span className="error-message">
//                         Failed to load real data, using demo data: {error}
//                     </span>
//                 </div>
//                 <div className="tree-container">
//                     {getMockData().children.map((child) =>
//                         renderNode(child, 0, "")
//                     )}
//                 </div>
//             </div>
//         );
//     }

//     return (
//         <div className="file-explorer">
//             <div className="explorer-header">
//                 <h2>File System Explorer</h2>
//                 <div className="explorer-controls">
//                     <button
//                         className="refresh-button"
//                         onClick={loadFileSystemData}
//                         title="Refresh data"
//                     >
//                         <RefreshCw size={16} />
//                         Refresh
//                     </button>
//                     <button
//                         className={`toggle-hidden ${
//                             showHidden ? "active" : ""
//                         }`}
//                         onClick={() => setShowHidden(!showHidden)}
//                         title={
//                             showHidden
//                                 ? "Hide hidden files"
//                                 : "Show hidden files"
//                         }
//                     >
//                         {showHidden ? <Eye size={16} /> : <EyeOff size={16} />}
//                         {showHidden ? "Hide Hidden" : "Show Hidden"}
//                     </button>
//                 </div>
//             </div>

//             <div className="tree-container">
//                 {dataToRender.children && dataToRender.children.map((child, index) =>
//                     renderNode(child, 0, "")
//                 )}
//             </div>

//             <div className="explorer-footer">
//                 <div className="legend">
//                     <div className="legend-item">
//                         <HardDrive size={14} className="drive-icon" />
//                         <span>Drive</span>
//                     </div>
//                     <div className="legend-item">
//                         <Database size={14} className="partition-icon" />
//                         <span>Partition</span>
//                     </div>
//                     <div className="legend-item">
//                         <Lock size={14} className="hidden-icon" />
//                         <span>Hidden Region</span>
//                     </div>
//                     <div className="legend-item">
//                         <Shield size={14} className="system-icon" />
//                         <span>System Region</span>
//                     </div>
//                 </div>
//                 <div className="status-info">
//                     <small>
//                         {fileSystemData ? "Live data from /mnt/demo" : "Demo data"}
//                         {error && " (fallback mode)"}
//                     </small>
//                 </div>
//                 <div className="proceed-section">
//                     <button
//                         className="proceed-button"
//                         onClick={onProceedToMethodSelection}
//                         disabled={!driveFileSystem}
//                     >
//                         <span>Proceed to Wipe Methods</span>
//                         <ArrowRight size={16} />
//                     </button>
//                 </div>
//             </div>
//         </div>
//     );
// };

// export default DriveTree;



import React, { useState, useCallback, useEffect } from "react";
import {
    ChevronRight,
    ChevronDown,
    Folder,
    FolderOpen,
    File,
    HardDrive,
    Eye,
    EyeOff,
    Database,
    Settings,
    Lock,
    Shield,
    RefreshCw,
    Loader,
    ArrowRight,
} from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import "../styles/DriveTree.css";

const DriveTree = ({ selectedDrive, driveFileSystem, onProceedToMethodSelection, dispatch }) => {
    const [expandedNodes, setExpandedNodes] = useState(
        new Set(["root"])
    );
    const [selectedNode, setSelectedNode] = useState(null);
    const [showHidden, setShowHidden] = useState(true);
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState(null);

    // Load drive contents when selectedDrive changes
    useEffect(() => {
        if (selectedDrive && !driveFileSystem) {
            loadDriveFileSystem();
        }
    }, [selectedDrive]);

    const loadDriveFileSystem = async () => {
        if (!selectedDrive) return;
        
        setLoading(true);
        setError(null);
        
        try {
            console.log('Loading file system for drive:', selectedDrive.path);
            const data = await invoke('get_drive_file_system', { 
                drivePath: selectedDrive.path 
            });
            console.log('Received drive file system data:', data);
            
            dispatch({
                type: 'FETCH_DRIVE_CONTENTS_SUCCESS',
                payload: data
            });

            // Auto-expand first drive if available
            if (data.children && data.children.length > 0) {
                const firstDrive = data.children[0];
                const driveId = getNodeId(firstDrive, "");
                setExpandedNodes(prev => new Set([...prev, driveId]));
            }
        } catch (error) {
            console.error('Failed to load drive file system:', error);
            const errorMsg = error.toString();
            setError(errorMsg);
            dispatch({
                type: 'FETCH_DRIVE_CONTENTS_FAILURE',
                payload: errorMsg
            });
        } finally {
            setLoading(false);
        }
    };

    const getNodeId = useCallback((node, parentPath = "") => {
        const path = parentPath ? `${parentPath}/${node.name}` : node.name;
        return path.replace(/[^a-zA-Z0-9]/g, "-").toLowerCase();
    }, []);

    const toggleExpanded = useCallback((nodeId) => {
        setExpandedNodes((prev) => {
            const newSet = new Set(prev);
            if (newSet.has(nodeId)) {
                newSet.delete(nodeId);
            } else {
                newSet.add(nodeId);
            }
            return newSet;
        });
    }, []);

    const getIcon = (node, isExpanded) => {
        const iconProps = { size: 16, className: "icon" };

        switch (node.icon) {
            case "drive":
                return <HardDrive {...iconProps} className="icon drive-icon" />;
            case "partition":
                return (
                    <Database {...iconProps} className="icon partition-icon" />
                );
            case "folder":
                return isExpanded ? (
                    <FolderOpen {...iconProps} />
                ) : (
                    <Folder {...iconProps} />
                );
            case "hidden":
                return <Lock {...iconProps} className="icon hidden-icon" />;
            case "system":
                return <Shield {...iconProps} className="icon system-icon" />;
            case "database":
                return (
                    <Database {...iconProps} className="icon database-icon" />
                );
            case "settings":
                return (
                    <Settings {...iconProps} className="icon settings-icon" />
                );
            default:
                return <File {...iconProps} />;
        }
    };

    const renderNode = (node, level = 0, parentPath = "") => {
        if (node.hidden && !showHidden) return null;

        const nodeId = getNodeId(node, parentPath);
        const hasChildren = node.children && node.children.length > 0;
        const isExpanded = expandedNodes.has(nodeId);
        const isSelected = selectedNode === nodeId;

        const handleClick = () => {
            setSelectedNode(nodeId);
            if (hasChildren) {
                toggleExpanded(nodeId);
            }
        };

        const getNodeClass = () => {
            let baseClass = "tree-node";
            if (isSelected) baseClass += " selected";
            if (node.hidden) baseClass += " hidden-node";
            // Handle both 'type' and 'node_type' properties
            const nodeType = node.type || node.node_type;
            if (nodeType === "drive") baseClass += " drive-node";
            if (nodeType === "partition") baseClass += " partition-node";
            return baseClass;
        };

        return (
            <div key={nodeId} className="tree-item">
                <div
                    className={getNodeClass()}
                    style={{ paddingLeft: `${level * 20 + 8}px` }}
                    onClick={handleClick}
                >
                    <span className="node-expand">
                        {hasChildren ? (
                            isExpanded ? (
                                <ChevronDown size={16} />
                            ) : (
                                <ChevronRight size={16} />
                            )
                        ) : (
                            <span
                                style={{ width: 16, display: "inline-block" }}
                            />
                        )}
                    </span>

                    <span className="node-icon">
                        {getIcon(node, isExpanded)}
                    </span>

                    <span className="node-name">
                        {node.name}
                        {node.hidden && (
                            <EyeOff size={12} className="hidden-indicator" />
                        )}
                    </span>

                    {node.size && (
                        <span className="node-size">{node.size}</span>
                    )}
                </div>

                {hasChildren && isExpanded && (
                    <div className="children">
                        {node.children.map((child, index) =>
                            renderNode(
                                child,
                                level + 1,
                                `${parentPath}/${node.name}`
                            )
                        )}
                    </div>
                )}
            </div>
        );
    };

    if (loading) {
        return (
            <div className="file-explorer">
                <div className="explorer-header">
                    <h2>Drive Contents Explorer</h2>
                </div>
                <div className="loading-container">
                    <Loader className="loading-spinner" size={24} />
                    <span>Loading drive contents...</span>
                </div>
            </div>
        );
    }

    return (
        <div className="file-explorer">
            <div className="explorer-header">
                <h2>Drive Contents Explorer</h2>
                <div className="explorer-controls">
                    <button
                        className="refresh-button"
                        onClick={loadDriveFileSystem}
                        title="Refresh drive contents"
                    >
                        <RefreshCw size={16} />
                        Refresh
                    </button>
                    <button
                        className={`toggle-hidden ${
                            showHidden ? "active" : ""
                        }`}
                        onClick={() => setShowHidden(!showHidden)}
                        title={
                            showHidden
                                ? "Hide hidden files"
                                : "Show hidden files"
                        }
                    >
                        {showHidden ? <Eye size={16} /> : <EyeOff size={16} />}
                        {showHidden ? "Hide Hidden" : "Show Hidden"}
                    </button>
                </div>
            </div>

            <div className="tree-container">
                {error ? (
                    <div className="error-container">
                        <span className="error-message">
                            Error loading drive contents: {error}
                        </span>
                        <button 
                            onClick={loadDriveFileSystem} 
                            className="retry-button"
                        >
                            <RefreshCw size={16} />
                            Retry
                        </button>
                    </div>
                ) : driveFileSystem && driveFileSystem.children ? (
                    driveFileSystem.children.map((child, index) =>
                        renderNode(child, 0, "")
                    )
                ) : (
                    <div className="no-data">
                        <HardDrive size={48} className="no-data-icon" />
                        <span>No drive contents loaded</span>
                    </div>
                )}
            </div>

            <div className="explorer-footer">
                <div className="legend">
                    <div className="legend-item">
                        <HardDrive size={14} className="drive-icon" />
                        <span>Drive</span>
                    </div>
                    <div className="legend-item">
                        <Database size={14} className="partition-icon" />
                        <span>Partition</span>
                    </div>
                    <div className="legend-item">
                        <Lock size={14} className="hidden-icon" />
                        <span>Hidden Region</span>
                    </div>
                    <div className="legend-item">
                        <Shield size={14} className="system-icon" />
                        <span>System Region</span>
                    </div>
                </div>
                
                <div className="proceed-section">
                    <button
                        className="proceed-button"
                        onClick={onProceedToMethodSelection}
                        disabled={!driveFileSystem}
                    >
                        <span>Proceed to Wipe Methods</span>
                        <ArrowRight size={16} />
                    </button>
                </div>
            </div>
        </div>
    );
};

export default DriveTree;