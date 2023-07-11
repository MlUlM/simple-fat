pub trait CommonReadable{
    /// Jump instruction to boot code.
    fn jmp_boot(&self) -> [u8; 3];


    fn oem_name(&self) -> &str;


}



/// This structure indicates fields up to offset36 in `Bbp` or Boot sector
pub(crate) struct Common{

}



impl CommonReadable for Common{
    fn jmp_boot(&self) -> [u8; 3] {
        todo!()
    }

    fn oem_name(&self) -> &str {
        todo!()
    }
}

