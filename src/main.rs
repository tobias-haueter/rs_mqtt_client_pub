use figlet_rs::FIGfont;
use colored::*;

mod client;
mod input;

fn main() {

    // CLI Style
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("rs_MQTT");
    assert!(figure.is_some());

    println!("----------------------------------------------------------------");
    println!("{}", figure.unwrap());
    println!("{}","Welcome to RUST MQTT Publisher Client!".blue());
    println!("{}"," - Publishing [1Hz] with: index | local_time_now".blue());
    println!("{}"," - Topic: DateTimeLocal/".blue());
    println!("----------------------------------------------------------------");

    // Fetch user input and start publisher
    if input::msg_start() {
        client::publisher(
            input::host_ip_v4(), 
            input::host_port(), 
            input::msg_qos(), 
            input::usr(), 
            input::pwd());
            println!("----------------------------------------------------------------");
    }
}
