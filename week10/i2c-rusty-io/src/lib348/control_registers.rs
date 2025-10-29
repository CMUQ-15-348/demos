use core::ptr::{read_volatile, write_volatile};

/* Some helper functions to directly read/write registers.
 * The are unsafe because they dereference raw pointers.
 * The are volatile because the compiler should not optimize them away.
 */
pub fn read_reg(addr: u32) -> u32 {
    unsafe { read_volatile(addr as *const u32) }
}

pub fn write_reg(addr: u32, value: u32) {
    unsafe {
        write_volatile(addr as *mut u32, value);
    }
}

pub fn set_bits(addr: u32, mask: u32) {
    unsafe {
        // Atomic set on write.  See 2.1.2 in the datasheet
        write_volatile((addr + 0x2000) as *mut u32, mask);
    }
}

pub fn clear_bits(addr: u32, mask: u32) {
    unsafe {
        // Atomic clear on write.  See 2.1.2 in the datasheet
        write_volatile((addr + 0x3000) as *mut u32, mask);
    }
}

/* Some base addresses for pointers.  These are taken directly from the rp2040 datasheet. */
pub const ROM_BASE: u32 = 0x0000_0000_u32;
pub const XIP_BASE: u32 = 0x1000_0000_u32;
pub const XIP_MAIN_BASE: u32 = 0x1000_0000_u32;
pub const XIP_NOALLOC_BASE: u32 = 0x1100_0000_u32;
pub const XIP_NOCACHE_BASE: u32 = 0x1200_0000_u32;
pub const XIP_NOCACHE_NOALLOC_BASE: u32 = 0x1300_0000_u32;
pub const XIP_CTRL_BASE: u32 = 0x1400_0000_u32;
pub const XIP_SRAM_BASE: u32 = 0x1500_0000_u32;
pub const XIP_SRAM_END: u32 = 0x1500_4000_u32;
pub const XIP_SSI_BASE: u32 = 0x1800_0000_u32;
pub const SRAM_BASE: u32 = 0x2000_0000_u32;
pub const SRAM_STRIPED_BASE: u32 = 0x2000_0000_u32;
pub const SRAM_STRIPED_END: u32 = 0x2004_0000_u32;
pub const SRAM4_BASE: u32 = 0x2004_0000_u32;
pub const SRAM5_BASE: u32 = 0x2004_1000_u32;
pub const SRAM_END: u32 = 0x2004_2000_u32;
pub const SRAM0_BASE: u32 = 0x2100_0000_u32;
pub const SRAM1_BASE: u32 = 0x2101_0000_u32;
pub const SRAM2_BASE: u32 = 0x2102_0000_u32;
pub const SRAM3_BASE: u32 = 0x2103_0000_u32;
pub const SYSINFO_BASE: u32 = 0x4000_0000_u32;
pub const SYSCFG_BASE: u32 = 0x4000_4000_u32;
pub const CLOCKS_BASE: u32 = 0x4000_8000_u32;
pub const RESETS_BASE: u32 = 0x4000_c000_u32;
pub const PSM_BASE: u32 = 0x4001_0000_u32;
pub const IO_BANK0_BASE: u32 = 0x4001_4000_u32;
pub const IO_QSPI_BASE: u32 = 0x4001_8000_u32;
pub const PADS_BANK0_BASE: u32 = 0x4001_c000_u32;
pub const PADS_QSPI_BASE: u32 = 0x4002_0000_u32;
pub const XOSC_BASE: u32 = 0x4002_4000_u32;
pub const PLL_SYS_BASE: u32 = 0x4002_8000_u32;
pub const PLL_USB_BASE: u32 = 0x4002_c000_u32;
pub const BUSCTRL_BASE: u32 = 0x4003_0000_u32;
pub const UART0_BASE: u32 = 0x4003_4000_u32;
pub const UART1_BASE: u32 = 0x4003_8000_u32;
pub const SPI0_BASE: u32 = 0x4003_c000_u32;
pub const SPI1_BASE: u32 = 0x4004_0000_u32;
pub const I2C0_BASE: u32 = 0x4004_4000_u32;
pub const I2C1_BASE: u32 = 0x4004_8000_u32;
pub const ADC_BASE: u32 = 0x4004_c000_u32;
pub const PWM_BASE: u32 = 0x4005_0000_u32;
pub const TIMER_BASE: u32 = 0x4005_4000_u32;
pub const WATCHDOG_BASE: u32 = 0x4005_8000_u32;
pub const RTC_BASE: u32 = 0x4005_c000_u32;
pub const ROSC_BASE: u32 = 0x4006_0000_u32;
pub const VREG_AND_CHIP_RESET_BASE: u32 = 0x4006_4000_u32;
pub const TBMAN_BASE: u32 = 0x4006_c000_u32;
pub const DMA_BASE: u32 = 0x5000_0000_u32;
pub const USBCTRL_DPRAM_BASE: u32 = 0x5010_0000_u32;
pub const USBCTRL_BASE: u32 = 0x5010_0000_u32;
pub const USBCTRL_REGS_BASE: u32 = 0x5011_0000_u32;
pub const PIO0_BASE: u32 = 0x5020_0000_u32;
pub const PIO1_BASE: u32 = 0x5030_0000_u32;
pub const XIP_AUX_BASE: u32 = 0x5040_0000_u32;
pub const SIO_BASE: u32 = 0xd000_0000_u32;
pub const PPB_BASE: u32 = 0xe000_0000_u32;

/* Some offsets into those registers */
pub const IC_ENABLE: u32 = 0x6c;
pub const IC_CON: u32 = 0x00;
pub const IC_TAR: u32 = 0x04;
pub const IC_DATA_CMD: u32 = 0x10;
pub const IC_STATUS: u32 = 0x70;
pub const IC_TXFLR: u32 = 0x74;
pub const TIMEHR: u32 = 0x08;
pub const TIMELR: u32 = 0x0c;
pub const RESET_DONE: u32 = 0x08;
pub const SIO_GPIO_OE_SET: u32 = 0x24;
pub const SIO_GPIO_OE_CLR: u32 = 0x28;
pub const SIO_GPIO_OUT: u32 = 0x10;
pub const SIO_GPIO_OUT_SET: u32 = 0x14;
pub const SIO_GPIO_OUT_CLR: u32 = 0x18;
pub const SIO_GPIO_OUT_XOR: u32 = 0x1c;
pub const SIO_GPIO_IN: u32 = 0x04;
