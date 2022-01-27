use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal_w_frontend::uart::{Parity, Uart};

const ADDR_NANO3: u16 = 0x68;

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the primary UART and configure it for 115.2 kbit/s, no
    // parity bit, 8 data bits and 1 stop bit.
    let mut uart = Uart::setup(115_200, Parity::None, 8, 1)?;
println!("UART Initialized");
    // Configure read() to block until at least 1 byte is received.
    uart.set_read_mode(1, Duration::default())?;

    let mut n1 = [1 as u8;1];
    let mut n2 = [1 as u8;1];
    let a:u8 = 10;

    
    loop {
        n2[0] = n1[0];
        println!("Pi sends {}", n2[0]);
        uart.write(n2[0]).unwrap();
        thread::sleep(Duration::from_millis(200));

        n1 = uart.read().unwrap();        
        println!("Pi gets {}", n1[0]);
        if n1[0] > 244 { n1[0] = 1; }
        n1[0] = n1[0] + a;      
    }
}