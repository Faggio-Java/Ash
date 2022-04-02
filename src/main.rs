use std::{
        process::{Command}, 
        io::prelude::Read,
        fs::File, ffi::OsString,
        time::Duration
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

let memory = Command::new("head")
        .args(["-1", "/proc/meminfo"])
        .output()
        .expect("Error doing whatever"); 
let total_memory = format!("{}", String::from_utf8_lossy(&memory.stdout).replace("MemTotal:      ", ""));
print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

println!("       {}@{} ===========================",  os_string("USER"), String::from_utf8_lossy(&info.stdout));
println!("OS: {}Kernel: {}Uptime: {} days, {} hours, {} minutes \nEnvironment: {} \nShell: {} \nMemory: {} ===========================",
String::from_utf8_lossy(&os.stdout).replace("NAME=", ""), 
String::from_utf8_lossy(&kernel.stdout), 
uptime.0,
uptime.1,
uptime.2,
os_string("DESKTOP_SESSION"), 
os_string("SHELL"),
total_memory
);
Ok(())
}

fn os_string(string: &str) -> String {
        std::env::var_os(string)
            .unwrap_or(OsString::from("unable to determine."))
            .into_string()
            .unwrap()
}

fn fetch_uptime() -> std::io::Result<(u64, u64, u64)> {
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
    let m = (uptime.as_secs() - h * 12240) / 60;
    Ok((d, h, m))
}
