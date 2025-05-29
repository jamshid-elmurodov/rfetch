use local_ip_address::local_ip;
use std::collections::HashSet;
use std::path::Path;
use sysinfo::{Disks, System};
use winit::event_loop::EventLoop;

pub struct Info {
    sys: System,
}

impl Info {
    pub fn new() -> Self {
        let mut sys = System::new();
        sys.refresh_all();

        Self { sys }
    }

    pub fn user(&self) -> String {
        std::env::var("USER").unwrap_or("N/A".to_string())
    }

    pub fn host(&self) -> String {
        System::host_name().unwrap_or("N/A".to_string())
    }

    pub fn os(&self) -> String {
        format!(
            "{} {} {} {}",
            std::env::consts::OS.to_owned(),
            System::os_version().unwrap_or("".to_string()),
            System::name().unwrap_or("".to_string()),
            std::env::consts::ARCH
        )
    }

    pub fn kernel(&self) -> String {
        System::kernel_version().unwrap_or("N/A".to_string())
    }

    pub fn uptime(&self) -> String {
        format!(
            "{}d {}h {}m",
            System::uptime() / (24 * 3600),
            (System::uptime() % (24 * 3600)) / 3600,
            (System::uptime() % 3600) / 60
        )
    }

    pub fn shell(&self) -> String {
        if let Ok(var) = std::env::var("SHELL") {
            return Path::file_name(Path::new(&var))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }

        "N/A".to_string()
    }

    pub fn display(&self) -> String {
        let event_loop = EventLoop::new();
        let mut display = "N/A".to_string();
        event_loop.unwrap().primary_monitor().map(|m| {
            let size = m.size();
            display = format!(
                "{}x{} @ {}hz",
                size.width,
                size.height,
                m.refresh_rate_millihertz().unwrap() / 1000
            );
        });

        display
    }

    pub fn cpu(&self) -> String {
        self.sys
            .cpus()
            .first()
            .map(|cpu| format!("{}", cpu.brand()))
            .unwrap_or("N/A".to_string())
    }

    pub fn memory(&self) -> String {
        format!(
            "{}MiB/{}MiB",
            self.sys.used_memory() / 1024 / 1024,
            self.sys.total_memory() / 1024 / 1024
        )
    }

    pub fn disks(&self) -> HashSet<String> {
        let disks = Disks::new_with_refreshed_list();
        let divider = 1024u64.pow(3) as f64;

        disks
            .iter()
            .map(|disk| {
                let total = disk.total_space() as f64 / divider;
                let available = disk.available_space() as f64 / divider;
                let used = total - available;
                let used_percentage = if total > 0.0 {
                    (used / total) * 100.0
                } else {
                    0.0
                };

                format!(
                    "{} {:.2}GB/{:.2}GB ({:.1}% used)",
                    disk.name().to_str().unwrap_or("unknown"),
                    used,
                    total,
                    used_percentage
                )
            })
            .collect()
    }

    pub fn battery() -> String {
        let manager = battery::Manager::new().unwrap();

        if let Some(Ok(battery)) = manager.batteries().unwrap().next() {
            let charge = battery.state_of_charge().value * 100.0;
            let state = format!("{:?}", battery.state());
            return format!("{:.0}% [{}]", charge, state);
        }

        "".to_string()
    }

    pub fn ip() -> String {
        if let Ok(ip) = local_ip() {
            return format!("{}", ip);
        }

        "N/A".to_string()
    }
}
