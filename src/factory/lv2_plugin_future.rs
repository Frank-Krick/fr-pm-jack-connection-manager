use crate::domain::lv2_plugin::Lv2Plugin;
use crate::domain::port::Port;
use crate::factory::FutureState;
use crate::registry::pipewire_port_registry::{PipewireNode, PipewirePortRegistry};
use dashmap::DashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Debug)]
pub struct Lv2PluginFuture {
    pub index: u16,
    pub pipewire_port_registry: PipewirePortRegistry,
    pub future_states: Arc<DashMap<u16, FutureState>>,
}

impl Future for Lv2PluginFuture {
    type Output = Lv2Plugin;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let nodes = self
            .pipewire_port_registry
            .get_nodes()
            .into_iter()
            .filter(|n| {
                n.name
                    .contains(format!("effect_{}:", self.index.clone()).as_str())
            })
            .collect::<Vec<PipewireNode>>();
        if nodes.is_empty() {
            self.future_states.alter(&self.index, |k, fs| FutureState {
                waker: Some(context.waker().clone()),
            });
            Poll::Pending
        } else {
            let node = nodes.first().unwrap();
            let input_ports = node.in_ports.iter()
                .map(|p| Port::from(p));
            let output_ports = node.in_ports.iter()
                .map(|p| Port::from(p));
            self.future_states.remove(&self.index);
            Poll::Ready(Lv2Plugin {
                index: self.index,
                name: node.name.clone(),
                input_ports: input_ports.collect(),
                output_ports: output_ports.collect(),
            })
        }
    }
}
