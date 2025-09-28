use crate::lib348::control_registers::*;

pub const RESET_PLL_SYS: u32 = 12; // Select PLL_SYS to be reset
pub const RESET_PLL_USB: u32 = 13; // Select PLL_USB to be reset

/*
 * Configure the system clock to 125 MHz.
 * There is a nice reference in the SDK:
 * https://github.com/raspberrypi/pico-sdk/blob/ee68c78d0afae2b69c03ae1a72bf5cc267a2d94c/src/rp2_common/pico_runtime_init/runtime_init_clocks.c#L40
 *
 * In the early weeks of the class, you are not required to understand what
 * this code does. Once we cover the clocks, you should be able to follow
 * along here while using the datasheet as a reference.
 *
 * This use of this code is optional.  If you don't do it, then the default
 * system clock comes from the ring oscillator, which is about 6 Mhz.
 */
pub fn init_clocks() {
    // Enable the XOSC (2.16.7)
    write_reg(XOSC_BASE + 0x00, 0x00fabaa0);
    while read_reg(XOSC_BASE + 0x04) & 0x8000_0000_u32 == 0 {
        // Wait for the XOSC to be ready
    }

    // Set the CLK_SYS glitchless mux to 0 (CLK_REF) so that we can mess with the CLK_SYS sources without causing issues.
    clear_bits(CLOCKS_BASE + 0x3c, 0x1);
    while read_reg(CLOCKS_BASE + 0x44) != 0x1 {
        // Wait for the glitchless mux to be set
    }

    // Set the CLK_REF glitchless mux to 0 (Ring oscillator)
    clear_bits(CLOCKS_BASE + 0x30, 0b11);
    while read_reg(CLOCKS_BASE + 0x38) != 1 {
        // Wait for the glitchless mux to be set
    }

    // Reset and configure PLL_SYS with refdiv = 1, fbdiv = 125, pd1 = 6, pd2 = 2
    // This 125 MHz
    init_pll(PLL_SYS_BASE, RESET_PLL_SYS, 1, 125, 6, 2);

    // Reset and configure PLL_USB with refdiv = 1, fbdiv = 100, pd1 = 5, pd2 = 5
    // This is 48 Mhz
    init_pll(PLL_USB_BASE, RESET_PLL_USB, 1, 100, 5, 5);

    // Move CLK_REF over to the crystal oscillator
    write_reg(CLOCKS_BASE + 0x34, 1 << 8); // Divider is 1
    write_reg(CLOCKS_BASE + 0x30, 0x2); // SRC = XOSC (glitchless)

    // Move CLK_SYS over to the aux src (which is the PLL)
    write_reg(CLOCKS_BASE + 0x40, 1 << 8); // Divider is 1
    clear_bits(CLOCKS_BASE + 0x3c, 0b111 << 5); // Auxsrc: PLL_SYS
    set_bits(CLOCKS_BASE + 0x3c, 0x0000_0001); // Set the glitchless mux to 1 (CLKSRC_CLK_SYS_AUX) so that we now use the PLL coming in on AUX.

    // Set the USB clock to be the same as PLL_USB (which should be 48Mhz)
    write_reg(CLOCKS_BASE + 0x54, 0); // Disable the clock by clearing bit 11.  This also sets AUXSRC to 0, which is PLL_USB
    let _ = read_reg(CLOCKS_BASE); // Read the register just to stall for some cycles (we're waiting to make sure the clock peripheral clock is actually stopped)
    set_bits(CLOCKS_BASE + 0x54, 1 << 11); // Enable it

    // Set the ADC clock to be the same as PLL_USB (which should be 48Mhz)
    write_reg(CLOCKS_BASE + 0x60, 0); // Disable the clock by clearing bit 11.  This also sets AUXSRC to 0, which is PLL_USB
    let _ = read_reg(CLOCKS_BASE); // Read the register just to stall for some cycles (we're waiting to make sure the clock peripheral clock is actually stopped)
    set_bits(CLOCKS_BASE + 0x60, 1 << 11); // Enable it

    // Set the peripheral clock to be the same as clk_sys
    write_reg(CLOCKS_BASE + 0x48, 0); // Disable the clock by clearing bit 11.  This also sets AUXSRC to 0, which is CLK_SYS
    let _ = read_reg(CLOCKS_BASE); // Read the register just to stall for some cycles (we're waiting to make sure the clock peripheral clock is actually stopped)
    set_bits(CLOCKS_BASE + 0x48, 1 << 11); // Enable it

    // Configure the watchdog tick counter so that it divides by 12, leading to one tick every us.  (Because the XOSC is 12MHz.)
    // Without this being set properly, the TIMER doesn't count at the correct interval.
    write_reg(WATCHDOG_BASE + 0x2c, 12 | 1 << 9); // Set the divider to 12 and enable the watchdog

    // Configure the timer not to pause during debugging. Otherwise, we can't single step debug through a delay()
    // See...
    // https://github.com/raspberrypi/debugprobe/issues/45
    // https://github.com/raspberrypi/pico-sdk/issues/1586
    write_reg(TIMER_BASE + 0x2c, 0);
}

fn init_pll(pll_base: u32, peri_num: u32, refdiv: u32, fbdiv: u32, pd1: u32, pd2: u32) {
    // Reset, then deassert the reset on the PLL
    // See Section 2.14 in the datasheet for details
    set_bits(RESETS_BASE, 1 << peri_num); // Write 1 to reset
    clear_bits(RESETS_BASE, 1 << peri_num); // Write 0 to deassert reset

    // Load refdiv and fbdiv
    write_reg(pll_base + 0x00, refdiv);
    write_reg(pll_base + 0x08, fbdiv);

    // Clear the PD and VCO bits to enable the PLL
    clear_bits(pll_base + 0x04, 1 << 0 | 1 << 5);
    while read_reg(pll_base + 0x00) & 1 << 31 == 0 {
        // Wait for the PLL to be ready
    }

    // Setup the post dividers
    write_reg(pll_base + 0x0C, pd1 << 16 | pd2 << 12);

    // Turn on the post divider
    clear_bits(pll_base + 0x04, 1 << 3);
}
