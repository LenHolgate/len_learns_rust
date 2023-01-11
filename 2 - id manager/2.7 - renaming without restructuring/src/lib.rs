#![allow(dead_code)]

mod interval;
mod intervals;
mod id_manager;
mod smart_id;
mod thread_safe_id_manager;

pub use thread_safe_id_manager::ThreadSafeIdManager as IdManager;
pub use smart_id::SmartId as Id;