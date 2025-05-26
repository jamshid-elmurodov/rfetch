use std::collections::HashSet;
use sysinfo::{Disks, System};
use winit::event_loop::EventLoop;

pub struct Info {
    user_info: String,
    os: String,
    kernel: String,
    uptime: String,
    shell: String,
    active_processes: String,
    resolution: String,
    cpu: String,
    memory: String,
    disks: HashSet<String>,
}

impl Info {
    pub fn get_info() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        Info {
            user_info: user_info(),
            os: os(),
            kernel: kernel(),
            uptime: uptime(),
            shell: shell(),
            active_processes: active_processes(&sys),
            resolution: display(),
            cpu: cpu(&sys),
            memory: memory(&sys),
            disks: disks()
        }
    }

    pub fn user_info(&self) -> &str {
        &self.user_info
    }

    pub fn os(&self) -> &str {
        &self.os
    }
    
    pub fn kernel(&self) -> &str {
        &self.kernel
    }

    pub fn uptime(&self) -> &str {
        &self.uptime
    }

    pub fn shell(&self) -> &str {
        &self.shell
    }

    pub fn active_processes(&self) -> &str {
        &self.active_processes
    }

    pub fn resolution(&self) -> &str {
        &self.resolution
    }

    pub fn cpu(&self) -> &str {
        &self.cpu
    }

    pub fn memory(&self) -> &str {
        &self.memory
    }

    pub fn disks(&self) -> &HashSet<String> {
        &self.disks
    }
}

fn user_info() -> String {
    format!(
        "{}@{}",
        std::env::var("USER").unwrap_or(String::from("unknown")),
        System::host_name().unwrap_or(String::from("unknown"))
    )
}

fn os() -> String {
    format!(
        "{} {} {}",
        std::env::consts::OS,
        System::os_version().unwrap_or(String::from("unknown")),
        System::cpu_arch()
    )
}

fn kernel() -> String {
    format!(
        "{} {}",
        System::name().unwrap_or(String::new()),
        System::kernel_version().unwrap_or(String::from("unknown"))
    )
}

fn uptime() -> String {
    let uptime = System::uptime();

    let days = uptime / (24 * 3600);
    let hours = (uptime % (24 * 3600)) / 3600;
    let minutes = (uptime % 3600) / 60;

    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}

fn shell() -> String {
    format!("{}", std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string()))
}

fn active_processes(sys: &System) -> String {
    format!("{}", sys.processes().len())
}

fn display() -> String {
    let event_loop = EventLoop::new();
    let primary_monitor = event_loop;

    match primary_monitor {
        Ok(event_loop) => match event_loop.primary_monitor() {
            Some(monitor) => {
                let size = monitor.size();
                format!("{}hz {}x{}", monitor.refresh_rate_millihertz().unwrap() / 1000,size.width, size.height)
            }
            None => String::from("unknown"),
        },
        Err(_) => String::from("Unknown display"),
    }
}

fn cpu(sys: &System) -> String {
    if let Some(first_cpu) = sys.cpus().first() {
        return format!("{}", first_cpu.brand());
    }

    "unknown".to_string()
}

fn memory(sys: &System) -> String {
    let used = sys.used_memory() / 1024 / 1024;
    let total = sys.total_memory() / 1024 / 1024;

    format!(
        "{}MiB/{}MiB ({}% used)",
        used,
        total,
        used * 100 / total
    )
}

fn disks() -> HashSet<String> {
    let disks = Disks::new_with_refreshed_list();
    let divider = 1024u64.pow(3) as f64;
    let mut set: HashSet<String> = HashSet::new();

    disks.iter().for_each(|disk| {
        let total = disk.total_space() as f64 / divider;
        let available = disk.available_space() as f64 / divider;
        let used = total - available;
        let used_percentage = if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        };

        set.insert(format!(
            "{} {:.2}GB/{:.2}GB ({:.1}% used)",
            disk.name().to_str().unwrap_or("unknown"),
            used,
            total,
            used_percentage
        ));
    });

    set
}