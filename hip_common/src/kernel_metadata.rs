// TODO: instead of writing metadata to a temporary buffer.
// write it directly into a section string

use crate::unwrap_or_return;
use goblin::elf::{Elf, SectionHeader};
use std::ops::Range;

pub fn get_section<'a>(section_name: &str, module: &'a [u8]) -> Option<&'a [u8]> {
    let header = Elf::parse_header(module).ok()?;
    if header.e_shoff == 0 {
        return None;
    }
    let ctx = goblin::container::Ctx::new(header.container().ok()?, header.endianness().ok()?);
    let section_headers = SectionHeader::parse(
        module,
        header.e_shoff as usize,
        header.e_shnum as usize,
        ctx,
    )
    .ok()?;
    let string_table_section_header = match section_headers.get(header.e_shstrndx as usize) {
        Some(h) => h,
        None => return None,
    };
    let string_section = &module[section_range(string_table_section_header)];
    let zluda_rt_section_header = unwrap_or_return!(
        section_headers.iter().find(|header| {
            if let Some(nul_pos) = memchr::memchr(0, &string_section[header.sh_name..]) {
                let name = &string_section[header.sh_name..header.sh_name + nul_pos];
                return name == section_name.as_bytes();
            }
            false
        }),
        None
    );
    Some(&module[section_range(zluda_rt_section_header)])
}

fn section_range(section: &SectionHeader) -> Range<usize> {
    section.sh_offset as usize
        ..(section.sh_offset as usize).saturating_add(section.sh_size as usize)
}

pub mod zluda {
    use crate::unwrap_or_continue;
    use capnp::message::{ReaderOptions, TypedBuilder, TypedReader};
    use std::num::NonZeroU32;

    pub const SECTION_STR: &'static str = "zluda_meta";

    pub fn write<'a>(
        sm_version: u32,
        kernels: impl ExactSizeIterator<Item = (&'a str, Option<NonZeroU32>, Option<NonZeroU32>)>,
    ) -> Vec<u8> {
        let mut builder = TypedBuilder::<crate::zluda_capnp::metadata::Owned>::new_default();
        let result = builder.init_root();
        let mut version1 = result.init_version1();
        version1.set_sm_version(sm_version);
        let mut kernels_emitter = version1.init_kernels(kernels.len() as u32);
        for (i, (name, min, max)) in kernels.enumerate() {
            let mut kernel_emitter = kernels_emitter.reborrow().get(i as u32);
            kernel_emitter.set_name(name);
            let min = min.map(NonZeroU32::get).unwrap_or_default();
            kernel_emitter.set_min_goup_size(min);
            let max = max.map(NonZeroU32::get).unwrap_or_default();
            kernel_emitter.set_max_group_size(max);
        }
        capnp::serialize::write_message_to_words(builder.borrow_inner())
    }

    pub fn read<'a>(
        mut message: &'a [u8],
        mut on_kernel: impl FnMut(&'a str, u32, u32),
    ) -> Result<u32, capnp::Error> {
        let reader =
            capnp::serialize::read_message_from_flat_slice(&mut message, ReaderOptions::new())?;
        let typed_reader = TypedReader::<_, crate::zluda_capnp::metadata::Owned>::new(reader);
        let root = typed_reader.get()?;
        let version1 = root.get_version1()?;
        let kernels = version1.get_kernels()?;
        for kernel in kernels.iter() {
            on_kernel(
                // This just transmutes the lifetime
                // The string slice points to the underlying `message` byte array, so it is safe
                unsafe { std::mem::transmute(unwrap_or_continue!(kernel.get_name())) },
                kernel.get_min_goup_size(),
                kernel.get_max_group_size(),
            );
        }
        Ok(version1.get_sm_version())
    }
}

pub mod zluda_rt6 {
    use crate::raytracing::{Variable, VariablesBlock};
    use capnp::message::{ReaderOptions, TypedBuilder, TypedReader};
    use rustc_hash::FxHashMap;
    use std::{alloc::Layout, ffi::CString};

    pub const SECTION_STR: &'static str = "zluda_rt6_meta";

    pub fn write(
        attributes: &VariablesBlock,
        variables: &VariablesBlock,
        is_callable: bool,
    ) -> Vec<u8> {
        let mut builder = TypedBuilder::<crate::zluda_rt6_capnp::metadata::Owned>::new_default();
        let result = builder.init_root();
        let mut version1 = result.init_version1();
        version1.set_attributes_align(attributes.layout.align() as u32);
        version1.set_attributes_size(attributes.layout.size() as u32);
        let mut attributes_builder = version1
            .reborrow()
            .init_attributes(attributes.variables.len() as u32);
        for (idx, (name, var)) in attributes.variables.iter().enumerate() {
            let mut attribute_builder = attributes_builder.reborrow().get(idx as u32);
            attribute_builder.set_name(&name.as_c_str().to_string_lossy());
            attribute_builder.set_offset(var.offset);
            attribute_builder.set_size(var.size);
        }
        version1.set_variables_align(variables.layout.align() as u32);
        version1.set_variables_size(variables.layout.size() as u32);
        let mut variables_builder = version1
            .reborrow()
            .init_variables(variables.variables.len() as u32);
        for (idx, (name, var)) in variables.variables.iter().enumerate() {
            let mut variables_builder = variables_builder.reborrow().get(idx as u32);
            variables_builder.set_name(&name.as_c_str().to_string_lossy());
            variables_builder.set_offset(var.offset);
            variables_builder.set_size(var.size);
            variables_builder.set_default_value(&var.default_value);
        }
        version1.set_is_callable(is_callable);
        capnp::serialize::write_message_to_words(builder.borrow_inner())
    }

    pub fn read(mut message: &[u8]) -> Result<Metadata, capnp::Error> {
        let reader =
            capnp::serialize::read_message_from_flat_slice(&mut message, ReaderOptions::new())?;
        let typed_reader = TypedReader::<_, crate::zluda_rt6_capnp::metadata::Owned>::new(reader);
        let root = typed_reader.get()?;
        let version1 = root.get_version1()?;
        let mut attribute_variables = FxHashMap::default();
        let attribute_size = version1.get_attributes_size();
        let attribute_align = version1.get_attributes_align();
        for attribute in version1.get_attributes()? {
            let name = attribute.get_name()?;
            let offset = attribute.get_offset();
            let size = attribute.get_size();
            attribute_variables.insert(
                CString::new(name).map_err(|err| capnp::Error {
                    kind: capnp::ErrorKind::Failed,
                    description: err.to_string(),
                })?,
                Variable {
                    offset,
                    size,
                    default_value: Vec::new(),
                },
            );
        }
        let attribute_variables = new_block(attribute_variables, attribute_size, attribute_align)?;
        let variables_size = version1.get_variables_size();
        let variables_align = version1.get_variables_align();
        let mut variables = FxHashMap::default();
        for variable in version1.get_variables()? {
            let name = variable.get_name()?;
            let offset = variable.get_offset();
            let size = variable.get_size();
            let default_value = variable.get_default_value()?.to_vec();
            variables.insert(
                CString::new(name).map_err(|err| capnp::Error {
                    kind: capnp::ErrorKind::Failed,
                    description: err.to_string(),
                })?,
                Variable {
                    offset,
                    size,
                    default_value,
                },
            );
        }
        let variables = new_block(variables, variables_size, variables_align)?;
        let is_callable = version1.get_is_callable();
        Ok(Metadata {
            attribute_variables,
            variables,
            is_callable,
        })
    }

    fn new_block(
        variables: FxHashMap<CString, Variable>,
        size: u32,
        align: u32,
    ) -> Result<VariablesBlock, capnp::Error> {
        let attribute_variables = VariablesBlock {
            variables,
            layout: Layout::from_size_align(size as usize, align as usize).map_err(|err| {
                capnp::Error {
                    kind: capnp::ErrorKind::Failed,
                    description: err.to_string(),
                }
            })?,
        };
        Ok(attribute_variables)
    }

    pub struct Metadata {
        pub attribute_variables: VariablesBlock,
        pub variables: VariablesBlock,
        pub is_callable: bool,
    }
}
