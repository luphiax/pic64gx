MEMORY
{
    RAM : ORIGIN = 0x91C00000, LENGTH = 1M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
/* Core interrupt sources and trap handlers */
PROVIDE(MachineSoft = DefaultHandler);
PROVIDE(_start_MachineSoft_trap = _start_DefaultHandler_trap);
PROVIDE(MachineTimer = DefaultHandler);
PROVIDE(_start_MachineTimer_trap = _start_DefaultHandler_trap);
PROVIDE(MachineExternal = DefaultHandler);
PROVIDE(_start_MachineExternal_trap = _start_DefaultHandler_trap);

