#[cfg(not(windows))] extern crate interfaces;
#[cfg(not(windows))] use interfaces::Interface;
#[cfg(not(windows))] const BROWSER: &str = "firefox";

#[cfg(windows)] mod win_interfaces;
#[cfg(windows)] use win_interfaces::Interface;
#[cfg(windows)] const BROWSER: &str = "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe";

use std::process::Command;

fn main() {
    print_ifaces();
    iface_up();
    launch_browser();
    //#[cfg(not(debug_assertions))]
    iface_down();
}

fn launch_browser() {
    match Command::new(BROWSER).spawn() {
        Ok(mut child) => {
            #[cfg(debug_assertions)]
            println!("Launched Browser");
            //println!("{:?}", child);
            //println!("{:?}", child.wait());
            //println!("{:?}", child.wait());
            //println!("{:?}", child.output().unwrap());
            #[cfg(debug_assertions)]
            child.wait_with_output();
            #[cfg(not(debug_assertions))]
            child.wait();
            
            //#[cfg(debug_assertions)]
            println!("Browser Exited");
        },
        Err(err) => println!("{}", err),
    }
}

fn iface_up() {
    println!("Bring interfaces up");
    //let interfaces = Interface::get_all();
    match Interface::get_all() {
        Ok(mut interfaces) => {
            for ref mut iface in &mut interfaces {
                //println!("{:?}", iface.set_up(true));
                iface.set_up(true);
                #[cfg(debug_assertions)]
                println!("Interface: {}, is now up: {}", iface.name, iface.is_up());
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
                //println!("{:?}", iface.set_up(false));
                //println!("{:?}\n\n", iface.is_up());
                iface.set_up(false);
                #[cfg(debug_assertions)]
                println!("Interface {}, is now up: {}", iface.name, iface.is_up());
            }
        },
        Err(err) => println!("{}", err),
    }
}

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
