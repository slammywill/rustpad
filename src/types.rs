use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum DeviceMode {
    Session = 0x0,
    Note = 0x1,
    Custom1 = 0x4,
    Custom2 = 0x5,
    Custom3 = 0x6,
    Custom4 = 0x7,
    DawFaders = 0xD,
    Programmer = 0x7F,
}

pub struct LaunchpadDevice {
    conn_out: Arc<Mutex<MidiOutputConnection>>,
    conn_in: Option<MidiInputConnection<()>>,
    device_mode: DeviceMode,
}

impl LaunchpadDevice {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let midi_out = MidiOutput::new("Launchpad X Output")?;
        let midi_in = MidiInput::new("Launchpad X Input")?;

        let out_ports = midi_out.ports();
        let in_ports = midi_in.ports();

        let out_port = out_ports.get(1).ok_or("Invalid output port")?;
        let in_port = in_ports.get(1).ok_or("Invalid input port")?;

        let conn_out = midi_out.connect(out_port, "launchpad-connection")?;
        let conn_out = Arc::new(Mutex::new(conn_out));

        let mut device = LaunchpadDevice {
            conn_out,
            conn_in: None,
            device_mode: DeviceMode::Programmer,
        };

        device.setup_input_connection(midi_in, in_port)?;
        let _ = device.set_mode(DeviceMode::Programmer);

        Ok(device)
    }

    fn setup_input_connection(
        &mut self,
        midi_in: MidiInput,
        port: &midir::MidiInputPort,
    ) -> Result<(), Box<dyn Error>> {
        let conn_out = Arc::clone(&self.conn_out);

        let callback = move |timestamp: u64, message: &[u8], _: &mut ()| {
            Self::handle_incoming_message(timestamp, message, &conn_out);
        };

        self.conn_in = Some(midi_in.connect(port, "launchpad-input", callback, ())?);
        Ok(())
    }

    fn handle_incoming_message(
        timestamp: u64,
        message: &[u8],
        conn_out: &Arc<Mutex<MidiOutputConnection>>,
    ) {
        println!("Received at {}: {:?}", timestamp, message);
        let note = message[1];

        // Velocity
        match message[2] {
            0x0 => {
                // Note Off
                if let Ok(mut conn) = conn_out.lock() {
                    let _ = conn.send(&[0x80, note, 0x0]);
                }
            }
            _ => {
                // Note On
                if let Ok(mut conn) = conn_out.lock() {
                    let velocity = 0x1;
                    let _ = conn.send(&[0x90, note, velocity]);
                }
            }
        }
    }

    pub fn set_pad_color(&self, pad: u8, color: u8) -> Result<(), Box<dyn Error>> {
        if let Ok(mut conn) = self.conn_out.lock() {
            conn.send(&[0x90, pad, color])?;
        }
        Ok(())
    }

    pub fn remove_pad_color(&self, pad: u8) -> Result<(), Box<dyn Error>> {
        if let Ok(mut conn) = self.conn_out.lock() {
            conn.send(&[0x80, pad])?;
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: DeviceMode) -> Result<(), Box<dyn Error>> {
        if let Ok(mut conn) = self.conn_out.lock() {
            conn.send(&[0xF0, 0x0, 0x20, 0x29, 0x2, 0xC, 0x0, mode as u8, 0x7F])?;
            self.device_mode = mode;
        }
        Ok(())
    }
}
