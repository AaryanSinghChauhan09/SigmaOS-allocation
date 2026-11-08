//! Memory zones for allocation.

/// Zone type
#[derive(Debug, Clone, PartialEq)]
pub enum ZoneType {
    /// DMA zone
    DMA,
    /// DMA32 zone
    DMA32,
    /// Normal zone
    Normal,
    /// Movable zone
    Movable,
    /// Device zone
    Device,
}

/// Zone state
#[derive(Debug, Clone, PartialEq)]
pub enum ZoneState {
    /// Active zone
    Active,
    /// Inactive zone
    Inactive,
    /// Full zone
    Full,
    /// Error zone
    Error,
}

/// Zone
#[derive(Debug, Clone)]
pub struct Zone {
    /// Zone ID
    pub id: String,
    /// Zone type
    pub zone_type: ZoneType,
    /// Zone state
    pub state: ZoneState,
    /// Total memory in MB
    pub total_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Allocated memory in MB
    pub allocated_mb: u64,
    /// Fragmentation percentage
    pub fragmentation: f64,
    /// Allocation count
    pub alloc_count: u64,
    /// Free count
    pub free_count: u64,
    /// Zone created timestamp
    pub created_at: u64,
    /// Zone updated timestamp
    pub updated_at: u64,
}

impl Zone {
    /// Create new zone
    pub fn new(id: String, zone_type: ZoneType) -> Self {
        Self {
            id,
            zone_type,
            state: ZoneState::Active,
            total_mb: 0,
            free_mb: 0,
            allocated_mb: 0,
            fragmentation: 0.0,
            alloc_count: 0,
            free_count: 0,
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
    
    /// Add memory to zone
    pub fn add_memory(&mut self, mb: u64) {
        self.total_mb += mb;
        self.free_mb += mb;
        self.update_timestamp();
        self.update_state();
    }
    
    /// Remove memory from zone
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
            self.alloc_count += 1;
            self.update_timestamp();
            self.update_state();
            self.update_fragmentation();
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
            self.free_count += 1;
            self.update_timestamp();
            self.update_state();
            self.update_fragmentation();
            true
        } else {
            false
        }
    }
    
    /// Update state
    fn update_state(&mut self) {
        if self.free_mb == 0 && self.allocated_mb > 0 {
            self.state = ZoneState::Full;
        } else if self.free_mb == 0 && self.allocated_mb == 0 {
            self.state = ZoneState::Inactive;
        } else {
            self.state = ZoneState::Active;
        }
    }
    
    /// Update fragmentation
    fn update_fragmentation(&mut self) {
        if self.total_mb == 0 {
            self.fragmentation = 0.0;
        } else {
            // Simplified fragmentation calculation
            self.fragmentation = (self.alloc_count as f64 / (self.alloc_count + self.free_count) as f64) * 100.0;
        }
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Check if zone is active
    pub fn is_active(&self) -> bool {
        self.state == ZoneState::Active
    }
    
    /// Check if zone is inactive
    pub fn is_inactive(&self) -> bool {
        self.state == ZoneState::Inactive
    }
    
    /// Check if zone is full
    pub fn is_full(&self) -> bool {
        self.state == ZoneState::Full
    }
    
    /// Check if zone has error
    pub fn has_error(&self) -> bool {
        self.state == ZoneState::Error
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
    
    /// Get zone age in seconds
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
    
    /// Check if zone has memory
    pub fn has_memory(&self) -> bool {
        self.total_mb > 0
    }
    
    /// Check if zone has free memory
    pub fn has_free_memory(&self) -> bool {
        self.free_mb > 0
    }
    
    /// Check if zone is empty
    pub fn is_empty(&self) -> bool {
        self.total_mb == 0 && self.free_mb == 0 && self.allocated_mb == 0
    }
}
