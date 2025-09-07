#[repr(i32)]
pub enum Ports {
    DISCOVERY = 34254,
    GAME = 7878,
}
pub const BROADCAST_INTERVAL: u64 = 2;
pub const BROADCAST_IDENTIFIER: &str = "GAME";
pub const MULTICAST_IP: &str = "239.255.42.78";
