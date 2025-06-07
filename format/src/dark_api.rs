use crate::CudaDisplay;
use cuda_types::dark_api::*;

impl CudaDisplay for FatbincWrapper {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ magic: ")?;
        CudaDisplay::write(&self.magic, "", 0, writer)?;
        writer.write_all(b", version: ")?;
        CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(b", data: ")?;
        CudaDisplay::write(&self.data, "", 0, writer)?;
        writer.write_all(b", filename_or_fatbins: ")?;
        CudaDisplay::write(&self.filename_or_fatbins, "", 0, writer)?;
        writer.write_all(b" }")
    }
}

impl CudaDisplay for FatbinHeader {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(b"{ magic: ")?;
        CudaDisplay::write(&self.magic, "", 0, writer)?;
        writer.write_all(b", version: ")?;
        CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(b", header_size: ")?;
        CudaDisplay::write(&self.header_size, "", 0, writer)?;
        writer.write_all(b", files_size: ")?;
        CudaDisplay::write(&self.files_size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}