_max_hart_id = 1;

MEMORY
{
    /* Workaround for the pcrel issue, as described here:
       https://github.com/rust-embedded/riscv-rt/issues/25#issuecomment-491518168 */
    SRAM : ORIGIN = 0xffffffff80000000, LENGTH = 6M
    AI_SRAM : ORIGIN = 0xffffffff80600000, LENGTH = 2M
    SRAM_NOCACHE : ORIGIN = 0x40000000, LENGTH = 6M
    AI_SRAM_NOCACHE : ORIGIN = 0x40600000, LENGTH = 2M
}
