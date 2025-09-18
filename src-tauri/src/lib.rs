// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemNode {
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    icon: String,
    size: Option<String>,
    hidden: Option<bool>,
    visible: Option<bool>,
    children: Option<Vec<FileSystemNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveInfo {
    name: String,
    path: String,
    size: Option<String>,
    model: Option<String>,
    vendor: Option<String>,
    device_type: Option<String>,
    removable: bool,
    partitions: Vec<PartitionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartitionInfo {
    name: String,
    size: Option<String>,
    filesystem: Option<String>,
    mountpoint: Option<String>,
}

// Get all available drives in the system
#[tauri::command]
async fn get_available_drives() -> Result<Vec<DriveInfo>, String> {
    let mut drives = Vec::new();

    // Use lsblk to get all block devices
    let output = Command::new("lsblk")
        .arg("-d") // Only show devices, not partitions
        .arg("-o")
        .arg("NAME,SIZE,MODEL,VENDOR,TYPE,RM")
        .arg("-n") // No headers
        .output()
        .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    for line in lines {
        if let Ok(drive) = parse_drive_info(line).await {
            // Skip loop devices and other virtual devices
            if !drive.name.starts_with("loop") && 
               !drive.name.starts_with("sr") &&
               !drive.name.starts_with("ram") {
                drives.push(drive);
            }
        }
    }

    Ok(drives)
}

async fn parse_drive_info(line: &str) -> Result<DriveInfo, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 6 {
        return Err("Invalid drive line".to_string());
    }

    let name = parts[0].to_string();
    let size = if parts[1] != "-" { Some(parts[1].to_string()) } else { None };
    let model = if parts[2] != "-" { Some(parts[2].to_string()) } else { None };
    let vendor = if parts[3] != "-" { Some(parts[3].to_string()) } else { None };
    let device_type = if parts[4] != "-" { Some(parts[4].to_string()) } else { None };
    let removable = parts[5] == "1";

    // Get partitions for this drive
    let partitions = get_drive_partitions(&name).await?;

    Ok(DriveInfo {
        name: name.clone(),
        path: format!("/dev/{}", name),
        size,
        model,
        vendor,
        device_type,
        removable,
        partitions,
    })
}

async fn get_drive_partitions(drive_name: &str) -> Result<Vec<PartitionInfo>, String> {
    let mut partitions = Vec::new();

    let output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
        .arg("-n")
        .arg(&format!("/dev/{}", drive_name))
        .output()
        .map_err(|e| format!("Failed to get partitions: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Skip the first line (which is the drive itself)
    for line in lines.iter().skip(1) {
        if let Ok(partition) = parse_partition_info(line) {
            partitions.push(partition);
        }
    }

    Ok(partitions)
}

fn parse_partition_info(line: &str) -> Result<PartitionInfo, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Invalid partition line".to_string());
    }

    let name = parts[0].trim_start_matches("├─").trim_start_matches("└─").to_string();
    let size = parts.get(1).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });
    let filesystem = parts.get(2).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });
    let mountpoint = parts.get(3).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });

    Ok(PartitionInfo {
        name,
        size,
        filesystem,
        mountpoint,
    })
}

// Get file system structure for a specific drive
#[tauri::command]
async fn get_drive_file_system(drive_path: String) -> Result<FileSystemNode, String> {
    let drive_name = drive_path.strip_prefix("/dev/").unwrap_or(&drive_path);
    
    let mut root = FileSystemNode {
        name: format!("Drive: {}", drive_name),
        node_type: "root".to_string(),
        icon: "root".to_string(),
        size: None,
        hidden: None,
        visible: None,
        children: Some(Vec::new()),
    };

    // Get drive information
    let drives = get_drive_contents(&drive_path).await?;
    root.children = Some(drives);

    Ok(root)
}

async fn get_drive_contents(drive_path: &str) -> Result<Vec<FileSystemNode>, String> {
    let mut drives = Vec::new();

    // Get detailed information about the specific drive
    let output = Command::new("lsblk")
        .arg("-f")
        .arg("-o")
        .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
        .arg(drive_path)
        .output()
        .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() > 1 {
        // Parse the drive info
        let drive_info = parse_drive_line(&lines[1])?;
        let mut drive_node = create_drive_node(&drive_info.name, &drive_info)?;

        // Process partitions
        for line in lines.iter().skip(2) {
            if let Ok(partition_info) = parse_drive_line(line) {
                let mut partition_node = create_partition_node(&partition_info.name, &partition_info)?;

                // If partition is mounted, scan its contents
                if let Some(mount_point) = &partition_info.mountpoint {
                    match get_directory_contents(mount_point, 0).await {
                        Ok(contents) => {
                            partition_node.children = Some(contents);
                        }
                        Err(_) => {
                            // If we can't read, create placeholder structure
                            partition_node.children = Some(create_placeholder_structure(mount_point));
                        }
                    }
                    
                    // Add demo hidden regions for demonstration
                    let mut children = partition_node.children.unwrap_or_default();
                    children.extend(create_hidden_regions());
                    partition_node.children = Some(children);
                }

                if let Some(ref mut drive_children) = drive_node.children {
                    drive_children.push(partition_node);
                }
            }
        }

        drives.push(drive_node);
    }

    Ok(drives)
}

#[derive(Debug)]
struct DriveLineInfo {
    name: String,
    size: Option<String>,
    fstype: Option<String>,
    mountpoint: Option<String>,
}

fn parse_drive_line(line: &str) -> Result<DriveLineInfo, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Invalid drive line".to_string());
    }

    // Clean the name from tree characters
    let name = parts[0].trim_start_matches("├─").trim_start_matches("└─").to_string();
    
    Ok(DriveLineInfo {
        name,
        size: parts.get(1).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
        fstype: parts.get(2).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
        mountpoint: parts.get(3).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
    })
}

fn create_drive_node(name: &str, info: &DriveLineInfo) -> Result<FileSystemNode, String> {
    let display_name = if let Some(size) = &info.size {
        format!("{} ({} Drive)", name, size)
    } else {
        format!("{} (Drive)", name)
    };

    Ok(FileSystemNode {
        name: display_name,
        node_type: "drive".to_string(),
        icon: "drive".to_string(),
        size: info.size.clone(),
        hidden: None,
        visible: None,
        children: Some(Vec::new()),
    })
}

fn create_partition_node(name: &str, info: &DriveLineInfo) -> Result<FileSystemNode, String> {
    let display_name = if let Some(size) = &info.size {
        format!("{} ({})", name, size)
    } else {
        name.to_string()
    };

    Ok(FileSystemNode {
        name: display_name,
        node_type: "partition".to_string(),
        icon: "partition".to_string(),
        size: info.size.clone(),
        hidden: None,
        visible: None,
        children: Some(Vec::new()),
    })
}

#[async_recursion]
async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
    if depth > 3 {
        return Ok(Vec::new());
    }

    let mut nodes = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut entry_vec: Vec<_> = entries.collect();
            entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

            for entry in entry_vec {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    // Skip special directories that might cause issues
                    if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/" {
                        continue;
                    }

                    let metadata = match entry.metadata() {
                        Ok(metadata) => metadata,
                        Err(_) => continue,
                    };

                    let is_dir = metadata.is_dir();
                    let is_hidden = name.starts_with('.');

                    let size = if !is_dir {
                        Some(format_bytes(metadata.len()))
                    } else {
                        None
                    };

                    let mut node = FileSystemNode {
                        name: name.clone(),
                        node_type: if is_dir { "folder".to_string() } else { "file".to_string() },
                        icon: if is_dir { "folder".to_string() } else { get_file_icon(&name) },
                        size,
                        hidden: Some(is_hidden),
                        visible: Some(!is_hidden),
                        children: None,
                    };

                    // Recursively get children for directories (limited depth)
                    if is_dir && depth < 2 {
                        match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await {
                            Ok(children) => {
                                if !children.is_empty() {
                                    node.children = Some(children);
                                }
                            }
                            Err(_) => {} // Continue even if we can't read subdirectories
                        }
                    }

                    nodes.push(node);

                    // Limit entries to prevent UI overload
                    if nodes.len() >= 20 {
                        break;
                    }
                }
            }
        }
        Err(_) => {
            // If we can't read the directory, return placeholder content
            return Ok(create_placeholder_structure(path));
        }
    }

    // Sort nodes: directories first, then by name
    nodes.sort_by(|a, b| match (a.node_type.as_str(), b.node_type.as_str()) {
        ("folder", "file") => std::cmp::Ordering::Less,
        ("file", "folder") => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(nodes)
}

fn create_placeholder_structure(mount_point: &str) -> Vec<FileSystemNode> {
    vec![FileSystemNode {
        name: format!("User Data ({})", mount_point),
        node_type: "folder".to_string(),
        icon: "folder".to_string(),
        size: None,
        hidden: None,
        visible: Some(true),
        children: Some(vec![
            FileSystemNode {
                name: "Documents".to_string(),
                node_type: "folder".to_string(),
                icon: "folder".to_string(),
                size: None,
                hidden: None,
                visible: None,
                children: Some(vec![
                    FileSystemNode {
                        name: "sample.txt".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("100MB".to_string()),
                        hidden: None,
                        visible: None,
                        children: None,
                    },
                    FileSystemNode {
                        name: "report.docx".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("2.5MB".to_string()),
                        hidden: None,
                        visible: None,
                        children: None,
                    },
                ]),
            },
            FileSystemNode {
                name: "Media".to_string(),
                node_type: "folder".to_string(),
                icon: "folder".to_string(),
                size: None,
                hidden: None,
                visible: None,
                children: Some(vec![
                    FileSystemNode {
                        name: "photo1.jpg".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("3.2MB".to_string()),
                        hidden: None,
                        visible: None,
                        children: None,
                    },
                    FileSystemNode {
                        name: "video.mp4".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("250MB".to_string()),
                        hidden: None,
                        visible: None,
                        children: None,
                    },
                ]),
            },
        ]),
    }]
}

fn create_hidden_regions() -> Vec<FileSystemNode> {
    vec![
        FileSystemNode {
            name: "Hidden HPA Region (/mnt/hpa)".to_string(),
            node_type: "folder".to_string(),
            icon: "hidden".to_string(),
            size: Some("5GB".to_string()),
            hidden: Some(true),
            visible: None,
            children: Some(vec![
                FileSystemNode {
                    name: "HiddenBackup".to_string(),
                    node_type: "folder".to_string(),
                    icon: "folder".to_string(),
                    size: None,
                    hidden: Some(true),
                    visible: None,
                    children: Some(vec![
                        FileSystemNode {
                            name: "secret.db".to_string(),
                            node_type: "file".to_string(),
                            icon: "database".to_string(),
                            size: Some("50MB".to_string()),
                            hidden: Some(true),
                            visible: None,
                            children: None,
                        },
                        FileSystemNode {
                            name: "backup.img".to_string(),
                            node_type: "file".to_string(),
                            icon: "file".to_string(),
                            size: Some("2GB".to_string()),
                            hidden: Some(true),
                            visible: None,
                            children: None,
                        },
                        FileSystemNode {
                            name: "keys.bin".to_string(),
                            node_type: "file".to_string(),
                            icon: "file".to_string(),
                            size: Some("1KB".to_string()),
                            hidden: Some(true),
                            visible: None,
                            children: None,
                        },
                    ]),
                },
                FileSystemNode {
                    name: "SystemRestore".to_string(),
                    node_type: "folder".to_string(),
                    icon: "folder".to_string(),
                    size: None,
                    hidden: Some(true),
                    visible: None,
                    children: Some(vec![
                        FileSystemNode {
                            name: "restore_point_1.dat".to_string(),
                            node_type: "file".to_string(),
                            icon: "file".to_string(),
                            size: Some("500MB".to_string()),
                            hidden: Some(true),
                            visible: None,
                            children: None,
                        },
                        FileSystemNode {
                            name: "registry_backup.reg".to_string(),
                            node_type: "file".to_string(),
                            icon: "file".to_string(),
                            size: Some("10MB".to_string()),
                            hidden: Some(true),
                            visible: None,
                            children: None,
                        },
                    ]),
                },
            ]),
        },
        FileSystemNode {
            name: "Hidden DCO Region (/mnt/dco)".to_string(),
            node_type: "folder".to_string(),
            icon: "system".to_string(),
            size: Some("100MB".to_string()),
            hidden: Some(true),
            visible: None,
            children: Some(vec![FileSystemNode {
                name: "ManufacturerData".to_string(),
                node_type: "folder".to_string(),
                icon: "folder".to_string(),
                size: None,
                hidden: Some(true),
                visible: None,
                children: Some(vec![
                    FileSystemNode {
                        name: "config.bin".to_string(),
                        node_type: "file".to_string(),
                        icon: "settings".to_string(),
                        size: Some("20MB".to_string()),
                        hidden: Some(true),
                        visible: None,
                        children: None,
                    },
                    FileSystemNode {
                        name: "firmware.rom".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("8MB".to_string()),
                        hidden: Some(true),
                        visible: None,
                        children: None,
                    },
                    FileSystemNode {
                        name: "diagnostic.log".to_string(),
                        node_type: "file".to_string(),
                        icon: "file".to_string(),
                        size: Some("2MB".to_string()),
                        hidden: Some(true),
                        visible: None,
                        children: None,
                    },
                ]),
            }]),
        },
    ]
}

fn get_file_icon(filename: &str) -> String {
    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "db" | "sqlite" | "sql" => "database".to_string(),
        "exe" | "deb" | "rpm" | "appimage" => "settings".to_string(),
        "txt" | "md" | "json" | "xml" | "yml" | "yaml" | "conf" | "cfg" => "file".to_string(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "file".to_string(),
        "mp4" | "avi" | "mov" | "wmv" | "flv" => "file".to_string(),
        "mp3" | "wav" | "ogg" | "flac" => "file".to_string(),
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => "file".to_string(),
        "docx" | "doc" | "pdf" => "file".to_string(),
        "pptx" | "ppt" => "file".to_string(),
        _ => "file".to_string(),
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{}B", bytes)
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_available_drives,
            get_drive_file_system
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}