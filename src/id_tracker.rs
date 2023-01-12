use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref ID_TRACKER: Mutex<IDTracker> = Mutex::new(IDTracker::new());
}

/// Struct to keep track of which IDs are available for various things that take IDs (trees, etc.)
pub struct IDTracker {
    /// The last used general-purpose ID
    prev_id: usize,
}

impl IDTracker {
    fn new() -> IDTracker {
        IDTracker { prev_id: 0 }
    }

    /// Get a general-purpose ID. Guaranteed to be unique
    pub fn get_id(&mut self) -> usize {
        self.prev_id += 1;
        self.prev_id
    }
}
