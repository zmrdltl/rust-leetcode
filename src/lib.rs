pub mod numbers;

pub use numbers::*;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY: &str = "https://github.com/zmrdltl/rust-leetcode";

#[cfg(test)]
mod tests {
    // Common test utilities and shared tests go here
}
