use std::{
        process::{Command}, 
        io::prelude::Read,
        fs::File, ffi::OsString,
        time::Duration,
    };


fn main() -> std::io::Result<()> {
let kernel = Command::new("uname")
    .arg("-r")
    .output()
    .expect("Error doing whatever"); 

title();
fetch_distro();
fetch_uptime();
print!("Kernel: {}", String::from_utf8_lossy(&kernel.stdout));
fetch_desktop();
fetch_mem();
Ok(())
}

fn os_string(string: &str) -> String {
    std::env::var_os(string)
        .unwrap_or(OsString::from("unable to determine."))
        .into_string()
        .unwrap()
}

fn title() {
   let info = std::fs::read_to_string("/etc/hostname").unwrap();
   for line in info.lines() {
         println!("       {}@{} \n ===========================", os_string("USER"), line);
   } 
}

fn fetch_desktop() {
let desktop = std::env::var_os("DESKTOP_SESSION").unwrap_or(OsString::from("unable to determine.")).into_string().unwrap();
let shell = std::env::var_os("SHELL").unwrap_or(OsString::from("unable to determine.")).into_string().unwrap();
println!("Environment: {} \nShell: {}", desktop, shell)
}

fn fetch_distro() {
    let info = std::fs::read_to_string("/etc/os-release").unwrap();
    for line in info.lines() {
       if line.contains("NAME=") {
          let pretty = line.replace("NAME=", "");
          println!("OS: {}", pretty);
       } 
    } 
 }

fn fetch_mem() {
   let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
   for line in meminfo.lines() {
      if line.contains("MemAvailable:") {
         let apretty = line.replace("MemAvailable:", "").replace("kB", "").replace(" ", "");
          let total: u32 = apretty.parse().expect("Oh No");
           let mem_temp = total / 1000;

         let text = std::fs::read_to_string("/proc/meminfo").unwrap();
         for line in text.lines() {
            if line.contains("MemTotal:") {
               let tpretty = line.replace("MemTotal:", "").replace("kB", "").replace(" ", ""); 
                let total: u32 = tpretty.parse().expect("Oh No");
                 let mem_total = total / 1000;
                  let mem_used = mem_total - mem_temp;
               println!("Memory: {}Mb / {}Mb \n ===========================", mem_used, mem_total);
            }
         } 
      }
   }   
}

fn fetch_uptime() {
    let upinfo = std::fs::read_to_string("/proc/uptime").unwrap();
    for line in upinfo.lines() {
       let uptime = Duration::from_secs_f32(
           line.split_whitespace()
               .next()
               .unwrap_or("0")
               .parse::<f32>()
               .unwrap_or(0.0),
       );
       let d = uptime.as_secs() / 86400;
       let h = (uptime.as_secs() - d * 86400) / 3600;
       let m = (uptime.as_secs() - d * 86400 - h * 3600) / 60;
       println!("Uptime: {} days, {} hours, {} minutes", d, h, m)
    }
}
