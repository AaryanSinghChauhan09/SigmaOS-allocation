//! NUMA (Non-Uniform Memory Access) support for allocation.

use std::collections::HashMap;

/// NUMA node
#[derive(Debug, Clone)]
pub struct NumaNode {
    /// Node ID
    pub id: u32,
    /// Node memory in MB
    pub memory_mb: u64,
    /// Node free memory in MB
    pub free_mb: u64,
    /// Node allocated memory in MB
    pub allocated_mb: u64,
    /// Node CPU count
    pub cpu_count: u32,
    /// Node state
    pub state: NumaState,
    /// Node created timestamp
    pub created_at: u64,
    /// Node updated timestamp
    pub updated_at: u64,
}

/// NUMA state
#[derive(Debug, Clone, PartialEq)]
pub enum NumaState {
    /// Online node
    Online,
    /// Offline node
    Offline,
    /// Error node
    Error,
}

impl NumaNode {
    /// Create new NUMA node
    pub fn new(id: u32) -> Self {
        Self {
            id,
            memory_mb: 0,
            free_mb: 0,
            allocated_mb: 0,
            cpu_count: 0,
            state: NumaState::Online,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create new NUMA node with memory
    pub fn with_memory(id: u32, memory_mb: u64) -> Self {
        let mut node = Self::new(id);
        node.add_memory(memory_mb);
        node
    }
    
    /// Add memory to node
    pub fn add_memory(&mut self, mb: u64) {
        self.memory_mb += mb;
        self.free_mb += mb;
        self.update_timestamp();
    }
    
    /// Remove memory from node
    pub fn remove_memory(&mut self, mb: u64) -> bool {
        if self.free_mb >= mb {
            self.memory_mb -= mb;
            self.free_mb -= mb;
            self.update_timestamp();
            true
        } else {
            false
        }
    }
    
    /// Allocate memory
    pub fn alloc(&mut self, mb: u64) -> bool {
        if self.free_mb >= mb {
            self.free_mb -= mb;
            self.allocated_mb += mb;
            self.update_timestamp();
            true
        } else {
            false
        }
    }
    
    /// Free memory
    pub fn free(&mut self, mb: u64) -> bool {
        if self.allocated_mb >= mb {
            self.allocated_mb -= mb;
            self.free_mb += mb;
            self.update_timestamp();
            true
        } else {
            false
        }
    }
    
    /// Set CPU count
    pub fn set_cpu_count(&mut self, count: u32) {
        self.cpu_count = count;
        self.update_timestamp();
    }
    
    /// Set state
    pub fn set_state(&mut self, state: NumaState) {
        self.state = state;
        self.update_timestamp();
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if node is online
    pub fn is_online(&self) -> bool {
        self.state == NumaState::Online
    }
    
    /// Check if node is offline
    pub fn is_offline(&self) -> bool {
        self.state == NumaState::Offline
    }
    
    /// Check if node has error
    pub fn has_error(&self) -> bool {
        self.state == NumaState::Error
    }
    
    /// Get memory usage percentage
    pub fn get_usage_percent(&self) -> f64 {
        if self.memory_mb == 0 {
            0.0
        } else {
            (self.allocated_mb as f64 / self.memory_mb as f64) * 100.0
        }
    }
    
    /// Get free memory percentage
    pub fn get_free_percent(&self) -> f64 {
        if self.memory_mb == 0 {
            0.0
        } else {
            (self.free_mb as f64 / self.memory_mb as f64) * 100.0
        }
    }
    
    /// Get node age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Get time since update in seconds
    pub fn get_time_since_update(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.updated_at
    }
    
    /// Check if node has memory
    pub fn has_memory(&self) -> bool {
        self.memory_mb > 0
    }
    
    /// Check if node has free memory
    pub fn has_free_memory(&self) -> bool {
        self.free_mb > 0
    }
    
    /// Check if node is empty
    pub fn is_empty(&self) -> bool {
        self.memory_mb == 0 && self.free_mb == 0 && self.allocated_mb == 0
    }
}

/// NUMA distance
#[derive(Debug, Clone)]
pub struct NumaDistance {
    /// Source node ID
    pub from_node: u32,
    /// Target node ID
    pub to_node: u32,
    /// Distance value
    pub distance: u32,
    /// Distance created timestamp
    pub created_at: u64,
}

impl NumaDistance {
    /// Create new NUMA distance
    pub fn new(from_node: u32, to_node: u32, distance: u32) -> Self {
        Self {
            from_node,
            to_node,
            distance,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// NUMA topology
#[derive(Debug, Clone)]
pub struct NumaTopology {
    /// Nodes
    pub nodes: Vec<NumaNode>,
    /// Distances
    pub distances: Vec<NumaDistance>,
    /// Topology created timestamp
    pub created_at: u64,
}

impl NumaTopology {
    /// Create new NUMA topology
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            distances: vec![],
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Add node
    pub fn add_node(&mut self, node: NumaNode) -> bool {
        if self.nodes.iter().any(|n| n.id == node.id) {
            return false;
        }
        self.nodes.push(node);
        true
    }
    
    /// Get node
    pub fn get_node(&self, id: u32) -> Option<&NumaNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    
    /// Get node mutably
    pub fn get_node_mut(&mut self, id: u32) -> Option<&mut NumaNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }
    
    /// Remove node
    pub fn remove_node(&mut self, id: u32) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            self.nodes.remove(pos);
            true
        } else {
            false
        }
    }
    
    /// Add distance
    pub fn add_distance(&mut self, distance: NumaDistance) -> bool {
        // Remove existing distance if exists
        self.distances.retain(|d| !(d.from_node == distance.from_node && d.to_node == distance.to_node));
        self.distances.push(distance);
        true
    }
    
    /// Get distance
    pub fn get_distance(&self, from_node: u32, to_node: u32) -> Option<u32> {
        self.distances.iter()
            .find(|d| d.from_node == from_node && d.to_node == to_node)
            .map(|d| d.distance)
    }
    
    /// Get node count
    pub fn get_node_count(&self) -> usize {
        self.nodes.len()
    }
    
    /// Get online node count
    pub fn get_online_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| n.is_online()).count()
    }
    
    /// Get total memory
    pub fn get_total_memory(&self) -> u64 {
        self.nodes.iter().map(|n| n.memory_mb).sum()
    }
    
    /// Get total free memory
    pub fn get_total_free_memory(&self) -> u64 {
        self.nodes.iter().map(|n| n.free_mb).sum()
    }
    
    /// Get total allocated memory
    pub fn get_total_allocated_memory(&self) -> u64 {
        self.nodes.iter().map(|n| n.allocated_mb).sum()
    }
    
    /// Get topology age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Check if topology is empty
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    
    /// Check if topology has nodes
    pub fn has_nodes(&self) -> bool {
        !self.nodes.is_empty()
    }
    
    /// Check if topology has distances
    pub fn has_distances(&self) -> bool {
        !self.distances.is_empty()
    }
    
    /// Get local node (node 0)
    pub fn get_local_node(&self) -> Option<&NumaNode> {
        self.get_node(0)
    }
    
    /// Get remote nodes
    pub fn get_remote_nodes(&self) -> Vec<&NumaNode> {
        self.nodes.iter().filter(|n| n.id != 0).collect()
    }
    
    /// Get nearest node
    pub fn get_nearest_node(&self, from_node: u32) -> Option<u32> {
        let mut min_distance = u32::MAX;
        let mut nearest_node = None;
        
        for distance in &self.distances {
            if distance.from_node == from_node && distance.to_node != from_node {
                if distance.distance < min_distance {
                    min_distance = distance.distance;
                    nearest_node = Some(distance.to_node);
                }
            }
        }
        
        nearest_node
    }
}

impl Default for NumaTopology {
    fn default() -> Self {
        Self::new()
    }
}
