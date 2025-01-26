mod msg;
mod types;

use midir::{MidiInput, MidiOutput};
use std::error::Error;
use std::thread;
use std::time::Duration;

use msg::set_mode_msg;
use types::DeviceMode;

fn input_callback<T>(timestamp: u64, msg: &[u8], _data: &mut T) {
    for byte in msg {
        print!("{:02X} ", byte); // Print each byte in hexadecimal format
    }
    println!(); // Move to the next line after printing the message
}

fn main() -> Result<(), Box<dyn Error>> {
    // Set up MIDI ports
    let midi_out = MidiOutput::new("Launchpad X Output")?;
    let midi_in = MidiInput::new("Launchpad X Input")?;

    let out_ports = midi_out.ports();
    let in_ports = midi_in.ports();
    let out_port = out_ports.get(1).ok_or("Invalid output port")?;
    let in_port = in_ports.get(1).ok_or("Invalid input port")?;

    // Connect to output port
    let mut conn_out = midi_out.connect(out_port, "launchpad-connection")?;
    conn_out.send(&set_mode_msg(DeviceMode::Programmer))?;

    let mut conn_in = midi_in.connect(in_port, "in_port", input_callback, ());

    loop {
        thread::sleep(Duration::from_millis(10));
    }
}
