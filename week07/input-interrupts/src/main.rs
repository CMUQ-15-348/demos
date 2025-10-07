#![no_std]
#![no_main]

// Declare that there is a lib348 library inside this project
pub mod lib348;

// Use some crates provides by others
//use cortex_m;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Use our own lib348 library components that are inside this project
use crate::lib348::control_registers::*;
use crate::lib348::sio;
use crate::lib348::sys_clock;

const LED_PIN: u32 = 25;
const INP_PIN: u32 = 2;

/* The stage 2 bootloader.
 * This is taken from https://github.com/rp-rs/rp2040-boot2/blob/main/bin/boot2_w25q080.padded.bin
 * xxd --include <file.bin> can format the bytes for you.
 */
#[link_section = ".boot2"]
#[no_mangle]
#[used]
pub static BOOT2_FIRMWARE: [u8; 256] = [
    0x00, 0xb5, 0x32, 0x4b, 0x21, 0x20, 0x58, 0x60, 0x98, 0x68, 0x02, 0x21, 0x88, 0x43, 0x98, 0x60,
    0xd8, 0x60, 0x18, 0x61, 0x58, 0x61, 0x2e, 0x4b, 0x00, 0x21, 0x99, 0x60, 0x02, 0x21, 0x59, 0x61,
    0x01, 0x21, 0xf0, 0x22, 0x99, 0x50, 0x2b, 0x49, 0x19, 0x60, 0x01, 0x21, 0x99, 0x60, 0x35, 0x20,
    0x00, 0xf0, 0x44, 0xf8, 0x02, 0x22, 0x90, 0x42, 0x14, 0xd0, 0x06, 0x21, 0x19, 0x66, 0x00, 0xf0,
    0x34, 0xf8, 0x19, 0x6e, 0x01, 0x21, 0x19, 0x66, 0x00, 0x20, 0x18, 0x66, 0x1a, 0x66, 0x00, 0xf0,
    0x2c, 0xf8, 0x19, 0x6e, 0x19, 0x6e, 0x19, 0x6e, 0x05, 0x20, 0x00, 0xf0, 0x2f, 0xf8, 0x01, 0x21,
    0x08, 0x42, 0xf9, 0xd1, 0x00, 0x21, 0x99, 0x60, 0x1b, 0x49, 0x19, 0x60, 0x00, 0x21, 0x59, 0x60,
    0x1a, 0x49, 0x1b, 0x48, 0x01, 0x60, 0x01, 0x21, 0x99, 0x60, 0xeb, 0x21, 0x19, 0x66, 0xa0, 0x21,
    0x19, 0x66, 0x00, 0xf0, 0x12, 0xf8, 0x00, 0x21, 0x99, 0x60, 0x16, 0x49, 0x14, 0x48, 0x01, 0x60,
    0x01, 0x21, 0x99, 0x60, 0x01, 0xbc, 0x00, 0x28, 0x00, 0xd0, 0x00, 0x47, 0x12, 0x48, 0x13, 0x49,
    0x08, 0x60, 0x03, 0xc8, 0x80, 0xf3, 0x08, 0x88, 0x08, 0x47, 0x03, 0xb5, 0x99, 0x6a, 0x04, 0x20,
    0x01, 0x42, 0xfb, 0xd0, 0x01, 0x20, 0x01, 0x42, 0xf8, 0xd1, 0x03, 0xbd, 0x02, 0xb5, 0x18, 0x66,
    0x18, 0x66, 0xff, 0xf7, 0xf2, 0xff, 0x18, 0x6e, 0x18, 0x6e, 0x02, 0xbd, 0x00, 0x00, 0x02, 0x40,
    0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x07, 0x00, 0x00, 0x03, 0x5f, 0x00, 0x21, 0x22, 0x00, 0x00,
    0xf4, 0x00, 0x00, 0x18, 0x22, 0x20, 0x00, 0xa0, 0x00, 0x01, 0x00, 0x10, 0x08, 0xed, 0x00, 0xe0,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x74, 0xb2, 0x4e, 0x7a,
];

/* The vector table. We don't have a good way to only change one handler, so
 * we are defining the entire table. They are all the default handlers except
 * for number 13, which is declared below.
 * We declare the default handler as an extern below. The linker sets it up
 * thanks to the device.x file.
 */
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 26] = [
    DefHandler,   //0
    DefHandler,   //1
    DefHandler,   //2
    DefHandler,   //3
    DefHandler,   //4
    DefHandler,   //5
    DefHandler,   //6
    DefHandler,   //7
    DefHandler,   //8
    DefHandler,   //9
    DefHandler,   //10
    DefHandler,   //11
    DefHandler,   //12
    IO_IRQ_BANK0, //13
    DefHandler,   //14
    DefHandler,   //15
    DefHandler,   //16
    DefHandler,   //17
    DefHandler,   //18
    DefHandler,   //19
    DefHandler,   //20
    DefHandler,   //21
    DefHandler,   //22
    DefHandler,   //23
    DefHandler,   //24
    DefHandler,   //25
];

extern "C" {
    fn DefHandler();
}

fn io_reset() {
    // Reset, then deassert the reset on IO_BANK0
    // See Section 2.14 in the datasheet for details
    set_bits(RESETS_BASE, 1 << 5); // Write 1 to reset
    clear_bits(RESETS_BASE, 1 << 5); // Write 0 to deassert reset

    // Reset, then deassert the reset on PADS_BANK0
    // See Section 2.14 in the datasheet for details
    set_bits(RESETS_BASE, 1 << 8); // Write 1 to reset
    clear_bits(RESETS_BASE, 1 << 8); // Write 0 to deassert reset
}

// A shared integer that represents the number of interrupts which have occurred.
static SHARED_DATA: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(None));

/*
 * The actual main function.
 */
#[entry]
fn main() -> ! {
    // Initialize the shared data
    cortex_m::interrupt::free(|cs| {
        *SHARED_DATA.borrow(cs).borrow_mut() = Some(0);
    });

    // Initialize the clocks and IO pins
    sys_clock::init_clocks();
    io_reset();
    sio::init_output(LED_PIN);
    sio::init_input(INP_PIN);
    sio::into_pulldown(INP_PIN);

    // Enable rising edge interrupt on GPIO2 using the INTE register
    write_reg(IO_BANK0_BASE + 0x100, 1 << 11);

    // Clear any pending rising edge using the INTR register
    write_reg(IO_BANK0_BASE + 0x0f0, 1 << 11);

    // Enable IO_BANK0 interrupt in NVIC (interrupt 13)
    write_reg(NVIC_ISER, 1 << 13);

    info!("Start");

    loop {}
}

// Interrupt handler for IO_BANK0 (GPIO interrupts)
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn IO_IRQ_BANK0() {
    // Check: Was this an interrupt from pin 2?
    let status = read_reg(IO_BANK0_BASE + 0x120);
    if (status & (1 << 11)) != 0 {
        // increment the shared counter that represents the number of interrupts seen
        cortex_m::interrupt::free(|cs| {
            if let Some(ref mut data) = *SHARED_DATA.borrow(cs).borrow_mut() {
                *data += 1;
                info!("Value is {}", *data);
            }
        });
        // Clear the interrupt
        write_reg(IO_BANK0_BASE + 0x0f0, 1 << 11); // Clear rising edge event

        // Toggle the LED for fun
        write_reg(SIO_BASE + 0x1c, 1 << LED_PIN);
    }
}
