use sysinfo::{System, SystemExt};
use std::process::Command;
use std::ffi::OsString;

fn main() {
let sys = System::new_all();

let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Error doing whatever"); 

let os = Command::new("head")
        .args(["-1", "/etc/os-release"])
        .output()
        .expect("Error doing whatever"); 

let memory = Command::new("head")
        .args(["-1", "/proc/meminfo"])
        .output()
        .expect("Error doing whatever"); 

let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("Error doing whatever");

let total_memory = format!("{}", String::from_utf8_lossy(&memory.stdout).replace("MemTotal:      ", ""));
let title = format!("{}@{}", os_string("USER"), sys.host_name().unwrap());

print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

println!("        {}\n ===========================", title);
println!("OS: {}Kernel: {}Uptime: {}Environment: {} \nShell: {} \nMemory: {} ===========================",
String::from_utf8_lossy(&os.stdout).replace("NAME=", ""), 
String::from_utf8_lossy(&kernel.stdout), 
String::from_utf8_lossy(&uptime.stdout),
os_string("DESKTOP_SESSION"), 
os_string("SHELL"),
total_memory
);
}

fn os_string(string: &str) -> String {
    std::env::var_os(string)
        .unwrap_or(OsString::from("unable to determine."))
        .into_string()
        .unwrap()
}