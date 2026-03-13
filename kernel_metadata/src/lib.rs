use object::read::elf::FileHeader as _;
use object::write::elf as elf_write;
use object::{elf, Endian, Endianness, Object, ObjectSection};
use rkyv::{rancor, Archive, Deserialize, Serialize};
use static_assertions::assert_eq_align;
use std::{io, mem};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
pub struct KernelMetadataV1 {
    pub sm_version: u32,
}

// If the alignment is larger than u64, then we need different padding
assert_eq_align!(ArchivedKernelMetadataV1, u32);

impl KernelMetadataV1 {
    pub fn new(sm_version: u32) -> Self {
        Self { sm_version }
    }

    const SECTION: &'static str = "zluda";

    pub fn write_object(&self, main_elf: &[u8], writer: &mut impl io::Write) -> io::Result<()> {
        let main_header = elf::FileHeader64::<object::Endianness>::parse(main_elf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        let endian = main_header
            .endian()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        let data = rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut buf = Vec::new();
        let mut writer_elf = elf_write::Writer::new(endian, true, &mut buf);
        writer_elf.reserve_file_header();
        let section_name = writer_elf.add_section_name(Self::SECTION.as_bytes());
        writer_elf.reserve_section_index();
        let section_offset = writer_elf.reserve(data.len() + mem::size_of::<u64>(), 8);
        writer_elf.reserve_shstrtab_section_index();
        writer_elf.reserve_shstrtab();
        writer_elf.reserve_section_headers();
        writer_elf
            .write_file_header(&elf_write::FileHeader {
                os_abi: main_header.e_ident().os_abi,
                abi_version: main_header.e_ident().abi_version,
                e_type: main_header.e_type.get(endian),
                e_machine: main_header.e_machine.get(endian),
                e_entry: 0,
                e_flags: main_header.e_flags.get(endian),
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        writer_elf.write_align(8);
        writer_elf.write(&endian.write_u64_bytes(1));
        writer_elf.write(&data);
        writer_elf.write_shstrtab();
        writer_elf.write_null_section_header();
        writer_elf.write_section_header(&elf_write::SectionHeader {
            name: Some(section_name),
            sh_type: elf::SHT_PROGBITS,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: section_offset as u64,
            sh_size: (data.len() + mem::size_of::<u64>()) as u64,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 8,
            sh_entsize: 0,
        });
        writer_elf.write_shstrtab_section_header();
        writer.write_all(&buf)
    }

    pub fn read_object(elf_bytes: &[u8]) -> Option<Self> {
        let elf_file = object::read::elf::ElfFile64::<Endianness>::parse(elf_bytes).ok()?;
        let zluda_section = elf_file.section_by_name(Self::SECTION)?;
        let data = zluda_section.data().ok()?;
        if data.len() < 8 {
            return None;
        }
        let version = u64::from_le_bytes(data[..8].try_into().unwrap());
        if version != 1 {
            return None;
        }
        rkyv::from_bytes::<KernelMetadataV1, rancor::Error>(&data[8..]).ok()
    }
}
