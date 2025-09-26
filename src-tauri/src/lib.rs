
// // src-tauri/src/main.rs
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use async_recursion::async_recursion;
// use serde::{Deserialize, Serialize};
// use std::fs;
// use std::path::Path;
// use std::process::Command;

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct FileSystemNode {
//     name: String,
//     #[serde(rename = "type")]
//     node_type: String,
//     icon: String,
//     size: Option<String>,
//     hidden: Option<bool>,
//     visible: Option<bool>,
//     children: Option<Vec<FileSystemNode>>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct DriveInfo {
//     name: String,
//     path: String,
//     size: Option<String>,
//     model: Option<String>,
//     vendor: Option<String>,
//     device_type: Option<String>,
//     removable: bool,
//     partitions: Vec<PartitionInfo>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct PartitionInfo {
//     name: String,
//     size: Option<String>,
//     filesystem: Option<String>,
//     mountpoint: Option<String>,
// }

// // Get all available drives in the system
// #[tauri::command]
// async fn get_available_drives() -> Result<Vec<DriveInfo>, String> {
//     let mut drives = Vec::new();

//     // Use lsblk to get all block devices
//     let output = Command::new("lsblk")
//         .arg("-d") // Only show devices, not partitions
//         .arg("-o")
//         .arg("NAME,SIZE,MODEL,VENDOR,TYPE,RM")
//         .arg("-n") // No headers
//         .output()
//         .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     for line in lines {
//         if let Ok(drive) = parse_drive_info(line).await {
//             // Skip loop devices and other virtual devices
//             if !drive.name.starts_with("loop") && 
//                !drive.name.starts_with("sr") &&
//                !drive.name.starts_with("ram") {
//                 drives.push(drive);
//             }
//         }
//     }

//     Ok(drives)
// }

// async fn parse_drive_info(line: &str) -> Result<DriveInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.len() < 6 {
//         return Err("Invalid drive line".to_string());
//     }

//     let name = parts[0].to_string();
//     let size = if parts[1] != "-" { Some(parts[1].to_string()) } else { None };
//     let model = if parts[2] != "-" { Some(parts[2].to_string()) } else { None };
//     let vendor = if parts[3] != "-" { Some(parts[3].to_string()) } else { None };
//     let device_type = if parts[4] != "-" { Some(parts[4].to_string()) } else { None };
//     let removable = parts[5] == "1";

//     // Get partitions for this drive
//     let partitions = get_drive_partitions(&name).await?;

//     Ok(DriveInfo {
//         name: name.clone(),
//         path: format!("/dev/{}", name),
//         size,
//         model,
//         vendor,
//         device_type,
//         removable,
//         partitions,
//     })
// }

// async fn get_drive_partitions(drive_name: &str) -> Result<Vec<PartitionInfo>, String> {
//     let mut partitions = Vec::new();

//     let output = Command::new("lsblk")
//         .arg("-o")
//         .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
//         .arg("-n")
//         .arg(&format!("/dev/{}", drive_name))
//         .output()
//         .map_err(|e| format!("Failed to get partitions: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     // Skip the first line (which is the drive itself)
//     for line in lines.iter().skip(1) {
//         if let Ok(partition) = parse_partition_info(line) {
//             partitions.push(partition);
//         }
//     }

//     Ok(partitions)
// }

// fn parse_partition_info(line: &str) -> Result<PartitionInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.is_empty() {
//         return Err("Invalid partition line".to_string());
//     }

//     let name = parts[0].trim_start_matches("├─").trim_start_matches("└─").to_string();
//     let size = parts.get(1).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });
//     let filesystem = parts.get(2).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });
//     let mountpoint = parts.get(3).and_then(|s| if *s != "-" { Some(s.to_string()) } else { None });

//     Ok(PartitionInfo {
//         name,
//         size,
//         filesystem,
//         mountpoint,
//     })
// }

// // Get file system structure for a specific drive
// #[tauri::command]
// async fn get_drive_file_system(drive_path: String) -> Result<FileSystemNode, String> {
//     let drive_name = drive_path.strip_prefix("/dev/").unwrap_or(&drive_path);
    
//     let mut root = FileSystemNode {
//         name: format!("Drive: {}", drive_name),
//         node_type: "root".to_string(),
//         icon: "root".to_string(),
//         size: None,
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     };

//     // Get drive information
//     let drives = get_drive_contents(&drive_path).await?;
//     root.children = Some(drives);

//     Ok(root)
// }

// async fn get_drive_contents(drive_path: &str) -> Result<Vec<FileSystemNode>, String> {
//     let mut drives = Vec::new();

//     // Get detailed information about the specific drive
//     let output = Command::new("lsblk")
//         .arg("-f")
//         .arg("-o")
//         .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
//         .arg(drive_path)
//         .output()
//         .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         // Parse the drive info
//         let drive_info = parse_drive_line(&lines[1])?;
//         let mut drive_node = create_drive_node(&drive_info.name, &drive_info)?;

//         // Process partitions
//         for line in lines.iter().skip(2) {
//             if let Ok(partition_info) = parse_drive_line(line) {
//                 let mut partition_node = create_partition_node(&partition_info.name, &partition_info)?;

//                 // If partition is mounted, scan its contents
//                 if let Some(mount_point) = &partition_info.mountpoint {
//                     match get_directory_contents(mount_point, 0, true).await {
//                         Ok(contents) => {
//                             partition_node.children = Some(contents);
//                         }
//                         Err(_) => {
//                             // If we can't read, create placeholder structure
//                             partition_node.children = Some(create_placeholder_structure(mount_point));
//                         }
//                     }
                    
//                     // Add demo hidden regions for demonstration
//                     let mut children = partition_node.children.unwrap_or_default();
//                     children.extend(create_hidden_regions());
//                     partition_node.children = Some(children);
//                 } else {
//                     // For unmounted partitions, try to mount them temporarily
//                     match try_mount_partition(&partition_info.name, &partition_info.fstype).await {
//                         Ok(mount_point) => {
//                             // Successfully mounted, now scan contents
//                             match get_directory_contents(&mount_point, 0, true).await {
//                                 Ok(contents) => {
//                                     partition_node.children = Some(contents);
//                                 }
//                                 Err(_) => {
//                                     partition_node.children = Some(create_placeholder_structure(&mount_point));
//                                 }
//                             }
                            
//                             // Add demo hidden regions for demonstration
//                             let mut children = partition_node.children.unwrap_or_default();
//                             children.extend(create_hidden_regions());
//                             partition_node.children = Some(children);
//                         }
//                         Err(_) => {
//                             // If mounting fails, show unmounted structure
//                             partition_node.children = Some(create_unmounted_partition_structure(&partition_info.name));
//                         }
//                     }
//                 }

//                 if let Some(ref mut drive_children) = drive_node.children {
//                     drive_children.push(partition_node);
//                 }
//             }
//         }

//         drives.push(drive_node);
//     }

//     Ok(drives)
// }

// #[derive(Debug)]
// struct DriveLineInfo {
//     name: String,
//     size: Option<String>,
//     fstype: Option<String>,
//     mountpoint: Option<String>,
// }

// fn parse_drive_line(line: &str) -> Result<DriveLineInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.is_empty() {
//         return Err("Invalid drive line".to_string());
//     }

//     // Clean the name from tree characters
//     let name = parts[0].trim_start_matches("├─").trim_start_matches("└─").to_string();
    
//     Ok(DriveLineInfo {
//         name,
//         size: parts.get(1).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//         fstype: parts.get(2).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//         mountpoint: parts.get(3).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//     })
// }

// fn create_drive_node(name: &str, info: &DriveLineInfo) -> Result<FileSystemNode, String> {
//     let display_name = if let Some(size) = &info.size {
//         format!("{} ({} Drive)", name, size)
//     } else {
//         format!("{} (Drive)", name)
//     };

//     Ok(FileSystemNode {
//         name: display_name,
//         node_type: "drive".to_string(),
//         icon: "drive".to_string(),
//         size: info.size.clone(),
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     })
// }

// fn create_partition_node(name: &str, info: &DriveLineInfo) -> Result<FileSystemNode, String> {
//     let display_name = if let Some(size) = &info.size {
//         format!("{} ({})", name, size)
//     } else {
//         name.to_string()
//     };

//     Ok(FileSystemNode {
//         name: display_name,
//         node_type: "partition".to_string(),
//         icon: "partition".to_string(),
//         size: info.size.clone(),
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     })
// }

// fn is_cache_or_temp_directory(name: &str) -> bool {
//     let cache_dirs = [
//         "cache", "tmp", "temp", ".cache", ".tmp", "node_modules", 
//         "__pycache__", ".git", ".svn", ".hg", "Cache", "Temp",
//         "cache2", "CachedData", "logs", ".logs", "log"
//     ];
    
//     cache_dirs.iter().any(|&cache_dir| 
//         name.to_lowercase().contains(&cache_dir.to_lowercase())
//     )
// }

// #[async_recursion]
// async fn get_directory_contents(path: &str, depth: usize, is_root: bool) -> Result<Vec<FileSystemNode>, String> {
//     // Remove the depth limit completely - fetch everything
//     let mut nodes = Vec::new();

//     match fs::read_dir(path) {
//         Ok(entries) => {
//             let mut entry_vec: Vec<_> = entries.collect();
//             entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

//             for entry in entry_vec {
//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     let name = entry.file_name().to_string_lossy().to_string();

//                     // Skip special directories that might cause issues only at root level
//                     if is_root && matches!(name.as_str(), "proc" | "sys" | "dev" | "run") && path == "/" {
//                         continue;
//                     }

//                     let metadata = match entry.metadata() {
//                         Ok(metadata) => metadata,
//                         Err(_) => continue, // Skip files we can't access
//                     };

//                     let is_dir = metadata.is_dir();
//                     let is_hidden = name.starts_with('.');

//                     let size = if !is_dir {
//                         Some(format_bytes(metadata.len()))
//                     } else {
//                         None
//                     };

//                     let mut node = FileSystemNode {
//                         name: name.clone(),
//                         node_type: if is_dir { "folder".to_string() } else { "file".to_string() },
//                         icon: if is_dir { "folder".to_string() } else { get_file_icon(&name) },
//                         size,
//                         hidden: Some(is_hidden),
//                         visible: Some(!is_hidden),
//                         children: None,
//                     };

//                     // For directories, always try to get children (no depth limit)
//                     if is_dir {
//                         // For cache/temp directories, limit the number of entries but still scan
//                         let limit_entries = is_cache_or_temp_directory(&name);
                        
//                         match get_directory_contents(&entry_path.to_string_lossy(), depth + 1, false).await {
//                             Ok(mut children) => {
//                                 if !children.is_empty() {
//                                     // If it's a cache/temp directory, limit to first 10 entries
//                                     if limit_entries && children.len() > 10 {
//                                         children.truncate(10);
//                                         // Add a placeholder to indicate there are more files
//                                         children.push(FileSystemNode {
//                                             name: format!("... and {} more items", children.len().saturating_sub(10)),
//                                             node_type: "placeholder".to_string(),
//                                             icon: "info".to_string(),
//                                             size: None,
//                                             hidden: None,
//                                             visible: Some(true),
//                                             children: None,
//                                         });
//                                     }
//                                     node.children = Some(children);
//                                 }
//                             }
//                             Err(_) => {
//                                 // If we can't read the directory, add a placeholder
//                                 node.children = Some(vec![FileSystemNode {
//                                     name: "Access denied or permission required".to_string(),
//                                     node_type: "error".to_string(),
//                                     icon: "warning".to_string(),
//                                     size: None,
//                                     hidden: None,
//                                     visible: Some(true),
//                                     children: None,
//                                 }]);
//                             }
//                         }
//                     }

//                     nodes.push(node);

//                     // Only limit total entries for the root level or cache directories
//                     if (is_root && nodes.len() >= 50) || 
//                        (is_cache_or_temp_directory(&std::path::Path::new(path).file_name()
//                            .unwrap_or_default().to_string_lossy()) && nodes.len() >= 20) {
//                         break;
//                     }
//                 }
//             }
//         }
//         Err(_) => {
//             // If we can't read the directory, return placeholder content
//             return Ok(create_placeholder_structure(path));
//         }
//     }

//     // Sort nodes: directories first, then by name
//     nodes.sort_by(|a, b| match (a.node_type.as_str(), b.node_type.as_str()) {
//         ("folder", "file") => std::cmp::Ordering::Less,
//         ("file", "folder") => std::cmp::Ordering::Greater,
//         _ => a.name.cmp(&b.name),
//     });

//     Ok(nodes)
// }

// async fn try_mount_partition(partition_name: &str, fstype: &Option<String>) -> Result<String, String> {
//     let device_path = format!("/dev/{}", partition_name);
    
//     // First, check if the partition is already mounted somewhere
//     let output = Command::new("findmnt")
//         .arg("-n")  // No headers
//         .arg("-o")  // Output format
//         .arg("TARGET")
//         .arg(&device_path)
//         .output();
    
//     if let Ok(output) = output {
//         if output.status.success() {
//             let mount_point = String::from_utf8_lossy(&output.stdout).trim().to_string();
//             if !mount_point.is_empty() {
//                 println!("Partition {} already mounted at: {}", partition_name, mount_point);
//                 return Ok(mount_point);
//             }
//         }
//     }

//     // If not mounted, try to mount it temporarily
//     let mount_point = format!("/tmp/surakshit_mount_{}", partition_name);

//     // Create mount directory if it doesn't exist
//     if let Err(e) = std::fs::create_dir_all(&mount_point) {
//         return Err(format!("Failed to create mount directory: {}", e));
//     }

//     println!("Attempting to mount {} to {}", device_path, mount_point);

//     // Try different mounting strategies
//     let mount_result = try_mount_with_different_options(&device_path, &mount_point, fstype).await;
    
//     match mount_result {
//         Ok(_) => {
//             println!("Successfully mounted {} at {}", device_path, mount_point);
//             Ok(mount_point)
//         },
//         Err(e) => {
//             println!("Failed to mount {}: {}", device_path, e);
//             // Clean up the directory if mounting failed
//             let _ = std::fs::remove_dir(&mount_point);
//             Err(e)
//         }
//     }
// }

// async fn try_mount_with_different_options(device_path: &str, mount_point: &str, fstype: &Option<String>) -> Result<(), String> {
//     println!("Trying to mount {} with filesystem type: {:?}", device_path, fstype);
    
//     // Strategy 1: Try with filesystem type if provided
//     if let Some(fs) = fstype {
//         if !fs.is_empty() && fs != "-" {
//             println!("Strategy 1: Mounting with detected filesystem: {}", fs);
//             let output = Command::new("sudo")
//                 .arg("mount")
//                 .arg("-t")
//                 .arg(fs)
//                 .arg("-o")
//                 .arg("ro,noatime") // Read-only and no access time updates for safety
//                 .arg(device_path)
//                 .arg(mount_point)
//                 .output()
//                 .map_err(|e| format!("Command execution failed: {}", e))?;
            
//             if output.status.success() {
//                 println!("Successfully mounted with filesystem type: {}", fs);
//                 return Ok(());
//             } else {
//                 let stderr = String::from_utf8_lossy(&output.stderr);
//                 println!("Failed with detected filesystem {}: {}", fs, stderr);
//             }
//         }
//     }

//     // Strategy 2: Try auto-detection with read-only
//     println!("Strategy 2: Auto-detection mounting");
//     let output = Command::new("sudo")
//         .arg("mount")
//         .arg("-o")
//         .arg("ro,noatime")
//         .arg(device_path)
//         .arg(mount_point)
//         .output()
//         .map_err(|e| format!("Command execution failed: {}", e))?;
    
//     if output.status.success() {
//         println!("Successfully mounted with auto-detection");
//         return Ok(());
//     } else {
//         let stderr = String::from_utf8_lossy(&output.stderr);
//         println!("Auto-detection failed: {}", stderr);
//     }

//     // Strategy 3: Try common filesystems
//     let common_filesystems = ["ext4", "ext3", "ext2", "ntfs", "vfat", "exfat", "xfs", "btrfs"];
    
//     for fs in &common_filesystems {
//         println!("Strategy 3: Trying filesystem: {}", fs);
//         let output = Command::new("sudo")
//             .arg("mount")
//             .arg("-t")
//             .arg(fs)
//             .arg("-o")
//             .arg("ro,noatime")
//             .arg(device_path)
//             .arg(mount_point)
//             .output()
//             .map_err(|e| format!("Command execution failed: {}", e))?;
        
//         if output.status.success() {
//             println!("Successfully mounted with filesystem: {}", fs);
//             return Ok(());
//         } else {
//             let stderr = String::from_utf8_lossy(&output.stderr);
//             println!("Failed with filesystem {}: {}", fs, stderr);
//         }
//     }

//     Err("Failed to mount partition with any supported filesystem".to_string())
// }

// async fn cleanup_mount_point(mount_point: &str) -> Result<(), String> {
//     // Unmount the partition
//     let output = Command::new("sudo")
//         .arg("umount")
//         .arg(mount_point)
//         .output()
//         .map_err(|e| format!("Failed to execute umount command: {}", e))?;

//     if !output.status.success() {
//         let stderr = String::from_utf8_lossy(&output.stderr);
//         eprintln!("Warning: Failed to unmount {}: {}", mount_point, stderr);
//     }

//     // Remove the temporary mount directory
//     if let Err(e) = std::fs::remove_dir(mount_point) {
//         eprintln!("Warning: Failed to remove mount directory {}: {}", mount_point, e);
//     }

//     Ok(())
// }

// // Add a cleanup command for unmounting temporary mounts
// #[tauri::command]
// async fn cleanup_temporary_mounts() -> Result<(), String> {
//     // Find all temporary mount points created by this application
//     let tmp_dir = "/tmp";
//     if let Ok(entries) = std::fs::read_dir(tmp_dir) {
//         for entry in entries {
//             if let Ok(entry) = entry {
//                 let name = entry.file_name();
//                 let name_str = name.to_string_lossy();
//                 if name_str.starts_with("surakshit_mount_") {
//                     let mount_path = entry.path();
//                     if let Some(mount_str) = mount_path.to_str() {
//                         let _ = cleanup_mount_point(mount_str).await;
//                     }
//                 }
//             }
//         }
//     }
//     Ok(())
// }

// fn create_unmounted_partition_structure(partition_name: &str) -> Vec<FileSystemNode> {
//     vec![FileSystemNode {
//         name: format!("⚠️ Partition {} - Mount Failed", partition_name),
//         node_type: "error".to_string(),
//         icon: "warning".to_string(),
//         size: None,
//         hidden: None,
//         visible: Some(true),
//         children: Some(vec![
//             FileSystemNode {
//                 name: "Unable to automatically mount this partition".to_string(),
//                 node_type: "info".to_string(),
//                 icon: "info".to_string(),
//                 size: None,
//                 hidden: None,
//                 visible: Some(true),
//                 children: None,
//             },
//             FileSystemNode {
//                 name: "Possible reasons:".to_string(),
//                 node_type: "info".to_string(),
//                 icon: "info".to_string(),
//                 size: None,
//                 hidden: None,
//                 visible: Some(true),
//                 children: Some(vec![
//                     FileSystemNode {
//                         name: "• Unsupported filesystem".to_string(),
//                         node_type: "info".to_string(),
//                         icon: "info".to_string(),
//                         size: None,
//                         hidden: None,
//                         visible: Some(true),
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "• Corrupted partition".to_string(),
//                         node_type: "info".to_string(),
//                         icon: "info".to_string(),
//                         size: None,
//                         hidden: None,
//                         visible: Some(true),
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "• Permission issues".to_string(),
//                         node_type: "info".to_string(),
//                         icon: "info".to_string(),
//                         size: None,
//                         hidden: None,
//                         visible: Some(true),
//                         children: None,
//                     },
//                 ]),
//             },
//             FileSystemNode {
//                 name: format!("Try mounting manually: sudo mount /dev/{} /mnt/{}", partition_name, partition_name),
//                 node_type: "info".to_string(),
//                 icon: "info".to_string(),
//                 size: None,
//                 hidden: None,
//                 visible: Some(true),
//                 children: None,
//             }
//         ]),
//     }]
// }

// fn create_placeholder_structure(mount_point: &str) -> Vec<FileSystemNode> {
//     vec![FileSystemNode {
//         name: format!("User Data ({})", mount_point),
//         node_type: "folder".to_string(),
//         icon: "folder".to_string(),
//         size: None,
//         hidden: None,
//         visible: Some(true),
//         children: Some(vec![
//             FileSystemNode {
//                 name: "Documents".to_string(),
//                 node_type: "folder".to_string(),
//                 icon: "folder".to_string(),
//                 size: None,
//                 hidden: None,
//                 visible: None,
//                 children: Some(vec![
//                     FileSystemNode {
//                         name: "sample.txt".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("100MB".to_string()),
//                         hidden: None,
//                         visible: None,
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "report.docx".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("2.5MB".to_string()),
//                         hidden: None,
//                         visible: None,
//                         children: None,
//                     },
//                 ]),
//             },
//             FileSystemNode {
//                 name: "Media".to_string(),
//                 node_type: "folder".to_string(),
//                 icon: "folder".to_string(),
//                 size: None,
//                 hidden: None,
//                 visible: None,
//                 children: Some(vec![
//                     FileSystemNode {
//                         name: "photo1.jpg".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("3.2MB".to_string()),
//                         hidden: None,
//                         visible: None,
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "video.mp4".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("250MB".to_string()),
//                         hidden: None,
//                         visible: None,
//                         children: None,
//                     },
//                 ]),
//             },
//         ]),
//     }]
// }

// fn create_hidden_regions() -> Vec<FileSystemNode> {
//     vec![
//         FileSystemNode {
//             name: "Hidden HPA Region (/mnt/hpa)".to_string(),
//             node_type: "folder".to_string(),
//             icon: "hidden".to_string(),
//             size: Some("5GB".to_string()),
//             hidden: Some(true),
//             visible: None,
//             children: Some(vec![
//                 FileSystemNode {
//                     name: "HiddenBackup".to_string(),
//                     node_type: "folder".to_string(),
//                     icon: "folder".to_string(),
//                     size: None,
//                     hidden: Some(true),
//                     visible: None,
//                     children: Some(vec![
//                         FileSystemNode {
//                             name: "secret.db".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "database".to_string(),
//                             size: Some("50MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "backup.img".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("2GB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "keys.bin".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("1KB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                     ]),
//                 },
//                 FileSystemNode {
//                     name: "SystemRestore".to_string(),
//                     node_type: "folder".to_string(),
//                     icon: "folder".to_string(),
//                     size: None,
//                     hidden: Some(true),
//                     visible: None,
//                     children: Some(vec![
//                         FileSystemNode {
//                             name: "restore_point_1.dat".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("500MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "registry_backup.reg".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("10MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                     ]),
//                 },
//             ]),
//         },
//         FileSystemNode {
//             name: "Hidden DCO Region (/mnt/dco)".to_string(),
//             node_type: "folder".to_string(),
//             icon: "system".to_string(),
//             size: Some("100MB".to_string()),
//             hidden: Some(true),
//             visible: None,
//             children: Some(vec![FileSystemNode {
//                 name: "ManufacturerData".to_string(),
//                 node_type: "folder".to_string(),
//                 icon: "folder".to_string(),
//                 size: None,
//                 hidden: Some(true),
//                 visible: None,
//                 children: Some(vec![
//                     FileSystemNode {
//                         name: "config.bin".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "settings".to_string(),
//                         size: Some("20MB".to_string()),
//                         hidden: Some(true),
//                         visible: None,
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "firmware.rom".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("8MB".to_string()),
//                         hidden: Some(true),
//                         visible: None,
//                         children: None,
//                     },
//                     FileSystemNode {
//                         name: "diagnostic.log".to_string(),
//                         node_type: "file".to_string(),
//                         icon: "file".to_string(),
//                         size: Some("2MB".to_string()),
//                         hidden: Some(true),
//                         visible: None,
//                         children: None,
//                     },
//                 ]),
//             }]),
//         },
//     ]
// }

// fn get_file_icon(filename: &str) -> String {
//     let extension = Path::new(filename)
//         .extension()
//         .and_then(|ext| ext.to_str())
//         .unwrap_or("")
//         .to_lowercase();

//     match extension.as_str() {
//         "db" | "sqlite" | "sql" => "database".to_string(),
//         "exe" | "deb" | "rpm" | "appimage" => "settings".to_string(),
//         "txt" | "md" | "json" | "xml" | "yml" | "yaml" | "conf" | "cfg" => "file".to_string(),
//         "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "file".to_string(),
//         "mp4" | "avi" | "mov" | "wmv" | "flv" => "file".to_string(),
//         "mp3" | "wav" | "ogg" | "flac" => "file".to_string(),
//         "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => "file".to_string(),
//         "docx" | "doc" | "pdf" => "file".to_string(),
//         "pptx" | "ppt" => "file".to_string(),
//         _ => "file".to_string(),
//     }
// }

// fn format_bytes(bytes: u64) -> String {
//     const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
//     let mut size = bytes as f64;
//     let mut unit_index = 0;

//     while size >= 1024.0 && unit_index < UNITS.len() - 1 {
//         size /= 1024.0;
//         unit_index += 1;
//     }

//     if unit_index == 0 {
//         format!("{}B", bytes)
//     } else {
//         format!("{:.1}{}", size, UNITS[unit_index])
//     }
// }

// // Test function to check if mounting commands work
// #[tauri::command]
// async fn test_mount_commands() -> Result<String, String> {
//     let mut results = Vec::new();
    
//     // Test sudo access
//     let output = Command::new("sudo")
//         .arg("-n")  // Non-interactive
//         .arg("whoami")
//         .output()
//         .map_err(|e| format!("Failed to test sudo: {}", e))?;
    
//     if output.status.success() {
//         results.push("✓ Sudo access working".to_string());
//     } else {
//         results.push("✗ Sudo access failed".to_string());
//         let stderr = String::from_utf8_lossy(&output.stderr);
//         results.push(format!("Sudo error: {}", stderr));
//     }
    
//     // Test findmnt command
//     let output = Command::new("findmnt")
//         .arg("--version")
//         .output();
    
//     match output {
//         Ok(output) if output.status.success() => {
//             results.push("✓ findmnt command available".to_string());
//         }
//         _ => {
//             results.push("✗ findmnt command not available".to_string());
//         }
//     }
    
//     // Test mount command availability
//     let output = Command::new("mount")
//         .arg("--version")
//         .output();
    
//     match output {
//         Ok(output) if output.status.success() => {
//             results.push("✓ mount command available".to_string());
//         }
//         _ => {
//             results.push("✗ mount command not available".to_string());
//         }
//     }
    
//     // Test creating temporary directory
//     let test_dir = "/tmp/surakshit_test";
//     match std::fs::create_dir_all(test_dir) {
//         Ok(_) => {
//             results.push("✓ Can create temporary directories".to_string());
//             let _ = std::fs::remove_dir(test_dir);
//         }
//         Err(e) => {
//             results.push(format!("✗ Cannot create temporary directories: {}", e));
//         }
//     }
    
//     Ok(results.join("\n"))
// }

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![
//             get_available_drives,
//             get_drive_file_system,
//             cleanup_temporary_mounts,
//             test_mount_commands
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


// src-tauri/src/lib.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::os::unix::fs::PermissionsExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, Manager};


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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeProgress {
    current_file: String,
    current_path: String,
    file_size: Option<String>,
    files_processed: u64,
    total_size_processed: u64,
    percentage: f64,
    stage: String,
    is_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeResult {
    success: bool,
    files_erased: u64,
    total_size: String,
    duration_seconds: u64,
    start_time: String,
    end_time: String,
    method: String,
    drive_info: DriveInfo,
}

// Global flag to control wipe process
static WIPE_CANCELLED: AtomicBool = AtomicBool::new(false);

#[tauri::command]
async fn get_available_drives() -> Result<Vec<DriveInfo>, String> {
    let mut drives = Vec::new();

    let output = Command::new("lsblk")
        .arg("-d")
        .arg("-o")
        .arg("NAME,SIZE,MODEL,VENDOR,TYPE,RM")
        .arg("-n")
        .output()
        .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    for line in lines {
        if let Ok(drive) = parse_drive_info(line).await {
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

    let drives = get_drive_contents(&drive_path).await?;
    root.children = Some(drives);

    Ok(root)
}

#[tauri::command]
async fn start_wipe_operation(
    drive_path: String,
    method: String,
    app: tauri::AppHandle,
) -> Result<WipeResult, String> {
    println!("Starting wipe operation for drive: {} with method: {}", drive_path, method);
    
    // Reset cancellation flag
    WIPE_CANCELLED.store(false, Ordering::Relaxed);
    
    let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let start_time_str = format_timestamp(start_time);
    
    // Get drive information
    let drive_info = get_drive_info_by_path(&drive_path).await?;
    
    // Get all mounted partitions for this drive
    let mount_points = get_mounted_partitions(&drive_path).await?;
    
    let mut total_files_processed = 0u64;
    let mut total_size_processed = 0u64;
    
    // Process each mounted partition
    for mount_point in mount_points {
        println!("Processing mount point: {}", mount_point);
        
        let result = delete_files_recursive(
            &mount_point,
            &method,
            &app,
            &mut total_files_processed,
            &mut total_size_processed,
        ).await?;
        
        if WIPE_CANCELLED.load(Ordering::Relaxed) {
            return Err("Wipe operation was cancelled".to_string());
        }
    }
    
    let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let end_time_str = format_timestamp(end_time);
    let duration = end_time - start_time;
    
    // Send completion event
    let _ = app.emit("wipe-complete", ());
    
    Ok(WipeResult {
        success: true,
        files_erased: total_files_processed,
        total_size: format_bytes(total_size_processed),
        duration_seconds: duration,
        start_time: start_time_str,
        end_time: end_time_str,
        method,
        drive_info,
    })
}

#[tauri::command]
async fn cancel_wipe_operation() -> Result<(), String> {
    println!("Cancelling wipe operation");
    WIPE_CANCELLED.store(true, Ordering::Relaxed);
    Ok(())
}

async fn cleanup_mount_point(mount_point: &str) -> Result<(), String> {
    let output = Command::new("sudo")
        .arg("umount")
        .arg(mount_point)
        .output()
        .map_err(|e| format!("Failed to execute umount command: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Failed to unmount {}: {}", mount_point, stderr);
    }

    if let Err(e) = std::fs::remove_dir(mount_point) {
        eprintln!("Warning: Failed to remove mount directory {}: {}", mount_point, e);
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
            // FileSystemNode {
            //     name: "Documents".to_string(),
            //     node_type: "folder".to_string(),
            //     icon: "folder".to_string(),
            //     size: None,
            //     hidden: None,
            //     visible: None,
            //     children: Some(vec![
            //         FileSystemNode {
            //             name: "sample.txt".to_string(),
            //             node_type: "file".to_string(),
            //             icon: "file".to_string(),
            //             size: Some("100MB".to_string()),
            //             hidden: None,
            //             visible: None,
            //             children: None,
            //         },
            //     ]),
            // },
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
                    ]),
                },
            ]),
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

#[tauri::command]
async fn test_mount_commands() -> Result<String, String> {
    let mut results = Vec::new();
    
    let output = Command::new("sudo")
        .arg("-n")
        .arg("whoami")
        .output()
        .map_err(|e| format!("Failed to test sudo: {}", e))?;
    
    if output.status.success() {
        results.push("✓ Sudo access working".to_string());
    } else {
        results.push("✗ Sudo access failed".to_string());
    }
    
    Ok(results.join("\n"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_available_drives,
            get_drive_file_system,
            start_wipe_operation,
            cancel_wipe_operation,
            cleanup_temporary_mounts,
            test_mount_commands,
            check_sudo_access
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn get_drive_info_by_path(drive_path: &str) -> Result<DriveInfo, String> {
    let drives = get_available_drives().await?;
    
    for drive in drives {
        if drive.path == drive_path {
            return Ok(drive);
        }
    }
    
    Err("Drive not found".to_string())
}

async fn get_mounted_partitions(drive_path: &str) -> Result<Vec<String>, String> {
    let mut mount_points = Vec::new();
    let drive_name = drive_path.strip_prefix("/dev/").unwrap_or(drive_path);
    
    // Get partition information
    let output = Command::new("lsblk")
        .arg("-o")
        .arg("NAME,MOUNTPOINT")
        .arg("-n")
        .arg(drive_path)
        .output()
        .map_err(|e| format!("Failed to get mount points: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    for line in stdout.lines().skip(1) { // Skip the drive itself
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] != "-" {
            mount_points.push(parts[1].to_string());
        }
    }
    
    // If no mounted partitions found, try to mount them
    if mount_points.is_empty() {
        let partitions = get_drive_partitions(drive_name).await?;
        for partition in partitions {
            if let Ok(mount_point) = try_mount_partition(&partition.name, &partition.filesystem).await {
                mount_points.push(mount_point);
            }
        }
    }
    
    Ok(mount_points)
}

// #[async_recursion]
// async fn delete_files_recursive(
//     path: &str,
//     method: &str,
//     app: &tauri::AppHandle,
//     files_processed: &mut u64,
//     total_size: &mut u64,
// ) -> Result<(), String> {
//     if WIPE_CANCELLED.load(Ordering::Relaxed) {
//         return Err("Operation cancelled".to_string());
//     }

//     match fs::read_dir(path) {
//         Ok(entries) => {
//             for entry in entries {
//                 if WIPE_CANCELLED.load(Ordering::Relaxed) {
//                     return Err("Operation cancelled".to_string());
//                 }

//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     let path_str = entry_path.to_string_lossy().to_string();
//                     let file_name = entry.file_name().to_string_lossy().to_string();

//                     // Skip special directories
//                     if matches!(file_name.as_str(), "." | ".." | "proc" | "sys" | "dev" | "run") {
//                         continue;
//                     }

//                     match entry.metadata() {
//                         Ok(metadata) => {
//                             let file_size = metadata.len();
//                             let formatted_size = format_bytes(file_size);
                            
//                             if metadata.is_dir() {
//                                 // Send progress update for directory
//                                 let progress = WipeProgress {
//                                     current_file: file_name.clone(),
//                                     current_path: path_str.clone(),
//                                     file_size: Some(formatted_size.clone()),
//                                     files_processed: *files_processed,
//                                     total_size_processed: *total_size,
//                                     percentage: 0.0, // Will be calculated in frontend
//                                     stage: format!("Processing directory: {}", file_name),
//                                     is_complete: false,
//                                 };
                                
//                                 let _ = app.emit("wipe-progress", &progress);
                                
//                                 // Recursively delete directory contents
//                                 delete_files_recursive(
//                                     &path_str,
//                                     method,
//                                     app,
//                                     files_processed,
//                                     total_size,
//                                 ).await?;
                                
//                                 // Delete the empty directory
//                                 match delete_single_item(&path_str, true).await {
//                                     Ok(_) => {
//                                         *files_processed += 1;
//                                         println!("Deleted directory: {}", path_str);
//                                     }
//                                     Err(e) => {
//                                         println!("Failed to delete directory {}: {}", path_str, e);
//                                     }
//                                 }
//                             } else {
//                                 // Send progress update for file
//                                 let progress = WipeProgress {
//                                     current_file: file_name.clone(),
//                                     current_path: path_str.clone(),
//                                     file_size: Some(formatted_size.clone()),
//                                     files_processed: *files_processed,
//                                     total_size_processed: *total_size,
//                                     percentage: 0.0,
//                                     stage: format!("Deleting file: {}", file_name),
//                                     is_complete: false,
//                                 };
                                
//                                 let _ = app.emit("wipe-progress", &progress);
                                
//                                 // Delete the file
//                                 match delete_single_item(&path_str, false).await {
//                                     Ok(_) => {
//                                         *files_processed += 1;
//                                         *total_size += file_size;
//                                         println!("Deleted file: {} ({})", path_str, formatted_size);
//                                     }
//                                     Err(e) => {
//                                         println!("Failed to delete file {}: {}", path_str, e);
//                                     }
//                                 }
//                             }
                            
//                             // Small delay to make progress visible
//                             tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
//                         }
//                         Err(e) => {
//                             println!("Cannot read metadata for {}: {}", path_str, e);
//                         }
//                     }
//                 }
//             }
//         }
//         Err(e) => {
//             println!("Cannot read directory {}: {}", path, e);
//         }
//     }

//     Ok(())
// }

// async fn delete_single_item(path: &str, is_dir: bool) -> Result<(), String> {
//     // First, try to change permissions if needed
//     if let Err(_) = make_writable(path).await {
//         println!("Warning: Could not change permissions for {}", path);
//     }

//     if is_dir {
//         fs::remove_dir(path)
//             .map_err(|e| format!("Failed to remove directory {}: {}", path, e))
//     } else {
//         fs::remove_file(path)
//             .map_err(|e| format!("Failed to remove file {}: {}", path, e))
//     }
// }

// async fn make_writable(path: &str) -> Result<(), String> {
//     match fs::metadata(path) {
//         Ok(metadata) => {
//             let mut permissions = metadata.permissions();
//             // Add write permission for owner
//             permissions.set_mode(permissions.mode() | 0o200);
//             fs::set_permissions(path, permissions)
//                 .map_err(|e| format!("Failed to set permissions: {}", e))
//         }
//         Err(e) => Err(format!("Failed to get metadata: {}", e))
//     }
// }

#[async_recursion]
async fn delete_files_recursive(
    path: &str,
    method: &str,
    app: &tauri::AppHandle,
    files_processed: &mut u64,
    total_size: &mut u64,
) -> Result<(), String> {
    if WIPE_CANCELLED.load(Ordering::Relaxed) {
        return Err("Operation cancelled".to_string());
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if WIPE_CANCELLED.load(Ordering::Relaxed) {
                    return Err("Operation cancelled".to_string());
                }

                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    let path_str = entry_path.to_string_lossy().to_string();
                    let file_name = entry.file_name().to_string_lossy().to_string();

                    // Skip special directories and system mount points
                    if matches!(file_name.as_str(), "." | ".." | "proc" | "sys" | "dev" | "run" | "boot" | "etc") {
                        continue;
                    }

                    match entry.metadata() {
                        Ok(metadata) => {
                            let file_size = metadata.len();
                            let formatted_size = format_bytes(file_size);
                            
                            if metadata.is_dir() {
                                // Send progress update for directory
                                let progress = WipeProgress {
                                    current_file: file_name.clone(),
                                    current_path: path_str.clone(),
                                    file_size: Some(formatted_size.clone()),
                                    files_processed: *files_processed,
                                    total_size_processed: *total_size,
                                    percentage: 0.0,
                                    stage: format!("Processing directory: {}", file_name),
                                    is_complete: false,
                                };
                                
                                let _ = app.emit("wipe-progress", &progress);
                                
                                // Recursively delete directory contents first
                                delete_files_recursive(
                                    &path_str,
                                    method,
                                    app,
                                    files_processed,
                                    total_size,
                                ).await?;
                                
                                // Delete the empty directory using system commands
                                match delete_directory_with_commands(&path_str).await {
                                    Ok(_) => {
                                        *files_processed += 1;
                                        println!("Deleted directory: {}", path_str);
                                    }
                                    Err(e) => {
                                        println!("Failed to delete directory {}: {}", path_str, e);
                                    }
                                }
                            } else {
                                // Send progress update for file
                                let progress = WipeProgress {
                                    current_file: file_name.clone(),
                                    current_path: path_str.clone(),
                                    file_size: Some(formatted_size.clone()),
                                    files_processed: *files_processed,
                                    total_size_processed: *total_size,
                                    percentage: 0.0,
                                    stage: format!("Deleting file: {}", file_name),
                                    is_complete: false,
                                };
                                
                                let _ = app.emit("wipe-progress", &progress);
                                
                                // Delete the file using system commands
                                match delete_file_with_commands(&path_str, method).await {
                                    Ok(_) => {
                                        *files_processed += 1;
                                        *total_size += file_size;
                                        println!("Deleted file: {} ({})", path_str, formatted_size);
                                    }
                                    Err(e) => {
                                        println!("Failed to delete file {}: {}", path_str, e);
                                    }
                                }
                            }
                            
                            // Small delay to make progress visible and avoid overwhelming the system
                            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        }
                        Err(e) => {
                            println!("Cannot read metadata for {}: {}", path_str, e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Cannot read directory {}: {}", path, e);
        }
    }

    Ok(())
}

async fn delete_file_with_commands(file_path: &str, method: &str) -> Result<(), String> {
    // First, make the file writable using chmod
    let chmod_result = Command::new("sudo")
        .arg("chmod")
        .arg("666") // Give read/write permissions to everyone
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to execute chmod: {}", e))?;
    
    if !chmod_result.status.success() {
        let stderr = String::from_utf8_lossy(&chmod_result.stderr);
        println!("Warning: chmod failed for {}: {}", file_path, stderr);
        // Continue anyway - might still be able to delete
    }

    // Perform different deletion methods based on the selected method
    match method {
        "Clear" => {
            // Simple deletion
            delete_file_simple(file_path).await
        }
        "Purge" => {
            // Overwrite with random data before deletion
            overwrite_and_delete(file_path, 3).await
        }
        "Destroy" => {
            // Military-grade deletion with multiple passes
            overwrite_and_delete(file_path, 7).await
        }
        _ => {
            // Default to simple deletion
            delete_file_simple(file_path).await
        }
    }
}

async fn delete_directory_with_commands(dir_path: &str) -> Result<(), String> {
    // Make directory writable first
    let chmod_result = Command::new("sudo")
        .arg("chmod")
        .arg("755") // Give full permissions to owner, read/execute to others
        .arg(dir_path)
        .output()
        .map_err(|e| format!("Failed to execute chmod on directory: {}", e))?;
    
    if !chmod_result.status.success() {
        let stderr = String::from_utf8_lossy(&chmod_result.stderr);
        println!("Warning: chmod failed for directory {}: {}", dir_path, stderr);
    }

    // Remove the directory using rm command
    let rm_result = Command::new("sudo")
        .arg("rmdir")
        .arg(dir_path)
        .output()
        .map_err(|e| format!("Failed to execute rmdir: {}", e))?;
    
    if rm_result.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&rm_result.stderr);
        Err(format!("Failed to remove directory {}: {}", dir_path, stderr))
    }
}

async fn delete_file_simple(file_path: &str) -> Result<(), String> {
    let rm_result = Command::new("sudo")
        .arg("rm")
        .arg("-f") // Force removal, ignore nonexistent files
        .arg(file_path)
        .output()
        .map_err(|e| format!("Failed to execute rm: {}", e))?;
    
    if rm_result.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&rm_result.stderr);
        Err(format!("Failed to remove file {}: {}", file_path, stderr))
    }
}

async fn overwrite_and_delete(file_path: &str, passes: u32) -> Result<(), String> {
    // Get file size first
    let file_size = match fs::metadata(file_path) {
        Ok(metadata) => metadata.len(),
        Err(_) => 1024, // Default size if we can't read metadata
    };

    // Perform multiple overwrite passes using dd command
    for pass in 1..=passes {
        println!("Overwriting {} - Pass {}/{}", file_path, pass, passes);
        
        let pattern = match pass % 3 {
            0 => "random", // Random data
            1 => "zero",   // All zeros
            _ => "ones",   // All ones (0xFF)
        };

        let dd_result = match pattern {
            "random" => {
                Command::new("sudo")
                    .arg("dd")
                    .arg("if=/dev/urandom")
                    .arg(&format!("of={}", file_path))
                    .arg(&format!("bs=1024"))
                    .arg(&format!("count={}", (file_size / 1024) + 1))
                    .arg("conv=notrunc") // Don't truncate the file
                    .output()
            }
            "zero" => {
                Command::new("sudo")
                    .arg("dd")
                    .arg("if=/dev/zero")
                    .arg(&format!("of={}", file_path))
                    .arg(&format!("bs=1024"))
                    .arg(&format!("count={}", (file_size / 1024) + 1))
                    .arg("conv=notrunc")
                    .output()
            }
            _ => { // ones
                // Create a temporary file with all 0xFF bytes, then use it
                Command::new("sudo")
                    .arg("sh")
                    .arg("-c")
                    .arg(&format!("yes $'\\xff' | tr -d '\\n' | dd of={} bs=1024 count={} conv=notrunc 2>/dev/null || true", 
                          file_path, (file_size / 1024) + 1))
                    .output()
            }
        };

        match dd_result {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("Warning: overwrite pass {} failed for {}: {}", pass, file_path, stderr);
                }
            }
            Err(e) => {
                println!("Warning: failed to execute overwrite command: {}", e);
            }
        }

        // Add a small delay between passes
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Check if operation was cancelled
        if WIPE_CANCELLED.load(Ordering::Relaxed) {
            return Err("Operation cancelled during overwrite".to_string());
        }
    }

    // After overwriting, delete the file
    delete_file_simple(file_path).await
}

// Enhanced function to handle different file systems and permission issues
async fn force_delete_with_attributes(file_path: &str) -> Result<(), String> {
    // Try to remove any special attributes that might prevent deletion
    let chattr_result = Command::new("sudo")
        .arg("chattr")
        .arg("-i") // Remove immutable attribute
        .arg("-a") // Remove append-only attribute
        .arg(file_path)
        .output();
    
    // Don't treat chattr failures as errors since not all filesystems support it
    if let Ok(output) = chattr_result {
        if !output.status.success() {
            println!("Note: chattr command failed (this is normal on some filesystems)");
        }
    }

    // Try to remove extended attributes
    let setfattr_result = Command::new("sudo")
        .arg("setfattr")
        .arg("-x")
        .arg("user.dosattrib") // Remove DOS attributes if present
        .arg(file_path)
        .output();
    
    // Again, don't treat this as an error
    if let Ok(output) = setfattr_result {
        if !output.status.success() {
            println!("Note: setfattr command failed (this is normal if no extended attributes exist)");
        }
    }

    // Now try to delete
    delete_file_simple(file_path).await
}

// Add this helper function to check if we have sudo access
#[tauri::command]
async fn check_sudo_access() -> Result<bool, String> {
    let output = Command::new("sudo")
        .arg("-n") // Non-interactive
        .arg("true") // Simple command that always succeeds
        .output()
        .map_err(|e| format!("Failed to check sudo access: {}", e))?;
    
    Ok(output.status.success())
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{UNIX_EPOCH, Duration};
    let dt = UNIX_EPOCH + Duration::from_secs(timestamp);
    format!("{:?}", dt) // Simple formatting, you can use chrono for better formatting
}

// Include all the existing helper functions from your original code...
async fn get_drive_contents(drive_path: &str) -> Result<Vec<FileSystemNode>, String> {
    let mut drives = Vec::new();

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
        let drive_info = parse_drive_line(&lines[1])?;
        let mut drive_node = create_drive_node(&drive_info.name, &drive_info)?;

        for line in lines.iter().skip(2) {
            if let Ok(partition_info) = parse_drive_line(line) {
                let mut partition_node = create_partition_node(&partition_info.name, &partition_info)?;

                if let Some(mount_point) = &partition_info.mountpoint {
                    match get_directory_contents(mount_point, 0, true).await {
                        Ok(contents) => {
                            partition_node.children = Some(contents);
                        }
                        Err(_) => {
                            partition_node.children = Some(create_placeholder_structure(mount_point));
                        }
                    }
                    
                    let mut children = partition_node.children.unwrap_or_default();
                    // children.extend(create_hidden_regions());
                    partition_node.children = Some(children);
                } else {
                    match try_mount_partition(&partition_info.name, &partition_info.fstype).await {
                        Ok(mount_point) => {
                            match get_directory_contents(&mount_point, 0, true).await {
                                Ok(contents) => {
                                    partition_node.children = Some(contents);
                                }
                                Err(_) => {
                                    partition_node.children = Some(create_placeholder_structure(&mount_point));
                                }
                            }
                            
                            let mut children = partition_node.children.unwrap_or_default();
                            // children.extend(create_hidden_regions());
                            partition_node.children = Some(children);
                        }
                        Err(_) => {
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

// Keep all your existing helper functions...
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
    let mut nodes = Vec::new();

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut entry_vec: Vec<_> = entries.collect();
            entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

            for entry in entry_vec {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    if is_root && matches!(name.as_str(), "proc" | "sys" | "dev" | "run") && path == "/" {
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

                    if is_dir {
                        let limit_entries = is_cache_or_temp_directory(&name);
                        
                        match get_directory_contents(&entry_path.to_string_lossy(), depth + 1, false).await {
                            Ok(mut children) => {
                                if !children.is_empty() {
                                    if limit_entries && children.len() > 10 {
                                        children.truncate(10);
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

                    if (is_root && nodes.len() >= 50) || 
                       (is_cache_or_temp_directory(&std::path::Path::new(path).file_name()
                           .unwrap_or_default().to_string_lossy()) && nodes.len() >= 20) {
                        break;
                    }
                }
            }
        }
        Err(_) => {
            return Ok(create_placeholder_structure(path));
        }
    }

    nodes.sort_by(|a, b| match (a.node_type.as_str(), b.node_type.as_str()) {
        ("folder", "file") => std::cmp::Ordering::Less,
        ("file", "folder") => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(nodes)
}

async fn try_mount_partition(partition_name: &str, fstype: &Option<String>) -> Result<String, String> {
    let device_path = format!("/dev/{}", partition_name);
    
    let output = Command::new("findmnt")
        .arg("-n")
        .arg("-o")
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

    let mount_point = format!("/tmp/surakshit_mount_{}", partition_name);

    if let Err(e) = std::fs::create_dir_all(&mount_point) {
        return Err(format!("Failed to create mount directory: {}", e));
    }

    let mount_result = try_mount_with_different_options(&device_path, &mount_point, fstype).await;
    
    match mount_result {
        Ok(_) => {
            println!("Successfully mounted {} at {}", device_path, mount_point);
            Ok(mount_point)
        },
        Err(e) => {
            println!("Failed to mount {}: {}", device_path, e);
            let _ = std::fs::remove_dir(&mount_point);
            Err(e)
        }
    }
}

async fn try_mount_with_different_options(device_path: &str, mount_point: &str, fstype: &Option<String>) -> Result<(), String> {
    if let Some(fs) = fstype {
        if !fs.is_empty() && fs != "-" {
            let output = Command::new("sudo")
                .arg("mount")
                .arg("-t")
                .arg(fs)
                .arg("-o")
                // .arg("ro,noatime")
                .arg("rw,noatime")
                .arg(device_path)
                .arg(mount_point)
                .output()
                .map_err(|e| format!("Command execution failed: {}", e))?;
            
            if output.status.success() {
                return Ok(());
            }
        }
    }

    let output = Command::new("sudo")
        .arg("mount")
        .arg("-o")
        // .arg("ro,noatime")
        .arg("rw,noatime")
        .arg(device_path)
        .arg(mount_point)
        .output()
        .map_err(|e| format!("Command execution failed: {}", e))?;
    
    if output.status.success() {
        return Ok(());
    }

    let common_filesystems = ["ext4", "ext3", "ext2", "ntfs", "vfat", "exfat", "xfs", "btrfs"];
    
    for fs in &common_filesystems {
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
            return Ok(());
        }
    }

    Err("Failed to mount partition with any supported filesystem".to_string())
}

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