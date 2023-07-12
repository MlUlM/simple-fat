pub mod buffer;

pub trait Fat32BootSectorReadable {
    fn root_cluster_no(&self) -> u32;
}