pub mod buffer;

pub trait Fat32BootSectorReadable {
    fn root_clusters(&self) -> u32;
}