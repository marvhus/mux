#[repr(C, packed)]
pub struct MultibootInfo {
    pub flags: u32, // enum MultibootInfoFlag

    pub mem_lower: u32,
    pub mem_upper: u32,
}
