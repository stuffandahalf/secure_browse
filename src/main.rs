extern crate interfaces;

use interfaces::Interface;
use std::process::Command;

#[cfg(target_os = "windows")]
const BROWSER: &str = "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe";

#[cfg(target_os = "linux")]
const BROWSER: &str = "firefox";


fn main() {
    //print_ifaces();
    iface_up();
    launch_browser();
    iface_down();
}

fn iface_up() {
    println!("Bring interfaces up");
    //let interfaces = Interface::get_all();
    match Interface::get_all() {
        Ok(mut interfaces) => {
            for ref mut iface in &mut interfaces {
                println!("{:?}", iface.set_up(true));
                println!("{:?}\n\n", iface.is_up());
            }
        },
        //Err(err) => println!("No interfaces detected"),
        Err(err) => println!("{}", err),
    }
}

fn iface_down() {
    println!("Bring interfaces down");
    match Interface::get_all() {
        Ok(mut interfaces) => {
            for ref mut iface in &mut interfaces {
                println!("{:?}", iface.set_up(false));
                println!("{:?}\n\n", iface.is_up());
            }
        },
        Err(err) => println!("{}", err),
    }
}

fn launch_browser() {
    match Command::new(BROWSER).spawn() {
        Ok(mut child) => {
            println!("{:?}", child);
            println!("{:?}", child.wait());
        },
        Err(err) => println!("{}", err),
    }
}

fn print_ifaces() {
    match Interface::get_all() {
        Ok(interfaces) => {
            for ref iface in &interfaces {
                println!("{}\tis up: {}", iface.name, iface.is_up());
            }
        }
        Err(err) => println!("{}", err),
    }
}
