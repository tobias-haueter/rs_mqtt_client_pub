use figlet_rs::FIGfont;
use colored::*;

mod client;
mod input;

fn main() {

    // CLI header style
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("rs_MQTT");
    assert!(figure.is_some());

    println!("----------------------------------------------------------------");
    println!("{}", figure.unwrap());
    println!("{}","Welcome to RUST MQTT Publisher Client!".blue());
    println!("{}"," - Publishing index with local time".blue());
    println!("{}"," - Topic: DateTimeLocal/".blue());
    println!("----------------------------------------------------------------");

    // Fetch user input and start publisher
    //if input::msg_start() {
        client::publisher(
            input::host_ip_v4(), 
            input::host_port(), 
            input::msg_qos(),
            input::keep_alive_interval(), 
            input::publish_interval(),
            input::usr(), 
            input::pwd(),
            input::run_app());
            println!("----------------------------------------------------------------");
    //}

}
