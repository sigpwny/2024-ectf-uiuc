MEMORY {
    ROM         (rx) : ORIGIN = 0x00000000, LENGTH = 0x00010000 /* 64kB ROM */
    BOOTLOADER  (rx) : ORIGIN = 0x10000000, LENGTH = 0x0000E000 /* Bootloader flash */
    FLASH       (rx) : ORIGIN = 0x1000E000, LENGTH = 0x00038000 /* Location of team firmware */
    RESERVED    (rw) : ORIGIN = 0x10046000, LENGTH = 0x00038000 /* Reserved */
    ROM_BL_PAGE (rw) : ORIGIN = 0x1007E000, LENGTH = 0x00002000 /* Reserved */
    RAM        (rwx): ORIGIN = 0x20000000, LENGTH = 0x00010000 /* 64kB RAM */
}

SECTIONS {
    .rom :
    {
        KEEP(*(.rom_vector))
        *(.rom_handlers*)
    } > ROM

    /* Binary import */
    .bin_storage :
    {
       FILL(0xFF)
      _bin_start_ = .;
      KEEP(*(.bin_storage_img))
      _bin_end_ = .;
      . = ALIGN(4);
    } > FLASH
    
    .rom_code :
    {
        . = ALIGN(16);
        _sran_code = .;
        *(.rom_code_section)
        _esran_code = .;
    } > ROM

    .flash_code :
    {
        . = ALIGN(16);
        _sran_code = .;
        *(.flash_code_section)
        _esran_code = .;
    } > FLASH

    .data :
    {
        _data = ALIGN(., 4);
        *(.data*)           /*read-write initialized data: initialized global variable*/
        *(.flashprog*)      /* Flash program */
    } > RAM AT>FLASH
    __load_data = LOADADDR(.data);

    /* Set stack top to end of RAM, and stack limit move down by
     * size of stack_dummy section */
    __StackTop = ORIGIN(RAM) + LENGTH(RAM);
    __StackLimit = __StackTop - SIZEOF(.stack_dummy);

    /* .stack_dummy section doesn't contains any symbols. It is only
     * used for linker to calculate size of stack sections, and assign
     * values to stack symbols later */
    .stack_dummy (COPY):
    {
        *(.stack*)
    } > RAM

    PROVIDE(__stack = __StackTop);
}
