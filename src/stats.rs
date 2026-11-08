//! Statistics for memory allocation.

/// Allocation statistics
#[derive(Debug, Clone)]
pub struct AllocStats {
    /// Total allocations
    pub total_allocs: u64,
    /// Successful allocations
    pub successful_allocs: u64,
    /// Failed allocations
    pub failed_allocs: u64,
    /// Total bytes allocated
    pub total_bytes_allocated: u64,
    /// Total bytes freed
    pub total_bytes_freed: u64,
    /// Current bytes allocated
    pub current_bytes_allocated: u64,
    /// Allocation success rate
    pub success_rate: f64,
    /// Average allocation size
    pub avg_alloc_size: f64,
    /// Peak allocated bytes
    pub peak_allocated_bytes: u64,
    /// Stats created timestamp
    pub created_at: u64,
    /// Stats updated timestamp
    pub updated_at: u64,
}

impl Default for AllocStats {
    fn default() -> Self {
        Self {
            total_allocs: 0,
            successful_allocs: 0,
            failed_allocs: 0,
            total_bytes_allocated: 0,
            total_bytes_freed: 0,
            current_bytes_allocated: 0,
            success_rate: 0.0,
            avg_alloc_size: 0.0,
            peak_allocated_bytes: 0,
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
}

impl AllocStats {
    /// Create new allocation stats
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Record allocation
    pub fn record_alloc(&mut self, size: u64, success: bool) {
        self.total_allocs += 1;
        
        if success {
            self.successful_allocs += 1;
            self.total_bytes_allocated += size;
            self.current_bytes_allocated += size;
            
            if self.current_bytes_allocated > self.peak_allocated_bytes {
                self.peak_allocated_bytes = self.current_bytes_allocated;
            }
        } else {
            self.failed_allocs += 1;
        }
        
        self.update_calculations();
        self.update_timestamp();
    }
    
    /// Record free
    pub fn record_free(&mut self, size: u64) {
        self.total_bytes_freed += size;
        self.current_bytes_allocated = self.current_bytes_allocated.saturating_sub(size);
        self.update_timestamp();
    }
    
    /// Update calculations
    fn update_calculations(&mut self) {
        if self.total_allocs > 0 {
            self.success_rate = (self.successful_allocs as f64 / self.total_allocs as f64) * 100.0;
        }
        
        if self.successful_allocs > 0 {
            self.avg_alloc_size = self.total_bytes_allocated as f64 / self.successful_allocs as f64;
        }
    }
    
    /// Update timestamp
    fn update_timestamp(&mut self) {
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Reset stats
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    
    /// Get stats age in seconds
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
    
    /// Check if stats have allocations
    pub fn has_allocations(&self) -> bool {
        self.total_allocs > 0
    }
    
    /// Check if stats have successful allocations
    pub fn has_successful_allocs(&self) -> bool {
        self.successful_allocs > 0
    }
    
    /// Check if stats have failed allocations
    pub fn has_failed_allocs(&self) -> bool {
        self.failed_allocs > 0
    }
    
    /// Check if stats have allocated memory
    pub fn has_allocated_memory(&self) -> bool {
        self.current_bytes_allocated > 0
    }
    
    /// Get memory efficiency (allocated vs total)
    pub fn get_memory_efficiency(&self) -> f64 {
        if self.total_bytes_allocated == 0 {
            0.0
        } else {
            (self.current_bytes_allocated as f64 / self.total_bytes_allocated as f64) * 100.0
        }
    }
}

/// Zone statistics
#[derive(Debug, Clone)]
pub struct ZoneStats {
    /// Zone ID
    pub zone_id: String,
    /// Total memory in MB
    pub total_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Allocated memory in MB
    pub allocated_mb: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// Fragmentation percentage
    pub fragmentation: f64,
    /// Allocation count
    pub alloc_count: u64,
    /// Free count
    pub free_count: u64,
}

/// NUMA statistics
#[derive(Debug, Clone)]
pub struct NumaStats {
    /// Node ID
    pub node_id: u32,
    /// Total memory in MB
    pub total_mb: u64,
    /// Free memory in MB
    pub free_mb: u64,
    /// Allocated memory in MB
    pub allocated_mb: u64,
    /// Usage percentage
    pub usage_percent: f64,
    /// CPU count
    pub cpu_count: u32,
    /// Node state
    pub state: String,
}
