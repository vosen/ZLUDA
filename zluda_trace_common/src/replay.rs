use rustc_hash::FxHashMap;
use std::io::{Read, Write};
use tar::Header;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Manifest {
    pub kernel_name: String,
    pub outputs: bool,
    pub config: LaunchConfig,
    pub parameters: Vec<Parameter>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LaunchConfig {
    pub grid_dim: (u32, u32, u32),
    pub block_dim: (u32, u32, u32),
    pub shared_mem_bytes: u32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Parameter {
    pub pointer_offsets: Vec<ParameterPointer>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ParameterPointer {
    pub offset_in_param: usize,
    pub offset_in_buffer: usize,
}

impl Manifest {
    const PATH: &'static str = "manifest.json";

    fn serialize(&self) -> std::io::Result<(Header, Vec<u8>)> {
        let vec = serde_json::to_vec(self)?;
        let mut header = Header::new_gnu();
        header.set_size(vec.len() as u64);
        Ok((header, vec))
    }
}

pub struct KernelParameter {
    pub data: Vec<u8>,
    pub device_ptrs: Vec<(usize, usize, Vec<u8>, Vec<u8>)>,
}

pub fn save(
    writer: impl Write,
    kernel_name: String,
    has_outputs: bool,
    config: LaunchConfig,
    source: String,
    kernel_params: Vec<KernelParameter>,
) -> std::io::Result<()> {
    let archive = zstd::Encoder::new(writer, 0)?;
    let mut builder = tar::Builder::new(archive);
    let (mut header, manifest) = Manifest {
        kernel_name,
        outputs: has_outputs,
        config,
        parameters: kernel_params
            .iter()
            .map(|param| Parameter {
                pointer_offsets: param
                    .device_ptrs
                    .iter()
                    .map(
                        |(offset_in_param, offset_in_buffer, _, _)| ParameterPointer {
                            offset_in_param: *offset_in_param,
                            offset_in_buffer: *offset_in_buffer,
                        },
                    )
                    .collect(),
            })
            .collect(),
    }
    .serialize()?;
    builder.append_data(&mut header, Manifest::PATH, &*manifest)?;
    let mut header = Header::new_gnu();
    header.set_size(source.len() as u64);
    builder.append_data(&mut header, "source.ptx", source.as_bytes())?;
    for (i, param) in kernel_params.into_iter().enumerate() {
        let path = format!("param_{i}.bin");
        let mut header = Header::new_gnu();
        header.set_size(param.data.len() as u64);
        builder.append_data(&mut header, &*path, &*param.data)?;
        for (offset_in_param, _, data_before, data_after) in param.device_ptrs {
            let path = format!("param_{i}_ptr_{offset_in_param}_pre.bin");
            let mut header = Header::new_gnu();
            header.set_size(data_before.len() as u64);
            builder.append_data(&mut header, &*path, &*data_before)?;
            if !has_outputs {
                continue;
            }
            let path = format!("param_{i}_ptr_{offset_in_param}_post.bin");
            let mut header = Header::new_gnu();
            header.set_size(data_after.len() as u64);
            builder.append_data(&mut header, &*path, &*data_after)?;
        }
    }
    builder.finish()?;
    builder.into_inner()?.finish()?;
    Ok(())
}

pub fn load(reader: impl Read) -> (Manifest, String, FxHashMap<String, Vec<u8>>) {
    let archive = zstd::Decoder::new(reader).unwrap();
    let mut archive = tar::Archive::new(archive);
    let mut manifest = None;
    let mut source = None;
    let mut buffers = FxHashMap::default();
    for entry in archive.entries().unwrap() {
        let mut entry = entry.unwrap();
        let path = entry.path().unwrap().to_string_lossy().to_string();
        match &*path {
            Manifest::PATH => {
                manifest = Some(serde_json::from_reader::<_, Manifest>(&mut entry).unwrap());
            }
            "source.ptx" => {
                let mut string = String::new();
                entry.read_to_string(&mut string).unwrap();
                dbg!(string.len());
                source = Some(string);
            }
            _ => {
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer).unwrap();
                buffers.insert(path, buffer);
            }
        }
    }
    let manifest = manifest.unwrap();
    let source = source.unwrap();
    (manifest, source, buffers)
}
