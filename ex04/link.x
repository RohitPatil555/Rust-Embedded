MEMORY
{
	FLASH : ORIGIN = 0x08000000, LENGTH = 128K
	STACK: ORIGIN = 0x20000000, LENGTH = 1K
	RAM : ORIGIN = 0x20000400, LENGTH = 7K
}

ENTRY(Reset)
EXTERN(RESET_VECTOR)

heap_start_addr = ORIGIN(RAM);
heap_end_addr = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(STACK) + LENGTH(STACK));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

