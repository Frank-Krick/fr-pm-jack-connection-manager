use crate::fr_pipewire::PipewirePort;
use crate::registry::pipewire_port_registry::PipewirePortRegistry;
use crate::service::mod_host_service::ModHostService;
use dashmap::DashMap;
use std::sync::Arc;

pub struct PerformanceMixerBuilder {
    pipewire_port_registry: PipewirePortRegistry,
    mod_host_service: Arc<ModHostService>,
}

impl PerformanceMixerBuilder {
    pub fn new(
        ports_map: Arc<DashMap<String, PipewirePort>>,
        mod_host_service: Arc<ModHostService>,
    ) -> PerformanceMixerBuilder {
        PerformanceMixerBuilder {
            mod_host_service,
            pipewire_port_registry: PipewirePortRegistry { ports_map },
        }
    }
}
