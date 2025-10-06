#![no_std]
#![no_main]

// Declare that there is a lib348 library inside this project
pub mod lib348;

// Use some crates provides by others
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp2040_boot2;

// Use our own lib348 library components that is inside this project
use crate::lib348::control_registers::*;
use crate::lib348::sys_clock;

/* The stage 2 bootloader. */
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const LED_PIN: u32 = 25;
const BTN_PIN: u32 = 15;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
enum States {
    WAITING,
    PRESSED,
    DEBOUNCE_RELEASE,
    RELEASED,
}

struct StateTableEntry {
    next: [States; 2],
    output: bool,
    min_time: u64,
}

static STATE_TABLE: [StateTableEntry; 4] = [
    StateTableEntry {
        // IDLE
        next: [States::WAITING, States::PRESSED],
        output: false,
        min_time: 0,
    },
    StateTableEntry {
        // PRESSED
        next: [States::DEBOUNCE_RELEASE, States::PRESSED],
        output: false,
        min_time: 7,
    },
    StateTableEntry {
        // MID
        next: [States::RELEASED, States::RELEASED],
        output: false,
        min_time: 3,
    },
    StateTableEntry {
        // RELEASED
        next: [States::WAITING, States::PRESSED],
        output: true,
        min_time: 0,
    },
];

fn read_pin(pin: u32) -> usize {
    let val = read_reg(SIO_BASE + SIO_GPIO_IN) as usize; // Read all pins
    let val = val & (1 << pin); // Mask out only the pin we we want
    let val = val >> pin; // Shift that pin's bit to the 1s position
    let val = val & 0x1; // Mask off any bits from other pins
    val
}

/* Run one step in our debouncing state machine.  Returns true if we
 * end up in an output state, and false otherwise.
 */
fn check_btn(time_counter: &mut u64, cur_state: &mut States, inp: usize) -> bool {
    // Check if we've been in the state long enough to move on if needed.
    if *time_counter > STATE_TABLE[*cur_state as usize].min_time {
        // Reset the time if I'm going to change states
        let new_state = STATE_TABLE[*cur_state as usize].next[inp];
        if *cur_state != new_state {
            *time_counter = 0;
        }

        // Move to the next state
        *cur_state = new_state;
    }
    STATE_TABLE[*cur_state as usize].output
}

/*
 * Code to initialize the pin/pad for a single output pin
 */
fn init_pins(output_pin: u32, input_pin: u32) {
    // Reset, then deassert the reset on IO_BANK0
    // See Section 2.14 in the datasheet for details
    set_bits(RESETS_BASE, 1 << 5); // Write 1 to reset
    clear_bits(RESETS_BASE, 1 << 5); // Write 0 to deassert reset

    // Reset, then deassert the reset on PADS_BANK0
    // See Section 2.14 in the datasheet for details
    set_bits(RESETS_BASE, 1 << 8); // Write 1 to reset
    clear_bits(RESETS_BASE, 1 << 8); // Write 0 to deassert reset

    // Configure the output pin pad.  Writing 0 disables input and enables output for that
    // pad. See Table 339 and Table 341 in the datasheet for details
    write_reg(PADS_BANK0_BASE + (output_pin + 1) * 4, 0);

    // Configure the input pin pad.  Enable output disable and input enable
    set_bits(PADS_BANK0_BASE + (input_pin + 1) * 4, 1 << 7 | 1 << 6);

    // Enable pulldown on the input pad
    set_bits(PADS_BANK0_BASE + (input_pin + 1) * 4, 1 << 2);
    // Disable pullup
    clear_bits(PADS_BANK0_BASE + (input_pin + 1) * 4, 1 << 3);

    // Configure IO_BANK0: Set GPIO??_CTRL.funcsel = 5, which selects SIO control.
    // The IO_BANK0 peripheral base address is 0x4001_4000. According to the
    // datasheet, each GPIO has 8 bytes of registers. For example, the GPIO15
    // CTRL register is located at:   offset = (15 * 8) + 4 = 124 (0x7C)
    // See Table 283, Table 285, and Table 279 in the datasheet for details
    write_reg(IO_BANK0_BASE + (output_pin * 8 + 4), 5);
    write_reg(IO_BANK0_BASE + (input_pin * 8 + 4), 5);

    // Configure SIO: Enable output for GPIO??.
    // The SIO peripheral base address is 0xD000_0000.
    // The GPIO_OE_SET register is at offset 0x024.
    // We first need to enable the output driver for GPIO??.
    // See Table 16 and Table 25 in the datasheet for details
    write_reg(SIO_BASE + SIO_GPIO_OE_SET, 1 << output_pin);
    write_reg(SIO_BASE + SIO_GPIO_OE_CLR, 1 << input_pin);
}

/*
 * The actual main function.
 */
#[entry]
fn main() -> ! {
    // Initialize the clocks and IO pins
    sys_clock::init_clocks();
    init_pins(LED_PIN, BTN_PIN);

    // Turn off the LED
    write_reg(SIO_BASE + SIO_GPIO_OUT_CLR, 1 << LED_PIN);

    let mut cur_state: States = States::WAITING;

    let mut time_counter: u64 = 0;
    info!("Go!");
    loop {
        // Increment our time counter.  How long have we been in this state?
        time_counter = time_counter + 1;

        // Run one step of our debouncing state machine
        let pin_val = read_pin(BTN_PIN);
        let res = check_btn(&mut time_counter, &mut cur_state, pin_val);

        // If we are in an output state, then do our output action.
        if res {
            // Toggle the LED
            info!("Toggle!");
            write_reg(SIO_BASE + SIO_GPIO_OUT_XOR, 1 << LED_PIN);
        }

        sys_clock::delay(1);
    }
}
