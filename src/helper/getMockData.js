export const getMockData = () => ({
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
                                        {
                                            name: "sample.txt",
                                            type: "file",
                                            size: "100MB",
                                            icon: "file",
                                        },
                                        {
                                            name: "report.docx",
                                            type: "file",
                                            size: "2.5MB",
                                            icon: "file",
                                        },
                                    ],
                                },
                                {
                                    name: "Media",
                                    type: "folder",
                                    icon: "folder",
                                    children: [
                                        {
                                            name: "photo1.jpg",
                                            type: "file",
                                            size: "3.2MB",
                                            icon: "file",
                                        },
                                        {
                                            name: "video.mp4",
                                            type: "file",
                                            size: "250MB",
                                            icon: "file",
                                        },
                                    ],
                                },
                            ],
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
                                        {
                                            name: "secret.db",
                                            type: "file",
                                            size: "50MB",
                                            icon: "database",
                                            hidden: true,
                                        },
                                        {
                                            name: "backup.img",
                                            type: "file",
                                            size: "2GB",
                                            icon: "file",
                                            hidden: true,
                                        },
                                    ],
                                },
                            ],
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
                                        {
                                            name: "config.bin",
                                            type: "file",
                                            size: "20MB",
                                            icon: "settings",
                                            hidden: true,
                                        },
                                        {
                                            name: "firmware.rom",
                                            type: "file",
                                            size: "8MB",
                                            icon: "file",
                                            hidden: true,
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
});
