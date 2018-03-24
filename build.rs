#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("blue-globe.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    
}
