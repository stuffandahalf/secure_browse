extern crate pnet;

use pnet::datalink::{self, NetworkInterface};
use std::process::{Child, Command};

const BROWSER: &str = "firefox";


fn main() {
    iface_up();
    match Command::new(BROWSER).spawn() {
        Ok(mut child) => {
            println!("{:?}", child);
            println!("{:?}", child.wait());
        },
        Err(err) => println!("{}", err),
    }
    iface_down();
}

fn iface_up() {
    println!("Bringing interfaces up");
    let interfaces = datalink::interfaces();
    for ref iface in &interfaces {
        if !iface.is_up() {
            #[cfg(target_os = "windows")]
            {
                let command: &str = &format!("netsh interface set interface \"{}\" ENABLE", iface.name);
                Command::new("cmd").args(&["/C", command]).output();
            }
        }
        //println!("{} is up? {}", iface.name, iface.is_up());
    }
}

fn iface_down() {
    println!("Bringing interfaces down");
    let interfaces = datalink::interfaces();
    for ref iface in &interfaces {
        if iface.is_up() {
            #[cfg(target_os = "windows")]
            {
                let command: &str = &format!("netsh interface set interface \"{}\" DISABLE", iface.name);
                Command::new("cmd").args(&["/C", command]).output();
            }
            
        }
        //println!("{} is up? {}", iface.name, iface.is_up());
    }
}
