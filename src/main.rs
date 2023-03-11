use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufReader, Read},
};

use clap::Parser;
use tinkerforge::{ip_connection::IpConnection, oled_128x64_v2_bricklet::*};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    host: String,
    #[arg(short, long)]
    port: u16,
    #[arg(short, long)]
    uid: String,
    #[arg(short, long)]
    path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let ipcon = IpConnection::new(); // Create IP connection.
    let oled = Oled128x64V2Bricklet::new(&args.uid, &ipcon); // Create device object.

    ipcon.connect((args.host, args.port)).recv()??; // Connect to brickd.
                                                    // Don't use device before ipcon is connected.
    let _ = oled.clear_display().recv();

    let file = File::open(args.path)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let parts = contents.split('\n').collect::<Vec<_>>();
    for (index, part) in parts.iter().enumerate() {
        let line = index as u8;
        println!("{} {}", line, part);
        oled.write_line(line, 0, part.to_string());
    }

    println!("Press enter to exit.");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    ipcon.disconnect();

    Ok(())
}
