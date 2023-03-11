use std::{error::Error, io, fs};

use tinkerforge::{ip_connection::IpConnection, oled_128x64_v2_bricklet::*};
const HOST: &str = "localhost";
const PORT: u16 = 4223;
const UID: &str = "Hit"; // Change XYZ to the UID of your OLED 128x64 Bricklet 2.0.

fn main() -> Result<(), Box<dyn Error>> {
    let ipcon = IpConnection::new(); // Create IP connection.
    let oled = Oled128x64V2Bricklet::new(UID, &ipcon); // Create device object.

    ipcon.connect((HOST, PORT)).recv()??; // Connect to brickd.
                                          // Don't use device before ipcon is connected.
    let _ = oled.clear_display().recv();

    let path = "./data/HelloWorld.txt";
    let contents = fs::read_to_string(path)?;
    let _ = oled.write_line(1, 0, contents);

    println!("Press enter to exit.");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    ipcon.disconnect();
    Ok(())
}
