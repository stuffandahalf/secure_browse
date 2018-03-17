extern crate interfaces;

use interfaces::Interface;

#[cfg(target_os = "windows")]
const BROWSER: &str = "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe";

#[cfg(target_os = "unix")]
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
    //iface_down();
}

fn iface_up() {
    println!("Bring interfaces up");
    //let interfaces = Interface::get_all();
    match Interface::get_all() {
        Some(interfaces) => {
            for ref iface in &interfaces {
                println!("{:?}", iface);
            }
        },
        None => println!("No interfaces detected");
    }
}

/*fn iface_up() {
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
}*/

/*fn iface_down() {
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
}*/
