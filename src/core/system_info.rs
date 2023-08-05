// TODO get the cpu usage
// TODO get the Disk usage
// TODO get the Memory usage
// TODO get the temperatures
// TODO get the network usage
// TODO get all the process and info

use sysinfo::{System, SystemExt};

#[allow(unused)]
#[derive(Default, Debug)]
pub struct SystemResources {
    cpu_usage: String,
    disk_usage: String,
    memory_usage: String,
    temperatures: String,
    netowork_usage: String,
}

pub struct SystemInfo {
    sys_resources: SystemResources,
    sysinfo: System,
}
impl SystemInfo {
    pub fn new(sys: System) -> Self {
        sys.refresh_all();
        sys.components()
        tokio::time::sleep(std::time::Duration::from_millis(200));

        Self {}
    }
}
