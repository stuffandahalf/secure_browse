#![windows_subsystem = "windows"]

use std::str;
use std::string::String;
use std::process::Command;

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    //pub addresses: Vec<Address>,
    //pub mac: HardwareAddr,
    pub state: bool,
}

impl Interface {
    pub fn get_all() -> Result<Vec<Interface>, &'static str> {
        let mut ifaces: Vec<Interface> = Vec::new();
        //let output = Command::new("cmd").args(&["/C", "netsh interface show interface"]).output();
        //println!("{:?}", output);
        match Command::new("cmd").args(&["/C", "netsh interface show interface"]).output() {
            Ok(output) => {
                let out_str = str::from_utf8(&output.stdout).unwrap();
                //#[cfg(debug_assertions)]
                //println!("Devices as returned by cmd: {:?}", out_str);
                let device_strs: Vec<&str> = out_str.split("\r\n").collect();
                for i in 3..device_strs.len() - 2 {
                    //println!("{}\t{}", i, device_strs[i as usize]);
                    let device_props: Vec<&str> = device_strs[i as usize].split_whitespace().collect();
                    
                    let mut iface_name = String::new();
                    for i in 3..device_props.len() {
                        iface_name.push_str(device_props[i as usize]);
                        iface_name.push(' ');
                    }
                    iface_name.pop();
                    
                    let iface_state = device_props[0] == "Enabled";
                    
                    let new_iface = Interface { 
                        name: iface_name,
                        state: iface_state,
                    };
                    ifaces.push(new_iface);
                }
                //println!("{:?}", ifaces);
                //println!("{:?}", device_strs);
            },
            Err(err) => println!("{}", err),
        }
        if ifaces.len() > 0 {
            Ok(ifaces)
        } else {
            Err("Could not get list of devices")
        }
    }
    
    pub fn set_up(&mut self, up: bool) -> Result<(), ()> {
        #[cfg(debug_assertions)]
        println!("Running set_up on interface {}", self.name);
        if self.is_up() != up {
            let command_state = match up {
                true => "enable",
                false => "disable",
            };
            let command = format!("netsh interface set interface \"{}\" admin={}", self.name, command_state);
            #[cfg(debug_assertions)]
            println!("Command run: {}", command);
            
            match Command::new("cmd").args(&["/C", &command]).output() {
                Ok(_output) => {
                    #[cfg(debug_assertions)]
                    {
                        let out_str = str::from_utf8(&_output.stdout).unwrap();
                        print!("CMD returned: {}", out_str);
                    }
                    self.state = up;
                },
                Err(err) => println!("{}", err),
            }
        }
        else {
            #[cfg(debug_assertions)]
            println!("Interface {} is already set", self.name);
        }
        Ok(())
    }
    
    pub fn is_up(&self) -> bool {
        /*let ifaces = Interface::get_all();
        for ref iface in &ifaces {
            if self.name == iface.name {
                return iface.state;
            }
        }
        return false;*/
        self.state
    }
}

