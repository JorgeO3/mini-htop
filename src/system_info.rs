use std::collections::VecDeque;

use super::utils::GB;
use sysinfo::{
    ComponentExt, CpuExt, DiskExt, NetworkExt, NetworksExt, Pid, ProcessExt, System, SystemExt,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub memory_usage: f32,
    pub cpu_usage: f32,
}

#[derive(Debug, Default, Clone)]
pub struct NetworkData {
    pub total_rx: f32,
    pub rx_per_second: f32,
    pub total_tx: f32,
    pub tx_per_second: f32,
}

#[derive(Debug, Default, Clone)]
pub struct DiskUsageData {
    pub name: String,
    pub used_space: f32,
    pub free_space: f32,
}

#[derive(Default, Debug, Clone)]
pub struct HistoricalMetric<T> {
    pub current_value: T,
    pub history: VecDeque<T>,
    pub max_history_length: usize,
}
impl<T> HistoricalMetric<T>
where
    T: Clone,
{
    fn new(value: T, max_history_length: usize) -> Self {
        let history = VecDeque::with_capacity(max_history_length);
        Self {
            current_value: value,
            history,
            max_history_length,
        }
    }

    pub fn update(&mut self, value: T) {
        self.current_value = value.clone();
        self.history.push_front(value);

        if self.history.len() > self.max_history_length {
            self.history.pop_back();
        }
    }

    pub fn get_values(&self) -> Vec<T> {
        let (slice1, slice2) = self.history.as_slices();
        slice1
            .iter()
            .chain(slice2.iter())
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[derive(Default, Debug, Clone)]
pub struct SystemResources {
    pub cpu_usage: HistoricalMetric<u64>,
    pub disk_usage: Vec<DiskUsageData>,
    pub ram_memory_usage: HistoricalMetric<u64>,
    pub swap_memory_usage: HistoricalMetric<u64>,
    pub component_temperature: Vec<(String, f32)>,
    pub network_usage: HistoricalMetric<NetworkData>,
    pub process_list: Vec<ProcessInfo>,
}

pub struct SystemInfo {
    pub sys_resources: SystemResources,
    pub sysinfo: System,
    pub enhanced_graphics: bool,
}
impl SystemInfo {
    pub fn new(mut sys: System, enhanced_graphics: bool) -> Self {
        sys.refresh_all();

        let cpu_usage = HistoricalMetric::new(0u64, 100);
        let disk_usage = Vec::new();
        let ram_memory_usage = HistoricalMetric::new(0u64, 100);
        let swap_memory_usage = HistoricalMetric::new(0u64, 100);
        let component_temperature = Vec::new();
        let network_usage = HistoricalMetric::new(NetworkData::default(), 100);
        let process_list = Vec::new();

        let sys_resources = SystemResources {
            cpu_usage,
            disk_usage,
            ram_memory_usage,
            swap_memory_usage,
            component_temperature,
            network_usage,
            process_list,
        };

        Self {
            sys_resources,
            sysinfo: sys,
            enhanced_graphics,
        }
    }

    pub fn update_info(&mut self) {
        self.sysinfo.refresh_all();
        self.update_cpu_usage();
        self.update_disk_usage();
        self.update_ram_memory_usage();
        self.update_swap_memory_usage();
        self.update_temperatures();
        self.update_network_usage();
        self.update_process_list();
    }

    fn update_cpu_usage(&mut self) {
        let cpus = self.sysinfo.cpus();
        let num_cpus = cpus.len() as u64;
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage() as u64).sum::<u64>();
        let current_usage = (usage / num_cpus) * 100;

        self.sys_resources.cpu_usage.update(current_usage);
    }

    fn update_disk_usage(&mut self) {
        let mut disk_info = Vec::new();

        for disk_usage in self.sysinfo.disks() {
            let total_space = disk_usage.total_space() as f32;
            let fre_space = disk_usage.available_space() as f32;
            let used_space = ((total_space - fre_space) / total_space) * 100.0;
            let name = disk_usage.name().to_str().unwrap().to_string();

            disk_info.push(DiskUsageData {
                name,
                used_space,
                free_space: fre_space / GB as f32,
            });
        }
        self.sys_resources.disk_usage = disk_info;
    }

    fn update_ram_memory_usage(&mut self) {
        let total_memory = self.sysinfo.total_memory();
        let used_memory = self.sysinfo.used_memory();
        let current_usage = (used_memory / total_memory) * 100;

        self.sys_resources.ram_memory_usage.update(current_usage);
    }

    fn update_swap_memory_usage(&mut self) {
        let total_swap = self.sysinfo.total_swap();
        let used_swap = self.sysinfo.used_swap();
        let current_usage = (used_swap / total_swap) * 100;

        self.sys_resources.swap_memory_usage.update(current_usage);
    }

    fn update_temperatures(&mut self) {
        let mut temperatures = Vec::new();

        for component in self.sysinfo.components() {
            let label = component.label().into();
            let temperature = component.temperature();
            temperatures.push((label, temperature));
        }
        self.sys_resources.component_temperature = temperatures;
    }

    fn update_network_usage(&mut self) {
        let (_name, network_data) = self
            .sysinfo
            .networks()
            .iter()
            .find(|network| network.0.starts_with('e'))
            .expect("this system don't implement a ethernet interface");

        let network_data = NetworkData {
            total_rx: network_data.total_received() as f32,
            rx_per_second: network_data.received() as f32,
            total_tx: network_data.total_transmitted() as f32,
            tx_per_second: network_data.transmitted() as f32,
        };

        self.sys_resources.network_usage.update(network_data);
    }

    fn update_process_list(&mut self) {
        let mut process_list = Vec::new();
        for (pid, process) in self.sysinfo.processes() {
            let pid = *pid;
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
        self.sys_resources.process_list = process_list;
    }
}
