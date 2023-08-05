// TODO get the cpu usage
// TODO get the Disk usage
// TODO get the Memory usage
// TODO get the temperatures
// TODO get the network usage
// TODO get all the process and info

use super::utils::GB;
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

#[allow(unused)]
#[derive(Default, Debug)]
pub struct SystemResources {
    cpu_usage: f32,
    disk_usage: Vec<(String, f32, f32)>,
    memory_usage: String,
    temperatures: String,
    netowork_usage: String,
}

pub struct SystemInfo {
    sys_resources: Option<SystemResources>,
    sysinfo: System,
}
impl SystemInfo {
    pub fn new(sys: System) -> Self {
        Self {
            sys_resources: None,
            sysinfo: sys,
        }
    }

    pub fn get_info(&self) -> SystemResources {
        let cpu_usage = self.get_cpu_usage();
        let disk_usage = self.get_disk_usage();
        SystemResources {
            cpu_usage,
            disk_usage,
            memory_usage: (),
            temperatures: (),
            netowork_usage: (),
        }
    }

    fn get_cpu_usage(&self) -> f32 {
        let num_cpus = self.sysinfo.cpus().len() as f32;
        let usage = self
            .sysinfo
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>();
        (usage / num_cpus) * 100.0
    }
    fn get_disk_usage(&self) -> Vec<(String, f32, f32)> {
        // Vec<(String, f32, f32) = Name, usage [%], free [GB]
        let mut disk_info = Vec::new();

        for disk_usage in self.sysinfo.disks() {
            let total_space = disk_usage.total_space() as f32;
            let available_space = disk_usage.available_space() as f32;
            let used_space = ((total_space - available_space as f32) / total_space) * 100.0;
            let name = disk_usage.name().to_str().unwrap().to_string();
            disk_info.push((name, used_space, (available_space / GB as f32)))
        }

        disk_info
    }
    fn get_memory_usage(&self) {}
    fn get_temperatures_usage(&self) {}
    fn get_network_usage(&self) {}
    fn get_process(&self) {}
}
