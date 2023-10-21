#if !defined (KERNEL_MULTIBOOT_H)
#define KERNEL_MULTIBOOT_H

#include "../include/types.h"

struct Multiboot_Info {
    // Multiboot info version number
    u32 flags; // enum Multiboot_Info_Flag

    // Available memory from BIOS
    u32 mem_lower;
    u32 mem_upper;

    // "root" partition
    u32 boot_device;

    // Kernel command line
    u32 cmdline;

    // Moot-Module list
    u32 mods_count;
    u32 mods_addr;

    union {
        struct { // The symbol table for a.out
            u32 tabsize;
            u32 strsize;
            u32 addr;
            u32 reserved;
        } aout_symbol_table;
        struct { // The section header table for ELF
            u32 num;
            u32 size;
            u32 addr;
            u32 shndx;
        } elf_section_header_table;
    } syms;

    // Memory Mapping buffer
    u32 mmap_length;
    u32 mmap_addr;

    // Drive Info Buffer
    u32 drives_length;
    u32 drives_addr;

    // ROM configuration table
    u32 config_table;

    // Boot Loader Name
    u32 boot_loader_name; // address to c string

    // APM table
    u32 apm_table;

    // Video
    u32 vbe_control_info;
    u32 vbe_mode_info;
    u16 vbe_mode;
    u16 vbe_interface_seg;
    u16 vbe_interface_off;
    u16 vbe_interface_len;

    u64 framebuffer_addr;
    u32 framebuffer_pitch;
    u32 framebuffer_width;
    u32 framebuffer_height;
    u8  framebuffer_bpp;
    u8  framebuffer_type; // enum Framebuffer_Type
    union {
        struct {
            u32 framebuffer_palette_addr;
            u16 framebuffer_palette_num_color;
        };
        struct {
            u8 framebuffer_red_field_position;
            u8 framebuffer_red_mask_size;
            u8 framebuffer_green_field_position;
            u8 framebuffer_green_mask_size;
            u8 framebuffer_blue_field_position;
            u8 framebuffer_blue_mask_size;
        };
    } color_info;
};

// MultibootInfo Framebuffer Type
enum Framebuffer_Type {
    Indexed = 0,
    RGB = 1,
    EGA_Text = 2,
};

// MultibootInfo Flags
enum Multiboot_Info_Flag {
    // is there basic lower/upper memory information?
    Memory = 0x00000001,
    // is there a boot device set?
    Boot_Device = 0x00000002,
    // is the command-line defined?
    CMD_Line = 0x00000004,
    // are there modules to do something with?
    Modules = 0x00000008,

    // These next two are mutually exclusive

    // is there a symbol table loaded?
    AOUT_Symbol_Table = 0x00000010,
    // is there an ELF section header table?
    ELF_Section_Header_Table = 0X00000020,

    // is there a full memory map?
    Memory_Map = 0x00000040,

    // Is there drive info?
    Drive_Info = 0x00000080,

    // Is there a config table?
    Config_Table = 0x00000100,

    // Is there a boot loader name?
    Boot_Loader_Name = 0x00000200,

    // Is there a APM table?
    APM_Table = 0x00000400,

    // Is there video information?
    VBE_Info = 0x00000800,
    Framebuffer_Info = 0x00001000,
};

#endif // KERNEL_MULTIBOOT_H
