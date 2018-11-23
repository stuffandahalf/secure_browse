//#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use] extern crate lazy_static;

#[cfg(not(windows))] extern crate interfaces;
#[cfg(not(windows))] use interfaces::Interface;

#[cfg(windows)] extern crate winreg;
#[cfg(windows)] use winreg::RegKey;
#[cfg(windows)] use winreg::enums::*;
#[cfg(windows)] mod win_interfaces;
#[cfg(windows)] use win_interfaces::Interface;

use std::process::Command;
use std::string::String;

use std::str;

lazy_static! {
    static ref BROWSER: String = get_browser();
}

fn main() {
    /*match Command::new("start").args(&["\"\"", "/MIN", "powershell -windowstyle hidden { echo Hello World }"]).spawn().expect("Failed to spawn").wait_with_output() {
        Ok(output) => println!("{}", str::from_utf8(&output.stdout).unwrap()),
        Err(err) => println!("{}", err)
    }*/
    #[cfg(debug_assertions)]
    print_ifaces();
    iface_up();
    launch_browser();
    //#[cfg(not(debug_assertions))]
    iface_down();
}

#[cfg(windows)]
fn get_browser() -> String {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key_path = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FileExts\\.html\\UserChoice").unwrap();
    let browser_key: String = key_path.get_value("ProgId").unwrap();
    #[cfg(debug_assertions)]
    println!("{:?}", browser_key);
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    let browser_path = hkcr.open_subkey(&*format!("{}\\shell\\open\\command", browser_key)).unwrap();
    #[cfg(debug_assertions)]
    println!("{:?}", browser_path);
    let browser_path_key: String = browser_path.get_value("").unwrap();
    let browser_vec: Vec<&str> = browser_path_key.split("\"").collect();
    String::from(browser_vec[1])
}

#[cfg(not(windows))]
fn get_browser() -> String {
    String::from("firefox")
}

fn launch_browser() {
    match Command::new(&*BROWSER).spawn() {
        Ok(mut child) => {
            #[cfg(debug_assertions)]
            println!("Launched Browser");
            //println!("{:?}", child);
            //println!("{:?}", child.wait());
            //println!("{:?}", child.wait());
            //println!("{:?}", child.output().unwrap());
            #[cfg(debug_assertions)]
            let _exit = child.wait_with_output();
            #[cfg(not(debug_assertions))]
            let _exit = child.wait();
            
            #[cfg(debug_assertions)]
            println!("Browser Exited");
        },
        Err(err) => println!("{}", err),
    }
}

fn iface_up() {
    #[cfg(debug_assertions)]
    println!("Bring interfaces up");
    //let interfaces = Interface::get_all();
    match Interface::get_all() {
        Ok(mut interfaces) => {
            for ref mut iface in &mut interfaces {
                //println!("{:?}", iface.set_up(true));
                let _ = iface.set_up(true);
                #[cfg(debug_assertions)]
                println!("Interface: {}, is now up: {}", iface.name, iface.is_up());
            }
        },
        //Err(err) => println!("No interfaces detected"),
        Err(err) => println!("{}", err),
    }
}

fn iface_down() {
    #[cfg(debug_assertions)]
    println!("Bring interfaces down");
    match Interface::get_all() {
        Ok(mut interfaces) => {
            for ref mut iface in &mut interfaces {
                //println!("{:?}", iface.set_up(false));
                //println!("{:?}\n\n", iface.is_up());
                let _ = iface.set_up(false);
                #[cfg(debug_assertions)]
                println!("Interface {}, is now up: {}", iface.name, iface.is_up());
            }
        },
        Err(err) => println!("{}", err),
    }
}


#[cfg(debug_assertions)]
fn print_ifaces() {
    println!("All available network interfaces");
    match Interface::get_all() {
        Ok(interfaces) => {
            for ref iface in &interfaces {
                println!("{}\tis up: {}", iface.name, iface.is_up());
            }
        }
        Err(err) => println!("{}", err),
    }
    println!("");
}
