use crate::Node;
use core::fmt;
use core::ops::{Deref, DerefMut};

use super::{InputPorts, OutputPorts, PortType};

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
pub struct BoxedNode(pub Box<dyn Node>);

/// A wrapper around a `Box<dyn Node>`.
///
/// Provides the necessary `Sized` implementation to allow for compatibility with the graph process
/// function.
///
/// Useful when the ability to send nodes from one thread to another is required. E.g. this is
/// common when initialising nodes or the audio graph itself on one thread before sending them to
/// the audio thread.
pub struct BoxedNodeSend(pub Box<dyn Node + Send>);

impl BoxedNode {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node,
    {
        Self::from(Box::new(node))
    }
}

impl BoxedNodeSend {
    /// Create a new `BoxedNode` around the given `node`.
    ///
    /// This is short-hand for `BoxedNode::from(Box::new(node))`.
    pub fn new<T>(node: T) -> Self
    where
        T: 'static + Node + Send,
    {
        Self::from(Box::new(node))
    }
}

impl Node for BoxedNode {
    fn get_output_ports(&self) -> &[u32] {
        self.0.get_output_ports()
    }
    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        self.0.get_port(name, port_type)
    }
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        self.0.process(inputs, output)
    }
}

impl Node for BoxedNodeSend {
    fn get_output_ports(&self) -> &[u32] {
        self.0.get_output_ports()
    }
    fn get_port(&self, name: &str, port_type: PortType) -> u32 {
        self.0.get_port(name, port_type)
    }
    fn process(&mut self, inputs: &InputPorts, output: &mut OutputPorts) {
        self.0.process(inputs, output)
    }
}

impl<T> From<Box<T>> for BoxedNode
where
    T: 'static + Node,
{
    fn from(n: Box<T>) -> Self {
        BoxedNode(n as Box<dyn Node>)
    }
}

impl<T> From<Box<T>> for BoxedNodeSend
where
    T: 'static + Node + Send,
{
    fn from(n: Box<T>) -> Self {
        BoxedNodeSend(n as Box<dyn Node + Send>)
    }
}

impl Into<Box<dyn Node>> for BoxedNode {
    fn into(self) -> Box<dyn Node> {
        self.0
    }
}

impl Into<Box<dyn Node + Send>> for BoxedNodeSend {
    fn into(self) -> Box<dyn Node + Send> {
        self.0
    }
}

impl fmt::Debug for BoxedNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNode").finish()
    }
}

impl fmt::Debug for BoxedNodeSend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BoxedNodeSend").finish()
    }
}

impl Deref for BoxedNode {
    type Target = Box<dyn Node>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BoxedNodeSend {
    type Target = Box<dyn Node + Send>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for BoxedNodeSend {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
