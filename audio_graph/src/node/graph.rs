//! Implementation of `Node` for a graph of nodes.
//!
//! Allows for nesting subgraphs within nodes of a graph.

use crate::{port_panic, Graph, Node, Processor, NO_NODE};
use petgraph::visit::GraphBase;
use std::collections::HashMap;

use super::{InputPorts, OutputPorts, PortType, NO_PORT};

pub struct GraphNode {
    output_node_id: <Graph as GraphBase>::NodeId,
    processor: Processor,
    graph: Graph,
    port_maps: HashMap<u32, Vec<(u32, <Graph as GraphBase>::NodeId)>>,
    port_names: HashMap<String, u32>,
    next_p_id: u32, // Keep track of next available port number to use
}

impl GraphNode {
    // Adds an input port
    pub fn add_port(&mut self, name: String) {
        self.port_names.insert(name, self.next_p_id);
        self.next_p_id += 1;
    }

    // Map an outward facing input port on the subgraph to an internal port
    pub fn map_port(
        &mut self,
        extern_port_name: String,
        internal_node_id: <Graph as GraphBase>::NodeId,
        internal_port_name: String,
    ) {
        let extern_port_n = self.get_port(&extern_port_name, PortType::In);

        // Lookup port number
        let internal_port_n = self
            .graph
            .node_weight_mut(internal_node_id)
            .expect(NO_NODE)
            .node
            .get_port(&internal_port_name, PortType::In);

        // Add port map if none exists, otherwise add it onto existing
        match self.port_maps.get_mut(&extern_port_n) {
            Some(arr) => {
                arr.push((internal_port_n, internal_node_id));
            }
            None => {
                self.port_maps
                    .insert(extern_port_n, vec![(internal_port_n, internal_node_id)]);
            }
        };
    }

    // Graph must have single output node
    pub fn new(
        processor: Processor,
        graph: Graph,
        output_node_id: <Graph as GraphBase>::NodeId,
    ) -> Self {
        GraphNode {
            output_node_id,
            processor,
            graph,
            port_maps: HashMap::new(),
            port_names: HashMap::new(),
            next_p_id: 0,
        }
    }
}

// TODO: Make work with ports
impl Node for GraphNode {
    fn get_output_ports(&self) -> &[u32] {
        let GraphNode {
            ref graph,
            output_node_id,
            ..
        } = *self;

        graph
            .node_weight(output_node_id)
            .expect(NO_NODE)
            .node
            .get_output_ports()
    }

    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        match port_type {
            PortType::In => match self.port_names.get(&name.to_string()) {
                Some(n) => *n,
                None => port_panic!(port_type, name),
            },
            // Check for output port name on output node of graph
            PortType::Out => self
                .graph
                .node_weight(self.output_node_id)
                .expect(NO_NODE)
                .node
                .get_port(name, PortType::Out),
        }
    }

    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        let GraphNode {
            ref mut processor,
            ref mut graph,
            ref port_maps,
            output_node_id,
            ..
        } = *self;

        for (port, input) in inputs {
            match port_maps.get(&port) {
                None => (),
                Some(internal_ports) => {
                    for &(port_id, node_id) in internal_ports {
                        // Get the internal
                        let internal_node_buf = &mut graph
                            .node_weight_mut(node_id)
                            .expect(NO_NODE)
                            .output_ports
                            .get_mut(&port_id)
                            .expect(NO_PORT);

                        for (internal_node_buf, in_buf) in
                            internal_node_buf.iter_mut().zip(input.buffers())
                        {
                            internal_node_buf.copy_from_slice(in_buf);
                        }
                    }
                }
            }
        }

        // Process the graph.

        processor.process(graph, output_node_id);

        // Get output node ports
        let out_node_ports = &mut graph
            .node_weight_mut(output_node_id)
            .expect("no node for graph node's output node ID")
            .output_ports;

        for (out_node_port_id, out_node_bufs) in out_node_ports {
            while let Some(out_bufs) = output.get_mut(out_node_port_id) {
                for (out_buf, out_node_buf) in out_bufs.iter_mut().zip(out_node_bufs.iter_mut()) {
                    out_buf.copy_from_slice(out_node_buf);
                }
            }
        }
    }
}
