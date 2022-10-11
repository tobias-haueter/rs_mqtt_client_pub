use std::process;
use std::{thread, time, io};
use chrono::{Local};
use figlet_rs::FIGfont;

extern crate paho_mqtt as mqtt;

fn main() {
    // CLI style -----------------------------------------------------------------

    // create header
    let standard_font = FIGfont::standard().unwrap();
    let title = standard_font.convert("mqtt_client");
    assert!(title.is_some());  

    println!("--------------------------------------------------------------------------------");
    println!("RUST");
    println!("{}", title.unwrap());
    println!("Local Time Publisher");
    println!("--------------------------------------------------------------------------------");

    // Connection data user input
    let mut ip_broker = String::new();
    let mut port_broker = String::new();

    println!("Chooes Broker IP: ");
    io::stdin()
    .read_line(&mut ip_broker)
    .expect("Failed to read ip address");
    println!("IP_BROKER -> {}", ip_broker.to_string());
    println!("--------------------------------------------------------------------------------");

    println!("Chooes Broker PORT: ");
    io::stdin()
    .read_line(&mut port_broker)
    .expect("Failed to read port number");
    println!("PORT_BROKER -> {}", port_broker.to_string());
    println!("--------------------------------------------------------------------------------");

    let connection_string = format!("tcp://{}:{}", ip_broker.trim(), port_broker);
    println!("-- Connection String -> {}", connection_string.trim());
    println!("-- Topic -> [DateTimeLocal]");
    println!("-- End Publishing -> [CTRL + C]");
    println!("--------------------------------------------------------------------------------");

    // Create a client & define connect options "tcp://localhost:1883"
    let cli = mqtt::Client::new(connection_string).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Create a message and publish it ----------------------------------------------------------------------
    let delay = time::Duration::from_millis(1000);
    let mut index = 0;
    loop {
        let local_time = Local::now();

        // creating message
        index = index + 1;
        let msg = format!("{} | local_time_now_is: {}", index.to_string(), local_time);
        println!("{}",msg);
        // payload
        let msg = mqtt::Message::new("DateTimeLocal", msg, mqtt::QOS_0);
        let tok = cli.publish(msg);
        

        // sending error
        if let Err(e) = tok {
        println!("Error sending message: {:?}", e);

        // Disconnect from the broker
        let tok = cli.disconnect(None);
        // tok.wait().unwrap();
        tok.unwrap();
        }

        thread::sleep(delay);
    }
}
