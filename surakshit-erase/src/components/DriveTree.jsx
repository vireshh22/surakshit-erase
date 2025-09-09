import React, { useState, useCallback } from 'react';
import { ChevronRight, ChevronDown, Folder, FolderOpen, File, HardDrive, Eye, EyeOff, Database, Settings, Lock, Shield } from 'lucide-react';
import '../styles/DriveTree.css';
// Mock data structure representing the file system
const mockFileSystemData = {
  name: "System Drives",
  type: "root",
  children: [
    {
      name: "sda (50GB VBOX HARDDISK)",
      type: "drive",
      icon: "drive",
      size: "50GB",
      children: [
        {
          name: "sda1 (45GB)",
          type: "partition",
          icon: "partition",
          size: "45GB",
          children: [
            {
              name: "Visible User Data (/mnt/data)",
              type: "folder",
              icon: "folder",
              visible: true,
              children: [
                {
                  name: "Documents",
                  type: "folder",
                  icon: "folder",
                  children: [
                    { name: "sample.txt", type: "file", size: "100MB", icon: "file" },
                    { name: "report.docx", type: "file", size: "2.5MB", icon: "file" },
                    { name: "presentation.pptx", type: "file", size: "15MB", icon: "file" }
                  ]
                },
                {
                  name: "Media",
                  type: "folder",
                  icon: "folder",
                  children: [
                    { name: "photo1.jpg", type: "file", size: "3.2MB", icon: "file" },
                    { name: "video.mp4", type: "file", size: "250MB", icon: "file" },
                    { name: "music.mp3", type: "file", size: "4.1MB", icon: "file" }
                  ]
                },
                {
                  name: "Downloads",
                  type: "folder",
                  icon: "folder",
                  children: [
                    { name: "installer.exe", type: "file", size: "50MB", icon: "file" },
                    { name: "archive.zip", type: "file", size: "125MB", icon: "file" }
                  ]
                }
              ]
            },
            {
              name: "Hidden HPA Region (/mnt/hpa)",
              type: "folder",
              icon: "hidden",
              hidden: true,
              children: [
                {
                  name: "HiddenBackup",
                  type: "folder",
                  icon: "folder",
                  hidden: true,
                  children: [
                    { name: "secret.db", type: "file", size: "50MB", icon: "database", hidden: true },
                    { name: "backup.img", type: "file", size: "2GB", icon: "file", hidden: true },
                    { name: "keys.bin", type: "file", size: "1KB", icon: "file", hidden: true }
                  ]
                },
                {
                  name: "SystemRestore",
                  type: "folder",
                  icon: "folder",
                  hidden: true,
                  children: [
                    { name: "restore_point_1.dat", type: "file", size: "500MB", icon: "file", hidden: true },
                    { name: "registry_backup.reg", type: "file", size: "10MB", icon: "file", hidden: true }
                  ]
                }
              ]
            },
            {
              name: "Hidden DCO Region (/mnt/dco)",
              type: "folder",
              icon: "system",
              hidden: true,
              children: [
                {
                  name: "ManufacturerData",
                  type: "folder",
                  icon: "folder",
                  hidden: true,
                  children: [
                    { name: "config.bin", type: "file", size: "20MB", icon: "settings", hidden: true },
                    { name: "firmware.rom", type: "file", size: "8MB", icon: "file", hidden: true },
                    { name: "diagnostic.log", type: "file", size: "2MB", icon: "file", hidden: true }
                  ]
                }
              ]
            }
          ]
        }
      ]
    },
    {
      name: "sdb (128GB USB Drive)",
      type: "drive",
      icon: "drive",
      size: "128GB",
      children: [
        {
          name: "sdb1 (120GB)",
          type: "partition",
          icon: "partition",
          size: "120GB",
          children: [
            {
              name: "USB Storage",
              type: "folder",
              icon: "folder",
              children: [
                { name: "vacation_photos.zip", type: "file", size: "2.1GB", icon: "file" },
                { name: "work_backup.7z", type: "file", size: "856MB", icon: "file" },
                { name: "readme.txt", type: "file", size: "1KB", icon: "file" }
              ]
            }
          ]
        }
      ]
    }
  ]
};

const DriveTree = ({ data }) => {
  const [expandedNodes, setExpandedNodes] = useState(new Set(['root', 'sda', 'sda1', 'visible-data']));
  const [selectedNode, setSelectedNode] = useState(null);
  const [showHidden, setShowHidden] = useState(true);

  const getNodeId = useCallback((node, parentPath = '') => {
    const path = parentPath ? `${parentPath}/${node.name}` : node.name;
    return path.replace(/[^a-zA-Z0-9]/g, '-').toLowerCase();
  }, []);

  const toggleExpanded = useCallback((nodeId) => {
    setExpandedNodes(prev => {
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
      case 'drive':
        return <HardDrive {...iconProps} className="icon drive-icon" />;
      case 'partition':
        return <Database {...iconProps} className="icon partition-icon" />;
      case 'folder':
        return isExpanded ? <FolderOpen {...iconProps} /> : <Folder {...iconProps} />;
      case 'hidden':
        return <Lock {...iconProps} className="icon hidden-icon" />;
      case 'system':
        return <Shield {...iconProps} className="icon system-icon" />;
      case 'database':
        return <Database {...iconProps} className="icon database-icon" />;
      case 'settings':
        return <Settings {...iconProps} className="icon settings-icon" />;
      default:
        return <File {...iconProps} />;
    }
  };

  const renderNode = (node, level = 0, parentPath = '') => {
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
      if (node.type === 'drive') baseClass += " drive-node";
      if (node.type === 'partition') baseClass += " partition-node";
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
              isExpanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />
            ) : (
              <span style={{ width: 16, display: 'inline-block' }} />
            )}
          </span>
          
          <span className="node-icon">
            {getIcon(node, isExpanded)}
          </span>
          
          <span className="node-name">
            {node.name}
            {node.hidden && <EyeOff size={12} className="hidden-indicator" />}
          </span>
          
          {node.size && (
            <span className="node-size">{node.size}</span>
          )}
        </div>
        
        {hasChildren && isExpanded && (
          <div className="children">
            {node.children.map(child => 
              renderNode(child, level + 1, `${parentPath}/${node.name}`)
            )}
          </div>
        )}
      </div>
    );
  };

  return (
    <div className="file-explorer">
      <div className="explorer-header">
        <h2>File System Explorer</h2>
        <div className="explorer-controls">
          <button 
            className={`toggle-hidden ${showHidden ? 'active' : ''}`}
            onClick={() => setShowHidden(!showHidden)}
            title={showHidden ? "Hide hidden files" : "Show hidden files"}
          >
            {showHidden ? <Eye size={16} /> : <EyeOff size={16} />}
            {showHidden ? "Hide Hidden" : "Show Hidden"}
          </button>
        </div>
      </div>
      
      <div className="tree-container">
        {mockFileSystemData.children.map(child => 
          renderNode(child, 0, '')
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
      </div>

      <style jsx>{`
        
      `}</style>
    </div>
  );
};

export default DriveTree;


