use std::net::Ipv4Addr;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select, Password};


// START
pub fn msg_start() -> bool {
    if Confirm::with_theme(&ColorfulTheme::default())
    .with_prompt("START with the configuration?")
    .interact()
    .unwrap()
    {
        return true;
    } else {
        return false;
    }
}


// HOST_IP_V4_ADDRESS 
pub fn host_ip_v4() -> String {
    let ip: Ipv4Addr = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("Broker IP:")
    .default("127.0.0.1".parse().unwrap())
    .interact()
    .unwrap();
    return ip.to_string()
}


// HOST_PORT 
pub fn host_port() -> String {
    let port = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Broker PORT:")
    .default(0)
    .item(1883)
    .item(8883)
    .interact()
    .unwrap();
let mut port_encode = String::new();
    if port == 0{
        port_encode = "1883".to_string(); 
    }
    if port == 1 {
        port_encode = "8883".to_string();
    }
    return port_encode
}


// QUALITY OF SERVICES
pub fn msg_qos() -> i32 {
    let qos = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("QoS:")
    .default(0)
    .item("QoS 0")
    .item("QoS 1")
    .item("QoS 2")
    .interact()
    .unwrap();
    return qos as i32
}


// USER NAME
pub fn usr() -> String{
    let username = Input::with_theme(&ColorfulTheme::default())
    .with_prompt("User Name:")
    .allow_empty(true)
    .interact()
    .unwrap();
    return username
}


// PASSWORD
pub fn pwd() -> String{
    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Password")
        .allow_empty_password(true)
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();
        println!("----------------------------------------------------------------");
        return password
}