// Profile management layer.
//
// Phase 3: loads/activates profiles from the AppState cache and publishes
// lifecycle events. Profile content parsing/application arrives in Phase 4.

pub mod profile_manager;

pub use profile_manager::ProfileManager;
