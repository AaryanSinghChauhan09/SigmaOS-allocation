//! Memory tiers for allocation (CXL, HBM, PMEM, DRAM).

/// Tier type
#[derive(Debug, Clone, PartialEq)]
pub enum TierType {
    /// DRAM tier
    DRAM,
    /// CXL tier
    CXL,
    /// HBM tier
    HBM,
    /// PMEM tier
    PMEM,
}

/// Tier state
#[derive(Debug, Clone, PartialEq)]
pub enum TierState {
    /// Active tier
    Active,
    /// Inactive tier
    Inactive,
    /// Full tier
    Full,
    /// Error tier
    Error,
}

/// Memory tier
#[derive(Debug, Clone)]
pub struct MemoryTier {
    /// Tier ID
    pub id: String,
    /// Tier type
    pub tier_type: TierType,
    /// Tier state
    pub state: TierState,
    /// Total memory in MB
    pub total_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Allocated memory in MB
    pub allocated_mb: u64,
    /// Tier latency in nanoseconds
    pub latency_ns: u64,
    /// Tier bandwidth in MB/s
    pub bandwidth_mbps: u64,
    /// Tier created timestamp
    pub created_at: u64,
    /// Tier updated timestamp
    pub updated_at: u64,
}

impl MemoryTier {
    /// Create new memory tier
    pub fn new(id: String, tier_type: TierType) -> Self {
        Self {
            id,
            tier_type,
            state: TierState::Active,
            total_mb: 0,
            free_mb: 0,
            allocated_mb: 0,
            latency_ns: 0,
            bandwidth_mbps: 0,
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
    
    /// Add memory to tier
    pub fn add_memory(&mut self, mb: u64) {
        self.total_mb += mb;
        self.free_mb += mb;
        self.update_timestamp();
        self.update_state();
    }
    
    /// Remove memory from tier
    pub fn remove_memory(&mut self, mb: u64) -> bool {
        if self.free_mb >= mb {
            self.total_mb -= mb;
            self.free_mb -= mb;
            self.update_timestamp();
            self.update_state();
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
            self.update_state();
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
            self.update_state();
            true
        } else {
            false
        }
    }
    
    /// Set latency
    pub fn set_latency(&mut self, latency_ns: u64) {
        self.latency_ns = latency_ns;
        self.update_timestamp();
    }
    
    /// Set bandwidth
    pub fn set_bandwidth(&mut self, bandwidth_mbps: u64) {
        self.bandwidth_mbps = bandwidth_mbps;
        self.update_timestamp();
    }
    
    /// Update state
    fn update_state(&mut self) {
        if self.free_mb == 0 && self.allocated_mb > 0 {
            self.state = TierState::Full;
        } else if self.free_mb == 0 && self.allocated_mb == 0 {
            self.state = TierState::Inactive;
        } else {
            self.state = TierState::Active;
        }
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if tier is active
    pub fn is_active(&self) -> bool {
        self.state == TierState::Active
    }
    
    /// Check if tier is inactive
    pub fn is_inactive(&self) -> bool {
        self.state == TierState::Inactive
    }
    
    /// Check if tier is full
    pub fn is_full(&self) -> bool {
        self.state == TierState::Full
    }
    
    /// Check if tier has error
    pub fn has_error(&self) -> bool {
        self.state == TierState::Error
    }
    
    /// Get memory usage percentage
    pub fn get_usage_percent(&self) -> f64 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.allocated_mb as f64 / self.total_mb as f64) * 100.0
        }
    }
    
    /// Get free memory percentage
    pub fn get_free_percent(&self) -> f64 {
        if self.total_mb == 0 {
            0.0
        } else {
            (self.free_mb as f64 / self.total_mb as f64) * 100.0
        }
    }
    
    /// Get tier age in seconds
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
    
    /// Check if tier has memory
    pub fn has_memory(&self) -> bool {
        self.total_mb > 0
    }
    
    /// Check if tier has free memory
    pub fn has_free_memory(&self) -> bool {
        self.free_mb > 0
    }
    
    /// Check if tier is empty
    pub fn is_empty(&self) -> bool {
        self.total_mb == 0 && self.free_mb == 0 && self.allocated_mb == 0
    }
}
