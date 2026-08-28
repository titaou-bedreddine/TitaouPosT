use std::fs::OpenOptions;
use std::io::Write;

pub fn open_serial_cash_drawer(com_port: u32, _baud_rate: u32) -> Result<String, String> {
    let port_path = format!(r"\\.\COM{}", com_port);

    let pulse: [u8; 5] = [0x1B, 0x70, 0x00, 0x19, 0xFA];

    match OpenOptions::new().write(true).read(false).open(&port_path) {
        Ok(mut file) => {
            file.write_all(&pulse)
                .map_err(|e| format!("Failed to send kick pulse to COM{}: {}", com_port, e))?;
            file.flush()
                .map_err(|e| format!("Failed to flush COM{}: {}", com_port, e))?;
            Ok(format!("Cash drawer opened successfully on COM{}", com_port))
        }
        Err(e) => {
            Err(format!("Could not open serial port COM{}: {}. Please verify port number and cable.", com_port, e))
        }
    }
}