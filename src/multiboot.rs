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
#[derive(Copy, Clone)]
pub struct MultibootMmapEntry {
    pub size: u32,
    pub addr_low: u32,
    pub addr_high: u32,
    pub len_low: u32,
    pub len_high: u32,
    pub typ: u32,
}

pub unsafe fn print_mmap_sections(
    info: *const MultibootInfo,
) {
    let mmap_length = (*info).mmap_length;
    println!("Available memory segments...");
    println!("mmap_length: {}", mmap_length);
    for i in 0..(*info).mmap_length {
        let entry = ((*info).mmap_addr + core::mem::size_of::<MultibootMmapEntry>() as u32 * i)
            as *const MultibootMmapEntry;
        let len = (*entry).len_low;
        let size = (*entry).size;
        if size == 0 { break; }
        let addr = (*entry).addr_low;
        println!("size: {}, len: {}, addr: {}", size, len, addr);
    }

}
