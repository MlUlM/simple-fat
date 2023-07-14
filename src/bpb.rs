use crate::bpb::fat32::{Fat32BootSector, Fat32BootSectorReadable};
use crate::bpb::general::buffer::GeneralBootSector;
use crate::bpb::general::GeneralBootSectorReadable;
use crate::dir::data::DataEntries;
use crate::dir::data::dir::DirEntries;
use crate::error::{FatDeviceError, FatResult};
use crate::FatDeviceAccessible;

mod general;
mod fat32;


pub trait BpbReadable {
    fn data_cluster_offset_at(&self, cluster_no: usize) -> FatResult<usize>;
}


#[derive(Clone, )]
pub struct BpbFat32<D> {
    general: GeneralBootSector<D>,
    fat32: Fat32BootSector<D>,

    pub(crate) device: D,
}


impl<D> FatDeviceAccessible for BpbFat32<D> where D: FatDeviceAccessible {
    fn read(&self, buff: &mut [u8], offset: usize, bytes: usize) -> Result<(), FatDeviceError> {
        self.device.read(buff, offset, bytes)
    }


    fn write(&mut self, buff: &[u8], offset: usize) -> Result<(), FatDeviceError> {
        self.device.write(buff, offset)
    }
}


impl<D> BpbFat32<D>
    where D: FatDeviceAccessible + Clone
{
    #[inline]
    pub fn new(device: D) -> BpbFat32<D> {
        Self {
            general: GeneralBootSector::new(device.clone()),
            fat32: Fat32BootSector::new(device.clone()),
            device,
        }
    }


    #[inline]
    pub fn root_dir(&self) -> FatResult<DataEntries<BpbFat32<D>>> {
        Ok(DataEntries::new(DirEntries::root(self.clone(), self.data_region_offset_fat32()?)))
    }


    pub(crate) fn data_region_offset_fat32(&self) -> FatResult<usize> {
        let bytes_per_sector = self.general.bytes_per_sector()? as usize;
        let reserve_bytes = self.general.reserved_sectors()? as usize * bytes_per_sector;
        let fat_bytes = self.general.num_fats()? as usize * self.fat32.sectors_per_fat()? as usize * bytes_per_sector;

        Ok(reserve_bytes + fat_bytes)
    }
}


impl<D> BpbReadable for BpbFat32<D> where D: FatDeviceAccessible + Clone {
    #[inline]
    fn data_cluster_offset_at(&self, cluster_no: usize) -> FatResult<usize> {
        let offset = (cluster_no - 2) * self.general.bytes_per_sector()? as usize * self.general.sectors_per_cluster()? as usize;
        Ok(offset + self.data_region_offset_fat32()?)
    }
}


#[inline]
pub(crate) fn buff_read_u16(buff: &[u8], index: usize) -> u16 {
    (buff[index + 1] as u16) << 8 | buff[index] as u16
}


#[inline]
pub(crate) fn buff_read_u32(buff: &[u8], index: usize) -> u32 {
    let shift = |i| (buff[index + i] as u32) << (i * 8);
    shift(3) | shift(2) | shift(1) | shift(0)
}


#[cfg(test)]
mod tests {
    use crate::bpb::BpbFat32;
    use crate::test::file_device;

    #[test]
    fn it_data_region_offset_fat32() {
        let bpb = BpbFat32::new(file_device());
        assert_eq!(bpb.data_region_offset_fat32().unwrap(), 0x102000);
    }
}
