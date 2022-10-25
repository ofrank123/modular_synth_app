#![feature(map_many_mut)]

use std::collections::HashMap;

pub use buffer::Buffer;
pub use node::{Input, Node};
use node::{OutputPorts, PortType};
use petgraph::data::DataMap;
use petgraph::stable_graph::EdgeIndex;
use petgraph::visit::{DfsPostOrder, GraphBase, Reversed, Visitable};
use petgraph::Incoming;

pub use mod_specs::*;
pub use node::{BoxedNode, BoxedNodeSend};

mod buffer;
mod mod_specs;
pub mod node;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

pub type Graph = petgraph::stable_graph::StableDiGraph<NodeData<BoxedNode>, (u32, u32), u32>;

type Port<'a> = (u32, &'a str);

/// Helper to add edges to graph using node idx and port name
pub fn add_graph_edge(
    graph: &mut Graph,
    (output_id, output_port): Port,
    (input_id, input_port): Port,
) -> u32 {
    let output_port_id = graph
        .node_weight(output_id.into())
        .expect(NO_NODE)
        .node
        .get_port(output_port, PortType::Out);
    let input_port_id = graph
        .node_weight(input_id.into())
        .expect(NO_NODE)
        .node
        .get_port(input_port, PortType::In);

    let idx = graph
        .add_edge(
            output_id.into(),
            input_id.into(),
            (output_port_id, input_port_id),
        )
        .index() as u32;
    console_log!("{}", idx);

    idx
}

pub fn remove_graph_edge(graph: &mut Graph, edge_id: u32) {
    graph.remove_edge(EdgeIndex::new(edge_id as usize));
}

pub struct Processor {
    // State related to the traversal of the audio graph starting from the output node.
    dfs_post_order: DfsPostOrder<<Graph as GraphBase>::NodeId, <Graph as Visitable>::Map>,
    // Solely for collecting the inputs of a node in order to apply its `Node::process` method.
    inputs: HashMap<u32, node::Input>,
}

pub struct NodeData<T: ?Sized> {
    pub output_ports: OutputPorts,
    pub node: T,
}

impl Processor {
    pub fn with_capacity(max_nodes: usize) -> Self
    where
        <Graph as Visitable>::Map: Default,
    {
        let mut dfs_post_order = DfsPostOrder::default();
        dfs_post_order.stack = Vec::with_capacity(max_nodes);
        let inputs = HashMap::with_capacity(max_nodes);
        Self {
            dfs_post_order,
            inputs,
        }
    }

    pub fn process(&mut self, graph: &mut Graph, node: <Graph as GraphBase>::NodeId) {
        process(self, graph, node)
    }
}

impl<T> NodeData<T> {
    /// Construct a new **NodeData** from an instance of its node type and buffers.
    pub fn new(node: T, output_ports: HashMap<u32, Vec<Buffer>>) -> Self {
        NodeData { node, output_ports }
    }
}

impl NodeData<BoxedNode> {
    /// The same as **new**, but boxes the given node data before storing it.
    pub fn boxed<T>(node: T) -> Self
    where
        T: 'static + Node,
    {
        let mut ports = HashMap::new();
        for &port in node.get_output_ports() {
            ports.insert(port, vec![Buffer::SILENT]);
        }

        NodeData::new(BoxedNode(Box::new(node)), ports)
    }
}

pub const NO_NODE: &str = "no node exists for the given index";

pub fn process(processor: &mut Processor, graph: &mut Graph, node: <Graph as GraphBase>::NodeId) {
    processor.dfs_post_order.reset(Reversed(&*graph));
    processor.dfs_post_order.move_to(node);
    while let Some(n) = processor.dfs_post_order.next(Reversed(&*graph)) {
        processor.inputs.clear();
        for in_n in (&*graph).neighbors_directed(n, Incoming) {
            let input_container = graph.node_weight(in_n).expect(NO_NODE);
            // Loop through edges connecting the two nodes
            for (out_port, in_port) in graph.edges_connecting(in_n, n).map(|e| e.weight()) {
                match input_container.output_ports.get(out_port) {
                    Some(buffers) => {
                        let input = node::Input::new(buffers);
                        processor.inputs.insert(*in_port, input);
                    }
                    None => {} // No output port on in_n
                }
            }
        }

        let data: &mut NodeData<BoxedNode> = graph.node_weight_mut(n).expect(NO_NODE);
        (*data)
            .node
            .process(&processor.inputs, &mut (*data).output_ports);
    }
}
