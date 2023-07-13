use crate::error::FatResult;
use crate::FatDeviceAccessible;

pub trait Fat32BootSectorReadable {
    fn sectors_per_fat(&self) -> FatResult<u32>;


    fn root_cluster_no(&self) -> FatResult<u32>;
}


#[derive(Clone)]
pub struct Fat32BootSector<D> {
    device: D,
}


impl<D> Fat32BootSector<D> where D: FatDeviceAccessible {
    #[inline]
    pub const fn new(device: D) -> Fat32BootSector<D> {
        Self {
            device
        }
    }
}


impl<D> Fat32BootSectorReadable for Fat32BootSector<D>
    where D: FatDeviceAccessible
{
    #[inline]
    fn sectors_per_fat(&self) -> FatResult<u32> {
        self.device.read_u32(36)
    }


    #[inline]
    fn root_cluster_no(&self) -> FatResult<u32> {
        self.device.read_u32(44)
    }
}


#[cfg(test)]
mod tests {
    use crate::raw::bpb::fat32::{Fat32BootSector, Fat32BootSectorReadable};
    use crate::test::file_device;

    #[test]
    fn it_root_clusters() {
        let fat32 = Fat32BootSector::new(file_device());
        assert_eq!(fat32.root_cluster_no().unwrap(), 2);
    }
}