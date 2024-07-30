use crate::fr_pipewire::PipewirePort;
use dashmap::DashMap;
use itertools::Itertools;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct PipewireNode {
    pub name: String,
    pub in_ports: Vec<PipewirePort>,
    pub out_ports: Vec<PipewirePort>,
}

#[derive(Clone, Debug)]
pub struct PipewirePortRegistry {
    pub(crate) ports_map: Arc<DashMap<String, PipewirePort>>,
}

impl PipewirePortRegistry {
    pub fn get_nodes(&self) -> Vec<PipewireNode> {
        let node_ids = self
            .ports_map
            .iter()
            .map(|p| String::from(&p.node_id))
            .unique()
            .collect::<Vec<String>>();

        let mut nodes_map = HashMap::<String, Vec<PipewirePort>>::new();

        for node_id in node_ids {
            let ports_for_node_id = self
                .ports_map
                .iter()
                .map(|p| p.value().clone())
                .filter(|p| p.node_id == node_id)
                .collect::<Vec<PipewirePort>>();
            nodes_map.insert(node_id, ports_for_node_id);
        }

        nodes_map
            .iter()
            .map(|ports| PipewireNode {
                name: match ports.1.first() {
                    None => String::from(""),
                    Some(port) => String::from(port.clone().path),
                },
                out_ports: ports
                    .1
                    .iter()
                    .filter(|p| p.direction == "out")
                    .map(|p| p.clone())
                    .collect(),
                in_ports: ports
                    .1
                    .iter()
                    .filter(|p| p.direction == "in")
                    .map(|p| p.clone())
                    .collect(),
            })
            .collect()
    }
}
