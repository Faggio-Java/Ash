use std::{
    process::{Command}, 
    time::Duration,
};


fn main() -> std::io::Result<()> {
let kernel = Command::new("uname")
   .arg("-r")
   .output()
   .expect("Error doing whatever"); 
let title = title()?;
let uptime = fetch_uptime()?;
let desktop = fetch_desktop()?;
let memory = fetch_mem()?;
let distro = fetch_distro()?.to_lowercase();

let ascii = match distro {
   arch => arch,
   arcolinux => arcolinux,
   artix =>  artix,
   centos => centos,
   crux => crux,
   debian => debian,
   fedora => fedora,
   gentoo => gentoo,
   manjaro => manjaro,
   nixos => nixos,
   opensuse => opensuse,
   slackware => slackware,
   void => void,
   _ => "crux".to_string(),
};

let path = ascii.replace(&ascii, "ascii/{}").replace("{}", &fetch_distro()?.to_lowercase());
 let distro_ascii = std::fs::read_to_string(path)?;
  let line_one = distro_ascii.lines().find(|f| f.contains("1")).unwrap();
  let line_two = distro_ascii.lines().find(|f| f.contains("2")).unwrap();
  let line_three = distro_ascii.lines().find(|f| f.contains("3")).unwrap();
  let line_four = distro_ascii.lines().find(|f| f.contains("4")).unwrap();
  let line_five = distro_ascii.lines().find(|f| f.contains("5")).unwrap();
  let line_six = distro_ascii.lines().find(|f| f.contains("6")).unwrap();
  let line_seven = distro_ascii.lines().find(|f| f.contains("7")).unwrap();
  let line_eight = distro_ascii.lines().find(|f| f.contains("8")).unwrap(); // im sure there is a better way to do this
    
print!("       {}@{} =========================== 
{} OS: {} 
{} Kernel: {}{} Uptime: {} days, {} hours, {} minutes 
{} Environment: {}
{} Shell: {} 
{} Cpu: {} 
{} Memory: {}Mb / {}Mb 
{}
", 
title.0, title.1,
line_one, fetch_distro()?,
line_two, String::from_utf8_lossy(&kernel.stdout),
line_three, uptime.0, uptime.1, uptime.2,
line_four, desktop.0, 
line_five, desktop.1,
line_six, fetch_cpu()?,
line_seven, memory.1, memory.0, 
line_eight);
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
