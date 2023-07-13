use crate::error::FatResult;
use crate::FatDeviceAccessible;
use crate::raw::bpb::general::CommonBootSectorReadable;

pub struct CommonBootSector<D> {
    device: D,
}


impl<D> CommonBootSector<D> where D: FatDeviceAccessible + Clone {
    #[inline]
    pub const fn new(device: D) -> CommonBootSector<D> {
        Self {
            device
        }
    }
}


impl<D> CommonBootSectorReadable for CommonBootSector<D>
    where D: FatDeviceAccessible + Clone
{
    #[inline]
    fn oem_name_buff(&self) -> FatResult<[u8; 8]> {
        let mut buff = [0; 8];
        self.device.read(&mut buff, 3, 8)?;

        Ok(buff)
    }


    #[inline]
    fn bytes_per_sector(&self) -> FatResult<u16> {
        self.device.read_u16(11)
    }


    #[inline]
    fn sectors_per_cluster(&self) -> FatResult<u8> {
        self.device.read_u8(13)
    }


    #[inline]
    fn reserved_sectors(&self) -> FatResult<u16> {
        self.device.read_u16(14)
    }


    #[inline]
    fn total_sector16(&self) -> FatResult<u16> {
        self.device.read_u16(19)
    }


    #[inline]
    fn total_sector32(&self) -> FatResult<u32> {
        self.device.read_u32(32)
    }


    #[inline]
    fn num_fats(&self) -> FatResult<u8> {
        self.device.read_u8(16)
    }
}


#[cfg(test)]
mod tests {
    use crate::raw::bpb::general::buffer::CommonBootSector;
    use crate::raw::bpb::general::CommonBootSectorReadable;
    use crate::test::file_device;

    #[cfg(feature = "alloc")]
    #[test]
    fn it_oem_name() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.oem_name().unwrap().to_str().unwrap(), "mkfs.fat");
    }


    #[test]
    fn it_sectors_per_cluster() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.sectors_per_cluster().unwrap(), 2);
    }


    #[test]
    fn it_reserved_sectors() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.reserved_sectors().unwrap(), 0x20);
    }


    #[test]
    fn it_total_sectors16_is_zero_if_fat32() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.total_sector16().unwrap(), 0x00);
    }


    #[test]
    fn it_total_sectors32_is_non_zero_if_fat32() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.total_sector32().unwrap(), 4 << 16);
    }


    #[test]
    fn it_num_fats() {
        let general = CommonBootSector::new(file_device());
        assert_eq!(general.num_fats().unwrap(), 2);
    }
}
