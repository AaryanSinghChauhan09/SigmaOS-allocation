//! Allocation manager for memory allocation.

use std::collections::HashMap;
use crate::policy::{AllocPolicy, AllocFlags, AllocResult};
use crate::zone::{Zone, ZoneType};
use crate::numa::{NumaNode, NumaDistance, NumaTopology};
use crate::tier::{MemoryTier, TierType};
use crate::stats::{AllocStats, ZoneStats, NumaStats};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Default allocation policy
    pub default_policy: AllocPolicy,
    /// Enable NUMA awareness
    pub numa_aware: bool,
    /// Enable memory tiers
    pub enable_tiers: bool,
    /// Enable statistics
    pub enable_stats: bool,
    /// Auto-reclaim memory
    pub auto_reclaim: bool,
    /// Max allocation size in MB
    pub max_alloc_size_mb: u64,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            default_policy: AllocPolicy::Buddy,
            numa_aware: true,
            enable_tiers: true,
            enable_stats: true,
            auto_reclaim: true,
            max_alloc_size_mb: 1024,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total zones
    pub total_zones: usize,
    /// Total NUMA nodes
    pub total_numa_nodes: usize,
    /// Total memory tiers
    pub total_tiers: usize,
    /// Total memory in MB
    pub total_memory_mb: u64,
    /// Free memory in MB
    pub free_memory_mb: u64,
    /// Allocated memory in MB
    pub allocated_memory_mb: u64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Total allocations
    pub total_allocations: u64,
    /// Successful allocations
    pub successful_allocations: u64,
    /// Failed allocations
    pub failed_allocations: u64,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Allocation manager
#[derive(Debug, Clone)]
pub struct AllocManager {
    /// Zones
    zones: HashMap<String, Zone>,
    /// NUMA topology
    numa_topology: NumaTopology,
    /// Memory tiers
    tiers: HashMap<String, MemoryTier>,
    /// Allocation stats
    alloc_stats: AllocStats,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for AllocManager {
    fn default() -> Self {
        Self {
            zones: HashMap::new(),
            numa_topology: NumaTopology::new(),
            tiers: HashMap::new(),
            alloc_stats: AllocStats::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_zones: 0,
                total_numa_nodes: 0,
                total_tiers: 0,
                total_memory_mb: 0,
                free_memory_mb: 0,
                allocated_memory_mb: 0,
                memory_usage_percent: 0.0,
                total_allocations: 0,
                successful_allocations: 0,
                failed_allocations: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

impl AllocManager {
    /// Create new allocation manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            zones: HashMap::new(),
            numa_topology: NumaTopology::new(),
            tiers: HashMap::new(),
            alloc_stats: AllocStats::new(),
            options,
            statistics: ManagerStatistics {
                total_zones: 0,
                total_numa_nodes: 0,
                total_tiers: 0,
                total_memory_mb: 0,
                free_memory_mb: 0,
                allocated_memory_mb: 0,
                memory_usage_percent: 0.0,
                total_allocations: 0,
                successful_allocations: 0,
                failed_allocations: 0,
                uptime: 0,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Add default zones
        let zones = vec![
            ("zone-dma".to_string(), ZoneType::DMA),
            ("zone-dma32".to_string(), ZoneType::DMA32),
            ("zone-normal".to_string(), ZoneType::Normal),
            ("zone-movable".to_string(), ZoneType::Movable),
        ];
        
        for (id, zone_type) in zones {
            let zone = Zone::new(id.clone(), zone_type);
            self.zones.insert(id, zone);
        }
        
        // Add default NUMA node
        let node = NumaNode::with_memory(0, 8192);
        self.numa_topology.add_node(node);
        
        // Add default memory tiers
        if self.options.enable_tiers {
            let dram = MemoryTier::new("dram-0".into(), TierType::DRAM);
            self.tiers.insert("dram-0".into(), dram);
            
            let cxl = MemoryTier::new("cxl-0".into(), TierType::CXL);
            self.tiers.insert("cxl-0".into(), cxl);
        }
        
        self.update_statistics();
    }
    
    // Zone methods
    
    /// Add zone
    pub fn add_zone(&mut self, id: String, zone_type: ZoneType) -> String {
        let zone = Zone::new(id.clone(), zone_type);
        self.zones.insert(id.clone(), zone);
        self.update_statistics();
        id
    }
    
    /// Get zone
    pub fn get_zone(&self, zone_id: &str) -> Option<&Zone> {
        self.zones.get(zone_id)
    }
    
    /// Get zone mutably
    pub fn get_zone_mut(&mut self, zone_id: &str) -> Option<&mut Zone> {
        self.zones.get_mut(zone_id)
    }
    
    /// Get all zones
    pub fn list_zones(&self) -> Vec<&Zone> {
        self.zones.values().collect()
    }
    
    /// Delete zone
    pub fn delete_zone(&mut self, zone_id: &str) -> bool {
        if self.zones.remove(zone_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add memory to zone
    pub fn zone_add_memory(&mut self, zone_id: &str, mb: u64) -> bool {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            zone.add_memory(mb);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Remove memory from zone
    pub fn zone_remove_memory(&mut self, zone_id: &str, mb: u64) -> bool {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            if zone.remove_memory(mb) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Allocate from zone
    pub fn zone_alloc(&mut self, zone_id: &str, mb: u64) -> AllocResult {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            if zone.alloc(mb) {
                let result = AllocResult::success(0, (mb * 1024 * 1024) as usize);
                self.alloc_stats.record_alloc(mb * 1024 * 1024, true);
                self.update_statistics();
                return result;
            }
        }
        
        let result = AllocResult::failure("Zone allocation failed".into());
        self.alloc_stats.record_alloc(mb * 1024 * 1024, false);
        self.update_statistics();
        result
    }
    
    /// Free to zone
    pub fn zone_free(&mut self, zone_id: &str, mb: u64) -> bool {
        if let Some(zone) = self.zones.get_mut(zone_id) {
            if zone.free(mb) {
                self.alloc_stats.record_free(mb * 1024 * 1024);
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Get zones by type
    pub fn get_zones_by_type(&self, zone_type: &ZoneType) -> Vec<&Zone> {
        self.zones.values()
            .filter(|z| z.zone_type == *zone_type)
            .collect()
    }
    
    /// Get active zones
    pub fn get_active_zones(&self) -> Vec<&Zone> {
        self.zones.values()
            .filter(|z| z.is_active())
            .collect()
    }
    
    /// Get zones with free memory
    pub fn get_zones_with_free_memory(&self) -> Vec<&Zone> {
        self.zones.values()
            .filter(|z| z.has_free_memory())
            .collect()
    }
    
    // NUMA methods
    
    /// Add NUMA node
    pub fn add_numa_node(&mut self, id: u32, memory_mb: u64) -> bool {
        let node = NumaNode::with_memory(id, memory_mb);
        let added = self.numa_topology.add_node(node);
        if added {
            self.update_statistics();
        }
        added
    }
    
    /// Get NUMA node
    pub fn get_numa_node(&self, id: u32) -> Option<&NumaNode> {
        self.numa_topology.get_node(id)
    }
    
    /// Get NUMA node mutably
    pub fn get_numa_node_mut(&mut self, id: u32) -> Option<&mut NumaNode> {
        self.numa_topology.get_node_mut(id)
    }
    
    /// Get all NUMA nodes
    pub fn list_numa_nodes(&self) -> Vec<&NumaNode> {
        self.numa_topology.nodes.iter().collect()
    }
    
    /// Remove NUMA node
    pub fn remove_numa_node(&mut self, id: u32) -> bool {
        let removed = self.numa_topology.remove_node(id);
        if removed {
            self.update_statistics();
        }
        removed
    }
    
    /// Set NUMA distance
    pub fn set_numa_distance(&mut self, from_node: u32, to_node: u32, distance: u32) -> bool {
        let dist = NumaDistance::new(from_node, to_node, distance);
        self.numa_topology.add_distance(dist)
    }
    
    /// Get NUMA distance
    pub fn get_numa_distance(&self, from_node: u32, to_node: u32) -> Option<u32> {
        self.numa_topology.get_distance(from_node, to_node)
    }
    
    /// Get NUMA topology
    pub fn get_numa_topology(&self) -> &NumaTopology {
        &self.numa_topology
    }
    
    // Memory tier methods
    
    /// Add memory tier
    pub fn add_tier(&mut self, id: String, tier_type: TierType) -> String {
        let tier = MemoryTier::new(id.clone(), tier_type);
        self.tiers.insert(id.clone(), tier);
        self.update_statistics();
        id
    }
    
    /// Get memory tier
    pub fn get_tier(&self, tier_id: &str) -> Option<&MemoryTier> {
        self.tiers.get(tier_id)
    }
    
    /// Get memory tier mutably
    pub fn get_tier_mut(&mut self, tier_id: &str) -> Option<&mut MemoryTier> {
        self.tiers.get_mut(tier_id)
    }
    
    /// Get all memory tiers
    pub fn list_tiers(&self) -> Vec<&MemoryTier> {
        self.tiers.values().collect()
    }
    
    /// Delete memory tier
    pub fn delete_tier(&mut self, tier_id: &str) -> bool {
        if self.tiers.remove(tier_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add memory to tier
    pub fn tier_add_memory(&mut self, tier_id: &str, mb: u64) -> bool {
        if let Some(tier) = self.tiers.get_mut(tier_id) {
            tier.add_memory(mb);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Allocate from tier
    pub fn tier_alloc(&mut self, tier_id: &str, mb: u64) -> AllocResult {
        if let Some(tier) = self.tiers.get_mut(tier_id) {
            if tier.alloc(mb) {
                let result = AllocResult::success(0, (mb * 1024 * 1024) as usize);
                self.alloc_stats.record_alloc(mb * 1024 * 1024, true);
                self.update_statistics();
                return result;
            }
        }
        
        let result = AllocResult::failure("Tier allocation failed".into());
        self.alloc_stats.record_alloc(mb * 1024 * 1024, false);
        self.update_statistics();
        result
    }
    
    /// Get tiers by type
    pub fn get_tiers_by_type(&self, tier_type: &TierType) -> Vec<&MemoryTier> {
        self.tiers.values()
            .filter(|t| t.tier_type == *tier_type)
            .collect()
    }
    
    /// Get active tiers
    pub fn get_active_tiers(&self) -> Vec<&MemoryTier> {
        self.tiers.values()
            .filter(|t| t.is_active())
            .collect()
    }
    
    // Allocation methods
    
    /// Allocate memory
    pub fn allocate(&mut self, size_mb: u64, _flags: Vec<AllocFlags>) -> AllocResult {
        if size_mb > self.options.max_alloc_size_mb {
            return AllocResult::failure("Allocation size exceeds maximum".into());
        }
        
        // Try zones first
        let zone_ids: Vec<String> = self.get_zones_with_free_memory()
            .iter()
            .map(|z| z.id.clone())
            .collect();
        for zone_id in zone_ids {
            if let Some(zone) = self.get_zone(&zone_id) {
                if zone.free_mb >= size_mb {
                    return self.zone_alloc(&zone_id, size_mb);
                }
            }
        }
        
        // Try tiers
        let tier_ids: Vec<String> = self.get_active_tiers()
            .iter()
            .map(|t| t.id.clone())
            .collect();
        for tier_id in tier_ids {
            if let Some(tier) = self.get_tier(&tier_id) {
                if tier.free_mb >= size_mb {
                    return self.tier_alloc(&tier_id, size_mb);
                }
            }
        }
        
        // Try NUMA nodes
        let node_ids: Vec<u32> = self.list_numa_nodes()
            .iter()
            .map(|n| n.id)
            .collect();
        for node_id in node_ids {
            if let Some(node) = self.get_numa_node(node_id) {
                if node.is_online() && node.free_mb >= size_mb {
                    return AllocResult::success(0, (size_mb * 1024 * 1024) as usize);
                }
            }
        }
        
        AllocResult::failure("No memory available".into())
    }
    
    /// Free memory
    pub fn free(&mut self, size_mb: u64) -> bool {
        // Simplified free - would need to track allocations
        self.alloc_stats.record_free(size_mb * 1024 * 1024);
        self.update_statistics();
        true
    }
    
    // Statistics methods
    
    /// Get zone statistics
    pub fn get_zone_stats(&self) -> Vec<ZoneStats> {
        self.zones.values()
            .map(|z| ZoneStats {
                zone_id: z.id.clone(),
                total_mb: z.total_mb,
                free_mb: z.free_mb,
                allocated_mb: z.allocated_mb,
                usage_percent: z.get_usage_percent(),
                fragmentation: z.fragmentation,
                alloc_count: z.alloc_count,
                free_count: z.free_count,
            })
            .collect()
    }
    
    /// Get NUMA statistics
    pub fn get_numa_stats(&self) -> Vec<NumaStats> {
        self.numa_topology.nodes.iter()
            .map(|n| NumaStats {
                node_id: n.id,
                total_mb: n.memory_mb,
                free_mb: n.free_mb,
                allocated_mb: n.allocated_mb,
                usage_percent: n.get_usage_percent(),
                cpu_count: n.cpu_count,
                state: format!("{:?}", n.state),
            })
            .collect()
    }
    
    /// Get allocation statistics
    pub fn get_alloc_stats(&self) -> &AllocStats {
        &self.alloc_stats
    }
    
    /// Reset allocation statistics
    pub fn reset_alloc_stats(&mut self) {
        self.alloc_stats.reset();
    }
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_zones = self.zones.len();
        self.statistics.total_numa_nodes = self.numa_topology.get_node_count();
        self.statistics.total_tiers = self.tiers.len();
        
        self.statistics.total_memory_mb = self.zones.values()
            .map(|z| z.total_mb)
            .sum::<u64>() + self.numa_topology.get_total_memory();
        
        self.statistics.free_memory_mb = self.zones.values()
            .map(|z| z.free_mb)
            .sum::<u64>() + self.numa_topology.get_total_free_memory();
        
        self.statistics.allocated_memory_mb = self.zones.values()
            .map(|z| z.allocated_mb)
            .sum::<u64>() + self.numa_topology.get_total_allocated_memory();
        
        if self.statistics.total_memory_mb > 0 {
            self.statistics.memory_usage_percent = 
                (self.statistics.allocated_memory_mb as f64 / self.statistics.total_memory_mb as f64) * 100.0;
        }
        
        self.statistics.total_allocations = self.alloc_stats.total_allocs;
        self.statistics.successful_allocations = self.alloc_stats.successful_allocs;
        self.statistics.failed_allocations = self.alloc_stats.failed_allocs;
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            zones: self.zones.values().cloned().collect(),
            numa_nodes: self.numa_topology.nodes.clone(),
            tiers: self.tiers.values().cloned().collect(),
            alloc_stats: self.alloc_stats.clone(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.zones.clear();
        for zone in export.zones {
            self.zones.insert(zone.id.clone(), zone);
        }
        
        self.numa_topology.nodes = export.numa_nodes;
        
        self.tiers.clear();
        for tier in export.tiers {
            self.tiers.insert(tier.id.clone(), tier);
        }
        
        self.alloc_stats = export.alloc_stats;
        self.update_statistics();
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Zones
    pub zones: Vec<Zone>,
    /// NUMA nodes
    pub numa_nodes: Vec<NumaNode>,
    /// Memory tiers
    pub tiers: Vec<MemoryTier>,
    /// Allocation stats
    pub alloc_stats: AllocStats,
}
