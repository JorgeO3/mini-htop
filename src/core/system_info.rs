#![allow(unused)]

use super::utils::GB;
use sysinfo::{
    ComponentExt, CpuExt, DiskExt, NetworkExt, NetworksExt, Pid, ProcessExt, System, SystemExt,
};

#[derive(Debug)]
pub struct ProcessInfo {
    pid: Pid,
    name: String,
    memory_usage: f32,
    cpu_usage: f32,
}

#[derive(Debug)]
pub struct NetworkData {
    total_rx: f32,
    rx_per_second: f32,
    total_tx: f32,
    tx_per_second: f32,
}

#[derive(Debug, Default)]
pub struct DiskUsageData {
    name: String,
    used_percentage: f32,
    free_gb: f32,
}

#[derive(Debug, Default)]
pub struct TemperatureData {
    component: String,
    temperature: f32,
}

#[derive(Default, Debug)]
pub struct SystemResources {
    cpu_usage: f32,
    disk_usage: Vec<DiskUsageData>,
    ram_memory_usage: f32,
    swap_memory_usage: f32,
    component_temperature: Vec<TemperatureData>,
    netowork_usage: Vec<NetworkData>,
    process: Vec<ProcessInfo>,
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
        let ram_memory_usage = self.get_ram_memory_usage();
        let swap_memory_usage = self.get_swap_memory_usage();
        let component_temperature = self.get_temperatures_usage();

        SystemResources {
            cpu_usage,
            disk_usage,
            ram_memory_usage,
            swap_memory_usage,
            component_temperature,
            netowork_usage: todo!(),
            process: todo!(),
        }
    }

    fn get_cpu_usage(&self) -> f32 {
        let cpus = self.sysinfo.cpus();
        let num_cpus = cpus.len() as f32;
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage()).sum::<f32>();

        (usage / num_cpus) * 100.0
    }

    fn get_disk_usage(&self) -> Vec<(String, f32, f32)> {
        let mut disk_info = Vec::new();

        for disk_usage in self.sysinfo.disks() {
            let total_space = disk_usage.total_space() as f32;
            let available_space = disk_usage.available_space() as f32;
            let used_space = ((total_space - available_space) / total_space) * 100.0;
            let name = disk_usage.name().to_str().unwrap().to_string();
            disk_info.push((name, used_space, (available_space / GB as f32)))
        }

        // Vec<(String, f32, f32) = Name, usage [%], free [GB]
        disk_info
    }

    fn get_ram_memory_usage(&self) -> f32 {
        let total_memory = self.sysinfo.total_memory() as f32;
        let used_memory = self.sysinfo.used_memory() as f32;

        // the retourned value is a percentaje
        (used_memory / total_memory) * 100.0
    }

    fn get_swap_memory_usage(&self) -> f32 {
        let total_swap = self.sysinfo.total_swap() as f32;
        let used_swap = self.sysinfo.used_swap() as f32;

        // the retourned value is a percentaje
        (used_swap / total_swap) * 100.0
    }
    fn get_temperatures_usage(&self) -> Vec<(String, f32)> {
        let cpu_info = self
            .sysinfo
            .components()
            .iter()
            .map(|component| (component.label().into(), component.temperature()))
            .collect();

        cpu_info
    }

    fn get_network_usage(&self) -> NetworkData {
        let (_name, network_data) = self
            .sysinfo
            .networks()
            .iter()
            .find(|network| network.0 == &String::from("lo"))
            .expect("the lo interface is not in the system");

        NetworkData {
            total_rx: network_data.total_received() as f32,
            rx_per_second: network_data.received() as f32,
            total_tx: network_data.total_transmitted() as f32,
            tx_per_second: network_data.transmitted() as f32,
        }
    }

    fn get_process_list(&self) -> Vec<ProcessInfo> {
        let mut process_list = Vec::new();

        for (pid, process) in self.sysinfo.processes() {
            let pid = pid.clone();
            let name = process.name().into();
            let memory_usage = process.memory() as f32;
            let cpu_usage = process.cpu_usage();

            process_list.push(ProcessInfo {
                pid,
                name,
                memory_usage,
                cpu_usage,
            });
        }

        process_list
    }
}
