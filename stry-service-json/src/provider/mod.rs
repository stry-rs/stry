pub mod discord;
pub mod google;
pub mod twitter;

pub trait Provider {
    fn build_client(&self) -> bool;
}
