use crate::error::FatResult;
use crate::FatDeviceAccessible;
use crate::raw::bpb::fat32::{Fat32BootSector, Fat32BootSectorReadable};
use crate::raw::bpb::general::buffer::GeneralBootSector;
use crate::raw::bpb::general::GeneralBootSectorReadable;
use crate::raw::dir::short::ShortDirEntry;

mod general;
mod fat32;


pub struct Bpb {
    pub bytes_per_sector: usize,
    pub reserved_sectors: usize,
    pub num_fats: usize,
    pub sectors_per_fat: usize,
    pub sectors_per_cluster: usize,
}


// impl Bpb {
//     #[inline]
//     pub const fn new(
//         bytes_per_sector: usize,
//         reserved_sectors: usize,
//         num_fats: usize,
//         sectors_per_fat: usize,
//         sectors_per_cluster: usize,
//     ) -> Self {
//         Self {
//             bytes_per_sector,
//             reserved_sectors,
//             num_fats,
//             sectors_per_fat,
//             sectors_per_cluster,
//         }
//     }
//
//
//     pub fn data_region_offset_fat32(&self) -> usize {
//         let reserve_bytes = self.reserved_sectors * self.bytes_per_sector;
//         let fat_bytes = self.num_fats * self.sectors_per_fat * self.bytes_per_sector;
//
//         reserve_bytes + fat_bytes
//     }
//
//
//     pub fn data_cluster_offset_at(&self, cluster_no: usize) -> usize {
//         let offset = (cluster_no - 2) * self.bytes_per_sector * self.sectors_per_cluster;
//         offset + self.data_region_offset_fat32()
//     }
// }
//


#[derive(Clone)]
pub struct BpbFat32<D> {
    general: GeneralBootSector<D>,
    fat32: Fat32BootSector<D>,
    device: D,
}


impl<D> BpbFat32<D>
    where D: FatDeviceAccessible + Clone
{
    pub fn new(device: D) -> BpbFat32<D> {
        Self {
            general: GeneralBootSector::new(device.clone()),
            fat32: Fat32BootSector::new(device.clone()),
            device,
        }
    }


    #[inline]
    pub fn root_dir(&self) -> FatResult<ShortDirEntry<D>> {
        Ok(ShortDirEntry::new(self.device.clone(), self.clone(), self.data_region_offset_fat32()?))
    }


    pub(crate) fn data_region_offset_fat32(&self) -> FatResult<usize> {
        let bytes_per_sector = self.general.bytes_per_sector()? as usize;
        let reserve_bytes = self.general.reserved_sectors()? as usize * bytes_per_sector;
        let fat_bytes = self.general.num_fats()? as usize * self.fat32.sectors_per_fat()? as usize * bytes_per_sector;

        Ok(reserve_bytes + fat_bytes)
    }


    pub fn data_cluster_at(&self, cluster_no: usize) -> FatResult<usize> {
        let offset = (cluster_no - 2) * self.general.bytes_per_sector()? as usize * self.general.sectors_per_cluster()? as usize;
        Ok(offset + self.data_region_offset_fat32()?)
    }
}


#[inline]
pub(crate) fn buff_read_u16(buff: &[u8], index: usize) -> u16 {
    // let (_, buff, _) = unsafe { buff[index..=index + 1].align_to::<u16>() };
    // buff[0]
    (buff[index + 1] as u16) << 8 | buff[index] as u16
}


#[inline]
pub(crate) fn buff_read_u32(buff: &[u8], index: usize) -> u32 {
    let shift = |i| (buff[index + i] as u32) << (i * 8);
    shift(3) | shift(2) | shift(1) | shift(0)
}


#[cfg(test)]
mod tests {
    use crate::raw::bpb::BpbFat32;
    use crate::test::file_device;

    #[test]
    fn it_data_region_offset_fat32() {
        let bpb = BpbFat32::new(file_device());
        assert_eq!(bpb.data_region_offset_fat32().unwrap(), 0x102000);
    }
}
