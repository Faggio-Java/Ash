use std::{
        process::{Command}, 
        io::prelude::Read,
        fs::File, ffi::OsString,
        time::Duration,
    };
    
fn main() -> std::io::Result<()> {
let hostname = File::open("/etc/hostname").unwrap();

let info = Command::new("head")
    .arg("-1")
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

let uptime = fetch_uptime()?;

println!("       {}@{} ===========================",  os_string("USER"), String::from_utf8_lossy(&info.stdout));
println!("OS: {}Kernel: {}Uptime: {} days, {} hours \nEnvironment: {} \nShell: {}",
String::from_utf8_lossy(&os.stdout).replace("NAME=", ""), 
String::from_utf8_lossy(&kernel.stdout), 
uptime.0, uptime.1,
os_string("DESKTOP_SESSION"), 
os_string("SHELL")
);
fetch_mem();

Ok(())
}

fn os_string(string: &str) -> String {
    std::env::var_os(string)
        .unwrap_or(OsString::from("unable to determine."))
        .into_string()
        .unwrap()
}

fn fetch_mem() -> std::io::Result<(String, String)> {
    let text = std::fs::read_to_string("/proc/meminfo").unwrap();
    let a = "0"; // placeholder variables
    let b = "0";
    for line in text.lines() {
        if line.contains("MemTotal:") {
            let tpretty = line.replace("MemTotal:", "").replace("kB", "").replace(" ", ""); 
             let total: u32 = tpretty.parse().expect("Oh No");
            let mb = total / 1000;
            println!("Memory: {}Mb", mb);
        };/* Commented Out For Now
        if line.contains("MemAvailable:") {
           let apretty = line.replace("MemAvailable:", "").replace("kB", "").replace(" ", "");
            println!("Memory Available: {}", apretty);
        }; */
}
Ok((a.to_string(), b.to_string()))
}

fn fetch_uptime() -> std::io::Result<(u64, u64)> {
    let mut file = File::open("/proc/uptime")?;
    let mut c = String::new();
     file.read_to_string(&mut c)?;
    let uptime = Duration::from_secs_f32(
        c.split_whitespace()
            .next()
            .unwrap_or("0")
            .parse::<f32>()
            .unwrap_or(0.0),
    );
    let d = uptime.as_secs() / 86400;
    let h = (uptime.as_secs() - d * 86400) / 3600;
    /* Commented out because of overflow issues past 2 days and 6 hours of uptime with this piece will fix later
      let m = (uptime.as_secs() - h * 12240) / 60;*/
    Ok((d, h))
}
