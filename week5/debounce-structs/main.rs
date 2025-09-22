#![no_std]
#![no_main]

// Declare that there is a lib348 library inside this project
pub mod lib348;

// Use some crates provides by others
// use cortex_m;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Use our own lib348 library components that are inside this project
use crate::lib348::control_registers::*;
use crate::lib348::sio; // You don't have this one because you are expected to write its functionality yourself.
use crate::lib348::sys_clock;

const LED_PIN: u32 = 25;
const SW1_PIN: u32 = 2;
const SW2_PIN: u32 = 3;

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

const WAITING: u8 = 0x00;
const PRESSED: u8 = 0x01;
const DEBOUNCE_RELEASE: u8 = 0x02;
const RELEASED: u8 = 0x03;

struct StateTable {
    next: [u8; 2],
    out: u8,
    min_time: u32,
}

const FSM: [StateTable; 4] = [
    // State WAITING
    StateTable {
        next: [WAITING, PRESSED],
        out: 0x00,
        min_time: 0,
    },
    // State PRESSED
    StateTable {
        next: [DEBOUNCE_RELEASE, PRESSED],
        out: 0x00,
        min_time: 5,
    },
    // State DEBOUNCE_RELEASE
    StateTable {
        next: [RELEASED, RELEASED],
        out: 0x00,
        min_time: 5,
    },
    // State RELEASED
    StateTable {
        next: [WAITING, PRESSED],
        out: 0x01,
        min_time: 0,
    },
];

// A simple struct to store the current status of a switch
struct SwitchStatus {
    pin: u32,         // The pin for this input
    cur_state: u8,    // This input's current state in the FSM
    next_change: u64, // The next time a state change is allowed for this input
}

fn debounce_input(debounce_status: &mut SwitchStatus, cur_time: u64) -> bool {
    // Have we been in our current state long enough?
    if cur_time >= debounce_status.next_change {
        // Read the current input and progress through states appropriately
        let inp = sio::read_input(debounce_status.pin) as usize;
        let next_state = FSM[debounce_status.cur_state as usize].next[inp];
        if debounce_status.cur_state != next_state {
            debounce_status.cur_state = next_state;
            debounce_status.next_change =
                cur_time + (FSM[debounce_status.cur_state as usize].min_time as u64);
        }
    }
    FSM[debounce_status.cur_state as usize].out == 1
}

/*
 * The actual main function.
 */
#[entry]
fn main() -> ! {
    // Initialize the clocks and IO pins
    sys_clock::init_clocks();
    io_reset();
    sio::init_output(LED_PIN);
    sio::init_input(SW1_PIN);
    sio::into_pulldown(SW1_PIN);
    sio::init_input(SW2_PIN);
    sio::into_pulldown(SW2_PIN);

    info!("Initialized");

    let mut sw1_status: SwitchStatus = SwitchStatus {
        pin: SW1_PIN,
        cur_state: WAITING,
        next_change: 0,
    };
    let mut sw2_status: SwitchStatus = SwitchStatus {
        pin: SW2_PIN,
        cur_state: WAITING,
        next_change: 0,
    };

    loop {
        // Current time in ms
        let cur_time = sys_clock::get_current_time() / 1000;

        if debounce_input(&mut sw1_status, cur_time) {
            info!("Switch 1 was pressed and released!");
        }
        if debounce_input(&mut sw2_status, cur_time) {
            info!("Switch 2 was pressed and released!");
        }
    }
}
