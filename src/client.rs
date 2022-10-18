use std::process;
use std::{thread, time};
use chrono::{Local};
use paho_mqtt as mqtt;

pub fn publisher(ip: String, port: String, qos: i32, username: String, password: String){
    
    // Create connection string--------------------------------------------------------------------
    let conn_string = format!("tcp://{ip}:{port}");

    let cli = mqtt::Client::new(conn_string).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Connect to the broker using the specified connection string and username and password ------
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(time::Duration::from_secs(20))
        .clean_session(true)
        .user_name(username)
        .password(password)
        .finalize();

    // Connect ------------------------------------------------------------------------------------
    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

    // Create a message and publish loop ----------------------------------------------------------
    let delay = time::Duration::from_millis(1000);
    let mut index = 0;
    loop {
        let local_time = Local::now();

        // creating message
        index = index + 1;
        let msg = format!("{} | local_time_now_is: {}", index.to_string(), local_time);
        println!("{}",msg);

        // payload
        let msg = mqtt::Message::new("DateTimeLocal", msg, qos);
        let tok = cli.publish(msg);
        
        // sending error
        if let Err(e) = tok {
        println!("Error sending message: {:?}", e);

        // Disconnect from the broker
        let tok = cli.disconnect(None);
        tok.unwrap();
        }

        thread::sleep(delay);
    }
}