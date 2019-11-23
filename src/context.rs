use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;

use crate::config::Config;

pub type Context = Rc<RefCell<ContextData>>;

pub fn new(verbose: u64, config_path: Option<&str>) -> Context {
    Rc::new(RefCell::new(ContextData::new(verbose, config_path)))
}

pub fn default() -> Context {
    Rc::new(RefCell::new(ContextData::default()))
}

pub struct ContextData {
    pub verbose: u64,
    pub config: Config,

    /// Environment passed to newly spawned processes.
    pub env: HashMap<String, String>,

    /// Extra trace option (set via `set -x`) outputs command trace to stdout.
    pub xtrace: bool,

    /// Whether or not to exit shell immediately if a command exit with non-zero status
    /// (set via `set -e`).
    pub errexit: bool,
}

impl ContextData {
    pub fn new(verbose: u64, config_path: Option<&str>) -> ContextData {
        ContextData {
            verbose,
            config: Config::new(config_path),
            env: env::vars().collect(),
            xtrace: false,
            errexit: false,
        }
    }
}

impl Default for ContextData {
    fn default() -> ContextData {
        ContextData {
            verbose: 0,
            config: Config::default(),
            env: HashMap::new(),
            xtrace: false,
            errexit: false,
        }
    }
}
