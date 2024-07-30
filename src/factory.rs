pub mod lv2_plugin_future;

use anyhow::Result;

use crate::fr_pipewire::PipewirePort;
use crate::registry::pipewire_port_registry::PipewirePortRegistry;
use crate::service::mod_host_service::ModHostService;
use dashmap::DashMap;
use std::sync::Arc;
use std::task::Waker;
use crate::factory::lv2_plugin_future::Lv2PluginFuture;
use crate::service::pipewire_service::PipewireService;

#[derive(Debug, Clone)]
pub struct FutureState {
    pub waker: Option<Waker>,
}

#[derive(Clone)]
pub struct PerformanceMixerFactory {
    future_states: Arc<DashMap<u16, FutureState>>,
    pipewire_port_registry: PipewirePortRegistry,
    mod_host_service: ModHostService,
    pipewire_service: PipewireService
}

impl PerformanceMixerFactory {
    pub fn new(
        ports_map: Arc<DashMap<String, PipewirePort>>,
        mod_host_service: ModHostService,
        pipewire_service: PipewireService
    ) -> PerformanceMixerFactory {
        PerformanceMixerFactory {
            mod_host_service,
            pipewire_service,
            pipewire_port_registry: PipewirePortRegistry { ports_map },
            future_states: Arc::new(DashMap::new())
        }
    }

    pub fn add_lv2_plugin(&mut self, uri: String) -> Result<Lv2PluginFuture> {
        let index = self.mod_host_service.add_lv2_plugin(uri)?;
        self.future_states.insert(index, FutureState { waker: None });
        Ok(Lv2PluginFuture {
            index,
            pipewire_port_registry: self.pipewire_port_registry.clone(),
            future_states: self.future_states.clone()
        })
    }

    pub fn data_changed(&self) {
        for future_state in self.future_states.iter() {
            match future_state.value().waker.clone() {
                None => {}
                Some(waker) => { waker.wake() }
            }
        }
    }
}
