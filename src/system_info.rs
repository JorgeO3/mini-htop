use super::utils::GB;
use sysinfo::{
    ComponentExt, CpuExt, DiskExt, NetworkExt, NetworksExt, Pid, ProcessExt, System, SystemExt,
};

#[derive(Debug)]
pub struct CustomPid(Pid);
impl Clone for CustomPid {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: CustomPid,
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

#[derive(Debug, Default, Clone)]
pub struct ComponentTemperature {
    pub label: String,
    pub temperature: f32,
}

#[derive(Default, Debug, Clone)]
pub struct SystemResources {
    pub cpu_usage: u64,
    pub disk_usage: Vec<DiskUsageData>,
    pub ram_memory_usage: f32,
    pub swap_memory_usage: f32,
    pub component_temperature: Vec<ComponentTemperature>,
    pub network_usage: NetworkData,
    pub process_list: Vec<ProcessInfo>,
}

pub struct SystemInfo {
    pub sys_resources: Option<SystemResources>,
    pub sysinfo: System,
    pub enhanced_graphics: bool,
}
impl SystemInfo {
    pub fn new(mut sys: System, enhanced_graphics: bool) -> Self {
        sys.refresh_all();
        Self {
            sys_resources: None,
            sysinfo: sys,
            enhanced_graphics,
        }
    }

    pub fn update_info(&mut self) {
        self.sysinfo.refresh_all();
        let cpu_usage = self.get_cpu_usage();
        let disk_usage = self.get_disk_usage();
        let ram_memory_usage = self.get_ram_memory_usage();
        let swap_memory_usage = self.get_swap_memory_usage();
        let component_temperature = self.get_temperatures();
        let network_usage = self.get_network_usage();
        let process_list = self.get_process_list();

        self.sys_resources = Some(SystemResources {
            cpu_usage,
            disk_usage,
            ram_memory_usage,
            swap_memory_usage,
            component_temperature,
            network_usage,
            process_list,
        });
    }

    fn get_cpu_usage(&self) -> u64 {
        let cpus = self.sysinfo.cpus();
        let num_cpus = cpus.len() as u64;
        let usage = cpus.iter().map(|cpu| cpu.cpu_usage() as u64).sum::<u64>();

        (usage / num_cpus) * 100
    }

    fn get_disk_usage(&self) -> Vec<DiskUsageData> {
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
    fn get_temperatures(&self) -> Vec<ComponentTemperature> {
        let mut temperatures = Vec::new();

        for component in self.sysinfo.components() {
            let label = component.label().into();
            let temperature = component.temperature();
            temperatures.push(ComponentTemperature { label, temperature });
        }

        temperatures
    }

    fn get_network_usage(&self) -> NetworkData {
        let (_name, network_data) = self
            .sysinfo
            .networks()
            .iter()
            .find(|network| network.0.starts_with('e'))
            .expect("this system don't implement a ethernet interface");

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
            let pid = CustomPid(*pid);
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
