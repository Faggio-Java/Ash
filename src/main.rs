use std::{
    process::{Command}, 
    time::Duration,
};


fn main() -> std::io::Result<()> {
let kernel = Command::new("uname")
   .arg("-r")
   .output()
   .expect("Error doing stuff"); 
let title = title()?;
let uptime = fetch_uptime()?;
let desktop = fetch_desktop()?;
let memory = fetch_mem()?;

print!("       {}@{} =========================== 
OS: {} 
Kernel: {}Uptime: {} days, {} hours, {} minutes
Environment: {} 
Shell: {} 
Cpu: {} 
Memory: {}Mb / {}Mb 
", 
title.0, title.1,
fetch_distro()?,
String::from_utf8_lossy(&kernel.stdout),
uptime.0, uptime.1, uptime.2,
desktop.0, desktop.1,
fetch_cpu()?,
memory.1, memory.0);
 Ok(())
}

fn title() -> std::io::Result<(String, String)> {
   let host = std::fs::read_to_string("/etc/hostname").unwrap();
    let hostname = format!("{}", host);
   let username = std::env::var_os("USER").unwrap_or("string".into()).into_string().unwrap();
   Ok((username, hostname))
}

fn fetch_desktop() -> std::io::Result<(String, String)> {
   let desktop = std::env::var_os("DESKTOP_SESSION").unwrap_or("string".into()).into_string().unwrap();
   let shell = std::env::var_os("SHELL").unwrap_or("string".into()).into_string().unwrap();
   Ok((desktop, shell))
}

fn fetch_cpu() -> std::io::Result<String> {
  let reader = std::fs::read_to_string("/proc/cpuinfo")?;
   let line = reader.lines().find(|f| f.contains("model name")).unwrap();
  let output = line.replace("model name", "").replace("CPU", " - ").replace("\t", "").replace(": ", "");

   Ok(output)
}


fn fetch_distro() -> std::io::Result<String> {
    let reader = std::fs::read_to_string("/etc/os-release")?;
     let line = reader.lines().find(|f| f.contains("NAME=")).unwrap();
    let output = line.replace("NAME=", "");
  Ok(output)
}

fn fetch_mem() -> std::io::Result<(u32, u32)> {
   let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    let line_one = meminfo.lines().find(|f| f.contains("MemAvailable:")).unwrap();
     let pretty_one = line_one.replace("MemAvailable:", "").replace(" ", "").replace("kB", "");
    let line_two = meminfo.lines().find(|f| f.contains("MemTotal:")).unwrap();
     let pretty_two = line_two.replace("MemTotal:", "").replace(" ", "").replace("kB", "");
    let total_one: u32 = pretty_one.parse().expect("Error at 52:23");
     let mem_temp = total_one / 1000;
    let total_two: u32 = pretty_two.parse().expect("Error at 54:23");
     let mem_total = total_two / 1000;
      let mem_used = mem_total - mem_temp;
Ok((mem_total, mem_used))
}

fn fetch_uptime() -> std::io::Result<(u64, u64, u64)> {
let upinfo = std::fs::read_to_string("/proc/uptime").unwrap();
   let uptime = Duration::from_secs_f32(
    upinfo.split_whitespace()
           .next()
           .unwrap_or("0")
           .parse::<f32>()
           .unwrap_or(0.0),
   );
   let d = uptime.as_secs() / 86400;
   let h = (uptime.as_secs() - d * 86400) / 3600;
   let m = (uptime.as_secs() - d * 86400 - h * 3600) / 60;
   Ok((d, h, m))
}
