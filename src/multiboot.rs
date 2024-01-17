#[repr(C, packed)]
#[allow(unused)]
#[derive(Copy, Clone)]
pub struct MultibootInfo {
    // Multiboot info version number
    pub flags: u32,

    // Available memory from BIOS
    pub mem_lower: u32,
    pub mem_upper: u32,

    // "root" partition
    pub boot_device: u32,

    // Kernel command line
    pub cmdline: u32,

    // Boot-Module list
    pub mods_count: u32,
    pub mods_addr: u32,

    pub dummy: [u8; 16],

    // Memory Mapping buffer
    pub mmap_length: u32,
    pub mmap_addr: u32,

    // Drive Info buffer
    pub drives_length: u32,
    pub drives_addr: u32,

    // ROM configuration table
    pub config_table: u32,

    // Boot Loader Name
    pub boot_loader_name: *const u8,

    // APM table
    pub apm_table: u32,
}

#[repr(C, packed)]
#[allow(unused)]
pub struct MultibootMmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub typ: u32,
}
