//! Command-line options for allocation.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Stats mode
    Stats,
    /// Zones mode
    Zones,
    /// NUMA mode
    Numa,
    /// Tiers mode
    Tiers,
    /// Alloc mode
    Alloc(u64),
    /// Free mode
    Free(u64),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Config file path
    pub config_path: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            config_path: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
