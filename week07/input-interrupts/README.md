# Interrupts Demo Instructions

## Getting Started With Interrupts

- Start a new project as usual:  
`cargo generate --git https://github.com/CMUQ-15-348/rp2040-template`
- Copy `device.x` from this demo directory into the directory of the new project.
- Put a default interrupt vector table into `main.rs`:  

```rust
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 26] = [
    DefHandler, //0
    DefHandler, //1
    DefHandler, //2
    DefHandler, //3
    DefHandler, //4
    DefHandler, //5
    DefHandler, //6
    DefHandler, //7
    DefHandler, //8
    DefHandler, //9
    DefHandler, //10
    DefHandler, //11
    DefHandler, //12
    DefHandler, //13
    DefHandler, //14
    DefHandler, //15
    DefHandler, //16
    DefHandler, //17
    DefHandler, //18
    DefHandler, //19
    DefHandler, //20
    DefHandler, //21
    DefHandler, //22
    DefHandler, //23
    DefHandler, //24
    DefHandler, //25
];

extern "C" {
    fn DefHandler();
}
```

- Modify `Cargo.toml`, changing the `cortex-m-rt` dependency line to:  
`cortex-m-rt = { version = "0.7.5", features = ["device"] }`
- Now `cargo build` should work and you can get started writing an interrupt driven program.
