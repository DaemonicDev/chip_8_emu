use std::keyboard::Keyboard;

pub struct Bus
{
     pub index: u16,
     pub pc: u16,
     pub memory: [u8; 4096],
     
     pub v: [u8; 16], //Registers
     
     //Peripherals
     pub keypad: Keypad,
     pub display: Display,
     
     pub stack: [u16; 16],

     pub stack_pointer: u8,

     pub delay_timer: u8,

     pub sound_timer: u8
}

// fn read_byte(memory: u16 [4096])

