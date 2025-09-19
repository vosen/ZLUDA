use std::io::Write;
use tar::Header;

#[derive(serde::Serialize, serde::Deserialize)]
struct Manifest {
    kernel_name: String,
    parameters: Vec<Parameter>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Parameter {
    pointer_offsets: Vec<ParameterPointer>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ParameterPointer {
    offset_in_param: usize,
    offset_in_buffer: usize,
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
    source: String,
    kernel_params: Vec<KernelParameter>,
) -> std::io::Result<()> {
    let archive = zstd::Encoder::new(writer, 0)?;
    let mut builder = tar::Builder::new(archive);
    let (mut header, manifest) = Manifest {
        kernel_name,
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
