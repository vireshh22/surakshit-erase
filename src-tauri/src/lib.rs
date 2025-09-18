// use std::fs;
// use std::path::Path;
// use serde::{Deserialize, Serialize};
// use tauri::State;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct DriveInfo {
//     pub device: String,
//     pub mount_point: Option<String>,
//     pub size: Option<String>,
//     pub used: Option<String>,
//     pub available: Option<String>,
//     pub filesystem: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct FileSystemItem {
//     pub name: String,
//     pub path: String,
//     pub is_directory: bool,
//     pub size: Option<u64>,
//     pub permissions: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct NavigationState {
//     pub current_path: String,
//     pub items: Vec<FileSystemItem>,
//     pub parent_path: Option<String>,
// }
// //     let lines: Vec<&str> = stdout.lines().collect();

// //     if lines.len() > 1 {
// //         // Parse the sdb drive info
// //         let sdb_info = parse_drive_line(&lines[1])?;
// //         let mut sdb_drive = create_drive_node("sdb", &sdb_info)?;

// //         // Look for sdb1 partition
// //         if lines.len() > 2 {
// //             let sdb1_info = parse_drive_line(&lines[2])?;
// //             let mut sdb1_partition = create_partition_node("sdb1", &sdb1_info)?;

// //             // If sdb1 is mounted to /mnt/demo, scan that directory
// //             if sdb1_info.mountpoint.as_deref() == Some("/mnt/demo") {
// //                 match get_directory_contents("/mnt/demo", 0).await {
// //                     Ok(contents) => {
// //                         sdb1_partition.children = Some(contents);
// //                     }
// //                     Err(_) => {
// //                         // Create demo structure if can't read actual directory
// //                         sdb1_partition.children = Some(create_demo_structure());
// //                     }
// //                 }
// //             }

// //             // Add demo hidden regions
// //             let mut children = sdb1_partition.children.unwrap_or_default();
// //             children.extend(create_hidden_regions());
// //             sdb1_partition.children = Some(children);

// //             sdb_drive.children = Some(vec![sdb1_partition]);
// //         }

// //         drives.push(sdb_drive);
// //     }

// //     // Add main system drive for comparison
// //     if let Ok(root_drive) = get_root_drive_info().await {
// //         drives.insert(0, root_drive);
// //     }

// //     Ok(drives)
// // }

// // #[derive(Debug)]
// // struct DriveInfo {
// //     name: String,
// //     size: Option<String>,
// //     fstype: Option<String>,
// //     mountpoint: Option<String>,
// // }

// // fn parse_drive_line(line: &str) -> Result<DriveInfo, String> {
// //     let parts: Vec<&str> = line.split_whitespace().collect();
// //     if parts.is_empty() {
// //         return Err("Invalid drive line".to_string());
// //     }

// //     Ok(DriveInfo {
// //         name: parts[0].to_string(),
// //         size: parts.get(1).map(|s| s.to_string()),
// //         fstype: parts.get(2).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
// //         mountpoint: parts.get(3).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
// //     })
// // }

// // fn create_drive_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
// //     let display_name = if let Some(size) = &info.size {
// //         format!("{} ({} Custom Drive)", name, size)
// //     } else {
// //         format!("{} (Custom Drive)", name)
// //     };

// //     Ok(FileSystemNode {
// //         name: display_name,
// //         node_type: "drive".to_string(),
// //         icon: "drive".to_string(),
// //         size: info.size.clone(),
// //         hidden: None,
// //         visible: None,
// //         children: Some(Vec::new()),
// //     })
// // }

// // fn create_partition_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
// //     let display_name = if let Some(size) = &info.size {
// //         format!("{} ({})", name, size)
// //     } else {
// //         name.to_string()
// //     };

// //     Ok(FileSystemNode {
// //         name: display_name,
// //         node_type: "partition".to_string(),
// //         icon: "partition".to_string(),
// //         size: info.size.clone(),
// //         hidden: None,
// //         visible: None,
// //         children: Some(Vec::new()),
// //     })
// // }

// // async fn get_root_drive_info() -> Result<FileSystemNode, String> {
// //     let output = Command::new("df")
// //         .arg("-h")
// //         .arg("/")
// //         .output()
// //         .map_err(|e| format!("Failed to get root drive info: {}", e))?;

// //     let stdout = String::from_utf8_lossy(&output.stdout);
// //     let lines: Vec<&str> = stdout.lines().collect();

// //     if lines.len() > 1 {
// //         let parts: Vec<&str> = lines[1].split_whitespace().collect();
// //         if parts.len() >= 4 {
// //             let device = parts[0];
// //             let size = parts[1];
// //             let used = parts[2];
// //             let available = parts[3];

// //             let drive_name = format!("{} ({} System Drive)", device, size);
// //             let partition_name = format!("{} ({})", device, size);

// //             let mut root_drive = FileSystemNode {
// //                 name: drive_name,
// //                 node_type: "drive".to_string(),
// //                 icon: "drive".to_string(),
// //                 size: Some(format!("{} used, {} available", used, available)),
// //                 hidden: None,
// //                 visible: None,
// //                 children: Some(Vec::new()),
// //             };

// //             let mut root_partition = FileSystemNode {
// //                 name: partition_name,
// //                 node_type: "partition".to_string(),
// //                 icon: "partition".to_string(),
// //                 size: Some(size.to_string()),
// //                 hidden: None,
// //                 visible: None,
// //                 children: Some(Vec::new()),
// //             };

// //             // Add some common system directories
// //             if let Ok(contents) = get_directory_contents("/", 1).await {
// //                 root_partition.children = Some(contents);
// //             }

// //             root_drive.children = Some(vec![root_partition]);
// //             return Ok(root_drive);
// //         }
// //     }

// //     Err("Could not get root drive info".to_string())
// // }

// // async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
// //     if depth > 2 {
// //         return Ok(Vec::new());
// //     }

// //     let mut nodes = Vec::new();
    
// //     match fs::read_dir(path) {
// //         Ok(entries) => {
// //             let mut entry_vec: Vec<_> = entries.collect();
// //             entry_vec.sort_by_key(|entry| {
// //                 entry.as_ref().map(|e| e.file_name()).unwrap_or_default()
// //             });

// //             for entry in entry_vec {
// //                 if let Ok(entry) = entry {
// //                     let entry_path = entry.path();
// //                     let name = entry.file_name().to_string_lossy().to_string();
                    
// //                     // Skip special directories that might cause issues
// //                     if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/" {
// //                         continue;
// //                     }

// //                     let metadata = match entry.metadata() {
// //                         Ok(metadata) => metadata,
// //                         Err(_) => continue,
// //                     };

// //                     let is_dir = metadata.is_dir();
// //                     let is_hidden = name.starts_with('.');
                    
// //                     let size = if !is_dir {
// //                         Some(format_bytes(metadata.len()))
// //                     } else {
// //                         None
// //                     };

// //                     let mut node = FileSystemNode {
// //                         name: name.clone(),
// //                         node_type: if is_dir { "folder".to_string() } else { "file".to_string() },
// //                         icon: if is_dir { "folder".to_string() } else { get_file_icon(&name) },
// //                         size,
// //                         hidden: Some(is_hidden),
// //                         visible: Some(!is_hidden),
// //                         children: None,
// //                     };

// //                     // Recursively get children for directories (limited depth)
// //                     if is_dir && depth < 2 {
// //                         match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await {
// //                             Ok(children) => {
// //                                 if !children.is_empty() {
// //                                     node.children = Some(children);
// //                                 }
// //                             }
// //                             Err(_) => {} // Continue even if we can't read subdirectories
// //                         }
// //                     }

// //                     nodes.push(node);

// //                     // Limit entries to prevent UI overload
// //                     if nodes.len() >= 15 {
// //                         break;
// //                     }
// //                 }
// //             }
// //         }
// //         Err(_) => {
// //             // If we can't read the directory, return demo content
// //             return Ok(create_demo_structure());
// //         }
// //     }

// //     // Sort nodes: directories first, then by name
// //     nodes.sort_by(|a, b| {
// //         match (a.node_type.as_str(), b.node_type.as_str()) {
// //             ("folder", "file") => std::cmp::Ordering::Less,
// //             ("file", "folder") => std::cmp::Ordering::Greater,
// //             _ => a.name.cmp(&b.name),
// //         }
// //     });

// //     Ok(nodes)
// // }

// // fn create_demo_structure() -> Vec<FileSystemNode> {
// //     vec![
// //         FileSystemNode {
// //             name: "Visible User Data (/mnt/demo)".to_string(),
// //             node_type: "folder".to_string(),
// //             icon: "folder".to_string(),
// //             size: None,
// //             hidden: None,
// //             visible: Some(true),
// //             children: Some(vec![
// //                 FileSystemNode {
// //                     name: "Documents".to_string(),
// //                     node_type: "folder".to_string(),
// //                     icon: "folder".to_string(),
// //                     size: None,
// //                     hidden: None,
// //                     visible: None,
// //                     children: Some(vec![
// //                         FileSystemNode {
// //                             name: "sample.txt".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("100MB".to_string()),
// //                             hidden: None,
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "report.docx".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("2.5MB".to_string()),
// //                             hidden: None,
// //                             visible: None,
// //                             children: None,
// //                         },
// //                     ]),
// //                 },
// //                 FileSystemNode {
// //                     name: "Media".to_string(),
// //                     node_type: "folder".to_string(),
// //                     icon: "folder".to_string(),
// //                     size: None,
// //                     hidden: None,
// //                     visible: None,
// //                     children: Some(vec![
// //                         FileSystemNode {
// //                             name: "photo1.jpg".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("3.2MB".to_string()),
// //                             hidden: None,
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "video.mp4".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("250MB".to_string()),
// //                             hidden: None,
// //                             visible: None,
// //                             children: None,
// //                         },
// //                     ]),
// //                 },
// //             ]),
// //         }
// //     ]
// // }

// // fn create_hidden_regions() -> Vec<FileSystemNode> {
// //     vec![
// //         FileSystemNode {
// //             name: "Hidden HPA Region (/mnt/hpa)".to_string(),
// //             node_type: "folder".to_string(),
// //             icon: "hidden".to_string(),
// //             size: Some("5GB".to_string()),
// //             hidden: Some(true),
// //             visible: None,
// //             children: Some(vec![
// //                 FileSystemNode {
// //                     name: "HiddenBackup".to_string(),
// //                     node_type: "folder".to_string(),
// //                     icon: "folder".to_string(),
// //                     size: None,
// //                     hidden: Some(true),
// //                     visible: None,
// //                     children: Some(vec![
// //                         FileSystemNode {
// //                             name: "secret.db".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "database".to_string(),
// //                             size: Some("50MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "backup.img".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("2GB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "keys.bin".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("1KB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                     ]),
// //                 },
// //                 FileSystemNode {
// //                     name: "SystemRestore".to_string(),
// //                     node_type: "folder".to_string(),
// //                     icon: "folder".to_string(),
// //                     size: None,
// //                     hidden: Some(true),
// //                     visible: None,
// //                     children: Some(vec![
// //                         FileSystemNode {
// //                             name: "restore_point_1.dat".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("500MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "registry_backup.reg".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("10MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                     ]),
// //                 },
// //             ]),
// //         },
// //         FileSystemNode {
// //             name: "Hidden DCO Region (/mnt/dco)".to_string(),
// //             node_type: "folder".to_string(),
// //             icon: "system".to_string(),
// //             size: Some("100MB".to_string()),
// //             hidden: Some(true),
// //             visible: None,
// //             children: Some(vec![
// //                 FileSystemNode {
// //                     name: "ManufacturerData".to_string(),
// //                     node_type: "folder".to_string(),
// //                     icon: "folder".to_string(),
// //                     size: None,
// //                     hidden: Some(true),
// //                     visible: None,
// //                     children: Some(vec![
// //                         FileSystemNode {
// //                             name: "config.bin".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "settings".to_string(),
// //                             size: Some("20MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "firmware.rom".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("8MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                         FileSystemNode {
// //                             name: "diagnostic.log".to_string(),
// //                             node_type: "file".to_string(),
// //                             icon: "file".to_string(),
// //                             size: Some("2MB".to_string()),
// //                             hidden: Some(true),
// //                             visible: None,
// //                             children: None,
// //                         },
// //                     ]),
// //                 },
// //             ]),
// //         },
// //     ]
// // }

// // fn get_file_icon(filename: &str) -> String {
// //     let extension = Path::new(filename)
// //         .extension()
// //         .and_then(|ext| ext.to_str())
// //         .unwrap_or("")
// //         .to_lowercase();
        
// //     match extension.as_str() {
// //         "db" | "sqlite" | "sql" => "database".to_string(),
// //         "exe" | "deb" | "rpm" | "appimage" => "settings".to_string(),
// //         "txt" | "md" | "json" | "xml" | "yml" | "yaml" | "conf" | "cfg" => "file".to_string(),
// //         "jpg" | "jpeg" | "png" | "gif" | "bmp" | "svg" => "file".to_string(),
// //         "mp4" | "avi" | "mov" | "wmv" | "flv" => "file".to_string(),
// //         "mp3" | "wav" | "ogg" | "flac" => "file".to_string(),
// //         "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" => "file".to_string(),
// //         "docx" | "doc" | "pdf" => "file".to_string(),
// //         "pptx" | "ppt" => "file".to_string(),
// //         _ => "file".to_string(),
// //     }
// // }

// // fn format_bytes(bytes: u64) -> String {
// //     const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
// //     let mut size = bytes as f64;
// //     let mut unit_index = 0;
    
// //     while size >= 1024.0 && unit_index < UNITS.len() - 1 {
// //         size /= 1024.0;
// //         unit_index += 1;
// //     }
    
// //     if unit_index == 0 {
// //         format!("{}B", bytes)
// //     } else {
// //         format!("{:.1}{}", size, UNITS[unit_index])
// //     }
// // }

// // #[cfg_attr(mobile, tauri::mobile_entry_point)]
// // pub fn run() {
// //     tauri::Builder::default()
// //         .invoke_handler(tauri::generate_handler![get_file_system_data])
// //         .run(tauri::generate_context!())
// //         .expect("error while running tauri application");
// // }


// // Prevents additional console window on Windows in release, DO NOT REMOVE!!
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

// #[tauri::command]
// async fn get_file_system_data() -> Result<FileSystemNode, String> {
//     let mut root = FileSystemNode {
//         name: "System Drives".to_string(),
//         node_type: "root".to_string(),
//         icon: "root".to_string(),
//         size: None,
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     };

//     // Get drive information
//     let drives = get_drives_info().await?;
//     root.children = Some(drives);

//     Ok(root)
// }

// async fn get_drives_info() -> Result<Vec<FileSystemNode>, String> {
//     let mut drives = Vec::new();

//     // Get information about sdb drive specifically
//     let output = Command::new("lsblk")
//         .arg("-f")
//         .arg("-o")
//         .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
//         .arg("/dev/sdb")
//         .output()
//         .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         // Parse the sdb drive info
//         let sdb_info = parse_drive_line(&lines[1])?;
//         let mut sdb_drive = create_drive_node("sdb", &sdb_info)?;

//         // Look for sdb1 partition
//         if lines.len() > 2 {
//             let sdb1_info = parse_drive_line(&lines[2])?;
//             let mut sdb1_partition = create_partition_node("sdb1", &sdb1_info)?;

//             // If sdb1 is mounted to /mnt/demo, scan that directory
//             if sdb1_info.mountpoint.as_deref() == Some("/mnt/demo") {
//                 match get_directory_contents("/mnt/demo", 0).await {
//                     Ok(contents) => {
//                         sdb1_partition.children = Some(contents);
//                     }
//                     Err(_) => {
//                         // Create demo structure if can't read actual directory
//                         sdb1_partition.children = Some(create_demo_structure());
//                     }
//                 }
//             }

//             // Add demo hidden regions
//             let mut children = sdb1_partition.children.unwrap_or_default();
//             children.extend(create_hidden_regions());
//             sdb1_partition.children = Some(children);

//             sdb_drive.children = Some(vec![sdb1_partition]);
//         }

//         drives.push(sdb_drive);
//     }

//     // Add main system drive for comparison
//     if let Ok(root_drive) = get_root_drive_info().await {
//         drives.insert(0, root_drive);
//     }

//     Ok(drives)
// }

// #[derive(Debug)]
// struct DriveInfo {
//     name: String,
//     size: Option<String>,
//     fstype: Option<String>,
//     mountpoint: Option<String>,
// }

// fn parse_drive_line(line: &str) -> Result<DriveInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.is_empty() {
//         return Err("Invalid drive line".to_string());
//     }

//     Ok(DriveInfo {
//         name: parts[0].to_string(),
//         size: parts.get(1).map(|s| s.to_string()),
//         fstype: parts
//             .get(2)
//             .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//         mountpoint: parts
//             .get(3)
//             .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//     })
// }

// fn create_drive_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
//     let display_name = if let Some(size) = &info.size {
//         format!("{} ({} Custom Drive)", name, size)
//     } else {
//         format!("{} (Custom Drive)", name)
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

// fn create_partition_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
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

// async fn get_root_drive_info() -> Result<FileSystemNode, String> {
//     let output = Command::new("df")
//         .arg("-h")
//         .arg("/")
//         .output()
//         .map_err(|e| format!("Failed to get root drive info: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         let parts: Vec<&str> = lines[1].split_whitespace().collect();
//         if parts.len() >= 4 {
//             let device = parts[0];
//             let size = parts[1];
//             let used = parts[2];
//             let available = parts[3];

//             let drive_name = format!("{} ({} System Drive)", device, size);
//             let partition_name = format!("{} ({})", device, size);

//             let mut root_drive = FileSystemNode {
//                 name: drive_name,
//                 node_type: "drive".to_string(),
//                 icon: "drive".to_string(),
//                 size: Some(format!("{} used, {} available", used, available)),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             let mut root_partition = FileSystemNode {
//                 name: partition_name,
//                 node_type: "partition".to_string(),
//                 icon: "partition".to_string(),
//                 size: Some(size.to_string()),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             // Add some common system directories
//             if let Ok(contents) = get_directory_contents("/", 1).await {
//                 root_partition.children = Some(contents);
//             }

//             root_drive.children = Some(vec![root_partition]);
//             return Ok(root_drive);
//         }
//     }

//     Err("Could not get root drive info".to_string())
// }

// #[async_recursion]
// async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
//     if depth > 2 {
//         return Ok(Vec::new());
//     }

//     let mut nodes = Vec::new();

//     match fs::read_dir(path) {
//         Ok(entries) => {
//             let mut entry_vec: Vec<_> = entries.collect();
//             entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

//             for entry in entry_vec {
//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     let name = entry.file_name().to_string_lossy().to_string();

//                     // Skip special directories that might cause issues
//                     if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/"
//                     {
//                         continue;
//                     }

//                     let metadata = match entry.metadata() {
//                         Ok(metadata) => metadata,
//                         Err(_) => continue,
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
//                         node_type: if is_dir {
//                             "folder".to_string()
//                         } else {
//                             "file".to_string()
//                         },
//                         icon: if is_dir {
//                             "folder".to_string()
//                         } else {
//                             get_file_icon(&name)
//                         },
//                         size,
//                         hidden: Some(is_hidden),
//                         visible: Some(!is_hidden),
//                         children: None,
//                     };

//                     // Recursively get children for directories (limited depth)
//                     if is_dir && depth < 2 {
//                         match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await
//                         {
//                             Ok(children) => {
//                                 if !children.is_empty() {
//                                     node.children = Some(children);
//                                 }
//                             }
//                             Err(_) => {} // Continue even if we can't read subdirectories
//                         }
//                     }

//                     nodes.push(node);

//                     // Limit entries to prevent UI overload
//                     if nodes.len() >= 15 {
//                         break;
//                     }
//                 }
//             }
//         }
//         Err(_) => {
//             // If we can't read the directory, return demo content
//             return Ok(create_demo_structure());
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

// fn create_demo_structure() -> Vec<FileSystemNode> {
//     vec![FileSystemNode {
//         name: "Visible User Data (/mnt/demo)".to_string(),
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

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![get_file_system_data])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


// use async_recursion::async_recursion;
// use serde::{Deserialize, Serialize};
// use std::fs;
// use std::path::Path;
// use std::process::Command;
// use tauri::State;

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct DriveInfo {
//     pub device: String,
//     pub mount_point: Option<String>,
//     pub size: Option<String>,
//     pub used: Option<String>,
//     pub available: Option<String>,
//     pub filesystem: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct FileSystemItem {
//     pub name: String,
//     pub path: String,
//     pub is_directory: bool,
//     pub size: Option<u64>,
//     pub permissions: Option<String>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct NavigationState {
//     pub current_path: String,
//     pub items: Vec<FileSystemItem>,
//     pub parent_path: Option<String>,
// }

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

// #[tauri::command]
// async fn get_file_system_data() -> Result<FileSystemNode, String> {
//     let mut root = FileSystemNode {
//         name: "System Drives".to_string(),
//         node_type: "root".to_string(),
//         icon: "root".to_string(),
//         size: None,
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     };

//     // Get drive information
//     let drives = get_drives_info().await?;
//     root.children = Some(drives);

//     Ok(root)
// }

// async fn get_drives_info() -> Result<Vec<FileSystemNode>, String> {
//     let mut drives = Vec::new();

//     // Get information about sdb drive specifically
//     let output = Command::new("lsblk")
//         .arg("-f")
//         .arg("-o")
//         .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
//         .arg("/dev/sdb")
//         .output()
//         .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         // Parse the sdb drive info
//         let sdb_info = parse_drive_line(&lines[1])?;
//         let mut sdb_drive = create_drive_node("sdb", &sdb_info)?;

//         // Look for sdb1 partition
//         if lines.len() > 2 {
//             let sdb1_info = parse_drive_line(&lines[2])?;
//             let mut sdb1_partition = create_partition_node("sdb1", &sdb1_info)?;

//             // If sdb1 is mounted to /mnt/demo, scan that directory
//             if sdb1_info.mountpoint.as_deref() == Some("/mnt/demo") {
//                 match get_directory_contents("/mnt/demo", 0).await {
//                     Ok(contents) => {
//                         sdb1_partition.children = Some(contents);
//                     }
//                     Err(_) => {
//                         // Create demo structure if can't read actual directory
//                         sdb1_partition.children = Some(create_demo_structure());
//                     }
//                 }
//             }

//             // Add demo hidden regions
//             let mut children = sdb1_partition.children.unwrap_or_default();
//             children.extend(create_hidden_regions());
//             sdb1_partition.children = Some(children);

//             sdb_drive.children = Some(vec![sdb1_partition]);
//         }

//         drives.push(sdb_drive);
//     }

//     // Add main system drive for comparison
//     if let Ok(root_drive) = get_root_drive_info().await {
//         drives.insert(0, root_drive);
//     }

//     Ok(drives)
// }

// #[derive(Debug)]
// struct DriveInfo {
//     name: String,
//     size: Option<String>,
//     fstype: Option<String>,
//     mountpoint: Option<String>,
// }

// fn parse_drive_line(line: &str) -> Result<DriveInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.is_empty() {
//         return Err("Invalid drive line".to_string());
//     }

//     Ok(DriveInfo {
//         name: parts[0].to_string(),
//         size: parts.get(1).map(|s| s.to_string()),
//         fstype: parts
//             .get(2)
//             .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//         mountpoint: parts
//             .get(3)
//             .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//     })
// }

// fn create_drive_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
//     let display_name = if let Some(size) = &info.size {
//         format!("{} ({} Custom Drive)", name, size)
//     } else {
//         format!("{} (Custom Drive)", name)
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

// fn create_partition_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
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

// async fn get_root_drive_info() -> Result<FileSystemNode, String> {
//     let output = Command::new("df")
//         .arg("-h")
//         .arg("/")
//         .output()
//         .map_err(|e| format!("Failed to get root drive info: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         let parts: Vec<&str> = lines[1].split_whitespace().collect();
//         if parts.len() >= 4 {
//             let device = parts[0];
//             let size = parts[1];
//             let used = parts[2];
//             let available = parts[3];

//             let drive_name = format!("{} ({} System Drive)", device, size);
//             let partition_name = format!("{} ({})", device, size);

//             let mut root_drive = FileSystemNode {
//                 name: drive_name,
//                 node_type: "drive".to_string(),
//                 icon: "drive".to_string(),
//                 size: Some(format!("{} used, {} available", used, available)),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             let mut root_partition = FileSystemNode {
//                 name: partition_name,
//                 node_type: "partition".to_string(),
//                 icon: "partition".to_string(),
//                 size: Some(size.to_string()),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             // Add some common system directories
//             if let Ok(contents) = get_directory_contents("/", 1).await {
//                 root_partition.children = Some(contents);
//             }

//             root_drive.children = Some(vec![root_partition]);
//             return Ok(root_drive);
//         }
//     }

//     Err("Could not get root drive info".to_string())
// }

// #[async_recursion]
// async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
//     if depth > 2 {
//         return Ok(Vec::new());
//     }

//     let mut nodes = Vec::new();

//     match fs::read_dir(path) {
//         Ok(entries) => {
//             let mut entry_vec: Vec<_> = entries.collect();
//             entry_vec.sort_by_key(|entry| entry.as_ref().map(|e| e.file_name()).unwrap_or_default());

//             for entry in entry_vec {
//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     let name = entry.file_name().to_string_lossy().to_string();

//                     // Skip special directories that might cause issues
//                     if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/"
//                     {
//                         continue;
//                     }

//                     let metadata = match entry.metadata() {
//                         Ok(metadata) => metadata,
//                         Err(_) => continue,
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
//                         node_type: if is_dir {
//                             "folder".to_string()
//                         } else {
//                             "file".to_string()
//                         },
//                         icon: if is_dir {
//                             "folder".to_string()
//                         } else {
//                             get_file_icon(&name)
//                         },
//                         size,
//                         hidden: Some(is_hidden),
//                         visible: Some(!is_hidden),
//                         children: None,
//                     };

//                     // Recursively get children for directories (limited depth)
//                     if is_dir && depth < 2 {
//                         match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await
//                         {
//                             Ok(children) => {
//                                 if !children.is_empty() {
//                                     node.children = Some(children);
//                                 }
//                             }
//                             Err(_) => {} // Continue even if we can't read subdirectories
//                         }
//                     }

//                     nodes.push(node);

//                     // Limit entries to prevent UI overload
//                     if nodes.len() >= 15 {
//                         break;
//                     }
//                 }
//             }
//         }
//         Err(_) => {
//             // If we can't read the directory, return demo content
//             return Ok(create_demo_structure());
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

// fn create_demo_structure() -> Vec<FileSystemNode> {
//     vec![FileSystemNode {
//         name: "Visible User Data (/mnt/demo)".to_string(),
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

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![get_file_system_data])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }




// // src-tauri/src/main.rs
// use tauri::Manager;
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

// #[tauri::command]
// async fn get_file_system_data() -> Result<FileSystemNode, String> {
//     let mut root = FileSystemNode {
//         name: "System Drives".to_string(),
//         node_type: "root".to_string(),
//         icon: "root".to_string(),
//         size: None,
//         hidden: None,
//         visible: None,
//         children: Some(Vec::new()),
//     };

//     // Get drive information
//     let drives = get_drives_info().await?;
//     root.children = Some(drives);

//     Ok(root)
// }

// async fn get_drives_info() -> Result<Vec<FileSystemNode>, String> {
//     let mut drives = Vec::new();

//     // Get information about sdb drive specifically
//     let output = Command::new("lsblk")
//         .arg("-f")
//         .arg("-o")
//         .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
//         .arg("/dev/sdb")
//         .output()
//         .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         // Parse the sdb drive info
//         let sdb_info = parse_drive_line(&lines[1])?;
//         let mut sdb_drive = create_drive_node("sdb", &sdb_info)?;

//         // Look for sdb1 partition
//         if lines.len() > 2 {
//             let sdb1_info = parse_drive_line(&lines[2])?;
//             let mut sdb1_partition = create_partition_node("sdb1", &sdb1_info)?;

//             // If sdb1 is mounted to /mnt/demo, scan that directory
//             if sdb1_info.mountpoint.as_deref() == Some("/mnt/demo") {
//                 match get_directory_contents("/mnt/demo", 0).await {
//                     Ok(contents) => {
//                         sdb1_partition.children = Some(contents);
//                     }
//                     Err(_) => {
//                         // Create demo structure if can't read actual directory
//                         sdb1_partition.children = Some(create_demo_structure());
//                     }
//                 }
//             }

//             // Add demo hidden regions
//             let mut children = sdb1_partition.children.unwrap_or_default();
//             children.extend(create_hidden_regions());
//             sdb1_partition.children = Some(children);

//             sdb_drive.children = Some(vec![sdb1_partition]);
//         }

//         drives.push(sdb_drive);
//     }

//     // Add main system drive for comparison
//     if let Ok(root_drive) = get_root_drive_info().await {
//         drives.insert(0, root_drive);
//     }

//     Ok(drives)
// }

// #[derive(Debug)]
// struct DriveInfo {
//     name: String,
//     size: Option<String>,
//     fstype: Option<String>,
//     mountpoint: Option<String>,
// }

// fn parse_drive_line(line: &str) -> Result<DriveInfo, String> {
//     let parts: Vec<&str> = line.split_whitespace().collect();
//     if parts.is_empty() {
//         return Err("Invalid drive line".to_string());
//     }

//     Ok(DriveInfo {
//         name: parts[0].to_string(),
//         size: parts.get(1).map(|s| s.to_string()),
//         fstype: parts.get(2).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//         mountpoint: parts.get(3).and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
//     })
// }

// fn create_drive_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
//     let display_name = if let Some(size) = &info.size {
//         format!("{} ({} Custom Drive)", name, size)
//     } else {
//         format!("{} (Custom Drive)", name)
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

// fn create_partition_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
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

// async fn get_root_drive_info() -> Result<FileSystemNode, String> {
//     let output = Command::new("df")
//         .arg("-h")
//         .arg("/")
//         .output()
//         .map_err(|e| format!("Failed to get root drive info: {}", e))?;

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     let lines: Vec<&str> = stdout.lines().collect();

//     if lines.len() > 1 {
//         let parts: Vec<&str> = lines[1].split_whitespace().collect();
//         if parts.len() >= 4 {
//             let device = parts[0];
//             let size = parts[1];
//             let used = parts[2];
//             let available = parts[3];

//             let drive_name = format!("{} ({} System Drive)", device, size);
//             let partition_name = format!("{} ({})", device, size);

//             let mut root_drive = FileSystemNode {
//                 name: drive_name,
//                 node_type: "drive".to_string(),
//                 icon: "drive".to_string(),
//                 size: Some(format!("{} used, {} available", used, available)),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             let mut root_partition = FileSystemNode {
//                 name: partition_name,
//                 node_type: "partition".to_string(),
//                 icon: "partition".to_string(),
//                 size: Some(size.to_string()),
//                 hidden: None,
//                 visible: None,
//                 children: Some(Vec::new()),
//             };

//             // Add some common system directories
//             if let Ok(contents) = get_directory_contents("/", 1).await {
//                 root_partition.children = Some(contents);
//             }

//             root_drive.children = Some(vec![root_partition]);
//             return Ok(root_drive);
//         }
//     }

//     Err("Could not get root drive info".to_string())
// }

// async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
//     if depth > 2 {
//         return Ok(Vec::new());
//     }

//     let mut nodes = Vec::new();
    
//     match fs::read_dir(path) {
//         Ok(entries) => {
//             let mut entry_vec: Vec<_> = entries.collect();
//             entry_vec.sort_by_key(|entry| {
//                 entry.as_ref().map(|e| e.file_name()).unwrap_or_default()
//             });

//             for entry in entry_vec {
//                 if let Ok(entry) = entry {
//                     let entry_path = entry.path();
//                     let name = entry.file_name().to_string_lossy().to_string();
                    
//                     // Skip special directories that might cause issues
//                     if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/" {
//                         continue;
//                     }

//                     let metadata = match entry.metadata() {
//                         Ok(metadata) => metadata,
//                         Err(_) => continue,
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

//                     // Recursively get children for directories (limited depth)
//                     if is_dir && depth < 2 {
//                         match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await {
//                             Ok(children) => {
//                                 if !children.is_empty() {
//                                     node.children = Some(children);
//                                 }
//                             }
//                             Err(_) => {} // Continue even if we can't read subdirectories
//                         }
//                     }

//                     nodes.push(node);

//                     // Limit entries to prevent UI overload
//                     if nodes.len() >= 15 {
//                         break;
//                     }
//                 }
//             }
//         }
//         Err(_) => {
//             // If we can't read the directory, return demo content
//             return Ok(create_demo_structure());
//         }
//     }

//     // Sort nodes: directories first, then by name
//     nodes.sort_by(|a, b| {
//         match (a.node_type.as_str(), b.node_type.as_str()) {
//             ("folder", "file") => std::cmp::Ordering::Less,
//             ("file", "folder") => std::cmp::Ordering::Greater,
//             _ => a.name.cmp(&b.name),
//         }
//     });

//     Ok(nodes)
// }

// fn create_demo_structure() -> Vec<FileSystemNode> {
//     vec![
//         FileSystemNode {
//             name: "Visible User Data (/mnt/demo)".to_string(),
//             node_type: "folder".to_string(),
//             icon: "folder".to_string(),
//             size: None,
//             hidden: None,
//             visible: Some(true),
//             children: Some(vec![
//                 FileSystemNode {
//                     name: "Documents".to_string(),
//                     node_type: "folder".to_string(),
//                     icon: "folder".to_string(),
//                     size: None,
//                     hidden: None,
//                     visible: None,
//                     children: Some(vec![
//                         FileSystemNode {
//                             name: "sample.txt".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("100MB".to_string()),
//                             hidden: None,
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "report.docx".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("2.5MB".to_string()),
//                             hidden: None,
//                             visible: None,
//                             children: None,
//                         },
//                     ]),
//                 },
//                 FileSystemNode {
//                     name: "Media".to_string(),
//                     node_type: "folder".to_string(),
//                     icon: "folder".to_string(),
//                     size: None,
//                     hidden: None,
//                     visible: None,
//                     children: Some(vec![
//                         FileSystemNode {
//                             name: "photo1.jpg".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("3.2MB".to_string()),
//                             hidden: None,
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "video.mp4".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("250MB".to_string()),
//                             hidden: None,
//                             visible: None,
//                             children: None,
//                         },
//                     ]),
//                 },
//             ]),
//         }
//     ]
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
//             children: Some(vec![
//                 FileSystemNode {
//                     name: "ManufacturerData".to_string(),
//                     node_type: "folder".to_string(),
//                     icon: "folder".to_string(),
//                     size: None,
//                     hidden: Some(true),
//                     visible: None,
//                     children: Some(vec![
//                         FileSystemNode {
//                             name: "config.bin".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "settings".to_string(),
//                             size: Some("20MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "firmware.rom".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("8MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                         FileSystemNode {
//                             name: "diagnostic.log".to_string(),
//                             node_type: "file".to_string(),
//                             icon: "file".to_string(),
//                             size: Some("2MB".to_string()),
//                             hidden: Some(true),
//                             visible: None,
//                             children: None,
//                         },
//                     ]),
//                 },
//             ]),
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

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![get_file_system_data])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }



// Prevents additional console window on Windows in release, DO NOT REMOVE!!
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

#[tauri::command]
async fn get_file_system_data() -> Result<FileSystemNode, String> {
    let mut root = FileSystemNode {
        name: "System Drives".to_string(),
        node_type: "root".to_string(),
        icon: "root".to_string(),
        size: None,
        hidden: None,
        visible: None,
        children: Some(Vec::new()),
    };

    // Get drive information
    let drives = get_drives_info().await?;
    root.children = Some(drives);

    Ok(root)
}

async fn get_drives_info() -> Result<Vec<FileSystemNode>, String> {
    let mut drives = Vec::new();

    // Get information about sdb drive specifically
    let output = Command::new("lsblk")
        .arg("-f")
        .arg("-o")
        .arg("NAME,SIZE,FSTYPE,MOUNTPOINT")
        .arg("/dev/sdb")
        .output()
        .map_err(|e| format!("Failed to execute lsblk: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() > 1 {
        // Parse the sdb drive info
        let sdb_info = parse_drive_line(&lines[1])?;
        let mut sdb_drive = create_drive_node("sdb", &sdb_info)?;

        // Look for sdb1 partition
        if lines.len() > 2 {
            let sdb1_info = parse_drive_line(&lines[2])?;
            let mut sdb1_partition = create_partition_node("sdb1", &sdb1_info)?;

            // If sdb1 is mounted to /mnt/demo, scan that directory
            if sdb1_info.mountpoint.as_deref() == Some("/mnt/demo") {
                match get_directory_contents("/mnt/demo", 0).await {
                    Ok(contents) => {
                        sdb1_partition.children = Some(contents);
                    }
                    Err(_) => {
                        // Create demo structure if can't read actual directory
                        sdb1_partition.children = Some(create_demo_structure());
                    }
                }
            }

            // Add demo hidden regions
            let mut children = sdb1_partition.children.unwrap_or_default();
            children.extend(create_hidden_regions());
            sdb1_partition.children = Some(children);

            sdb_drive.children = Some(vec![sdb1_partition]);
        }

        drives.push(sdb_drive);
    }

    // Add main system drive for comparison
    if let Ok(root_drive) = get_root_drive_info().await {
        drives.insert(0, root_drive);
    }

    Ok(drives)
}

#[derive(Debug)]
struct DriveInfo {
    name: String,
    size: Option<String>,
    fstype: Option<String>,
    mountpoint: Option<String>,
}

fn parse_drive_line(line: &str) -> Result<DriveInfo, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Invalid drive line".to_string());
    }

    Ok(DriveInfo {
        name: parts[0].to_string(),
        size: parts.get(1).map(|s| s.to_string()),
        fstype: parts
            .get(2)
            .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
        mountpoint: parts
            .get(3)
            .and_then(|s| if *s == "-" { None } else { Some(s.to_string()) }),
    })
}

fn create_drive_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
    let display_name = if let Some(size) = &info.size {
        format!("{} ({} Custom Drive)", name, size)
    } else {
        format!("{} (Custom Drive)", name)
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

fn create_partition_node(name: &str, info: &DriveInfo) -> Result<FileSystemNode, String> {
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

async fn get_root_drive_info() -> Result<FileSystemNode, String> {
    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .map_err(|e| format!("Failed to get root drive info: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.len() > 1 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 4 {
            let device = parts[0];
            let size = parts[1];
            let used = parts[2];
            let available = parts[3];

            let drive_name = format!("{} ({} System Drive)", device, size);
            let partition_name = format!("{} ({})", device, size);

            let mut root_drive = FileSystemNode {
                name: drive_name,
                node_type: "drive".to_string(),
                icon: "drive".to_string(),
                size: Some(format!("{} used, {} available", used, available)),
                hidden: None,
                visible: None,
                children: Some(Vec::new()),
            };

            let mut root_partition = FileSystemNode {
                name: partition_name,
                node_type: "partition".to_string(),
                icon: "partition".to_string(),
                size: Some(size.to_string()),
                hidden: None,
                visible: None,
                children: Some(Vec::new()),
            };

            // Add some common system directories
            if let Ok(contents) = get_directory_contents("/", 1).await {
                root_partition.children = Some(contents);
            }

            root_drive.children = Some(vec![root_partition]);
            return Ok(root_drive);
        }
    }

    Err("Could not get root drive info".to_string())
}

#[async_recursion]
async fn get_directory_contents(path: &str, depth: usize) -> Result<Vec<FileSystemNode>, String> {
    if depth > 2 {
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
                    if matches!(name.as_str(), "proc" | "sys" | "dev" | "run" | "tmp") && path == "/"
                    {
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
                        node_type: if is_dir {
                            "folder".to_string()
                        } else {
                            "file".to_string()
                        },
                        icon: if is_dir {
                            "folder".to_string()
                        } else {
                            get_file_icon(&name)
                        },
                        size,
                        hidden: Some(is_hidden),
                        visible: Some(!is_hidden),
                        children: None,
                    };

                    // Recursively get children for directories (limited depth)
                    if is_dir && depth < 2 {
                        match get_directory_contents(&entry_path.to_string_lossy(), depth + 1).await
                        {
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
                    if nodes.len() >= 15 {
                        break;
                    }
                }
            }
        }
        Err(_) => {
            // If we can't read the directory, return demo content
            return Ok(create_demo_structure());
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

fn create_demo_structure() -> Vec<FileSystemNode> {
    vec![FileSystemNode {
        name: "Visible User Data (/mnt/demo)".to_string(),
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
        .invoke_handler(tauri::generate_handler![get_file_system_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
