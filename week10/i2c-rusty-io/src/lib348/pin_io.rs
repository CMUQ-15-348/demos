use crate::lib348::control_registers::*;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

static PINS_TAKEN: Mutex<RefCell<[bool; 30]>> = Mutex::new(RefCell::new([
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false,
]));

pub fn get_pin(pin_number: u32) -> Option<Pin> {
    if pin_number < 30 {
        cortex_m::interrupt::free(|cs| {
            let rc = PINS_TAKEN.borrow(cs);
            let mut pin_arr = rc.borrow_mut();
            if pin_arr[pin_number as usize] == false {
                pin_arr[pin_number as usize] = true;
                Some(Pin {
                    pin_num: pin_number,
                })
            } else {
                None
            }
        })
    } else {
        None
    }
}

pub struct Pin {
    pub pin_num: u32,
}

// Unconfigured GPIO pin
impl Pin {
    pub fn into_output(self) -> OutputPin {
        // Configure the pads.  Writing 0 disables input and enables output for that
        // pad. See Table 339 and Table 341 in the datasheet for details
        write_reg(PADS_BANK0_BASE + (self.pin_num + 1) * 4, 0);

        // Configure IO_BANK0: Set GPIO??_CTRL.funcsel = 5, which selects SIO control.
        // The IO_BANK0 peripheral base address is 0x4001_4000. According to the
        // datasheet, each GPIO has 8 bytes of registers. For example, the GPIO15
        // CTRL register is located at:   offset = (15 * 8) + 4 = 124 (0x7C)
        // See Table 283, Table 285, and Table 279 in the datasheet for details
        write_reg(IO_BANK0_BASE + (self.pin_num * 8 + 4), 5);

        // Configure SIO: Enable output for GPIO??.
        // The SIO peripheral base address is 0xD000_0000.
        // The GPIO_OE_SET register is at offset 0x024.
        // We first need to enable the output driver for GPIO??.
        // See Table 16 and Table 25 in the datasheet for details
        write_reg(SIO_BASE + 0x024, 1 << self.pin_num);
        OutputPin { pin: self }
    }

    pub fn into_input(&self) {
        // Pretend that this had the code to configure self.pin_num for input
    }
}

pub struct OutputPin {
    pin: Pin,
}
impl OutputPin {
    pub fn set_hi(&self) {
        write_reg(SIO_BASE + 0x014, 1 << self.pin.pin_num);
    }

    pub fn set_lo(&self) {
        write_reg(SIO_BASE + 0x018, 1 << self.pin.pin_num);
    }
}
