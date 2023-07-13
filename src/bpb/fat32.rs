pub mod buffer;

pub trait Fat32BootSectorReadable {
    fn sectors_per_fat(&self) -> u32;


    fn root_cluster_no(&self) -> u32;
}