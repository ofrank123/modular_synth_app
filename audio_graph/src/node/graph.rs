//! Implementation of `Node` for a graph of nodes.
//!
//! Allows for nesting subgraphs within nodes of a graph.

use crate::{Buffer, Graph, Input, Node, NodeData, Processor};
use core::marker::PhantomData;
use petgraph::data::DataMapMut;
use petgraph::visit::{Data, GraphBase, IntoNeighborsDirected, Visitable};

pub struct GraphNode {
    pub processor: Processor,
    pub graph: Graph,
    pub input_nodes: Vec<<Graph as GraphBase>::NodeId>,
    pub output_node: <Graph as GraphBase>::NodeId,
    pub node_type: PhantomData<<Graph as GraphBase>::NodeId>,
}

impl Node for GraphNode {
    fn process(&mut self, inputs: &[Input], output: &mut [Buffer]) {
        let GraphNode {
            ref mut processor,
            ref mut graph,
            ref input_nodes,
            output_node,
            ..
        } = *self;

        // Write the input buffers to the input nodes.
        for (input, &in_n) in inputs.iter().zip(input_nodes) {
            let in_node_bufs = &mut graph
                .node_weight_mut(in_n)
                .expect("no node for graph node's input node ID")
                .buffers;
            for (in_node_buf, in_buf) in in_node_bufs.iter_mut().zip(input.buffers()) {
                in_node_buf.copy_from_slice(in_buf);
            }
        }

        // Process the graph.
        processor.process(graph, output_node);

        // Write the output node buffers to the output buffers.
        let out_node_bufs = &mut graph
            .node_weight_mut(output_node)
            .expect("no node for graph node's output node ID")
            .buffers;
        for (out_buf, out_node_buf) in output.iter_mut().zip(out_node_bufs) {
            out_buf.copy_from_slice(out_node_buf);
        }
    }
}
