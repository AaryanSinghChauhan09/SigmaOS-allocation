//! Allocation policies for memory allocation.

/// Allocation policy
#[derive(Debug, Clone, PartialEq)]
pub enum AllocPolicy {
    /// First fit
    FirstFit,
    /// Best fit
    BestFit,
    /// Worst fit
    WorstFit,
    /// Next fit
    NextFit,
    /// Buddy allocator
    Buddy,
}

/// Allocation flags
#[derive(Debug, Clone, PartialEq)]
pub enum AllocFlags {
    /// Zero memory
    Zero,
    /// High memory
    HighMem,
    /// DMA memory
    Dma,
    /// Movable memory
    Movable,
    /// Reclaimable memory
    Reclaimable,
    /// No reclaim
    NoReclaim,
    /// Custom flag
    Custom(String),
}

/// Allocation result
#[derive(Debug, Clone)]
pub struct AllocResult {
    /// Whether allocation succeeded
    pub success: bool,
    /// Allocated address
    pub address: Option<u64>,
    /// Allocated size in bytes
    pub size: usize,
    /// Error message if failed
    pub error: Option<String>,
    /// Allocation policy used
    pub policy: Option<AllocPolicy>,
    /// Zone used
    pub zone: Option<String>,
    /// NUMA node used
    pub numa_node: Option<u32>,
    /// Allocation timestamp
    pub timestamp: u64,
}

impl AllocResult {
    /// Create successful allocation result
    pub fn success(address: u64, size: usize) -> Self {
        Self {
            success: true,
            address: Some(address),
            size,
            error: None,
            policy: None,
            zone: None,
            numa_node: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Create failed allocation result
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            address: None,
            size: 0,
            error: Some(error),
            policy: None,
            zone: None,
            numa_node: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Set policy used
    pub fn set_policy(&mut self, policy: AllocPolicy) {
        self.policy = Some(policy);
    }
    
    /// Set zone used
    pub fn set_zone(&mut self, zone: String) {
        self.zone = Some(zone);
    }
    
    /// Set NUMA node used
    pub fn set_numa_node(&mut self, node: u32) {
        self.numa_node = Some(node);
    }
    
    /// Check if allocation was successful
    pub fn is_success(&self) -> bool {
        self.success
    }
    
    /// Check if allocation failed
    pub fn is_failure(&self) -> bool {
        !self.success
    }
    
    /// Get allocation age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.timestamp
    }
}
