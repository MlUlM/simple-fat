use crate::bpb::BpbReadable;
use crate::dir::data::dir::DirEntries;
use crate::dir::data::file::RegularFile;
use crate::dir::entry::short::ShortDirEntryReadable;
use crate::error::{FatError, FatResult};
use crate::FatDeviceAccessible;

pub mod dir;
pub mod file;

pub struct DataEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    dir_entries: DirEntries<D>,
}


impl<D> DataEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    pub fn new(dir_entries: DirEntries<D>) -> DataEntries<D> {
        Self {
            dir_entries
        }
    }


    pub fn find(&mut self, file_name: &str) -> Option<Data<D>> {
        let data = self.next()?;
        let name = data.name().ok();
        if name
            .map(|name| name.to_str().map(|name| name == file_name).unwrap_or(false))
            .unwrap_or(false)
        {
            return Some(data);
        }

        match data {
            Data::RegularFile(_) => { self.find(file_name) }
            Data::Dir(dir) => {
                if let Some(data) = dir.into_data_entries().find(file_name) {
                    Some(data)
                }else{
                    self.find(file_name)
                }
            }
        }
    }
}


impl<D> Iterator for DataEntries<D>
    where D: FatDeviceAccessible + Clone + BpbReadable
{
    type Item = Data<D>;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = self
            .dir_entries
            .next()?;

        if let Ok(data) = entry
            .into_detail()
            .and_then(|detail| detail.into_short())
            .and_then(|short| short.data()) {
            Some(data)
        } else {
            self.next()
        }
    }
}


pub enum Data<D>
    where D: FatDeviceAccessible + BpbReadable + Clone
{
    RegularFile(RegularFile<D>),
    Dir(DirEntries<D>),
}


impl<D> ShortDirEntryReadable for Data<D> where D: FatDeviceAccessible + BpbReadable + Clone {
    fn name_buff(&self) -> FatResult<[u8; 11]> {
        match self {
            Self::RegularFile(file) => { file.name_buff() }
            Self::Dir(dir) => { dir.name_buff() }
        }
    }


    fn first_cluster_no_hi(&self) -> FatResult<u16> {
        match self {
            Self::RegularFile(file) => { file.first_cluster_no_hi() }
            Self::Dir(dir) => { dir.first_cluster_no_hi() }
        }
    }


    fn first_cluster_no_lo(&self) -> FatResult<u16> {
        match self {
            Self::RegularFile(file) => { file.first_cluster_no_lo() }
            Self::Dir(dir) => { dir.first_cluster_no_lo() }
        }
    }


    fn file_size(&self) -> FatResult<u32> {
        match self {
            Self::RegularFile(file) => { file.file_size() }
            Self::Dir(dir) => { dir.file_size() }
        }
    }
}


impl<D> Data<D> where D: FatDeviceAccessible + Clone + BpbReadable {
    pub fn into_dir(self) -> FatResult<DirEntries<D>> {
        if let Self::Dir(dir) = self {
            Ok(dir)
        } else {
            Err(FatError::InvalidDirEntryType)
        }
    }


    pub fn into_regular_file(self) -> FatResult<RegularFile<D>> {
        if let Self::RegularFile(file) = self {
            Ok(file)
        } else {
            Err(FatError::InvalidDirEntryType)
        }
    }


    #[inline]
    pub fn is_regular_file(&self) -> bool {
        matches!(self, Self::RegularFile(_))
    }
}
