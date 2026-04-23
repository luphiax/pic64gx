/* # Developer notes

- Symbols that start with a double underscore (__) are considered "private"

- Symbols that start with a single underscore (_) are considered "semi-public"; they can be
  overridden in a user linker script, but should not be referred from user code

- `EXTERN` forces the linker to keep a symbol in the final binary. We use this to make sure a
  symbol is not dropped if it appears in or near the front of the linker arguments

- `PROVIDE` is used to provide default values that can be overridden by a user linker script
*/

PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 0);

/** TRAP ENTRY POINTS **/

EXTERN(_start_trap);
PROVIDE(_start_DefaultHandler_trap = _start_trap);
PROVIDE(_start_SupervisorSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineSoft_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineTimer_trap = _start_DefaultHandler_trap);
PROVIDE(_start_SupervisorExternal_trap = _start_DefaultHandler_trap);
PROVIDE(_start_MachineExternal_trap = _start_DefaultHandler_trap);

/** EXCEPTION HANDLERS **/

EXTERN(ExceptionHandler);
PROVIDE(InstructionMisaligned = ExceptionHandler);
PROVIDE(InstructionFault = ExceptionHandler);
PROVIDE(IllegalInstruction = ExceptionHandler);
PROVIDE(Breakpoint = ExceptionHandler);
PROVIDE(LoadMisaligned = ExceptionHandler);
PROVIDE(LoadFault = ExceptionHandler);
PROVIDE(StoreMisaligned = ExceptionHandler);
PROVIDE(StoreFault = ExceptionHandler);
PROVIDE(UserEnvCall = ExceptionHandler);
PROVIDE(SupervisorEnvCall = ExceptionHandler);
PROVIDE(MachineEnvCall = ExceptionHandler);
PROVIDE(InstructionPageFault = ExceptionHandler);
PROVIDE(LoadPageFault = ExceptionHandler);
PROVIDE(StorePageFault = ExceptionHandler);

/** INTERRUPT HANDLERS **/

EXTERN(DefaultHandler);
PROVIDE(SupervisorSoft = DefaultHandler);
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(SupervisorTimer = DefaultHandler);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(SupervisorExternal = DefaultHandler);
PROVIDE(MachineExternal = DefaultHandler);

SECTIONS
{
  .text.dummy (NOLOAD) :
  {
    . = ABSOLUTE(_stext);
  } > REGION_TEXT

  .text _stext :
  {
    KEEP(*(.init));
    KEEP(*(.init.rust));
    . = ALIGN(4);
    KEEP(*(.init.trap));
    . = ALIGN(4);
    *(.trap);
    *(.trap.rust);
    *(.text.abort);
    *(.text .text.*);
  } > REGION_TEXT

  .rodata : ALIGN(4)
  {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > REGION_RODATA

  .data : ALIGN(8)
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.sdata .sdata.* .sdata2 .sdata2.*);
    *(.data .data.*);
    . = ALIGN(8);
    _edata = .;
  } > REGION_DATA AT > REGION_RODATA

  .bss (NOLOAD) : ALIGN(8)
  {
    _sbss = .;
    *(.sbss .sbss.* .bss .bss.*);
    . = ALIGN(8);
    _ebss = .;
  } > REGION_BSS

  .heap (NOLOAD) :
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } > REGION_HEAP

  .stack (NOLOAD) :
  {
    _estack = .;
    . = ABSOLUTE(_stack_start);
    _sstack = .;
  } > REGION_STACK

  .got (INFO) :
  {
    KEEP(*(.got .got.*));
  }

  /DISCARD/ :
  {
    *(.eh_frame)
    *(.eh_frame_hdr)
  }
}

ASSERT(ORIGIN(REGION_TEXT) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_TEXT must be 4-byte aligned");

ASSERT(ORIGIN(REGION_RODATA) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_RODATA must be 4-byte aligned");

ASSERT(ORIGIN(REGION_DATA) % 8 == 0, "
ERROR(riscv-rt): the start of the REGION_DATA must be 8-byte aligned");

ASSERT(ORIGIN(REGION_HEAP) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_HEAP must be 4-byte aligned");

ASSERT(ORIGIN(REGION_STACK) % 4 == 0, "
ERROR(riscv-rt): the start of the REGION_STACK must be 4-byte aligned");

ASSERT(_stext % 4 == 0, "
ERROR(riscv-rt): `_stext` must be 4-byte aligned");

ASSERT(_sdata % 8 == 0 && _edata % 8 == 0, "
BUG(riscv-rt): .data is not 8-byte aligned");

ASSERT(_sidata % 8 == 0, "
BUG(riscv-rt): the LMA of .data is not 8-byte aligned");

ASSERT(_sbss % 8 == 0 && _ebss % 8 == 0, "
BUG(riscv-rt): .bss is not 8-byte aligned");

ASSERT(_sheap % 4 == 0, "
BUG(riscv-rt): start of .heap is not 4-byte aligned");

ASSERT(_stext + SIZEOF(.text) < ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT), "
ERROR(riscv-rt): The .text section must be placed inside the REGION_TEXT region.
Set _stext to an address smaller than 'ORIGIN(REGION_TEXT) + LENGTH(REGION_TEXT)'");

ASSERT(SIZEOF(.stack) > (_max_hart_id + 1) * _hart_stack_size, "
ERROR(riscv-rt): .stack section is too small for allocating stacks for all the harts.
Consider changing `_max_hart_id` or `_hart_stack_size`.");

ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported.");
