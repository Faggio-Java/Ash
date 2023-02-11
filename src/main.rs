use std::{
    process::{Command}, 
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let kernel = Command::new("uname").arg("-r").output().expect("Error running uname");
    let title = title()?;
    let uptime = fetch_uptime()?;
    let desktop = fetch_desktop()?;
    let memory = fetch_mem()?;
    let distro = fetch_distro()?;

    let def = "ascii/{}";
    let path = def.replace("{}", &distro.to_lowercase());
    let distro_ascii = std::fs::read_to_string(path)?;
    let lines = extract_lines(&distro_ascii);

    print!(
        "       {}@{} =========================== 
{} OS: {} 
{} Kernel: {}{} Uptime: {} days, {} hours, {} minutes 
{} Environment: {}
{} Shell: {} 
{} Cpu: {} 
{} Memory: {}Mb / {}Mb 
{}
",
        title.0, title.1,
        lines[0], distro,
        lines[1], String::from_utf8_lossy(&kernel.stdout),
        lines[2], uptime.0, uptime.1, uptime.2,
        lines[3], desktop.0,
        lines[4], desktop.1,
        lines[5], fetch_cpu()?,
        lines[6], memory.1, memory.0,
        lines[7]
    );
    Ok(())
}


fn extract_lines(distro_ascii: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for i in 1..=8 {
        if let Some(line) = distro_ascii.lines().find(|f| f.contains(i.to_string().as_str())) {
            lines.push(line.to_owned());
        }
    }
    lines
}

fn title() -> std::io::Result<(String, String)> {
   let host = std::fs::read_to_string("/etc/hostname").unwrap();
   let username = std::env::var_os("USER").unwrap_or_else(|| "string".into()).into_string().unwrap();
   Ok((username, host.to_string()))
}

fn fetch_desktop() -> std::io::Result<(String, String)> {
   let desktop = std::env::var_os("DESKTOP_SESSION").unwrap_or_else(|| "string".into()).into_string().unwrap();
   let shell = std::env::var_os("SHELL").unwrap_or_else(|| "string".into()).into_string().unwrap();
   Ok((desktop, shell))
}

fn fetch_cpu() -> std::io::Result<String> {
  let reader = std::fs::read_to_string("/proc/cpuinfo")?;
   let line = reader.lines().find(|f| f.contains("model name")).unwrap();
  let output = line.replace("model name", "").replace("CPU", " - ").replace('\t', "").replace(": ", "");

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

   let (mem_temp, mem_total) = meminfo
       .lines()
       .fold((0, 0), |(mem_temp, mem_total), line| {
           let mut parts = line.split(":");
           let key = parts.next().expect("Error").trim();
           let value = parts.next().expect("Error").trim().replace("kB", "");
           let value = value.trim().parse().ok().unwrap_or(0);

           match key {
               "MemAvailable" => (value / 1000, mem_total),
               "MemTotal" => (mem_temp, value / 1000),
               _ => (mem_temp, mem_total),
           }
       });
       
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
