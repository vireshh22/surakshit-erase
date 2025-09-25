
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
                    match get_directory_contents(mount_point, 0, true).await {
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
                } else {
                    // For unmounted partitions, try to mount them temporarily
                    match try_mount_partition(&partition_info.name, &partition_info.fstype).await {
                        Ok(mount_point) => {
                            // Successfully mounted, now scan contents
                            match get_directory_contents(&mount_point, 0, true).await {
                                Ok(contents) => {
                                    partition_node.children = Some(contents);
                                }
                                Err(_) => {
                                    partition_node.children = Some(create_placeholder_structure(&mount_point));
                                }
                            }
                            
                            // Add demo hidden regions for demonstration
                            let mut children = partition_node.children.unwrap_or_default();
                            children.extend(create_hidden_regions());
                            partition_node.children = Some(children);
                        }
                        Err(_) => {
                            // If mounting fails, show unmounted structure
                            partition_node.children = Some(create_unmounted_partition_structure(&partition_info.name));
                        }
                    }
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

fn is_cache_or_temp_directory(name: &str) -> bool {
    let cache_dirs = [
        "cache", "tmp", "temp", ".cache", ".tmp", "node_modules", 
        "__pycache__", ".git", ".svn", ".hg", "Cache", "Temp",
        "cache2", "CachedData", "logs", ".logs", "log"
    ];
    
    cache_dirs.iter().any(|&cache_dir| 
        name.to_lowercase().contains(&cache_dir.to_lowercase())
    )
}

#[async_recursion]
async fn get_directory_contents(path: &str, depth: usize, is_root: bool) -> Result<Vec<FileSystemNode>, String> {
    // Remove the depth limit completely - fetch everything
    let mut nodes = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut entry_vec: Vec<_> = entries.collect();
            entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

            for entry in entry_vec {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    // Skip special directories that might cause issues only at root level
                    if is_root && matches!(name.as_str(), "proc" | "sys" | "dev" | "run") && path == "/" {
                        continue;
                    }

                    let metadata = match entry.metadata() {
                        Ok(metadata) => metadata,
                        Err(_) => continue, // Skip files we can't access
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

                    // For directories, always try to get children (no depth limit)
                    if is_dir {
                        // For cache/temp directories, limit the number of entries but still scan
                        let limit_entries = is_cache_or_temp_directory(&name);
                        
                        match get_directory_contents(&entry_path.to_string_lossy(), depth + 1, false).await {
                            Ok(mut children) => {
                                if !children.is_empty() {
                                    // If it's a cache/temp directory, limit to first 10 entries
                                    if limit_entries && children.len() > 10 {
                                        children.truncate(10);
                                        // Add a placeholder to indicate there are more files
                                        children.push(FileSystemNode {
                                            name: format!("... and {} more items", children.len().saturating_sub(10)),
                                            node_type: "placeholder".to_string(),
                                            icon: "info".to_string(),
                                            size: None,
                                            hidden: None,
                                            visible: Some(true),
                                            children: None,
                                        });
                                    }
                                    node.children = Some(children);
                                }
                            }
                            Err(_) => {
                                // If we can't read the directory, add a placeholder
                                node.children = Some(vec![FileSystemNode {
                                    name: "Access denied or permission required".to_string(),
                                    node_type: "error".to_string(),
                                    icon: "warning".to_string(),
                                    size: None,
                                    hidden: None,
                                    visible: Some(true),
                                    children: None,
                                }]);
                            }
                        }
                    }

                    nodes.push(node);

                    // Only limit total entries for the root level or cache directories
                    if (is_root && nodes.len() >= 50) || 
                       (is_cache_or_temp_directory(&std::path::Path::new(path).file_name()
                           .unwrap_or_default().to_string_lossy()) && nodes.len() >= 20) {
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

async fn try_mount_partition(partition_name: &str, fstype: &Option<String>) -> Result<String, String> {
    let device_path = format!("/dev/{}", partition_name);
    
    // First, check if the partition is already mounted somewhere
    let output = Command::new("findmnt")
        .arg("-n")  // No headers
        .arg("-o")  // Output format
        .arg("TARGET")
        .arg(&device_path)
        .output();
    
    if let Ok(output) = output {
        if output.status.success() {
            let mount_point = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !mount_point.is_empty() {
                println!("Partition {} already mounted at: {}", partition_name, mount_point);
                return Ok(mount_point);
            }
        }
    }

    // If not mounted, try to mount it temporarily
    let mount_point = format!("/tmp/surakshit_mount_{}", partition_name);

    // Create mount directory if it doesn't exist
    if let Err(e) = std::fs::create_dir_all(&mount_point) {
        return Err(format!("Failed to create mount directory: {}", e));
    }

    println!("Attempting to mount {} to {}", device_path, mount_point);

    // Try different mounting strategies
    let mount_result = try_mount_with_different_options(&device_path, &mount_point, fstype).await;
    
    match mount_result {
        Ok(_) => {
            println!("Successfully mounted {} at {}", device_path, mount_point);
            Ok(mount_point)
        },
        Err(e) => {
            println!("Failed to mount {}: {}", device_path, e);
            // Clean up the directory if mounting failed
            let _ = std::fs::remove_dir(&mount_point);
            Err(e)
        }
    }
}

async fn try_mount_with_different_options(device_path: &str, mount_point: &str, fstype: &Option<String>) -> Result<(), String> {
    println!("Trying to mount {} with filesystem type: {:?}", device_path, fstype);
    
    // Strategy 1: Try with filesystem type if provided
    if let Some(fs) = fstype {
        if !fs.is_empty() && fs != "-" {
            println!("Strategy 1: Mounting with detected filesystem: {}", fs);
            let output = Command::new("sudo")
                .arg("mount")
                .arg("-t")
                .arg(fs)
                .arg("-o")
                .arg("ro,noatime") // Read-only and no access time updates for safety
                .arg(device_path)
                .arg(mount_point)
                .output()
                .map_err(|e| format!("Command execution failed: {}", e))?;
            
            if output.status.success() {
                println!("Successfully mounted with filesystem type: {}", fs);
                return Ok(());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Failed with detected filesystem {}: {}", fs, stderr);
            }
        }
    }

    // Strategy 2: Try auto-detection with read-only
    println!("Strategy 2: Auto-detection mounting");
    let output = Command::new("sudo")
        .arg("mount")
        .arg("-o")
        .arg("ro,noatime")
        .arg(device_path)
        .arg(mount_point)
        .output()
        .map_err(|e| format!("Command execution failed: {}", e))?;
    
    if output.status.success() {
        println!("Successfully mounted with auto-detection");
        return Ok(());
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Auto-detection failed: {}", stderr);
    }

    // Strategy 3: Try common filesystems
    let common_filesystems = ["ext4", "ext3", "ext2", "ntfs", "vfat", "exfat", "xfs", "btrfs"];
    
    for fs in &common_filesystems {
        println!("Strategy 3: Trying filesystem: {}", fs);
        let output = Command::new("sudo")
            .arg("mount")
            .arg("-t")
            .arg(fs)
            .arg("-o")
            .arg("ro,noatime")
            .arg(device_path)
            .arg(mount_point)
            .output()
            .map_err(|e| format!("Command execution failed: {}", e))?;
        
        if output.status.success() {
            println!("Successfully mounted with filesystem: {}", fs);
            return Ok(());
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Failed with filesystem {}: {}", fs, stderr);
        }
    }

    Err("Failed to mount partition with any supported filesystem".to_string())
}

async fn cleanup_mount_point(mount_point: &str) -> Result<(), String> {
    // Unmount the partition
    let output = Command::new("sudo")
        .arg("umount")
        .arg(mount_point)
        .output()
        .map_err(|e| format!("Failed to execute umount command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Failed to unmount {}: {}", mount_point, stderr);
    }

    // Remove the temporary mount directory
    if let Err(e) = std::fs::remove_dir(mount_point) {
        eprintln!("Warning: Failed to remove mount directory {}: {}", mount_point, e);
    }

    Ok(())
}

// Add a cleanup command for unmounting temporary mounts
#[tauri::command]
async fn cleanup_temporary_mounts() -> Result<(), String> {
    // Find all temporary mount points created by this application
    let tmp_dir = "/tmp";
    if let Ok(entries) = std::fs::read_dir(tmp_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("surakshit_mount_") {
                    let mount_path = entry.path();
                    if let Some(mount_str) = mount_path.to_str() {
                        let _ = cleanup_mount_point(mount_str).await;
                    }
                }
            }
        }
    }
    Ok(())
}

fn create_unmounted_partition_structure(partition_name: &str) -> Vec<FileSystemNode> {
    vec![FileSystemNode {
        name: format!("⚠️ Partition {} - Mount Failed", partition_name),
        node_type: "error".to_string(),
        icon: "warning".to_string(),
        size: None,
        hidden: None,
        visible: Some(true),
        children: Some(vec![
            FileSystemNode {
                name: "Unable to automatically mount this partition".to_string(),
                node_type: "info".to_string(),
                icon: "info".to_string(),
                size: None,
                hidden: None,
                visible: Some(true),
                children: None,
            },
            FileSystemNode {
                name: "Possible reasons:".to_string(),
                node_type: "info".to_string(),
                icon: "info".to_string(),
                size: None,
                hidden: None,
                visible: Some(true),
                children: Some(vec![
                    FileSystemNode {
                        name: "• Unsupported filesystem".to_string(),
                        node_type: "info".to_string(),
                        icon: "info".to_string(),
                        size: None,
                        hidden: None,
                        visible: Some(true),
                        children: None,
                    },
                    FileSystemNode {
                        name: "• Corrupted partition".to_string(),
                        node_type: "info".to_string(),
                        icon: "info".to_string(),
                        size: None,
                        hidden: None,
                        visible: Some(true),
                        children: None,
                    },
                    FileSystemNode {
                        name: "• Permission issues".to_string(),
                        node_type: "info".to_string(),
                        icon: "info".to_string(),
                        size: None,
                        hidden: None,
                        visible: Some(true),
                        children: None,
                    },
                ]),
            },
            FileSystemNode {
                name: format!("Try mounting manually: sudo mount /dev/{} /mnt/{}", partition_name, partition_name),
                node_type: "info".to_string(),
                icon: "info".to_string(),
                size: None,
                hidden: None,
                visible: Some(true),
                children: None,
            }
        ]),
    }]
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
            get_drive_file_system,
            cleanup_temporary_mounts,
            // test_mount_commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}