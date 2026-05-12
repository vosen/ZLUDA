use object::read::elf::FileHeader as _;
use object::write::elf as elf_write;
use object::{elf, Endian, Endianness, Object, ObjectSection};
use rkyv::api::high::HighSerializer;
use rkyv::ser::allocator::ArenaHandle;
use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Portable, Serialize};
use static_assertions::assert_eq_align;
use std::{io, mem};

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
pub struct ModuleMetadataV1 {
    pub sm_version: u32,
}

// If the alignment is larger than u64, then we need different padding
assert_eq_align!(ArchivedModuleMetadataV1, u32);

impl ModuleMetadataV1 {
    pub fn new(sm_version: u32) -> Self {
        Self { sm_version }
    }

    const SECTION: &'static str = "zluda";
    const VERSION: u64 = 1;

    pub fn write_object(&self, main_elf: &[u8], writer: &mut impl io::Write) -> io::Result<()> {
        write_object(self, Self::SECTION, Self::VERSION, main_elf, writer)
    }

    pub fn read_object<'a>(elf_bytes: &'a [u8]) -> Option<&'a ArchivedModuleMetadataV1> {
        read_object(elf_bytes, Self::SECTION, Self::VERSION)
    }
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct Global32Bit {
    pub name: String,
    pub initializer: Vec<u8>,
    pub align: u32,
}

#[derive(Archive, Serialize, Deserialize, Debug)]
pub struct ModuleMetadata32Bit {
    pub globals: Vec<Global32Bit>,
    pub explicit_arg_count: Vec<(String, u32)>,
}

// If the alignment is larger than u64, then we need different padding
assert_eq_align!(ArchivedModuleMetadata32Bit, u32);

impl ModuleMetadata32Bit {
    const SECTION: &'static str = "zluda32";
    const VERSION: u64 = 1;

    pub fn write_object(&self, main_elf: &[u8], writer: &mut impl io::Write) -> io::Result<()> {
        write_object(self, Self::SECTION, Self::VERSION, main_elf, writer)
    }

    pub fn read_object<'a>(elf_bytes: &'a [u8]) -> Option<&'a ArchivedModuleMetadata32Bit> {
        read_object(elf_bytes, Self::SECTION, Self::VERSION)
    }

    pub fn copy_object<'a>(elf_bytes: &'a [u8]) -> Option<ModuleMetadata32Bit> {
        read_object::<ArchivedModuleMetadata32Bit>(elf_bytes, Self::SECTION, Self::VERSION)
            .and_then(|archived| rkyv::deserialize::<_, rkyv::rancor::Failure>(archived).ok())
    }
}

pub fn write_object(
    this: &impl for<'a, 'b> Serialize<
        HighSerializer<AlignedVec, ArenaHandle<'b>, rkyv::rancor::Failure>,
    >,
    section: &str,
    version: u64,
    main_elf: &[u8],
    writer: &mut impl io::Write,
) -> io::Result<()> {
    let main_header = elf::FileHeader64::<object::Endianness>::parse(main_elf)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let endian = main_header
        .endian()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    let data = rkyv::to_bytes::<rkyv::rancor::Failure>(this)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut buf = Vec::new();
    let mut writer_elf = elf_write::Writer::new(endian, true, &mut buf);
    writer_elf.reserve_file_header();
    let section_name = writer_elf.add_section_name(section.as_bytes());
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
    writer_elf.write(&endian.write_u64_bytes(version));
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

fn read_object<'a, T: Portable>(elf_bytes: &'a [u8], section: &str, version: u64) -> Option<&'a T> {
    let elf_file = object::read::elf::ElfFile64::<Endianness>::parse(elf_bytes).ok()?;
    let zluda_section = elf_file.section_by_name(section)?;
    let data = zluda_section.data().ok()?;
    if data.len() < 8 {
        return None;
    }
    let in_section_version = u64::from_le_bytes(data[..8].try_into().unwrap());
    if in_section_version != version {
        return None;
    }
    Some(unsafe { rkyv::access_unchecked::<T>(&data[8..]) })
}
