use std::process::Command;
use std::ffi::OsString;
use std::fs::File;

fn main() {
let hostname = File::open("/etc/hostname").unwrap();

let info = Command::new("cat")
        .stdin(hostname)
        .output()
        .expect("Error doing whatever"); 

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

print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

println!("       {}@{} ===========================",  os_string("USER"), String::from_utf8_lossy(&info.stdout));
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
