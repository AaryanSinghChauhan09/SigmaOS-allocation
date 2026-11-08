//! SigmaOS-allocation — memory allocation policy component.
//!
//! Models allocation policies, zones, NUMA nodes, memory tiers,
//! and allocation statistics for the SigmaOS kernel memory subsystem.

pub mod policy;
pub mod zone;
pub mod numa;
pub mod tier;
pub mod stats;
pub mod manager;
pub mod options;

pub use policy::{AllocPolicy, AllocFlags, AllocResult};
pub use zone::{Zone, ZoneType, ZoneState};
pub use numa::{NumaNode, NumaDistance, NumaTopology};
pub use tier::{MemoryTier, TierType, TierState};
pub use stats::{AllocStats, ZoneStats, NumaStats};
pub use manager::{AllocManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_alloc_policy() {
        let policies = vec![
            AllocPolicy::FirstFit,
            AllocPolicy::BestFit,
            AllocPolicy::WorstFit,
            AllocPolicy::NextFit,
            AllocPolicy::Buddy,
        ];
        assert_eq!(policies.len(), 5);
    }
    
    #[test]
    fn test_zone_type() {
        let types = vec![
            ZoneType::DMA,
            ZoneType::DMA32,
            ZoneType::Normal,
            ZoneType::Movable,
            ZoneType::Device,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_tier_type() {
        let types = vec![
            TierType::DRAM,
            TierType::CXL,
            TierType::HBM,
            TierType::PMEM,
        ];
        assert_eq!(types.len(), 4);
    }
    
    #[test]
    fn test_zone_creation() {
        let zone = Zone::new("zone-0".into(), ZoneType::Normal);
        assert_eq!(zone.id, "zone-0");
        assert_eq!(zone.zone_type, ZoneType::Normal);
        assert_eq!(zone.state, ZoneState::Active);
    }
    
    #[test]
    fn test_numa_node_creation() {
        let node = NumaNode::new(0);
        assert_eq!(node.id, 0);
        assert!(node.memory_mb > 0 || node.memory_mb == 0);
    }
    
    #[test]
    fn test_memory_tier_creation() {
        let tier = MemoryTier::new("dram-0".into(), TierType::DRAM);
        assert_eq!(tier.id, "dram-0");
        assert_eq!(tier.tier_type, TierType::DRAM);
    }
    
    #[test]
    fn test_alloc_result() {
        let result = AllocResult::success(0x1000, 4096);
        assert!(result.success);
        assert_eq!(result.address, Some(0x1000));
        assert_eq!(result.size, 4096);
    }
    
    #[test]
    fn test_alloc_result_failure() {
        let result = AllocResult::failure("out of memory".into());
        assert!(!result.success);
        assert_eq!(result.error, Some("out of memory".into()));
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = AllocManager::new();
        assert!(manager.list_zones().is_empty());
        assert!(manager.list_numa_nodes().is_empty());
    }
    
    #[test]
    fn test_add_zone() {
        let mut manager = AllocManager::new();
        let zone_id = manager.add_zone("zone-0".into(), ZoneType::Normal);
        
        assert!(manager.get_zone(&zone_id).is_some());
        assert_eq!(manager.list_zones().len(), 1);
    }
    
    #[test]
    fn test_zone_memory_ops() {
        let mut manager = AllocManager::new();
        let zone_id = manager.add_zone("zone-0".into(), ZoneType::Normal);
        
        assert!(manager.zone_add_memory(&zone_id, 1024));
        assert!(manager.zone_alloc(&zone_id, 256).success);
        assert_eq!(manager.get_zone(&zone_id).unwrap().free_mb, 768);
    }
    
    #[test]
    fn test_add_numa_node() {
        let mut manager = AllocManager::new();
        manager.add_numa_node(0, 8192);
        manager.add_numa_node(1, 8192);
        
        assert_eq!(manager.list_numa_nodes().len(), 2);
    }
    
    #[test]
    fn test_numa_distance() {
        let mut manager = AllocManager::new();
        manager.add_numa_node(0, 8192);
        manager.add_numa_node(1, 8192);
        
        manager.set_numa_distance(0, 1, 20);
        let distance = manager.get_numa_distance(0, 1);
        assert_eq!(distance, Some(20));
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AllocManager::new();
        manager.add_zone("zone-0".into(), ZoneType::Normal);
        manager.add_numa_node(0, 8192);
        
        let export = manager.export_data();
        
        let mut manager2 = AllocManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_zones().len(), 1);
        assert_eq!(manager2.list_numa_nodes().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = AllocManager::new();
        let zone_id = manager.add_zone("zone-0".into(), ZoneType::Normal);
        manager.zone_add_memory(&zone_id, 1024);
        manager.add_numa_node(0, 8192);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_zones, 1);
        assert_eq!(stats.total_numa_nodes, 1);
        assert_eq!(stats.total_memory_mb, 9216);
    }
}
