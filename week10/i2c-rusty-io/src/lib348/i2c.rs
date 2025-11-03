use crate::lib348::control_registers::*;
use crate::lib348::pin_io;
use core::cell::Cell;

pub struct I2C1 {
    address: Cell<u8>,
    sda: pin_io::Pin,
    scl: pin_io::Pin,
}

impl I2C1 {
    pub fn new(sda: pin_io::Pin, scl: pin_io::Pin) -> Self {
        I2C1 {
            address: Cell::new(0u8),
            sda: sda,
            scl: scl,
        }
    }

    pub fn init(&self) {
        // Reset, then deassert the reset on I2C
        // See Section 2.14 in the datasheet for details
        set_bits(RESETS_BASE, 1 << 4); // Write 1 to reset
        clear_bits(RESETS_BASE, 1 << 4); // Write 0 to deassert reset

        // Configure the pads.  For I2C we need pull-up enabled, slew rate limited, schmitt trigger enabled.
        // See Table 339 and Table 341 in the datasheet for details
        let the_bits = (1 << 6) | (1 << 3) | (1 << 1) | (1 << 0); // This sets slew rate to 0.  I'm not sure what the datasheet means by "slew rate limited", so I'll try this.
        write_reg(PADS_BANK0_BASE + (self.sda.pin_num + 1) * 4, the_bits);
        write_reg(PADS_BANK0_BASE + (self.scl.pin_num + 1) * 4, the_bits);

        // Configure IO_BANK0: Set GPIO??_CTRL.funcsel = 3, which selects I2C.
        // The IO_BANK0 peripheral base address is 0x4001_4000. According to the datasheet,
        // each GPIO has 8 bytes of registers. For example, the GPIO15 CTRL register is located at:
        //   offset = (15 * 8) + 4 = 124 (0x7C)
        // See Table 283, Table 285, and Table 279 in the datasheet for details
        write_reg(IO_BANK0_BASE + (self.sda.pin_num * 8 + 4), 3);
        write_reg(IO_BANK0_BASE + (self.scl.pin_num * 8 + 4), 3);

        // Disable I2C
        clear_bits(I2C1_BASE + IC_ENABLE, 0x0000_0001);

        // Setup I2C the way we want: Master enable, slave disabled, standard speed, 7-bit addressing
        let the_bits = (1 << 0) | (1 << 6) | (1 << 1) | (0 << 4);
        write_reg(I2C1_BASE + IC_CON, the_bits);

        // Setup the control registers related to timing
        set_cnts(125_000_000, 400_000);

        // Enable I2C
        set_bits(I2C1_BASE + IC_ENABLE, 0x0000_0001);
    }

    fn set_address(&self, address: u8) {
        self.address.set(address);

        // Disable I2C
        clear_bits(I2C1_BASE + IC_ENABLE, 0x0000_0001);

        // Configure the slave address
        write_reg(I2C1_BASE + IC_TAR, self.address.get() as u32);

        // Enable I2C
        set_bits(I2C1_BASE + IC_ENABLE, 0x0000_0001);
    }

    pub fn write(&self, addr: u8, reg: u8, val: u8) {
        if addr != self.address.get() {
            self.set_address(addr);
        }

        // Write the register address.  Restart = 1, stop = 0, write mode
        write_reg(
            I2C1_BASE + IC_DATA_CMD,
            (1 << 10) | (0 << 9) | (0 << 8) | reg as u32,
        );

        // Write the value. Restart = 0, stop = 1, write mode
        write_reg(
            I2C1_BASE + IC_DATA_CMD,
            (0 << 10) | (1 << 9) | (0 << 8) | val as u32,
        );

        // Wait until any previous transfers are done (the I2C active bit should be 0 to proceed)
        while read_reg(I2C1_BASE + IC_STATUS) & (1 << 0) == 1 {
            // Wait for the I2C to be inactive
        }
    }

    pub fn read(&self, addr: u8, reg: u8, bytes: &mut [u8]) {
        if addr != self.address.get() {
            self.set_address(addr);
        }

        // Write the register address.  Restart = 1, stop = 0, write mode mode
        write_reg(
            I2C1_BASE + IC_DATA_CMD,
            (1 << 10) | (0 << 9) | (0 << 8) | reg as u32,
        );

        // Read num_bytes values
        for i in 0..bytes.len() {
            // Instruct pico to read one byte.  If it is the last byte, send a stop signal afterwards.
            if i == bytes.len() - 1 {
                // Restart = 0, stop = 1, read mode
                write_reg(
                    I2C1_BASE + IC_DATA_CMD,
                    (0 << 10) | (1 << 9) | (1 << 8) | reg as u32,
                );
            } else {
                // Restart = 0, stop = 0, read mode
                write_reg(
                    I2C1_BASE + IC_DATA_CMD,
                    (0 << 10) | (0 << 9) | (1 << 8) | reg as u32,
                );
            }

            // Wait for some data to appear in the read fifo
            while read_reg(I2C1_BASE + IC_STATUS) & (1 << 3) == 0 {
                // Wait for the RX FIFO to have data
            }
            // Read the value
            bytes[i] = (read_reg(I2C1_BASE + IC_DATA_CMD) & 0xFF) as u8;
        }
    }
}

/* Calculate the HCNT and LCNT values needed based on the clock rates
 * sys_clk_freq is probably 125_000_000
 * baudrate is likely 400_000
 */
fn set_cnts(sys_clk_freq: u32, baudrate: u32) {
    let period = (sys_clk_freq + baudrate / 2) / baudrate;
    let lcnt = period * 3 / 5;
    let hcnt = period - lcnt;

    let spklen = if lcnt < 16 { 1 } else { lcnt / 16 };

    let sda_tx_hold_count = if baudrate < 1_000_000 {
        ((sys_clk_freq * 3) / 10_000_000) + 1
    } else {
        ((sys_clk_freq * 3) / 20_000_000) + 1
    };

    write_reg(I2C1_BASE + IC_FS_SCL_HCNT, hcnt);
    write_reg(I2C1_BASE + IC_FS_SCL_LCNT, lcnt);
    write_reg(I2C1_BASE + IC_FS_SPKLEN, spklen);
    let val = read_reg(I2C1_BASE + IC_SDA_HOLD) & 0xffff0000;
    write_reg(I2C1_BASE + IC_SDA_HOLD, val | sda_tx_hold_count);
}
