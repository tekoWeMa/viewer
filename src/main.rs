use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
    thread,
    time::Duration,
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

    let ip_connection = IpConnection::new();
    let oled = Oled128x64V2Bricklet::new(&args.uid, &ip_connection); // Create device object.

    ip_connection.connect((args.host, args.port)).recv()??;
    let _ = oled.clear_display().recv();

    let file = File::open(args.path)?;
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let line_max_length = 21;
    let texts = contents
        .split('\n')
        .flat_map(|part| split_at(part, line_max_length))
        .collect::<Vec<_>>();

    let mut counter: usize = 0;
    for line_text in texts {
        let line_index = (counter % 8) as u8;
        write_line(&oled, line_index, line_text);

        counter += 1;
        if counter % 8 == 0 {
            thread::sleep(Duration::from_millis(750));
            let _ = oled.clear_display().recv();
        }
        thread::sleep(Duration::from_millis(250));
    }

    println!("Press enter to exit.");
    let mut _input = String::new();
    io::stdin().read_line(&mut _input)?;
    ip_connection.disconnect();

    Ok(())
}

fn write_line(oled: &Oled128x64V2Bricklet, line_index: u8, line_text: &str) {
    println!("{} {}", line_index, line_text);
    let _ = oled.write_line(line_index, 0, line_text.to_string()).recv();
}

fn split_at(text: &str, index: usize) -> Vec<&str> {
    let mut list = vec![];
    let mut current = text;

    while !current.is_empty() {
        if current.len() < index {
            list.push(current);
            break;
        }

        let (head, rest) = current.split_at(index);
        list.push(head);

        current = rest;
    }

    list
}
