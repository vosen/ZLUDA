use rkyv::{Archive, Deserialize, Serialize};
use static_assertions::assert_eq_align;
use std::{io, mem, ops::Deref};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
pub struct KernelMetadataV1 {
    pub ptx_version: u32,
}

// If the alignment is larger than u64, then we need different padding
assert_eq_align!(ArchivedKernelMetadataV1, u32);

impl KernelMetadataV1 {
    pub fn new(ptx_version: u32) -> Self {
        Self { ptx_version }
    }

    const SECTION: &'static str = "zluda";

    pub fn write_linker_section(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Serialization error"))?;
        Self::write_section_bytes(writer, bytes)
    }

    fn write_section_bytes(
        writer: &mut impl io::Write,
        bytes: impl Deref<Target = [u8]>,
    ) -> Result<(), io::Error> {
        writeln!(writer, ".section {}", Self::SECTION)?;
        writeln!(writer, ".p2align 3")?;
        writeln!(writer, ".quad 1")?;
        let remainder = Self::write_chunks_u64(&bytes, writer)?;
        let remainder = Self::write_remainder_u32(remainder, writer)?;
        Self::write_remainder_u8(remainder, writer)
    }

    fn write_chunks_u64<'a>(
        bytes: &'a impl Deref<Target = [u8]>,
        writer: &mut impl io::Write,
    ) -> Result<&'a [u8], io::Error> {
        let mut chunks = bytes.chunks_exact(mem::size_of::<u64>());
        while let Some(chunk) = chunks.next() {
            let val = u64::from_le_bytes(chunk.try_into().unwrap());
            writeln!(writer, ".quad 0x{:016x}", val)?;
        }
        Ok(chunks.remainder())
    }

    fn write_remainder_u32<'a>(
        bytes: &'a [u8],
        writer: &mut impl io::Write,
    ) -> Result<&'a [u8], io::Error> {
        Ok(if bytes.len() >= 4 {
            let val = u32::from_le_bytes(bytes[..4].try_into().unwrap());
            writeln!(writer, ".long 0x{:08x}", val)?;
            &bytes[4..]
        } else {
            bytes
        })
    }

    fn write_remainder_u8<'a>(
        bytes: &'a [u8],
        writer: &mut impl io::Write,
    ) -> Result<(), io::Error> {
        for &byte in bytes {
            writeln!(writer, ".byte 0x{:02x}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn write_section() {
        let bytes = [
            0x16u8, 0x10, 0x15, 0x49, 0x1f, 0x66, 0x00, 0x98, 0xde, 0x22, 0xef, 0xca, 0x2c, 0xab,
            0xb6,
        ];
        let mut writer = Vec::new();
        super::KernelMetadataV1::write_section_bytes(&mut writer, &bytes[..]).unwrap();
        assert_eq!(std::str::from_utf8(&writer).unwrap(), ".section zluda\n.p2align 3\n.quad 1\n.quad 0x9800661f49151016\n.long 0xcaef22de\n.byte 0x2c\n.byte 0xab\n.byte 0xb6\n");
    }
}
