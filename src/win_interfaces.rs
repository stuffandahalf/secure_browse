use std::fmt;
use std::str;
use std::string::String;
use std::process::Command;

/*pub enum Kind {
    Ipv4,
    Ipv6,
    Link,
    Unknown(i32),
    Packet,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kind::Ipv4 => write!(f, "IPv4"),
            Kind::Ipv6 => write!(f, "IPv6"),
            Kind::Link => write!(f, "Link"),
            Kind::Unknown(v) => write!(f, "Unknown({})", v),
            Kind::Packet => write!(f, "Packet"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct HardwareAddr([u8; 6]);

impl HardwareAddr {
    /// Returns a new, empty `HardwareAddr` structure.  This is equivalent to the MAC address
    /// `00:00:00:00:00:00`.
    pub fn zero() -> HardwareAddr {
        HardwareAddr([0; 6])
    }

    /// Formats this hardware address in the standard MAC address format - 6 octets in hexadecimal
    /// format, each seperated by a colon.
    ///
    /// ```
    /// # use interfaces::HardwareAddr;
    /// let s = HardwareAddr::zero().as_string();
    /// assert_eq!(s, "00:00:00:00:00:00");
    /// ```
    pub fn as_string(&self) -> String {
        let &HardwareAddr(ref arr) = self;

        format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            arr[0],
            arr[1],
            arr[2],
            arr[3],
            arr[4],
            arr[5],
        )
    }

    /// Formats this hardware address as a sequence of hexadecimal numbers without the seperating
    /// colons.
    ///
    /// ```
    /// # use interfaces::HardwareAddr;
    /// let s = HardwareAddr::zero().as_bare_string();
    /// assert_eq!(s, "000000000000");
    /// ```
    pub fn as_bare_string(&self) -> String {
        let &HardwareAddr(ref arr) = self;

        format!("{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            arr[0],
            arr[1],
            arr[2],
            arr[3],
            arr[4],
            arr[5],
        )
    }

    /// Returns the raw bytes representing this hardware address.
    ///
    /// ```
    /// # use interfaces::HardwareAddr;
    /// let s = HardwareAddr::zero();
    /// assert_eq!(s.as_bytes(), &[0, 0, 0, 0, 0, 0]);
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        let &HardwareAddr(ref arr) = self;
        arr
    }
}*/

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
                    
                    let iface_state = if device_props[0] == "Enabled" {
                        true
                    } else {
                        false
                    };
                    
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
        if self.state != up {
            let command_state = match up {
                true => "enable",
                false => "disable",
            };
            let command = format!("netsh interface set interface \"{}\" admin={}", self.name, command_state);
            #[cfg(debug_assertions)]
            println!("Command run: {}", command);
            
            match Command::new("cmd").args(&["/C", &command]).output() {
                Ok(output) => {
                    #[cfg(debug_assertions)]
                    {
                        let out_str = str::from_utf8(&output.stdout).unwrap();
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

