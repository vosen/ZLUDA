// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
impl crate::CudaDisplay for cuda_types::nvml::nvmlDevice_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstance_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPciInfoExt_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(domain), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.domain, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bus), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bus, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(device), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.device, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pciDeviceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pciDeviceId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pciSubSystemId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pciSubSystemId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(baseClass), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.baseClass, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(subClass), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.subClass, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(busId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.busId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPciInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(busIdLegacy), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.busIdLegacy, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(domain), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.domain, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bus), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bus, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(device), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.device, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pciDeviceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pciDeviceId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pciSubSystemId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pciSubSystemId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(busId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.busId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEccErrorCounts_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(l1Cache), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.l1Cache, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(l2Cache), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.l2Cache, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(deviceMemory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.deviceMemory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(registerFile), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.registerFile, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUtilization_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(gpu), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpu, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memory, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMemory_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(total), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.total, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(free), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.free, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(used), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.used, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMemory_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(total), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.total, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(free), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.free, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(used), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.used, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlBAR1Memory_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(bar1Total), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bar1Total, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bar1Free), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bar1Free, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bar1Used), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bar1Used, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessInfo_v1_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(usedGpuMemory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.usedGpuMemory, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessInfo_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(usedGpuMemory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.usedGpuMemory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpuInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuInstanceId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(computeInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.computeInstanceId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessDetail_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(usedGpuMemory), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.usedGpuMemory, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpuInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuInstanceId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(computeInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.computeInstanceId, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(usedGpuCcProtectedMemory), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.usedGpuCcProtectedMemory, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessDetailList_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mode, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(numProcArrayEntries), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numProcArrayEntries, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(procArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.procArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDeviceAttributes_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(sharedCopyEngineCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.sharedCopyEngineCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedDecoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedDecoderCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedEncoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedEncoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedJpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedJpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedOfaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedOfaCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(gpuInstanceSliceCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.gpuInstanceSliceCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(computeInstanceSliceCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.computeInstanceSliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memorySizeMB), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memorySizeMB, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlC2cModeInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(isC2cEnabled), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isC2cEnabled, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlRowRemapperHistogramValues_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(max), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.max, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(high), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.high, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(partial), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.partial, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(low), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.low, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(none), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.none, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlBridgeChipType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlBridgeChipType_enum::NVML_BRIDGE_CHIP_PLX => {
                writer.write_all(stringify!(NVML_BRIDGE_CHIP_PLX).as_bytes())
            }
            &cuda_types::nvml::nvmlBridgeChipType_enum::NVML_BRIDGE_CHIP_BRO4 => {
                writer.write_all(stringify!(NVML_BRIDGE_CHIP_BRO4).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum::NVML_NVLINK_COUNTER_UNIT_CYCLES => {
                writer.write_all(stringify!(NVML_NVLINK_COUNTER_UNIT_CYCLES).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum::NVML_NVLINK_COUNTER_UNIT_PACKETS => {
                writer.write_all(stringify!(NVML_NVLINK_COUNTER_UNIT_PACKETS).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum::NVML_NVLINK_COUNTER_UNIT_BYTES => {
                writer.write_all(stringify!(NVML_NVLINK_COUNTER_UNIT_BYTES).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum::NVML_NVLINK_COUNTER_UNIT_RESERVED => {
                writer
                    .write_all(stringify!(NVML_NVLINK_COUNTER_UNIT_RESERVED).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountUnits_enum::NVML_NVLINK_COUNTER_UNIT_COUNT => {
                writer.write_all(stringify!(NVML_NVLINK_COUNTER_UNIT_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_NOP => {
                writer
                    .write_all(stringify!(NVML_NVLINK_COUNTER_PKTFILTER_NOP).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_READ => {
                writer
                    .write_all(stringify!(NVML_NVLINK_COUNTER_PKTFILTER_READ).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_WRITE => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_WRITE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_RATOM => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_RATOM).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_NRATOM => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_NRATOM).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_FLUSH => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_FLUSH).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_RESPDATA => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_RESPDATA).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_RESPNODATA => {
                writer
                    .write_all(
                        stringify!(NVML_NVLINK_COUNTER_PKTFILTER_RESPNODATA).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlNvLinkUtilizationCountPktTypes_enum::NVML_NVLINK_COUNTER_PKTFILTER_ALL => {
                writer
                    .write_all(stringify!(NVML_NVLINK_COUNTER_PKTFILTER_ALL).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkUtilizationControl_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(units), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.units, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pktfilter), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pktfilter, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkCapability_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_P2P_SUPPORTED => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_P2P_SUPPORTED).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_SYSMEM_ACCESS => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_SYSMEM_ACCESS).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_P2P_ATOMICS => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_P2P_ATOMICS).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_SYSMEM_ATOMICS => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_SYSMEM_ATOMICS).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_SLI_BRIDGE => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_SLI_BRIDGE).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_VALID => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_VALID).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkCapability_enum::NVML_NVLINK_CAP_COUNT => {
                writer.write_all(stringify!(NVML_NVLINK_CAP_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkErrorCounter_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_DL_REPLAY => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_DL_REPLAY).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_DL_RECOVERY => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_DL_RECOVERY).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_DL_CRC_FLIT => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_DL_CRC_FLIT).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_DL_CRC_DATA => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_DL_CRC_DATA).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_DL_ECC_DATA => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_DL_ECC_DATA).as_bytes())
            }
            &cuda_types::nvml::nvmlNvLinkErrorCounter_enum::NVML_NVLINK_ERROR_COUNT => {
                writer.write_all(stringify!(NVML_NVLINK_ERROR_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlIntNvLinkDeviceType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlIntNvLinkDeviceType_enum::NVML_NVLINK_DEVICE_TYPE_GPU => {
                writer.write_all(stringify!(NVML_NVLINK_DEVICE_TYPE_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlIntNvLinkDeviceType_enum::NVML_NVLINK_DEVICE_TYPE_IBMNPU => {
                writer.write_all(stringify!(NVML_NVLINK_DEVICE_TYPE_IBMNPU).as_bytes())
            }
            &cuda_types::nvml::nvmlIntNvLinkDeviceType_enum::NVML_NVLINK_DEVICE_TYPE_SWITCH => {
                writer.write_all(stringify!(NVML_NVLINK_DEVICE_TYPE_SWITCH).as_bytes())
            }
            &cuda_types::nvml::nvmlIntNvLinkDeviceType_enum::NVML_NVLINK_DEVICE_TYPE_UNKNOWN => {
                writer.write_all(stringify!(NVML_NVLINK_DEVICE_TYPE_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuLevel_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_INTERNAL => {
                writer.write_all(stringify!(NVML_TOPOLOGY_INTERNAL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_SINGLE => {
                writer.write_all(stringify!(NVML_TOPOLOGY_SINGLE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_MULTIPLE => {
                writer.write_all(stringify!(NVML_TOPOLOGY_MULTIPLE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_HOSTBRIDGE => {
                writer.write_all(stringify!(NVML_TOPOLOGY_HOSTBRIDGE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_NODE => {
                writer.write_all(stringify!(NVML_TOPOLOGY_NODE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuLevel_enum::NVML_TOPOLOGY_SYSTEM => {
                writer.write_all(stringify!(NVML_TOPOLOGY_SYSTEM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuP2PStatus_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_OK => {
                writer.write_all(stringify!(NVML_P2P_STATUS_OK).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_CHIPSET_NOT_SUPPORED => {
                writer
                    .write_all(
                        stringify!(NVML_P2P_STATUS_CHIPSET_NOT_SUPPORED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_CHIPSET_NOT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(NVML_P2P_STATUS_CHIPSET_NOT_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_GPU_NOT_SUPPORTED => {
                writer
                    .write_all(stringify!(NVML_P2P_STATUS_GPU_NOT_SUPPORTED).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_IOH_TOPOLOGY_NOT_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(NVML_P2P_STATUS_IOH_TOPOLOGY_NOT_SUPPORTED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_DISABLED_BY_REGKEY => {
                writer
                    .write_all(stringify!(NVML_P2P_STATUS_DISABLED_BY_REGKEY).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_NOT_SUPPORTED => {
                writer.write_all(stringify!(NVML_P2P_STATUS_NOT_SUPPORTED).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PStatus_enum::NVML_P2P_STATUS_UNKNOWN => {
                writer.write_all(stringify!(NVML_P2P_STATUS_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuP2PCapsIndex_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_READ => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_READ).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_WRITE => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_WRITE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_NVLINK => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_NVLINK).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_ATOMICS => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_ATOMICS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_PCI => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_PCI).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_PROP => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_PROP).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuP2PCapsIndex_enum::NVML_P2P_CAPS_INDEX_UNKNOWN => {
                writer.write_all(stringify!(NVML_P2P_CAPS_INDEX_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlBridgeChipInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(fwVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fwVersion, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlBridgeChipHierarchy_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(bridgeCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bridgeCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bridgeChipInfo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bridgeChipInfo, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSamplingType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_TOTAL_POWER_SAMPLES => {
                writer.write_all(stringify!(NVML_TOTAL_POWER_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_GPU_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_GPU_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_MEMORY_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_MEMORY_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_ENC_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_ENC_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_DEC_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_DEC_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_PROCESSOR_CLK_SAMPLES => {
                writer.write_all(stringify!(NVML_PROCESSOR_CLK_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_MEMORY_CLK_SAMPLES => {
                writer.write_all(stringify!(NVML_MEMORY_CLK_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_MODULE_POWER_SAMPLES => {
                writer.write_all(stringify!(NVML_MODULE_POWER_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_JPG_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_JPG_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_OFA_UTILIZATION_SAMPLES => {
                writer.write_all(stringify!(NVML_OFA_UTILIZATION_SAMPLES).as_bytes())
            }
            &cuda_types::nvml::nvmlSamplingType_enum::NVML_SAMPLINGTYPE_COUNT => {
                writer.write_all(stringify!(NVML_SAMPLINGTYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPcieUtilCounter_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPcieUtilCounter_enum::NVML_PCIE_UTIL_TX_BYTES => {
                writer.write_all(stringify!(NVML_PCIE_UTIL_TX_BYTES).as_bytes())
            }
            &cuda_types::nvml::nvmlPcieUtilCounter_enum::NVML_PCIE_UTIL_RX_BYTES => {
                writer.write_all(stringify!(NVML_PCIE_UTIL_RX_BYTES).as_bytes())
            }
            &cuda_types::nvml::nvmlPcieUtilCounter_enum::NVML_PCIE_UTIL_COUNT => {
                writer.write_all(stringify!(NVML_PCIE_UTIL_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlValueType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_DOUBLE => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_DOUBLE).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_UNSIGNED_INT => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_UNSIGNED_INT).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_UNSIGNED_LONG => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_UNSIGNED_LONG).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_UNSIGNED_LONG_LONG => {
                writer
                    .write_all(stringify!(NVML_VALUE_TYPE_UNSIGNED_LONG_LONG).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_SIGNED_LONG_LONG => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_SIGNED_LONG_LONG).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_SIGNED_INT => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_SIGNED_INT).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_UNSIGNED_SHORT => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_UNSIGNED_SHORT).as_bytes())
            }
            &cuda_types::nvml::nvmlValueType_enum::NVML_VALUE_TYPE_COUNT => {
                writer.write_all(stringify!(NVML_VALUE_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPerfPolicyType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_POWER => {
                writer.write_all(stringify!(NVML_PERF_POLICY_POWER).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_THERMAL => {
                writer.write_all(stringify!(NVML_PERF_POLICY_THERMAL).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_SYNC_BOOST => {
                writer.write_all(stringify!(NVML_PERF_POLICY_SYNC_BOOST).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_BOARD_LIMIT => {
                writer.write_all(stringify!(NVML_PERF_POLICY_BOARD_LIMIT).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_LOW_UTILIZATION => {
                writer.write_all(stringify!(NVML_PERF_POLICY_LOW_UTILIZATION).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_RELIABILITY => {
                writer.write_all(stringify!(NVML_PERF_POLICY_RELIABILITY).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_TOTAL_APP_CLOCKS => {
                writer
                    .write_all(stringify!(NVML_PERF_POLICY_TOTAL_APP_CLOCKS).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_TOTAL_BASE_CLOCKS => {
                writer
                    .write_all(stringify!(NVML_PERF_POLICY_TOTAL_BASE_CLOCKS).as_bytes())
            }
            &cuda_types::nvml::nvmlPerfPolicyType_enum::NVML_PERF_POLICY_COUNT => {
                writer.write_all(stringify!(NVML_PERF_POLICY_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlViolationTime_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(referenceTime), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.referenceTime, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(violationTime), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.violationTime, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlThermalTarget_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_NONE => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_GPU => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_MEMORY => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_MEMORY).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_POWER_SUPPLY => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_POWER_SUPPLY).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_BOARD => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_BOARD).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_VCD_BOARD => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_VCD_BOARD).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_VCD_INLET => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_VCD_INLET).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_VCD_OUTLET => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_VCD_OUTLET).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_ALL => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_ALL).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalTarget_t::NVML_THERMAL_TARGET_UNKNOWN => {
                writer.write_all(stringify!(NVML_THERMAL_TARGET_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlThermalController_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_NONE => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_GPU_INTERNAL => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_CONTROLLER_GPU_INTERNAL).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_ADM1032 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_ADM1032).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_ADT7461 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_ADT7461).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_MAX6649 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_MAX6649).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_MAX1617 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_MAX1617).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_LM99 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_LM99).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_LM89 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_LM89).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_LM64 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_LM64).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_G781 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_G781).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_ADT7473 => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_ADT7473).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_SBMAX6649 => {
                writer
                    .write_all(stringify!(NVML_THERMAL_CONTROLLER_SBMAX6649).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_VBIOSEVT => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_VBIOSEVT).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_OS => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_OS).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_NVSYSCON_CANOAS => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_CONTROLLER_NVSYSCON_CANOAS).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_NVSYSCON_E551 => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_CONTROLLER_NVSYSCON_E551).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_MAX6649R => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_MAX6649R).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_ADT7473S => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_ADT7473S).as_bytes())
            }
            &cuda_types::nvml::nvmlThermalController_t::NVML_THERMAL_CONTROLLER_UNKNOWN => {
                writer.write_all(stringify!(NVML_THERMAL_CONTROLLER_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuThermalSettings_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sensor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sensor, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuThermalSettings_t__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(controller), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.controller, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(defaultMinTemp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.defaultMinTemp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(defaultMaxTemp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.defaultMaxTemp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(currentTemp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.currentTemp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(target), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.target, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlCoolerControl_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlCoolerControl_enum::NVML_THERMAL_COOLER_SIGNAL_NONE => {
                writer.write_all(stringify!(NVML_THERMAL_COOLER_SIGNAL_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlCoolerControl_enum::NVML_THERMAL_COOLER_SIGNAL_TOGGLE => {
                writer
                    .write_all(stringify!(NVML_THERMAL_COOLER_SIGNAL_TOGGLE).as_bytes())
            }
            &cuda_types::nvml::nvmlCoolerControl_enum::NVML_THERMAL_COOLER_SIGNAL_VARIABLE => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_COOLER_SIGNAL_VARIABLE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlCoolerControl_enum::NVML_THERMAL_COOLER_SIGNAL_COUNT => {
                writer.write_all(stringify!(NVML_THERMAL_COOLER_SIGNAL_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlCoolerTarget_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlCoolerTarget_enum::NVML_THERMAL_COOLER_TARGET_NONE => {
                writer.write_all(stringify!(NVML_THERMAL_COOLER_TARGET_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlCoolerTarget_enum::NVML_THERMAL_COOLER_TARGET_GPU => {
                writer.write_all(stringify!(NVML_THERMAL_COOLER_TARGET_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlCoolerTarget_enum::NVML_THERMAL_COOLER_TARGET_MEMORY => {
                writer
                    .write_all(stringify!(NVML_THERMAL_COOLER_TARGET_MEMORY).as_bytes())
            }
            &cuda_types::nvml::nvmlCoolerTarget_enum::NVML_THERMAL_COOLER_TARGET_POWER_SUPPLY => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_COOLER_TARGET_POWER_SUPPLY).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlCoolerTarget_enum::NVML_THERMAL_COOLER_TARGET_GPU_RELATED => {
                writer
                    .write_all(
                        stringify!(NVML_THERMAL_COOLER_TARGET_GPU_RELATED).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlCoolerInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(index), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.index, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(signalType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.signalType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(target), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.target, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUUIDType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlUUIDType_t::NVML_UUID_TYPE_NONE => {
                writer.write_all(stringify!(NVML_UUID_TYPE_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlUUIDType_t::NVML_UUID_TYPE_ASCII => {
                writer.write_all(stringify!(NVML_UUID_TYPE_ASCII).as_bytes())
            }
            &cuda_types::nvml::nvmlUUIDType_t::NVML_UUID_TYPE_BINARY => {
                writer.write_all(stringify!(NVML_UUID_TYPE_BINARY).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEnableState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlEnableState_enum::NVML_FEATURE_DISABLED => {
                writer.write_all(stringify!(NVML_FEATURE_DISABLED).as_bytes())
            }
            &cuda_types::nvml::nvmlEnableState_enum::NVML_FEATURE_ENABLED => {
                writer.write_all(stringify!(NVML_FEATURE_ENABLED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDramEncryptionInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encryptionState), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encryptionState, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlBrandType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_UNKNOWN => {
                writer.write_all(stringify!(NVML_BRAND_UNKNOWN).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_QUADRO => {
                writer.write_all(stringify!(NVML_BRAND_QUADRO).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_TESLA => {
                writer.write_all(stringify!(NVML_BRAND_TESLA).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVS => {
                writer.write_all(stringify!(NVML_BRAND_NVS).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_GRID => {
                writer.write_all(stringify!(NVML_BRAND_GRID).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_GEFORCE => {
                writer.write_all(stringify!(NVML_BRAND_GEFORCE).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_TITAN => {
                writer.write_all(stringify!(NVML_BRAND_TITAN).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_VAPPS => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_VAPPS).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_VPC => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_VPC).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_VCS => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_VCS).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_VWS => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_VWS).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_CLOUD_GAMING => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_CLOUD_GAMING).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_VGAMING => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_VGAMING).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_QUADRO_RTX => {
                writer.write_all(stringify!(NVML_BRAND_QUADRO_RTX).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA_RTX => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA_RTX).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_NVIDIA => {
                writer.write_all(stringify!(NVML_BRAND_NVIDIA).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_GEFORCE_RTX => {
                writer.write_all(stringify!(NVML_BRAND_GEFORCE_RTX).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_TITAN_RTX => {
                writer.write_all(stringify!(NVML_BRAND_TITAN_RTX).as_bytes())
            }
            &cuda_types::nvml::nvmlBrandType_enum::NVML_BRAND_COUNT => {
                writer.write_all(stringify!(NVML_BRAND_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlTemperatureThresholds_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_SHUTDOWN => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_SHUTDOWN).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_SLOWDOWN => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_SLOWDOWN).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_MEM_MAX => {
                writer
                    .write_all(stringify!(NVML_TEMPERATURE_THRESHOLD_MEM_MAX).as_bytes())
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_GPU_MAX => {
                writer
                    .write_all(stringify!(NVML_TEMPERATURE_THRESHOLD_GPU_MAX).as_bytes())
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MIN => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MIN).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_CURR => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_CURR).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MAX => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MAX).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_GPS_CURR => {
                writer
                    .write_all(
                        stringify!(NVML_TEMPERATURE_THRESHOLD_GPS_CURR).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlTemperatureThresholds_enum::NVML_TEMPERATURE_THRESHOLD_COUNT => {
                writer.write_all(stringify!(NVML_TEMPERATURE_THRESHOLD_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlTemperatureSensors_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlTemperatureSensors_enum::NVML_TEMPERATURE_GPU => {
                writer.write_all(stringify!(NVML_TEMPERATURE_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlTemperatureSensors_enum::NVML_TEMPERATURE_COUNT => {
                writer.write_all(stringify!(NVML_TEMPERATURE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMarginTemperature_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(marginTemperature), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.marginTemperature, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlComputeMode_enum::NVML_COMPUTEMODE_DEFAULT => {
                writer.write_all(stringify!(NVML_COMPUTEMODE_DEFAULT).as_bytes())
            }
            &cuda_types::nvml::nvmlComputeMode_enum::NVML_COMPUTEMODE_EXCLUSIVE_THREAD => {
                writer
                    .write_all(stringify!(NVML_COMPUTEMODE_EXCLUSIVE_THREAD).as_bytes())
            }
            &cuda_types::nvml::nvmlComputeMode_enum::NVML_COMPUTEMODE_PROHIBITED => {
                writer.write_all(stringify!(NVML_COMPUTEMODE_PROHIBITED).as_bytes())
            }
            &cuda_types::nvml::nvmlComputeMode_enum::NVML_COMPUTEMODE_EXCLUSIVE_PROCESS => {
                writer
                    .write_all(stringify!(NVML_COMPUTEMODE_EXCLUSIVE_PROCESS).as_bytes())
            }
            &cuda_types::nvml::nvmlComputeMode_enum::NVML_COMPUTEMODE_COUNT => {
                writer.write_all(stringify!(NVML_COMPUTEMODE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClkMonFaultInfo_struct {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(clkApiDomain), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clkApiDomain, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(clkDomainFaultMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clkDomainFaultMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClkMonStatus_status {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(bGlobalStatus), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bGlobalStatus, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(clkMonListSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clkMonListSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(clkMonList), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clkMonList, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMemoryErrorType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlMemoryErrorType_enum::NVML_MEMORY_ERROR_TYPE_CORRECTED => {
                writer.write_all(stringify!(NVML_MEMORY_ERROR_TYPE_CORRECTED).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryErrorType_enum::NVML_MEMORY_ERROR_TYPE_UNCORRECTED => {
                writer
                    .write_all(stringify!(NVML_MEMORY_ERROR_TYPE_UNCORRECTED).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryErrorType_enum::NVML_MEMORY_ERROR_TYPE_COUNT => {
                writer.write_all(stringify!(NVML_MEMORY_ERROR_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvlinkVersion_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_INVALID => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_INVALID).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_1_0 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_1_0).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_2_0 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_2_0).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_2_2 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_2_2).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_3_0 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_3_0).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_3_1 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_3_1).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_4_0 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_4_0).as_bytes())
            }
            &cuda_types::nvml::nvmlNvlinkVersion_enum::NVML_NVLINK_VERSION_5_0 => {
                writer.write_all(stringify!(NVML_NVLINK_VERSION_5_0).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEccCounterType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlEccCounterType_enum::NVML_VOLATILE_ECC => {
                writer.write_all(stringify!(NVML_VOLATILE_ECC).as_bytes())
            }
            &cuda_types::nvml::nvmlEccCounterType_enum::NVML_AGGREGATE_ECC => {
                writer.write_all(stringify!(NVML_AGGREGATE_ECC).as_bytes())
            }
            &cuda_types::nvml::nvmlEccCounterType_enum::NVML_ECC_COUNTER_TYPE_COUNT => {
                writer.write_all(stringify!(NVML_ECC_COUNTER_TYPE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClockType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlClockType_enum::NVML_CLOCK_GRAPHICS => {
                writer.write_all(stringify!(NVML_CLOCK_GRAPHICS).as_bytes())
            }
            &cuda_types::nvml::nvmlClockType_enum::NVML_CLOCK_SM => {
                writer.write_all(stringify!(NVML_CLOCK_SM).as_bytes())
            }
            &cuda_types::nvml::nvmlClockType_enum::NVML_CLOCK_MEM => {
                writer.write_all(stringify!(NVML_CLOCK_MEM).as_bytes())
            }
            &cuda_types::nvml::nvmlClockType_enum::NVML_CLOCK_VIDEO => {
                writer.write_all(stringify!(NVML_CLOCK_VIDEO).as_bytes())
            }
            &cuda_types::nvml::nvmlClockType_enum::NVML_CLOCK_COUNT => {
                writer.write_all(stringify!(NVML_CLOCK_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClockId_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlClockId_enum::NVML_CLOCK_ID_CURRENT => {
                writer.write_all(stringify!(NVML_CLOCK_ID_CURRENT).as_bytes())
            }
            &cuda_types::nvml::nvmlClockId_enum::NVML_CLOCK_ID_APP_CLOCK_TARGET => {
                writer.write_all(stringify!(NVML_CLOCK_ID_APP_CLOCK_TARGET).as_bytes())
            }
            &cuda_types::nvml::nvmlClockId_enum::NVML_CLOCK_ID_APP_CLOCK_DEFAULT => {
                writer.write_all(stringify!(NVML_CLOCK_ID_APP_CLOCK_DEFAULT).as_bytes())
            }
            &cuda_types::nvml::nvmlClockId_enum::NVML_CLOCK_ID_CUSTOMER_BOOST_MAX => {
                writer.write_all(stringify!(NVML_CLOCK_ID_CUSTOMER_BOOST_MAX).as_bytes())
            }
            &cuda_types::nvml::nvmlClockId_enum::NVML_CLOCK_ID_COUNT => {
                writer.write_all(stringify!(NVML_CLOCK_ID_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDriverModel_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlDriverModel_enum::NVML_DRIVER_WDDM => {
                writer.write_all(stringify!(NVML_DRIVER_WDDM).as_bytes())
            }
            &cuda_types::nvml::nvmlDriverModel_enum::NVML_DRIVER_WDM => {
                writer.write_all(stringify!(NVML_DRIVER_WDM).as_bytes())
            }
            &cuda_types::nvml::nvmlDriverModel_enum::NVML_DRIVER_MCDM => {
                writer.write_all(stringify!(NVML_DRIVER_MCDM).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPStates_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_0 => {
                writer.write_all(stringify!(NVML_PSTATE_0).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_1 => {
                writer.write_all(stringify!(NVML_PSTATE_1).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_2 => {
                writer.write_all(stringify!(NVML_PSTATE_2).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_3 => {
                writer.write_all(stringify!(NVML_PSTATE_3).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_4 => {
                writer.write_all(stringify!(NVML_PSTATE_4).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_5 => {
                writer.write_all(stringify!(NVML_PSTATE_5).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_6 => {
                writer.write_all(stringify!(NVML_PSTATE_6).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_7 => {
                writer.write_all(stringify!(NVML_PSTATE_7).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_8 => {
                writer.write_all(stringify!(NVML_PSTATE_8).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_9 => {
                writer.write_all(stringify!(NVML_PSTATE_9).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_10 => {
                writer.write_all(stringify!(NVML_PSTATE_10).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_11 => {
                writer.write_all(stringify!(NVML_PSTATE_11).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_12 => {
                writer.write_all(stringify!(NVML_PSTATE_12).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_13 => {
                writer.write_all(stringify!(NVML_PSTATE_13).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_14 => {
                writer.write_all(stringify!(NVML_PSTATE_14).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_15 => {
                writer.write_all(stringify!(NVML_PSTATE_15).as_bytes())
            }
            &cuda_types::nvml::nvmlPStates_enum::NVML_PSTATE_UNKNOWN => {
                writer.write_all(stringify!(NVML_PSTATE_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClockOffset_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(type_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.type_, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pstate), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pstate, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(clockOffsetMHz), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clockOffsetMHz, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(minClockOffsetMHz), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minClockOffsetMHz, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxClockOffsetMHz), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxClockOffsetMHz, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlFanSpeedInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(fan), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fan, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(speed), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.speed, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDevicePerfModes_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(str_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.str_, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDeviceCurrentClockFreqs_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(str_), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.str_, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGom_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGom_enum::NVML_GOM_ALL_ON => {
                writer.write_all(stringify!(NVML_GOM_ALL_ON).as_bytes())
            }
            &cuda_types::nvml::nvmlGom_enum::NVML_GOM_COMPUTE => {
                writer.write_all(stringify!(NVML_GOM_COMPUTE).as_bytes())
            }
            &cuda_types::nvml::nvmlGom_enum::NVML_GOM_LOW_DP => {
                writer.write_all(stringify!(NVML_GOM_LOW_DP).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlInforomObject_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlInforomObject_enum::NVML_INFOROM_OEM => {
                writer.write_all(stringify!(NVML_INFOROM_OEM).as_bytes())
            }
            &cuda_types::nvml::nvmlInforomObject_enum::NVML_INFOROM_ECC => {
                writer.write_all(stringify!(NVML_INFOROM_ECC).as_bytes())
            }
            &cuda_types::nvml::nvmlInforomObject_enum::NVML_INFOROM_POWER => {
                writer.write_all(stringify!(NVML_INFOROM_POWER).as_bytes())
            }
            &cuda_types::nvml::nvmlInforomObject_enum::NVML_INFOROM_DEN => {
                writer.write_all(stringify!(NVML_INFOROM_DEN).as_bytes())
            }
            &cuda_types::nvml::nvmlInforomObject_enum::NVML_INFOROM_COUNT => {
                writer.write_all(stringify!(NVML_INFOROM_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMemoryLocation_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_L1_CACHE => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_L1_CACHE).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_L2_CACHE => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_L2_CACHE).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_DRAM => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_DRAM).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_DEVICE_MEMORY => {
                writer
                    .write_all(stringify!(NVML_MEMORY_LOCATION_DEVICE_MEMORY).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_REGISTER_FILE => {
                writer
                    .write_all(stringify!(NVML_MEMORY_LOCATION_REGISTER_FILE).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_TEXTURE_MEMORY => {
                writer
                    .write_all(
                        stringify!(NVML_MEMORY_LOCATION_TEXTURE_MEMORY).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_TEXTURE_SHM => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_TEXTURE_SHM).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_CBU => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_CBU).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_SRAM => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_SRAM).as_bytes())
            }
            &cuda_types::nvml::nvmlMemoryLocation_enum::NVML_MEMORY_LOCATION_COUNT => {
                writer.write_all(stringify!(NVML_MEMORY_LOCATION_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPageRetirementCause_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPageRetirementCause_enum::NVML_PAGE_RETIREMENT_CAUSE_MULTIPLE_SINGLE_BIT_ECC_ERRORS => {
                writer
                    .write_all(
                        stringify!(
                            NVML_PAGE_RETIREMENT_CAUSE_MULTIPLE_SINGLE_BIT_ECC_ERRORS
                        )
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlPageRetirementCause_enum::NVML_PAGE_RETIREMENT_CAUSE_DOUBLE_BIT_ECC_ERROR => {
                writer
                    .write_all(
                        stringify!(NVML_PAGE_RETIREMENT_CAUSE_DOUBLE_BIT_ECC_ERROR)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlPageRetirementCause_enum::NVML_PAGE_RETIREMENT_CAUSE_COUNT => {
                writer.write_all(stringify!(NVML_PAGE_RETIREMENT_CAUSE_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlRestrictedAPI_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlRestrictedAPI_enum::NVML_RESTRICTED_API_SET_APPLICATION_CLOCKS => {
                writer
                    .write_all(
                        stringify!(NVML_RESTRICTED_API_SET_APPLICATION_CLOCKS).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlRestrictedAPI_enum::NVML_RESTRICTED_API_SET_AUTO_BOOSTED_CLOCKS => {
                writer
                    .write_all(
                        stringify!(NVML_RESTRICTED_API_SET_AUTO_BOOSTED_CLOCKS)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlRestrictedAPI_enum::NVML_RESTRICTED_API_COUNT => {
                writer.write_all(stringify!(NVML_RESTRICTED_API_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessUtilizationSample_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.smUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decUtil, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.smUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(jpgUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.jpgUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ofaUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ofaUtil, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlProcessesUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(processSamplesCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.processSamplesCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lastSeenTimeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(procUtilArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.procUtilArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEccSramErrorStatus_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(aggregateUncParity), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.aggregateUncParity, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(aggregateUncSecDed), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.aggregateUncSecDed, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(aggregateCor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.aggregateCor, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(volatileUncParity), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.volatileUncParity, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(volatileUncSecDed), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.volatileUncSecDed, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(volatileCor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.volatileCor, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(aggregateUncBucketL2), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.aggregateUncBucketL2, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(aggregateUncBucketSm), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.aggregateUncBucketSm, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(aggregateUncBucketPcie), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.aggregateUncBucketPcie, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(aggregateUncBucketMcu), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.aggregateUncBucketMcu, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(aggregateUncBucketOther), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.aggregateUncBucketOther, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(bThresholdExceeded), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bThresholdExceeded, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPlatformInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ibGuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ibGuid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(rackGuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.rackGuid, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(chassisPhysicalSlotNumber), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.chassisPhysicalSlotNumber, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(computeSlotIndex), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.computeSlotIndex, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nodeIndex), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nodeIndex, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(peerType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.peerType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(moduleId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.moduleId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPlatformInfo_v2_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ibGuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ibGuid, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(chassisSerialNumber), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.chassisSerialNumber, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(slotNumber), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.slotNumber, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(trayIndex), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.trayIndex, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hostId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hostId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(peerType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.peerType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(moduleId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.moduleId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuUtilizationDomainId_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpuUtilizationDomainId_t::NVML_GPU_UTILIZATION_DOMAIN_GPU => {
                writer.write_all(stringify!(NVML_GPU_UTILIZATION_DOMAIN_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuUtilizationDomainId_t::NVML_GPU_UTILIZATION_DOMAIN_FB => {
                writer.write_all(stringify!(NVML_GPU_UTILIZATION_DOMAIN_FB).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuUtilizationDomainId_t::NVML_GPU_UTILIZATION_DOMAIN_VID => {
                writer.write_all(stringify!(NVML_GPU_UTILIZATION_DOMAIN_VID).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuUtilizationDomainId_t::NVML_GPU_UTILIZATION_DOMAIN_BUS => {
                writer.write_all(stringify!(NVML_GPU_UTILIZATION_DOMAIN_BUS).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuDynamicPstatesInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(flags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.flags, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(utilization), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.utilization, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlGpuDynamicPstatesInfo_st__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(bIsPresent), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bIsPresent, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(percentage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.percentage, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(incThreshold), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.incThreshold, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decThreshold), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decThreshold, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPowerValue_v2_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(powerScope), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.powerScope, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(powerValueMw), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.powerValueMw, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuVirtualizationMode {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpuVirtualizationMode::NVML_GPU_VIRTUALIZATION_MODE_NONE => {
                writer
                    .write_all(stringify!(NVML_GPU_VIRTUALIZATION_MODE_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuVirtualizationMode::NVML_GPU_VIRTUALIZATION_MODE_PASSTHROUGH => {
                writer
                    .write_all(
                        stringify!(NVML_GPU_VIRTUALIZATION_MODE_PASSTHROUGH).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpuVirtualizationMode::NVML_GPU_VIRTUALIZATION_MODE_VGPU => {
                writer
                    .write_all(stringify!(NVML_GPU_VIRTUALIZATION_MODE_VGPU).as_bytes())
            }
            &cuda_types::nvml::nvmlGpuVirtualizationMode::NVML_GPU_VIRTUALIZATION_MODE_HOST_VGPU => {
                writer
                    .write_all(
                        stringify!(NVML_GPU_VIRTUALIZATION_MODE_HOST_VGPU).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpuVirtualizationMode::NVML_GPU_VIRTUALIZATION_MODE_HOST_VSGA => {
                writer
                    .write_all(
                        stringify!(NVML_GPU_VIRTUALIZATION_MODE_HOST_VSGA).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlHostVgpuMode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlHostVgpuMode_enum::NVML_HOST_VGPU_MODE_NON_SRIOV => {
                writer.write_all(stringify!(NVML_HOST_VGPU_MODE_NON_SRIOV).as_bytes())
            }
            &cuda_types::nvml::nvmlHostVgpuMode_enum::NVML_HOST_VGPU_MODE_SRIOV => {
                writer.write_all(stringify!(NVML_HOST_VGPU_MODE_SRIOV).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuVmIdType {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuVmIdType::NVML_VGPU_VM_ID_DOMAIN_ID => {
                writer.write_all(stringify!(NVML_VGPU_VM_ID_DOMAIN_ID).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuVmIdType::NVML_VGPU_VM_ID_UUID => {
                writer.write_all(stringify!(NVML_VGPU_VM_ID_UUID).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuGuestInfoState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuGuestInfoState_enum::NVML_VGPU_INSTANCE_GUEST_INFO_STATE_UNINITIALIZED => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_INSTANCE_GUEST_INFO_STATE_UNINITIALIZED)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlVgpuGuestInfoState_enum::NVML_VGPU_INSTANCE_GUEST_INFO_STATE_INITIALIZED => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_INSTANCE_GUEST_INFO_STATE_INITIALIZED)
                            .as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGridLicenseFeatureCode_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_UNKNOWN => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_UNKNOWN).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_VGPU => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_VGPU).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_NVIDIA_RTX => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_NVIDIA_RTX).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_VWORKSTATION => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_VWORKSTATION)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_GAMING => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_GAMING).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGridLicenseFeatureCode_t::NVML_GRID_LICENSE_FEATURE_CODE_COMPUTE => {
                writer
                    .write_all(
                        stringify!(NVML_GRID_LICENSE_FEATURE_CODE_COMPUTE).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuCapability_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_NVLINK_P2P => {
                writer.write_all(stringify!(NVML_VGPU_CAP_NVLINK_P2P).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_GPUDIRECT => {
                writer.write_all(stringify!(NVML_VGPU_CAP_GPUDIRECT).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_MULTI_VGPU_EXCLUSIVE => {
                writer
                    .write_all(stringify!(NVML_VGPU_CAP_MULTI_VGPU_EXCLUSIVE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_EXCLUSIVE_TYPE => {
                writer.write_all(stringify!(NVML_VGPU_CAP_EXCLUSIVE_TYPE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_EXCLUSIVE_SIZE => {
                writer.write_all(stringify!(NVML_VGPU_CAP_EXCLUSIVE_SIZE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuCapability_enum::NVML_VGPU_CAP_COUNT => {
                writer.write_all(stringify!(NVML_VGPU_CAP_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuDriverCapability_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuDriverCapability_enum::NVML_VGPU_DRIVER_CAP_HETEROGENEOUS_MULTI_VGPU => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_DRIVER_CAP_HETEROGENEOUS_MULTI_VGPU)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlVgpuDriverCapability_enum::NVML_VGPU_DRIVER_CAP_WARM_UPDATE => {
                writer.write_all(stringify!(NVML_VGPU_DRIVER_CAP_WARM_UPDATE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuDriverCapability_enum::NVML_VGPU_DRIVER_CAP_COUNT => {
                writer.write_all(stringify!(NVML_VGPU_DRIVER_CAP_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDeviceVgpuCapability_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_FRACTIONAL_MULTI_VGPU => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_FRACTIONAL_MULTI_VGPU).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_PROFILES => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_PROFILES)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_SIZES => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_SIZES)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_READ_DEVICE_BUFFER_BW => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_READ_DEVICE_BUFFER_BW).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_WRITE_DEVICE_BUFFER_BW => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_WRITE_DEVICE_BUFFER_BW)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_DEVICE_STREAMING => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_DEVICE_STREAMING).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_MINI_QUARTER_GPU => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_MINI_QUARTER_GPU).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_COMPUTE_MEDIA_ENGINE_GPU => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_COMPUTE_MEDIA_ENGINE_GPU)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_WARM_UPDATE => {
                writer.write_all(stringify!(NVML_DEVICE_VGPU_CAP_WARM_UPDATE).as_bytes())
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_HOMOGENEOUS_PLACEMENTS => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_HOMOGENEOUS_PLACEMENTS)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_SUPPORTED => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_SUPPORTED)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_ENABLED => {
                writer
                    .write_all(
                        stringify!(NVML_DEVICE_VGPU_CAP_MIG_TIMESLICING_ENABLED)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceVgpuCapability_enum::NVML_DEVICE_VGPU_CAP_COUNT => {
                writer.write_all(stringify!(NVML_DEVICE_VGPU_CAP_COUNT).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuHeterogeneousMode_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPlacementId_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPlacementList_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementIds), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementIds, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPlacementList_v2_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementIds), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementIds, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(mode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuTypeBar1Info_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bar1Size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bar1Size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuInstancesUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sampleValType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sampleValType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstanceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lastSeenTimeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuUtilArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuUtilArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuProcessUtilizationSample_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(processName), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.processName, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.smUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decUtil, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuProcessUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(processName), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.processName, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(smUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.smUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(jpgUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.jpgUtil, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ofaUtil), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ofaUtil, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuProcessesUtilizationInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuProcessCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuProcessCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lastSeenTimeStamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuProcUtilArray), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuProcUtilArray, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuRuntimeState_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerParams_t__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(avgFactor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.avgFactor, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeslice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeslice, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerParams_t__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeslice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeslice, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerLogEntries_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timestamp), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timestamp, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeRunTotal), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeRunTotal, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeRun), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeRun, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(swRunlistId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.swRunlistId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(targetTimeSlice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.targetTimeSlice, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(cumulativePreemptionTime), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.cumulativePreemptionTime, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlVgpuSchedulerSetParams_t__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(avgFactor), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.avgFactor, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(frequency), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.frequency, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlVgpuSchedulerSetParams_t__bindgen_ty_2 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(timeslice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeslice, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuSchedulerCapabilities_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(supportedSchedulers), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.supportedSchedulers, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxTimeslice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxTimeslice, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(minTimeslice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minTimeslice, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(isArrModeSupported), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isArrModeSupported, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(maxFrequencyForARR), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxFrequencyForARR, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(minFrequencyForARR), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minFrequencyForARR, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(maxAvgFactorForARR), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxAvgFactorForARR, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(minAvgFactorForARR), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minAvgFactorForARR, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuLicenseExpiry_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(year), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.year, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(month), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.month, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(day), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.day, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hour), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hour, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(min), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.min, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sec), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sec, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuLicenseInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(isLicensed), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isLicensed, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(licenseExpiry), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.licenseExpiry, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(currentState), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.currentState, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGridLicenseExpiry_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(year), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.year, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(month), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.month, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(day), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.day, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hour), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hour, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(min), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.min, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sec), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sec, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGridLicensableFeature_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(featureCode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.featureCode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(featureState), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.featureState, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(licenseInfo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.licenseInfo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(productName), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.productName, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(featureEnabled), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.featureEnabled, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(licenseExpiry), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.licenseExpiry, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGridLicensableFeatures_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(
                concat!("{ ", stringify!(isGridLicenseSupported), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.isGridLicenseSupported, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(licensableFeaturesCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.licensableFeaturesCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(gridLicensableFeatures), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.gridLicensableFeatures, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s::NVML_GPU_RECOVERY_ACTION_NONE => {
                writer.write_all(stringify!(NVML_GPU_RECOVERY_ACTION_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s::NVML_GPU_RECOVERY_ACTION_GPU_RESET => {
                writer
                    .write_all(stringify!(NVML_GPU_RECOVERY_ACTION_GPU_RESET).as_bytes())
            }
            &cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s::NVML_GPU_RECOVERY_ACTION_NODE_REBOOT => {
                writer
                    .write_all(
                        stringify!(NVML_GPU_RECOVERY_ACTION_NODE_REBOOT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s::NVML_GPU_RECOVERY_ACTION_DRAIN_P2P => {
                writer
                    .write_all(stringify!(NVML_GPU_RECOVERY_ACTION_DRAIN_P2P).as_bytes())
            }
            &cuda_types::nvml::nvmlDeviceGpuRecoveryAction_s::NVML_GPU_RECOVERY_ACTION_DRAIN_AND_RESET => {
                writer
                    .write_all(
                        stringify!(NVML_GPU_RECOVERY_ACTION_DRAIN_AND_RESET).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuTypeIdInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuTypeIds), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuTypeIds, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuTypeMaxInstance_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuTypeId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuTypeId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxInstancePerGI), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxInstancePerGI, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlActiveVgpuInstanceInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstances), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstances, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuCreatablePlacementInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuTypeId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuTypeId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementIds), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementIds, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placementSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placementSize, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvLinkPowerThres_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(lowPwrThreshold), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.lowPwrThreshold, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUnit_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlHwbcEntry_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(hwbcId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hwbcId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(firmwareVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.firmwareVersion, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlFanState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlFanState_enum::NVML_FAN_NORMAL => {
                writer.write_all(stringify!(NVML_FAN_NORMAL).as_bytes())
            }
            &cuda_types::nvml::nvmlFanState_enum::NVML_FAN_FAILED => {
                writer.write_all(stringify!(NVML_FAN_FAILED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlLedColor_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlLedColor_enum::NVML_LED_COLOR_GREEN => {
                writer.write_all(stringify!(NVML_LED_COLOR_GREEN).as_bytes())
            }
            &cuda_types::nvml::nvmlLedColor_enum::NVML_LED_COLOR_AMBER => {
                writer.write_all(stringify!(NVML_LED_COLOR_AMBER).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlLedState_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(cause), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cause, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(color), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.color, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUnitInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(serial), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.serial, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(firmwareVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.firmwareVersion, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPSUInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(current), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.current, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(voltage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.voltage, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(power), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.power, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUnitFanInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(speed), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.speed, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlUnitFanSpeeds_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(fans), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.fans, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(count), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.count, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEventSet_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEventData_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(device), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.device, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(eventType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.eventType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(eventData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.eventData, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpuInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuInstanceId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(computeInstanceId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.computeInstanceId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemEventSet_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemEventSetCreateRequest_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(set), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.set, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemEventSetFreeRequest_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(set), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.set, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemRegisterEventRequest_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(eventTypes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.eventTypes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(set), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.set, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemEventData_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(eventType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.eventType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpuId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuId, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemEventSetWaitRequest_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(timeoutms), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.timeoutms, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(set), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.set, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(data), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.data, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(dataSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.dataSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numEvent), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numEvent, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlAccountingStats_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(gpuUtilization), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuUtilization, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memoryUtilization), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memoryUtilization, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxMemoryUsage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxMemoryUsage, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(time), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.time, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(startTime), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.startTime, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(isRunning), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isRunning, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEncoderQueryType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlEncoderQueryType_enum::NVML_ENCODER_QUERY_H264 => {
                writer.write_all(stringify!(NVML_ENCODER_QUERY_H264).as_bytes())
            }
            &cuda_types::nvml::nvmlEncoderQueryType_enum::NVML_ENCODER_QUERY_HEVC => {
                writer.write_all(stringify!(NVML_ENCODER_QUERY_HEVC).as_bytes())
            }
            &cuda_types::nvml::nvmlEncoderQueryType_enum::NVML_ENCODER_QUERY_AV1 => {
                writer.write_all(stringify!(NVML_ENCODER_QUERY_AV1).as_bytes())
            }
            &cuda_types::nvml::nvmlEncoderQueryType_enum::NVML_ENCODER_QUERY_UNKNOWN => {
                writer.write_all(stringify!(NVML_ENCODER_QUERY_UNKNOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlEncoderSessionInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(sessionId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sessionId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(codecType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.codecType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageFps), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageFps, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageLatency), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageLatency, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlFBCSessionType_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlFBCSessionType_enum::NVML_FBC_SESSION_TYPE_UNKNOWN => {
                writer.write_all(stringify!(NVML_FBC_SESSION_TYPE_UNKNOWN).as_bytes())
            }
            &cuda_types::nvml::nvmlFBCSessionType_enum::NVML_FBC_SESSION_TYPE_TOSYS => {
                writer.write_all(stringify!(NVML_FBC_SESSION_TYPE_TOSYS).as_bytes())
            }
            &cuda_types::nvml::nvmlFBCSessionType_enum::NVML_FBC_SESSION_TYPE_CUDA => {
                writer.write_all(stringify!(NVML_FBC_SESSION_TYPE_CUDA).as_bytes())
            }
            &cuda_types::nvml::nvmlFBCSessionType_enum::NVML_FBC_SESSION_TYPE_VID => {
                writer.write_all(stringify!(NVML_FBC_SESSION_TYPE_VID).as_bytes())
            }
            &cuda_types::nvml::nvmlFBCSessionType_enum::NVML_FBC_SESSION_TYPE_HWENC => {
                writer.write_all(stringify!(NVML_FBC_SESSION_TYPE_HWENC).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlFBCStats_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(sessionsCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sessionsCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageFPS), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageFPS, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageLatency), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageLatency, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlFBCSessionInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(sessionId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sessionId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(pid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vgpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(displayOrdinal), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.displayOrdinal, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sessionType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sessionType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sessionFlags), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sessionFlags, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hMaxResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hMaxResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vMaxResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vMaxResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(vResolution), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vResolution, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageFPS), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageFPS, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(averageLatency), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.averageLatency, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDetachGpuState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlDetachGpuState_enum::NVML_DETACH_GPU_KEEP => {
                writer.write_all(stringify!(NVML_DETACH_GPU_KEEP).as_bytes())
            }
            &cuda_types::nvml::nvmlDetachGpuState_enum::NVML_DETACH_GPU_REMOVE => {
                writer.write_all(stringify!(NVML_DETACH_GPU_REMOVE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPcieLinkState_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPcieLinkState_enum::NVML_PCIE_LINK_KEEP => {
                writer.write_all(stringify!(NVML_PCIE_LINK_KEEP).as_bytes())
            }
            &cuda_types::nvml::nvmlPcieLinkState_enum::NVML_PCIE_LINK_SHUT_DOWN => {
                writer.write_all(stringify!(NVML_PCIE_LINK_SHUT_DOWN).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlConfComputeSystemCaps_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(cpuCaps), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cpuCaps, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpusCaps), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpusCaps, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlConfComputeSystemState_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(environment), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.environment, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ccFeature), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ccFeature, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(devToolsMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.devToolsMode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemConfComputeSettings_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(environment), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.environment, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ccFeature), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ccFeature, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(devToolsMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.devToolsMode, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(multiGpuMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiGpuMode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlConfComputeMemSizeInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(protectedMemSizeKib), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.protectedMemSizeKib, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(unprotectedMemSizeKib), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.unprotectedMemSizeKib, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlConfComputeGpuCertificate_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(certChainSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.certChainSize, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(attestationCertChainSize), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.attestationCertChainSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(certChain), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.certChain, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(attestationCertChain), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.attestationCertChain, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlConfComputeGpuAttestationReport_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(
                concat!("{ ", stringify!(isCecAttestationReportPresent), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.isCecAttestationReportPresent, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(attestationReportSize), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.attestationReportSize, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(cecAttestationReportSize), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.cecAttestationReportSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nonce), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nonce, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(attestationReport), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.attestationReport, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(cecAttestationReport), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.cecAttestationReport, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlConfComputeSetKeyRotationThresholdInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(maxAttackerAdvantage), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.maxAttackerAdvantage, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlConfComputeGetKeyRotationThresholdInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(attackerAdvantage), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.attackerAdvantage, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuFabricInfo_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(clusterUuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clusterUuid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cliqueId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cliqueId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuFabricInfo_v2_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(clusterUuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.clusterUuid, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(status), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.status, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(cliqueId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.cliqueId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(healthMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.healthMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlInit_v2(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_nvmlInitWithFlags(
    writer: &mut (impl std::io::Write + ?Sized),
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "nvmlInitWithFlags", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlShutdown(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_nvmlErrorString(
    writer: &mut (impl std::io::Write + ?Sized),
    result: cuda_types::nvml::nvmlReturn_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(result), ": ").as_bytes())?;
    crate::CudaDisplay::write(&result, "nvmlErrorString", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetDriverVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "nvmlSystemGetDriverVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlSystemGetDriverVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetNVMLVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "nvmlSystemGetNVMLVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlSystemGetNVMLVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetCudaDriverVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    cudaDriverVersion: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(cudaDriverVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cudaDriverVersion,
        "nvmlSystemGetCudaDriverVersion",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetCudaDriverVersion_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    cudaDriverVersion: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(cudaDriverVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cudaDriverVersion,
        "nvmlSystemGetCudaDriverVersion_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetProcessName(
    writer: &mut (impl std::io::Write + ?Sized),
    pid: ::core::ffi::c_uint,
    name: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "nvmlSystemGetProcessName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "nvmlSystemGetProcessName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlSystemGetProcessName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetHicVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    hwbcCount: *mut ::core::ffi::c_uint,
    hwbcEntries: *mut cuda_types::nvml::nvmlHwbcEntry_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(hwbcCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hwbcCount, "nvmlSystemGetHicVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(hwbcEntries), ": ").as_bytes())?;
    crate::CudaDisplay::write(&hwbcEntries, "nvmlSystemGetHicVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetTopologyGpuSet(
    writer: &mut (impl std::io::Write + ?Sized),
    cpuNumber: ::core::ffi::c_uint,
    count: *mut ::core::ffi::c_uint,
    deviceArray: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(cpuNumber), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cpuNumber,
        "nvmlSystemGetTopologyGpuSet",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "nvmlSystemGetTopologyGpuSet", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(deviceArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &deviceArray,
        "nvmlSystemGetTopologyGpuSet",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlSystemDriverBranchInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(branch), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.branch, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlSystemGetDriverBranch(
    writer: &mut (impl std::io::Write + ?Sized),
    branchInfo: *mut cuda_types::nvml::nvmlSystemDriverBranchInfo_t,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(branchInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &branchInfo,
        "nvmlSystemGetDriverBranch",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlSystemGetDriverBranch", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetCount(
    writer: &mut (impl std::io::Write + ?Sized),
    unitCount: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unitCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unitCount, "nvmlUnitGetCount", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetHandleByIndex(
    writer: &mut (impl std::io::Write + ?Sized),
    index: ::core::ffi::c_uint,
    unit: *mut cuda_types::nvml::nvmlUnit_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(&index, "nvmlUnitGetHandleByIndex", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetHandleByIndex", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetUnitInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    info: *mut cuda_types::nvml::nvmlUnitInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetUnitInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlUnitGetUnitInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetLedState(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    state: *mut cuda_types::nvml::nvmlLedState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetLedState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "nvmlUnitGetLedState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetPsuInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    psu: *mut cuda_types::nvml::nvmlPSUInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetPsuInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(psu), ": ").as_bytes())?;
    crate::CudaDisplay::write(&psu, "nvmlUnitGetPsuInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetTemperature(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    type_: ::core::ffi::c_uint,
    temp: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetTemperature", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlUnitGetTemperature", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(&temp, "nvmlUnitGetTemperature", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetFanSpeedInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    fanSpeeds: *mut cuda_types::nvml::nvmlUnitFanSpeeds_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetFanSpeedInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fanSpeeds), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fanSpeeds, "nvmlUnitGetFanSpeedInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitGetDevices(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    deviceCount: *mut ::core::ffi::c_uint,
    devices: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitGetDevices", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(deviceCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&deviceCount, "nvmlUnitGetDevices", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(devices), ": ").as_bytes())?;
    crate::CudaDisplay::write(&devices, "nvmlUnitGetDevices", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCount_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    deviceCount: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(deviceCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&deviceCount, "nvmlDeviceGetCount_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAttributes_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    attributes: *mut cuda_types::nvml::nvmlDeviceAttributes_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAttributes_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &attributes,
        "nvmlDeviceGetAttributes_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByIndex_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    index: ::core::ffi::c_uint,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(&index, "nvmlDeviceGetHandleByIndex_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetHandleByIndex_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleBySerial(
    writer: &mut (impl std::io::Write + ?Sized),
    serial: *const ::core::ffi::c_char,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(serial), ": ").as_bytes())?;
    crate::CudaDisplay::write(&serial, "nvmlDeviceGetHandleBySerial", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetHandleBySerial", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByUUID(
    writer: &mut (impl std::io::Write + ?Sized),
    uuid: *const ::core::ffi::c_char,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "nvmlDeviceGetHandleByUUID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetHandleByUUID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByUUIDV(
    writer: &mut (impl std::io::Write + ?Sized),
    uuid: *const cuda_types::nvml::nvmlUUID_t,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "nvmlDeviceGetHandleByUUIDV", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetHandleByUUIDV", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByPciBusId_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pciBusId: *const ::core::ffi::c_char,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciBusId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pciBusId,
        "nvmlDeviceGetHandleByPciBusId_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetHandleByPciBusId_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetName(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    name: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(name), ": ").as_bytes())?;
    crate::CudaDisplay::write(&name, "nvmlDeviceGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBrand(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: *mut cuda_types::nvml::nvmlBrandType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBrand", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlDeviceGetBrand", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetIndex(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    index: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetIndex", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(&index, "nvmlDeviceGetIndex", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSerial(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    serial: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetSerial", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(serial), ": ").as_bytes())?;
    crate::CudaDisplay::write(&serial, "nvmlDeviceGetSerial", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetSerial", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetModuleId(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    moduleId: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetModuleId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(moduleId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&moduleId, "nvmlDeviceGetModuleId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetC2cModeInfoV(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    c2cModeInfo: *mut cuda_types::nvml::nvmlC2cModeInfo_v1_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetC2cModeInfoV", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(c2cModeInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &c2cModeInfo,
        "nvmlDeviceGetC2cModeInfoV",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemoryAffinity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    nodeSetSize: ::core::ffi::c_uint,
    nodeSet: *mut ::core::ffi::c_ulong,
    scope: cuda_types::nvml::nvmlAffinityScope_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMemoryAffinity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeSetSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nodeSetSize,
        "nvmlDeviceGetMemoryAffinity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(nodeSet), ": ").as_bytes())?;
    crate::CudaDisplay::write(&nodeSet, "nvmlDeviceGetMemoryAffinity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scope), ": ").as_bytes())?;
    crate::CudaDisplay::write(&scope, "nvmlDeviceGetMemoryAffinity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCpuAffinityWithinScope(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    cpuSetSize: ::core::ffi::c_uint,
    cpuSet: *mut ::core::ffi::c_ulong,
    scope: cuda_types::nvml::nvmlAffinityScope_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCpuAffinityWithinScope",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cpuSetSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cpuSetSize,
        "nvmlDeviceGetCpuAffinityWithinScope",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cpuSet), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &cpuSet,
        "nvmlDeviceGetCpuAffinityWithinScope",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(scope), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &scope,
        "nvmlDeviceGetCpuAffinityWithinScope",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCpuAffinity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    cpuSetSize: ::core::ffi::c_uint,
    cpuSet: *mut ::core::ffi::c_ulong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetCpuAffinity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cpuSetSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cpuSetSize, "nvmlDeviceGetCpuAffinity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cpuSet), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cpuSet, "nvmlDeviceGetCpuAffinity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetCpuAffinity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetCpuAffinity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceClearCpuAffinity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceClearCpuAffinity", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNumaNodeId(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    node: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNumaNodeId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(node), ": ").as_bytes())?;
    crate::CudaDisplay::write(&node, "nvmlDeviceGetNumaNodeId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTopologyCommonAncestor(
    writer: &mut (impl std::io::Write + ?Sized),
    device1: cuda_types::nvml::nvmlDevice_t,
    device2: cuda_types::nvml::nvmlDevice_t,
    pathInfo: *mut cuda_types::nvml::nvmlGpuTopologyLevel_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device1), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device1,
        "nvmlDeviceGetTopologyCommonAncestor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device2), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device2,
        "nvmlDeviceGetTopologyCommonAncestor",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pathInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pathInfo,
        "nvmlDeviceGetTopologyCommonAncestor",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTopologyNearestGpus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    level: cuda_types::nvml::nvmlGpuTopologyLevel_t,
    count: *mut ::core::ffi::c_uint,
    deviceArray: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetTopologyNearestGpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(level), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &level,
        "nvmlDeviceGetTopologyNearestGpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetTopologyNearestGpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(deviceArray), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &deviceArray,
        "nvmlDeviceGetTopologyNearestGpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetP2PStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device1: cuda_types::nvml::nvmlDevice_t,
    device2: cuda_types::nvml::nvmlDevice_t,
    p2pIndex: cuda_types::nvml::nvmlGpuP2PCapsIndex_t,
    p2pStatus: *mut cuda_types::nvml::nvmlGpuP2PStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device1, "nvmlDeviceGetP2PStatus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device2, "nvmlDeviceGetP2PStatus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(p2pIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p2pIndex, "nvmlDeviceGetP2PStatus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(p2pStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(&p2pStatus, "nvmlDeviceGetP2PStatus", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetUUID(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    uuid: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetUUID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "nvmlDeviceGetUUID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetUUID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMinorNumber(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minorNumber: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMinorNumber", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minorNumber), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minorNumber,
        "nvmlDeviceGetMinorNumber",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBoardPartNumber(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    partNumber: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBoardPartNumber", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(partNumber), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &partNumber,
        "nvmlDeviceGetBoardPartNumber",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetBoardPartNumber", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetInforomVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    object: cuda_types::nvml::nvmlInforomObject_t,
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetInforomVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(object), ": ").as_bytes())?;
    crate::CudaDisplay::write(&object, "nvmlDeviceGetInforomVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "nvmlDeviceGetInforomVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetInforomVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetInforomImageVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetInforomImageVersion",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &version,
        "nvmlDeviceGetInforomImageVersion",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &length,
        "nvmlDeviceGetInforomImageVersion",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetInforomConfigurationChecksum(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    checksum: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetInforomConfigurationChecksum",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(checksum), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &checksum,
        "nvmlDeviceGetInforomConfigurationChecksum",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceValidateInforom(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceValidateInforom", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetLastBBXFlushTime(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    timestamp: *mut ::core::ffi::c_ulonglong,
    durationUs: *mut ::core::ffi::c_ulong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetLastBBXFlushTime",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(timestamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &timestamp,
        "nvmlDeviceGetLastBBXFlushTime",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(durationUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &durationUs,
        "nvmlDeviceGetLastBBXFlushTime",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDisplayMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    display: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetDisplayMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(display), ": ").as_bytes())?;
    crate::CudaDisplay::write(&display, "nvmlDeviceGetDisplayMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDisplayActive(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    isActive: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetDisplayActive", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isActive), ": ").as_bytes())?;
    crate::CudaDisplay::write(&isActive, "nvmlDeviceGetDisplayActive", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPersistenceMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPersistenceMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceGetPersistenceMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPciInfoExt(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pci: *mut cuda_types::nvml::nvmlPciInfoExt_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPciInfoExt", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pci, "nvmlDeviceGetPciInfoExt", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPciInfo_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pci: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPciInfo_v3", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pci, "nvmlDeviceGetPciInfo_v3", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMaxPcieLinkGeneration(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    maxLinkGen: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMaxPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLinkGen), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLinkGen,
        "nvmlDeviceGetMaxPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuMaxPcieLinkGeneration(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    maxLinkGenDevice: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuMaxPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLinkGenDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLinkGenDevice,
        "nvmlDeviceGetGpuMaxPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMaxPcieLinkWidth(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    maxLinkWidth: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMaxPcieLinkWidth",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLinkWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLinkWidth,
        "nvmlDeviceGetMaxPcieLinkWidth",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCurrPcieLinkGeneration(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    currLinkGen: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCurrPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currLinkGen), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &currLinkGen,
        "nvmlDeviceGetCurrPcieLinkGeneration",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCurrPcieLinkWidth(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    currLinkWidth: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCurrPcieLinkWidth",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currLinkWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &currLinkWidth,
        "nvmlDeviceGetCurrPcieLinkWidth",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPcieThroughput(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    counter: cuda_types::nvml::nvmlPcieUtilCounter_t,
    value: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPcieThroughput", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(&counter, "nvmlDeviceGetPcieThroughput", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(&value, "nvmlDeviceGetPcieThroughput", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPcieReplayCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    value: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPcieReplayCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(value), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &value,
        "nvmlDeviceGetPcieReplayCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetClockInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: cuda_types::nvml::nvmlClockType_t,
    clock: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetClockInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlDeviceGetClockInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clock), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clock, "nvmlDeviceGetClockInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMaxClockInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: cuda_types::nvml::nvmlClockType_t,
    clock: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMaxClockInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlDeviceGetMaxClockInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clock), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clock, "nvmlDeviceGetMaxClockInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpcClkVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    offset: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpcClkVfOffset", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "nvmlDeviceGetGpcClkVfOffset", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetApplicationsClock(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clockType: cuda_types::nvml::nvmlClockType_t,
    clockMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetApplicationsClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockType,
        "nvmlDeviceGetApplicationsClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockMHz,
        "nvmlDeviceGetApplicationsClock",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDefaultApplicationsClock(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clockType: cuda_types::nvml::nvmlClockType_t,
    clockMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDefaultApplicationsClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockType,
        "nvmlDeviceGetDefaultApplicationsClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockMHz,
        "nvmlDeviceGetDefaultApplicationsClock",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetClock(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clockType: cuda_types::nvml::nvmlClockType_t,
    clockId: cuda_types::nvml::nvmlClockId_t,
    clockMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetClock", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clockType, "nvmlDeviceGetClock", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clockId, "nvmlDeviceGetClock", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(&clockMHz, "nvmlDeviceGetClock", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMaxCustomerBoostClock(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clockType: cuda_types::nvml::nvmlClockType_t,
    clockMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMaxCustomerBoostClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockType,
        "nvmlDeviceGetMaxCustomerBoostClock",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clockMHz,
        "nvmlDeviceGetMaxCustomerBoostClock",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedMemoryClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    count: *mut ::core::ffi::c_uint,
    clocksMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedMemoryClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetSupportedMemoryClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clocksMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clocksMHz,
        "nvmlDeviceGetSupportedMemoryClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedGraphicsClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memoryClockMHz: ::core::ffi::c_uint,
    count: *mut ::core::ffi::c_uint,
    clocksMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedGraphicsClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memoryClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memoryClockMHz,
        "nvmlDeviceGetSupportedGraphicsClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetSupportedGraphicsClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clocksMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clocksMHz,
        "nvmlDeviceGetSupportedGraphicsClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAutoBoostedClocksEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    isEnabled: *mut cuda_types::nvml::nvmlEnableState_t,
    defaultIsEnabled: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isEnabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isEnabled,
        "nvmlDeviceGetAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(defaultIsEnabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &defaultIsEnabled,
        "nvmlDeviceGetAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFanSpeed(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    speed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFanSpeed", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(speed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&speed, "nvmlDeviceGetFanSpeed", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFanSpeed_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
    speed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFanSpeed_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fan, "nvmlDeviceGetFanSpeed_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(speed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&speed, "nvmlDeviceGetFanSpeed_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFanSpeedRPM(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fanSpeed: *mut cuda_types::nvml::nvmlFanSpeedInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFanSpeedRPM", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fanSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fanSpeed, "nvmlDeviceGetFanSpeedRPM", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTargetFanSpeed(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
    targetSpeed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetTargetFanSpeed", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fan, "nvmlDeviceGetTargetFanSpeed", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(targetSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &targetSpeed,
        "nvmlDeviceGetTargetFanSpeed",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMinMaxFanSpeed(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minSpeed: *mut ::core::ffi::c_uint,
    maxSpeed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMinMaxFanSpeed", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minSpeed,
        "nvmlDeviceGetMinMaxFanSpeed",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxSpeed,
        "nvmlDeviceGetMinMaxFanSpeed",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFanControlPolicy_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
    policy: *mut cuda_types::nvml::nvmlFanControlPolicy_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetFanControlPolicy_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &fan,
        "nvmlDeviceGetFanControlPolicy_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(policy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &policy,
        "nvmlDeviceGetFanControlPolicy_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNumFans(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    numFans: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNumFans", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numFans), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numFans, "nvmlDeviceGetNumFans", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTemperature(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sensorType: cuda_types::nvml::nvmlTemperatureSensors_t,
    temp: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetTemperature", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sensorType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sensorType, "nvmlDeviceGetTemperature", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(&temp, "nvmlDeviceGetTemperature", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCoolerInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    coolerInfo: *mut cuda_types::nvml::nvmlCoolerInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetCoolerInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(coolerInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&coolerInfo, "nvmlDeviceGetCoolerInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlTemperature_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sensorType), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sensorType, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(temperature), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.temperature, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlDeviceGetTemperatureV(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    temperature: *mut cuda_types::nvml::nvmlTemperature_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetTemperatureV", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temperature), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temperature,
        "nvmlDeviceGetTemperatureV",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTemperatureThreshold(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    thresholdType: cuda_types::nvml::nvmlTemperatureThresholds_t,
    temp: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(thresholdType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &thresholdType,
        "nvmlDeviceGetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp,
        "nvmlDeviceGetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMarginTemperature(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    marginTempInfo: *mut cuda_types::nvml::nvmlMarginTemperature_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMarginTemperature",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(marginTempInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &marginTempInfo,
        "nvmlDeviceGetMarginTemperature",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetThermalSettings(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sensorIndex: ::core::ffi::c_uint,
    pThermalSettings: *mut cuda_types::nvml::nvmlGpuThermalSettings_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetThermalSettings", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sensorIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sensorIndex,
        "nvmlDeviceGetThermalSettings",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pThermalSettings), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pThermalSettings,
        "nvmlDeviceGetThermalSettings",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPerformanceState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pState: *mut cuda_types::nvml::nvmlPstates_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPerformanceState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pState), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pState,
        "nvmlDeviceGetPerformanceState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCurrentClocksEventReasons(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clocksEventReasons: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCurrentClocksEventReasons",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clocksEventReasons), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clocksEventReasons,
        "nvmlDeviceGetCurrentClocksEventReasons",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCurrentClocksThrottleReasons(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    clocksThrottleReasons: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCurrentClocksThrottleReasons",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(clocksThrottleReasons), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &clocksThrottleReasons,
        "nvmlDeviceGetCurrentClocksThrottleReasons",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedClocksEventReasons(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    supportedClocksEventReasons: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedClocksEventReasons",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(supportedClocksEventReasons), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &supportedClocksEventReasons,
        "nvmlDeviceGetSupportedClocksEventReasons",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedClocksThrottleReasons(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    supportedClocksThrottleReasons: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedClocksThrottleReasons",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer
        .write_all(
            concat!(stringify!(supportedClocksThrottleReasons), ": ").as_bytes(),
        )?;
    crate::CudaDisplay::write(
        &supportedClocksThrottleReasons,
        "nvmlDeviceGetSupportedClocksThrottleReasons",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pState: *mut cuda_types::nvml::nvmlPstates_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPowerState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pState), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pState, "nvmlDeviceGetPowerState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDynamicPstatesInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pDynamicPstatesInfo: *mut cuda_types::nvml::nvmlGpuDynamicPstatesInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDynamicPstatesInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pDynamicPstatesInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pDynamicPstatesInfo,
        "nvmlDeviceGetDynamicPstatesInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemClkVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    offset: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMemClkVfOffset", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "nvmlDeviceGetMemClkVfOffset", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMinMaxClockOfPState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: cuda_types::nvml::nvmlClockType_t,
    pstate: cuda_types::nvml::nvmlPstates_t,
    minClockMHz: *mut ::core::ffi::c_uint,
    maxClockMHz: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMinMaxClockOfPState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &type_,
        "nvmlDeviceGetMinMaxClockOfPState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pstate), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pstate,
        "nvmlDeviceGetMinMaxClockOfPState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minClockMHz,
        "nvmlDeviceGetMinMaxClockOfPState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxClockMHz,
        "nvmlDeviceGetMinMaxClockOfPState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedPerformanceStates(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pstates: *mut cuda_types::nvml::nvmlPstates_t,
    size: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedPerformanceStates",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pstates), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pstates,
        "nvmlDeviceGetSupportedPerformanceStates",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &size,
        "nvmlDeviceGetSupportedPerformanceStates",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpcClkMinMaxVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minOffset: *mut ::core::ffi::c_int,
    maxOffset: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpcClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minOffset,
        "nvmlDeviceGetGpcClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxOffset,
        "nvmlDeviceGetGpcClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemClkMinMaxVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minOffset: *mut ::core::ffi::c_int,
    maxOffset: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMemClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minOffset,
        "nvmlDeviceGetMemClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxOffset), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxOffset,
        "nvmlDeviceGetMemClkMinMaxVfOffset",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetClockOffsets(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    info: *mut cuda_types::nvml::nvmlClockOffset_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetClockOffsets", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlDeviceGetClockOffsets", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetClockOffsets(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    info: *mut cuda_types::nvml::nvmlClockOffset_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetClockOffsets", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlDeviceSetClockOffsets", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPerformanceModes(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    perfModes: *mut cuda_types::nvml::nvmlDevicePerfModes_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPerformanceModes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfModes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfModes,
        "nvmlDeviceGetPerformanceModes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCurrentClockFreqs(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    currentClockFreqs: *mut cuda_types::nvml::nvmlDeviceCurrentClockFreqs_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCurrentClockFreqs",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currentClockFreqs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &currentClockFreqs,
        "nvmlDeviceGetCurrentClockFreqs",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerManagementMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPowerManagementMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "nvmlDeviceGetPowerManagementMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerManagementLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    limit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPowerManagementLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(limit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &limit,
        "nvmlDeviceGetPowerManagementLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerManagementLimitConstraints(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minLimit: *mut ::core::ffi::c_uint,
    maxLimit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPowerManagementLimitConstraints",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minLimit,
        "nvmlDeviceGetPowerManagementLimitConstraints",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxLimit,
        "nvmlDeviceGetPowerManagementLimitConstraints",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerManagementDefaultLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    defaultLimit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPowerManagementDefaultLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(defaultLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &defaultLimit,
        "nvmlDeviceGetPowerManagementDefaultLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerUsage(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    power: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPowerUsage", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(power), ": ").as_bytes())?;
    crate::CudaDisplay::write(&power, "nvmlDeviceGetPowerUsage", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTotalEnergyConsumption(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    energy: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetTotalEnergyConsumption",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(energy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &energy,
        "nvmlDeviceGetTotalEnergyConsumption",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEnforcedPowerLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    limit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetEnforcedPowerLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(limit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &limit,
        "nvmlDeviceGetEnforcedPowerLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuOperationMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    current: *mut cuda_types::nvml::nvmlGpuOperationMode_t,
    pending: *mut cuda_types::nvml::nvmlGpuOperationMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuOperationMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &current,
        "nvmlDeviceGetGpuOperationMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pending), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pending,
        "nvmlDeviceGetGpuOperationMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemoryInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memory: *mut cuda_types::nvml::nvmlMemory_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMemoryInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memory, "nvmlDeviceGetMemoryInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemoryInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memory: *mut cuda_types::nvml::nvmlMemory_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMemoryInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(&memory, "nvmlDeviceGetMemoryInfo_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetComputeMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: *mut cuda_types::nvml::nvmlComputeMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetComputeMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceGetComputeMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCudaComputeCapability(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    major: *mut ::core::ffi::c_int,
    minor: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetCudaComputeCapability",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(major), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &major,
        "nvmlDeviceGetCudaComputeCapability",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minor), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minor,
        "nvmlDeviceGetCudaComputeCapability",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDramEncryptionMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    current: *mut cuda_types::nvml::nvmlDramEncryptionInfo_t,
    pending: *mut cuda_types::nvml::nvmlDramEncryptionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDramEncryptionMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &current,
        "nvmlDeviceGetDramEncryptionMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pending), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pending,
        "nvmlDeviceGetDramEncryptionMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetDramEncryptionMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    dramEncryption: *const cuda_types::nvml::nvmlDramEncryptionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetDramEncryptionMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(dramEncryption), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &dramEncryption,
        "nvmlDeviceSetDramEncryptionMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEccMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    current: *mut cuda_types::nvml::nvmlEnableState_t,
    pending: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetEccMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(&current, "nvmlDeviceGetEccMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pending), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pending, "nvmlDeviceGetEccMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDefaultEccMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    defaultMode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetDefaultEccMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(defaultMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &defaultMode,
        "nvmlDeviceGetDefaultEccMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBoardId(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    boardId: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBoardId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(boardId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&boardId, "nvmlDeviceGetBoardId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMultiGpuBoard(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    multiGpuBool: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMultiGpuBoard", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(multiGpuBool), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &multiGpuBool,
        "nvmlDeviceGetMultiGpuBoard",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetTotalEccErrors(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
    counterType: cuda_types::nvml::nvmlEccCounterType_t,
    eccCounts: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetTotalEccErrors", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(errorType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &errorType,
        "nvmlDeviceGetTotalEccErrors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counterType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counterType,
        "nvmlDeviceGetTotalEccErrors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eccCounts), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eccCounts,
        "nvmlDeviceGetTotalEccErrors",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDetailedEccErrors(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
    counterType: cuda_types::nvml::nvmlEccCounterType_t,
    eccCounts: *mut cuda_types::nvml::nvmlEccErrorCounts_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDetailedEccErrors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(errorType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &errorType,
        "nvmlDeviceGetDetailedEccErrors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counterType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counterType,
        "nvmlDeviceGetDetailedEccErrors",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eccCounts), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eccCounts,
        "nvmlDeviceGetDetailedEccErrors",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemoryErrorCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    errorType: cuda_types::nvml::nvmlMemoryErrorType_t,
    counterType: cuda_types::nvml::nvmlEccCounterType_t,
    locationType: cuda_types::nvml::nvmlMemoryLocation_t,
    count: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMemoryErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(errorType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &errorType,
        "nvmlDeviceGetMemoryErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counterType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counterType,
        "nvmlDeviceGetMemoryErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(locationType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &locationType,
        "nvmlDeviceGetMemoryErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetMemoryErrorCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetUtilizationRates(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut cuda_types::nvml::nvmlUtilization_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetUtilizationRates",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetUtilizationRates",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEncoderUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut ::core::ffi::c_uint,
    samplingPeriodUs: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetEncoderUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetEncoderUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samplingPeriodUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &samplingPeriodUs,
        "nvmlDeviceGetEncoderUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEncoderCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    encoderQueryType: cuda_types::nvml::nvmlEncoderType_t,
    encoderCapacity: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetEncoderCapacity", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(encoderQueryType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &encoderQueryType,
        "nvmlDeviceGetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(encoderCapacity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &encoderCapacity,
        "nvmlDeviceGetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEncoderStats(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sessionCount: *mut ::core::ffi::c_uint,
    averageFps: *mut ::core::ffi::c_uint,
    averageLatency: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetEncoderStats", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlDeviceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(averageFps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &averageFps,
        "nvmlDeviceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(averageLatency), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &averageLatency,
        "nvmlDeviceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetEncoderSessions(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sessionCount: *mut ::core::ffi::c_uint,
    sessionInfos: *mut cuda_types::nvml::nvmlEncoderSessionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetEncoderSessions", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlDeviceGetEncoderSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionInfos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionInfos,
        "nvmlDeviceGetEncoderSessions",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDecoderUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut ::core::ffi::c_uint,
    samplingPeriodUs: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDecoderUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetDecoderUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samplingPeriodUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &samplingPeriodUs,
        "nvmlDeviceGetDecoderUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetJpgUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut ::core::ffi::c_uint,
    samplingPeriodUs: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetJpgUtilization", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetJpgUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samplingPeriodUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &samplingPeriodUs,
        "nvmlDeviceGetJpgUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetOfaUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut ::core::ffi::c_uint,
    samplingPeriodUs: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetOfaUtilization", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetOfaUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samplingPeriodUs), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &samplingPeriodUs,
        "nvmlDeviceGetOfaUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFBCStats(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fbcStats: *mut cuda_types::nvml::nvmlFBCStats_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFBCStats", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fbcStats), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fbcStats, "nvmlDeviceGetFBCStats", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFBCSessions(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sessionCount: *mut ::core::ffi::c_uint,
    sessionInfo: *mut cuda_types::nvml::nvmlFBCSessionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFBCSessions", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlDeviceGetFBCSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionInfo,
        "nvmlDeviceGetFBCSessions",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDriverModel_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    current: *mut cuda_types::nvml::nvmlDriverModel_t,
    pending: *mut cuda_types::nvml::nvmlDriverModel_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetDriverModel_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(&current, "nvmlDeviceGetDriverModel_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pending), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pending, "nvmlDeviceGetDriverModel_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVbiosVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetVbiosVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "nvmlDeviceGetVbiosVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlDeviceGetVbiosVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBridgeChipInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    bridgeHierarchy: *mut cuda_types::nvml::nvmlBridgeChipHierarchy_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBridgeChipInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bridgeHierarchy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bridgeHierarchy,
        "nvmlDeviceGetBridgeChipInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetComputeRunningProcesses_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGraphicsRunningProcesses_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGraphicsRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetGraphicsRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetGraphicsRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMPSComputeRunningProcesses_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMPSComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetMPSComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetMPSComputeRunningProcesses_v3",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRunningProcessDetailList(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    plist: *mut cuda_types::nvml::nvmlProcessDetailList_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetRunningProcessDetailList",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(plist), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &plist,
        "nvmlDeviceGetRunningProcessDetailList",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceOnSameBoard(
    writer: &mut (impl std::io::Write + ?Sized),
    device1: cuda_types::nvml::nvmlDevice_t,
    device2: cuda_types::nvml::nvmlDevice_t,
    onSameBoard: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device1), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device1, "nvmlDeviceOnSameBoard", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device2), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device2, "nvmlDeviceOnSameBoard", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(onSameBoard), ": ").as_bytes())?;
    crate::CudaDisplay::write(&onSameBoard, "nvmlDeviceOnSameBoard", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAPIRestriction(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    apiType: cuda_types::nvml::nvmlRestrictedAPI_t,
    isRestricted: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAPIRestriction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(apiType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&apiType, "nvmlDeviceGetAPIRestriction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isRestricted), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isRestricted,
        "nvmlDeviceGetAPIRestriction",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSamples(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: cuda_types::nvml::nvmlSamplingType_t,
    lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    sampleValType: *mut cuda_types::nvml::nvmlValueType_t,
    sampleCount: *mut ::core::ffi::c_uint,
    samples: *mut cuda_types::nvml::nvmlSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetSamples", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlDeviceGetSamples", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lastSeenTimeStamp,
        "nvmlDeviceGetSamples",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sampleValType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sampleValType, "nvmlDeviceGetSamples", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sampleCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&sampleCount, "nvmlDeviceGetSamples", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(samples), ": ").as_bytes())?;
    crate::CudaDisplay::write(&samples, "nvmlDeviceGetSamples", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBAR1MemoryInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    bar1Memory: *mut cuda_types::nvml::nvmlBAR1Memory_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBAR1MemoryInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bar1Memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bar1Memory,
        "nvmlDeviceGetBAR1MemoryInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetViolationStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    perfPolicyType: cuda_types::nvml::nvmlPerfPolicyType_t,
    violTime: *mut cuda_types::nvml::nvmlViolationTime_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetViolationStatus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(perfPolicyType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &perfPolicyType,
        "nvmlDeviceGetViolationStatus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(violTime), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &violTime,
        "nvmlDeviceGetViolationStatus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetIrqNum(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    irqNum: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetIrqNum", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(irqNum), ": ").as_bytes())?;
    crate::CudaDisplay::write(&irqNum, "nvmlDeviceGetIrqNum", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNumGpuCores(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    numCores: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNumGpuCores", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numCores), ": ").as_bytes())?;
    crate::CudaDisplay::write(&numCores, "nvmlDeviceGetNumGpuCores", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPowerSource(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    powerSource: *mut cuda_types::nvml::nvmlPowerSource_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPowerSource", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(powerSource), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &powerSource,
        "nvmlDeviceGetPowerSource",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMemoryBusWidth(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    busWidth: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMemoryBusWidth", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(busWidth), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &busWidth,
        "nvmlDeviceGetMemoryBusWidth",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPcieLinkMaxSpeed(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    maxSpeed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPcieLinkMaxSpeed",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxSpeed,
        "nvmlDeviceGetPcieLinkMaxSpeed",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPcieSpeed(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pcieSpeed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPcieSpeed", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pcieSpeed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pcieSpeed, "nvmlDeviceGetPcieSpeed", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAdaptiveClockInfoStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    adaptiveClockStatus: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetAdaptiveClockInfoStatus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(adaptiveClockStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &adaptiveClockStatus,
        "nvmlDeviceGetAdaptiveClockInfoStatus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetBusType(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    type_: *mut cuda_types::nvml::nvmlBusType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetBusType", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(type_), ": ").as_bytes())?;
    crate::CudaDisplay::write(&type_, "nvmlDeviceGetBusType", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuFabricInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpuFabricInfo: *mut cuda_types::nvml::nvmlGpuFabricInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpuFabricInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuFabricInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuFabricInfo,
        "nvmlDeviceGetGpuFabricInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuFabricInfoV(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpuFabricInfo: *mut cuda_types::nvml::nvmlGpuFabricInfoV_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpuFabricInfoV", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuFabricInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuFabricInfo,
        "nvmlDeviceGetGpuFabricInfoV",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetConfComputeCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    capabilities: *mut cuda_types::nvml::nvmlConfComputeSystemCaps_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(capabilities), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capabilities,
        "nvmlSystemGetConfComputeCapabilities",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetConfComputeState(
    writer: &mut (impl std::io::Write + ?Sized),
    state: *mut cuda_types::nvml::nvmlConfComputeSystemState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "nvmlSystemGetConfComputeState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetConfComputeMemSizeInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memInfo: *mut cuda_types::nvml::nvmlConfComputeMemSizeInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetConfComputeMemSizeInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memInfo,
        "nvmlDeviceGetConfComputeMemSizeInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetConfComputeGpusReadyState(
    writer: &mut (impl std::io::Write + ?Sized),
    isAcceptingWork: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(isAcceptingWork), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isAcceptingWork,
        "nvmlSystemGetConfComputeGpusReadyState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetConfComputeProtectedMemoryUsage(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memory: *mut cuda_types::nvml::nvmlMemory_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetConfComputeProtectedMemoryUsage",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memory), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memory,
        "nvmlDeviceGetConfComputeProtectedMemoryUsage",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetConfComputeGpuCertificate(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpuCert: *mut cuda_types::nvml::nvmlConfComputeGpuCertificate_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetConfComputeGpuCertificate",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuCert), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuCert,
        "nvmlDeviceGetConfComputeGpuCertificate",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetConfComputeGpuAttestationReport(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpuAtstReport: *mut cuda_types::nvml::nvmlConfComputeGpuAttestationReport_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetConfComputeGpuAttestationReport",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuAtstReport), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuAtstReport,
        "nvmlDeviceGetConfComputeGpuAttestationReport",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetConfComputeKeyRotationThresholdInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    pKeyRotationThrInfo: *mut cuda_types::nvml::nvmlConfComputeGetKeyRotationThresholdInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pKeyRotationThrInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pKeyRotationThrInfo,
        "nvmlSystemGetConfComputeKeyRotationThresholdInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetConfComputeUnprotectedMemSize(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    sizeKiB: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetConfComputeUnprotectedMemSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sizeKiB), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sizeKiB,
        "nvmlDeviceSetConfComputeUnprotectedMemSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemSetConfComputeGpusReadyState(
    writer: &mut (impl std::io::Write + ?Sized),
    isAcceptingWork: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(isAcceptingWork), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isAcceptingWork,
        "nvmlSystemSetConfComputeGpusReadyState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemSetConfComputeKeyRotationThresholdInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    pKeyRotationThrInfo: *mut cuda_types::nvml::nvmlConfComputeSetKeyRotationThresholdInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pKeyRotationThrInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pKeyRotationThrInfo,
        "nvmlSystemSetConfComputeKeyRotationThresholdInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetConfComputeSettings(
    writer: &mut (impl std::io::Write + ?Sized),
    settings: *mut cuda_types::nvml::nvmlSystemConfComputeSettings_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(settings), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &settings,
        "nvmlSystemGetConfComputeSettings",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGspFirmwareVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    version: *mut ::core::ffi::c_char,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGspFirmwareVersion",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &version,
        "nvmlDeviceGetGspFirmwareVersion",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGspFirmwareMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    isEnabled: *mut ::core::ffi::c_uint,
    defaultMode: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGspFirmwareMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isEnabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isEnabled,
        "nvmlDeviceGetGspFirmwareMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(defaultMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &defaultMode,
        "nvmlDeviceGetGspFirmwareMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSramEccErrorStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    status: *mut cuda_types::nvml::nvmlEccSramErrorStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSramEccErrorStatus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &status,
        "nvmlDeviceGetSramEccErrorStatus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAccountingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAccountingMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceGetAccountingMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAccountingStats(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pid: ::core::ffi::c_uint,
    stats: *mut cuda_types::nvml::nvmlAccountingStats_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAccountingStats", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pid, "nvmlDeviceGetAccountingStats", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stats), ": ").as_bytes())?;
    crate::CudaDisplay::write(&stats, "nvmlDeviceGetAccountingStats", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAccountingPids(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    count: *mut ::core::ffi::c_uint,
    pids: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAccountingPids", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "nvmlDeviceGetAccountingPids", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pids), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pids, "nvmlDeviceGetAccountingPids", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAccountingBufferSize(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    bufferSize: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetAccountingBufferSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bufferSize,
        "nvmlDeviceGetAccountingBufferSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRetiredPages(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    cause: cuda_types::nvml::nvmlPageRetirementCause_t,
    pageCount: *mut ::core::ffi::c_uint,
    addresses: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetRetiredPages", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cause), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cause, "nvmlDeviceGetRetiredPages", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pageCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pageCount, "nvmlDeviceGetRetiredPages", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addresses), ": ").as_bytes())?;
    crate::CudaDisplay::write(&addresses, "nvmlDeviceGetRetiredPages", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRetiredPages_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    cause: cuda_types::nvml::nvmlPageRetirementCause_t,
    pageCount: *mut ::core::ffi::c_uint,
    addresses: *mut ::core::ffi::c_ulonglong,
    timestamps: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetRetiredPages_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(cause), ": ").as_bytes())?;
    crate::CudaDisplay::write(&cause, "nvmlDeviceGetRetiredPages_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pageCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pageCount,
        "nvmlDeviceGetRetiredPages_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(addresses), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &addresses,
        "nvmlDeviceGetRetiredPages_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(timestamps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &timestamps,
        "nvmlDeviceGetRetiredPages_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRetiredPagesPendingStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    isPending: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetRetiredPagesPendingStatus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isPending), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isPending,
        "nvmlDeviceGetRetiredPagesPendingStatus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRemappedRows(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    corrRows: *mut ::core::ffi::c_uint,
    uncRows: *mut ::core::ffi::c_uint,
    isPending: *mut ::core::ffi::c_uint,
    failureOccurred: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetRemappedRows", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(corrRows), ": ").as_bytes())?;
    crate::CudaDisplay::write(&corrRows, "nvmlDeviceGetRemappedRows", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uncRows), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uncRows, "nvmlDeviceGetRemappedRows", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isPending), ": ").as_bytes())?;
    crate::CudaDisplay::write(&isPending, "nvmlDeviceGetRemappedRows", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(failureOccurred), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &failureOccurred,
        "nvmlDeviceGetRemappedRows",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetRowRemapperHistogram(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    values: *mut cuda_types::nvml::nvmlRowRemapperHistogramValues_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetRowRemapperHistogram",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &values,
        "nvmlDeviceGetRowRemapperHistogram",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetArchitecture(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    arch: *mut cuda_types::nvml::nvmlDeviceArchitecture_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetArchitecture", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(arch), ": ").as_bytes())?;
    crate::CudaDisplay::write(&arch, "nvmlDeviceGetArchitecture", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetClkMonStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    status: *mut cuda_types::nvml::nvmlClkMonStatus_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetClkMonStatus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(status), ": ").as_bytes())?;
    crate::CudaDisplay::write(&status, "nvmlDeviceGetClkMonStatus", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetProcessUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    utilization: *mut cuda_types::nvml::nvmlProcessUtilizationSample_t,
    processSamplesCount: *mut ::core::ffi::c_uint,
    lastSeenTimeStamp: ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilization), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilization,
        "nvmlDeviceGetProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(processSamplesCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &processSamplesCount,
        "nvmlDeviceGetProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lastSeenTimeStamp,
        "nvmlDeviceGetProcessUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetProcessesUtilizationInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    procesesUtilInfo: *mut cuda_types::nvml::nvmlProcessesUtilizationInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetProcessesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(procesesUtilInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &procesesUtilInfo,
        "nvmlDeviceGetProcessesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPlatformInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    platformInfo: *mut cuda_types::nvml::nvmlPlatformInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPlatformInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(platformInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &platformInfo,
        "nvmlDeviceGetPlatformInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlUnitSetLedState(
    writer: &mut (impl std::io::Write + ?Sized),
    unit: cuda_types::nvml::nvmlUnit_t,
    color: cuda_types::nvml::nvmlLedColor_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(unit), ": ").as_bytes())?;
    crate::CudaDisplay::write(&unit, "nvmlUnitSetLedState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(color), ": ").as_bytes())?;
    crate::CudaDisplay::write(&color, "nvmlUnitSetLedState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetPersistenceMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetPersistenceMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceSetPersistenceMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetComputeMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: cuda_types::nvml::nvmlComputeMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetComputeMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceSetComputeMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetEccMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    ecc: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetEccMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ecc), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ecc, "nvmlDeviceSetEccMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceClearEccErrorCounts(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    counterType: cuda_types::nvml::nvmlEccCounterType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceClearEccErrorCounts",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counterType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counterType,
        "nvmlDeviceClearEccErrorCounts",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetDriverModel(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    driverModel: cuda_types::nvml::nvmlDriverModel_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetDriverModel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(driverModel), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &driverModel,
        "nvmlDeviceSetDriverModel",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(&flags, "nvmlDeviceSetDriverModel", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlClockLimitId_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlClockLimitId_enum::NVML_CLOCK_LIMIT_ID_RANGE_START => {
                writer.write_all(stringify!(NVML_CLOCK_LIMIT_ID_RANGE_START).as_bytes())
            }
            &cuda_types::nvml::nvmlClockLimitId_enum::NVML_CLOCK_LIMIT_ID_TDP => {
                writer.write_all(stringify!(NVML_CLOCK_LIMIT_ID_TDP).as_bytes())
            }
            &cuda_types::nvml::nvmlClockLimitId_enum::NVML_CLOCK_LIMIT_ID_UNLIMITED => {
                writer.write_all(stringify!(NVML_CLOCK_LIMIT_ID_UNLIMITED).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
pub fn write_nvmlDeviceSetGpuLockedClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minGpuClockMHz: ::core::ffi::c_uint,
    maxGpuClockMHz: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetGpuLockedClocks", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minGpuClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minGpuClockMHz,
        "nvmlDeviceSetGpuLockedClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxGpuClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxGpuClockMHz,
        "nvmlDeviceSetGpuLockedClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceResetGpuLockedClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceResetGpuLockedClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetMemoryLockedClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    minMemClockMHz: ::core::ffi::c_uint,
    maxMemClockMHz: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetMemoryLockedClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(minMemClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &minMemClockMHz,
        "nvmlDeviceSetMemoryLockedClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(maxMemClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &maxMemClockMHz,
        "nvmlDeviceSetMemoryLockedClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceResetMemoryLockedClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceResetMemoryLockedClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetApplicationsClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    memClockMHz: ::core::ffi::c_uint,
    graphicsClockMHz: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetApplicationsClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(memClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &memClockMHz,
        "nvmlDeviceSetApplicationsClocks",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(graphicsClockMHz), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &graphicsClockMHz,
        "nvmlDeviceSetApplicationsClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceResetApplicationsClocks(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceResetApplicationsClocks",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetAutoBoostedClocksEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    enabled: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(enabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &enabled,
        "nvmlDeviceSetAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetDefaultAutoBoostedClocksEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    enabled: cuda_types::nvml::nvmlEnableState_t,
    flags: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetDefaultAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(enabled), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &enabled,
        "nvmlDeviceSetDefaultAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(flags), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &flags,
        "nvmlDeviceSetDefaultAutoBoostedClocksEnabled",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetDefaultFanSpeed_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetDefaultFanSpeed_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fan, "nvmlDeviceSetDefaultFanSpeed_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetFanControlPolicy(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
    policy: cuda_types::nvml::nvmlFanControlPolicy_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetFanControlPolicy",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fan, "nvmlDeviceSetFanControlPolicy", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(policy), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &policy,
        "nvmlDeviceSetFanControlPolicy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetTemperatureThreshold(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    thresholdType: cuda_types::nvml::nvmlTemperatureThresholds_t,
    temp: *mut ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(thresholdType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &thresholdType,
        "nvmlDeviceSetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(temp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &temp,
        "nvmlDeviceSetTemperatureThreshold",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetPowerManagementLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    limit: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetPowerManagementLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(limit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &limit,
        "nvmlDeviceSetPowerManagementLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetGpuOperationMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: cuda_types::nvml::nvmlGpuOperationMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetGpuOperationMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceSetGpuOperationMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetAPIRestriction(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    apiType: cuda_types::nvml::nvmlRestrictedAPI_t,
    isRestricted: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetAPIRestriction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(apiType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&apiType, "nvmlDeviceSetAPIRestriction", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isRestricted), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isRestricted,
        "nvmlDeviceSetAPIRestriction",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetFanSpeed_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    fan: ::core::ffi::c_uint,
    speed: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetFanSpeed_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fan), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fan, "nvmlDeviceSetFanSpeed_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(speed), ": ").as_bytes())?;
    crate::CudaDisplay::write(&speed, "nvmlDeviceSetFanSpeed_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetGpcClkVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    offset: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetGpcClkVfOffset", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "nvmlDeviceSetGpcClkVfOffset", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetMemClkVfOffset(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    offset: ::core::ffi::c_int,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetMemClkVfOffset", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(offset), ": ").as_bytes())?;
    crate::CudaDisplay::write(&offset, "nvmlDeviceSetMemClkVfOffset", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetAccountingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetAccountingMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceSetAccountingMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceClearAccountingPids(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceClearAccountingPids",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetPowerManagementLimit_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    powerValue: *mut cuda_types::nvml::nvmlPowerValue_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetPowerManagementLimit_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(powerValue), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &powerValue,
        "nvmlDeviceSetPowerManagementLimit_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvlinkSupportedBwModes_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bwModes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bwModes, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(totalBwModes), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.totalBwModes, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvlinkGetBwMode_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bIsBest), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bIsBest, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bwMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bwMode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlNvlinkSetBwMode_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bSetBest), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bSetBest, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(bwMode), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.bwMode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlDeviceGetNvLinkState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    isActive: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNvLinkState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(&link, "nvmlDeviceGetNvLinkState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isActive), ": ").as_bytes())?;
    crate::CudaDisplay::write(&isActive, "nvmlDeviceGetNvLinkState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    version: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNvLinkVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(&link, "nvmlDeviceGetNvLinkVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(&version, "nvmlDeviceGetNvLinkVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkCapability(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    capability: cuda_types::nvml::nvmlNvLinkCapability_t,
    capResult: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkCapability",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(&link, "nvmlDeviceGetNvLinkCapability", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capability), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capability,
        "nvmlDeviceGetNvLinkCapability",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capResult,
        "nvmlDeviceGetNvLinkCapability",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkRemotePciInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    pci: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkRemotePciInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkRemotePciInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pci,
        "nvmlDeviceGetNvLinkRemotePciInfo_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkErrorCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: cuda_types::nvml::nvmlNvLinkErrorCounter_t,
    counterValue: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceGetNvLinkErrorCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counterValue), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counterValue,
        "nvmlDeviceGetNvLinkErrorCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceResetNvLinkErrorCounters(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceResetNvLinkErrorCounters",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceResetNvLinkErrorCounters",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetNvLinkUtilizationControl(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: ::core::ffi::c_uint,
    control: *mut cuda_types::nvml::nvmlNvLinkUtilizationControl_t,
    reset: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceSetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceSetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(control), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &control,
        "nvmlDeviceSetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(reset), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &reset,
        "nvmlDeviceSetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkUtilizationControl(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: ::core::ffi::c_uint,
    control: *mut cuda_types::nvml::nvmlNvLinkUtilizationControl_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceGetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(control), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &control,
        "nvmlDeviceGetNvLinkUtilizationControl",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkUtilizationCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: ::core::ffi::c_uint,
    rxcounter: *mut ::core::ffi::c_ulonglong,
    txcounter: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceGetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(rxcounter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &rxcounter,
        "nvmlDeviceGetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(txcounter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &txcounter,
        "nvmlDeviceGetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceFreezeNvLinkUtilizationCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: ::core::ffi::c_uint,
    freeze: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceFreezeNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceFreezeNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceFreezeNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(freeze), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &freeze,
        "nvmlDeviceFreezeNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceResetNvLinkUtilizationCounter(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    counter: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceResetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceResetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(counter), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &counter,
        "nvmlDeviceResetNvLinkUtilizationCounter",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkRemoteDeviceType(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    pNvLinkDeviceType: *mut cuda_types::nvml::nvmlIntNvLinkDeviceType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkRemoteDeviceType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkRemoteDeviceType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pNvLinkDeviceType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pNvLinkDeviceType,
        "nvmlDeviceGetNvLinkRemoteDeviceType",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetNvLinkDeviceLowPowerThreshold(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    info: *mut cuda_types::nvml::nvmlNvLinkPowerThres_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetNvLinkDeviceLowPowerThreshold",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlDeviceSetNvLinkDeviceLowPowerThreshold",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemSetNvlinkBwMode(
    writer: &mut (impl std::io::Write + ?Sized),
    nvlinkBwMode: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nvlinkBwMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nvlinkBwMode,
        "nvmlSystemSetNvlinkBwMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemGetNvlinkBwMode(
    writer: &mut (impl std::io::Write + ?Sized),
    nvlinkBwMode: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(nvlinkBwMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &nvlinkBwMode,
        "nvmlSystemGetNvlinkBwMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvlinkSupportedBwModes(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    supportedBwMode: *mut cuda_types::nvml::nvmlNvlinkSupportedBwModes_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvlinkSupportedBwModes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(supportedBwMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &supportedBwMode,
        "nvmlDeviceGetNvlinkSupportedBwModes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvlinkBwMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    getBwMode: *mut cuda_types::nvml::nvmlNvlinkGetBwMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetNvlinkBwMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(getBwMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&getBwMode, "nvmlDeviceGetNvlinkBwMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetNvlinkBwMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    setBwMode: *mut cuda_types::nvml::nvmlNvlinkSetBwMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetNvlinkBwMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(setBwMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&setBwMode, "nvmlDeviceSetNvlinkBwMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlEventSetCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    set: *mut cuda_types::nvml::nvmlEventSet_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(set), ": ").as_bytes())?;
    crate::CudaDisplay::write(&set, "nvmlEventSetCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceRegisterEvents(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    eventTypes: ::core::ffi::c_ulonglong,
    set: cuda_types::nvml::nvmlEventSet_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceRegisterEvents", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eventTypes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&eventTypes, "nvmlDeviceRegisterEvents", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(set), ": ").as_bytes())?;
    crate::CudaDisplay::write(&set, "nvmlDeviceRegisterEvents", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedEventTypes(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    eventTypes: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetSupportedEventTypes",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eventTypes), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &eventTypes,
        "nvmlDeviceGetSupportedEventTypes",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlEventSetWait_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    set: cuda_types::nvml::nvmlEventSet_t,
    data: *mut cuda_types::nvml::nvmlEventData_t,
    timeoutms: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(set), ": ").as_bytes())?;
    crate::CudaDisplay::write(&set, "nvmlEventSetWait_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "nvmlEventSetWait_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(timeoutms), ": ").as_bytes())?;
    crate::CudaDisplay::write(&timeoutms, "nvmlEventSetWait_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlEventSetFree(
    writer: &mut (impl std::io::Write + ?Sized),
    set: cuda_types::nvml::nvmlEventSet_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(set), ": ").as_bytes())?;
    crate::CudaDisplay::write(&set, "nvmlEventSetFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemEventSetCreate(
    writer: &mut (impl std::io::Write + ?Sized),
    request: *mut cuda_types::nvml::nvmlSystemEventSetCreateRequest_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(request), ": ").as_bytes())?;
    crate::CudaDisplay::write(&request, "nvmlSystemEventSetCreate", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemEventSetFree(
    writer: &mut (impl std::io::Write + ?Sized),
    request: *mut cuda_types::nvml::nvmlSystemEventSetFreeRequest_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(request), ": ").as_bytes())?;
    crate::CudaDisplay::write(&request, "nvmlSystemEventSetFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemRegisterEvents(
    writer: &mut (impl std::io::Write + ?Sized),
    request: *mut cuda_types::nvml::nvmlSystemRegisterEventRequest_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(request), ": ").as_bytes())?;
    crate::CudaDisplay::write(&request, "nvmlSystemRegisterEvents", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSystemEventSetWait(
    writer: &mut (impl std::io::Write + ?Sized),
    request: *mut cuda_types::nvml::nvmlSystemEventSetWaitRequest_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(request), ": ").as_bytes())?;
    crate::CudaDisplay::write(&request, "nvmlSystemEventSetWait", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceModifyDrainState(
    writer: &mut (impl std::io::Write + ?Sized),
    pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
    newState: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciInfo, "nvmlDeviceModifyDrainState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(newState), ": ").as_bytes())?;
    crate::CudaDisplay::write(&newState, "nvmlDeviceModifyDrainState", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceQueryDrainState(
    writer: &mut (impl std::io::Write + ?Sized),
    pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
    currentState: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciInfo, "nvmlDeviceQueryDrainState", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currentState), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &currentState,
        "nvmlDeviceQueryDrainState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceRemoveGpu_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
    gpuState: cuda_types::nvml::nvmlDetachGpuState_t,
    linkState: cuda_types::nvml::nvmlPcieLinkState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciInfo, "nvmlDeviceRemoveGpu_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuState), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpuState, "nvmlDeviceRemoveGpu_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(linkState), ": ").as_bytes())?;
    crate::CudaDisplay::write(&linkState, "nvmlDeviceRemoveGpu_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceDiscoverGpus(
    writer: &mut (impl std::io::Write + ?Sized),
    pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciInfo, "nvmlDeviceDiscoverGpus", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetFieldValues(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    valuesCount: ::core::ffi::c_int,
    values: *mut cuda_types::nvml::nvmlFieldValue_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetFieldValues", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(valuesCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &valuesCount,
        "nvmlDeviceGetFieldValues",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(&values, "nvmlDeviceGetFieldValues", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceClearFieldValues(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    valuesCount: ::core::ffi::c_int,
    values: *mut cuda_types::nvml::nvmlFieldValue_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceClearFieldValues", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(valuesCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &valuesCount,
        "nvmlDeviceClearFieldValues",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(values), ": ").as_bytes())?;
    crate::CudaDisplay::write(&values, "nvmlDeviceClearFieldValues", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVirtualizationMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pVirtualMode: *mut cuda_types::nvml::nvmlGpuVirtualizationMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVirtualizationMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pVirtualMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pVirtualMode,
        "nvmlDeviceGetVirtualizationMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHostVgpuMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pHostVgpuMode: *mut cuda_types::nvml::nvmlHostVgpuMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetHostVgpuMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pHostVgpuMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHostVgpuMode,
        "nvmlDeviceGetHostVgpuMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetVirtualizationMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    virtualMode: cuda_types::nvml::nvmlGpuVirtualizationMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetVirtualizationMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(virtualMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &virtualMode,
        "nvmlDeviceSetVirtualizationMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuHeterogeneousMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pHeterogeneousMode: *mut cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pHeterogeneousMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHeterogeneousMode,
        "nvmlDeviceGetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetVgpuHeterogeneousMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pHeterogeneousMode: *const cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pHeterogeneousMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHeterogeneousMode,
        "nvmlDeviceSetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetPlacementId(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    pPlacement: *mut cuda_types::nvml::nvmlVgpuPlacementId_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetPlacementId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pPlacement), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pPlacement,
        "nvmlVgpuInstanceGetPlacementId",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuTypeSupportedPlacements(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    pPlacementList: *mut cuda_types::nvml::nvmlVgpuPlacementList_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuTypeSupportedPlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlDeviceGetVgpuTypeSupportedPlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pPlacementList), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pPlacementList,
        "nvmlDeviceGetVgpuTypeSupportedPlacements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuTypeCreatablePlacements(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    pPlacementList: *mut cuda_types::nvml::nvmlVgpuPlacementList_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuTypeCreatablePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlDeviceGetVgpuTypeCreatablePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pPlacementList), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pPlacementList,
        "nvmlDeviceGetVgpuTypeCreatablePlacements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetGspHeapSize(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    gspHeapSize: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetGspHeapSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gspHeapSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gspHeapSize,
        "nvmlVgpuTypeGetGspHeapSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetFbReservation(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    fbReservation: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetFbReservation",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fbReservation), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &fbReservation,
        "nvmlVgpuTypeGetFbReservation",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetRuntimeStateSize(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    pState: *mut cuda_types::nvml::nvmlVgpuRuntimeState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetRuntimeStateSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pState), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pState,
        "nvmlVgpuInstanceGetRuntimeStateSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetVgpuCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    capability: cuda_types::nvml::nvmlDeviceVgpuCapability_t,
    state: cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetVgpuCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capability), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capability,
        "nvmlDeviceSetVgpuCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "nvmlDeviceSetVgpuCapabilities", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGridLicensableFeatures_v4(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGridLicensableFeatures_v4",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGridLicensableFeatures), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pGridLicensableFeatures,
        "nvmlDeviceGetGridLicensableFeatures_v4",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGetVgpuDriverCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    capability: cuda_types::nvml::nvmlVgpuDriverCapability_t,
    capResult: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(capability), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capability,
        "nvmlGetVgpuDriverCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capResult,
        "nvmlGetVgpuDriverCapabilities",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    capability: cuda_types::nvml::nvmlDeviceVgpuCapability_t,
    capResult: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capability), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capability,
        "nvmlDeviceGetVgpuCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capResult,
        "nvmlDeviceGetVgpuCapabilities",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetSupportedVgpus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuCount: *mut ::core::ffi::c_uint,
    vgpuTypeIds: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetSupportedVgpus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuCount,
        "nvmlDeviceGetSupportedVgpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeIds), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeIds,
        "nvmlDeviceGetSupportedVgpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetCreatableVgpus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuCount: *mut ::core::ffi::c_uint,
    vgpuTypeIds: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetCreatableVgpus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuCount,
        "nvmlDeviceGetCreatableVgpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeIds), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeIds,
        "nvmlDeviceGetCreatableVgpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetClass(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    vgpuTypeClass: *mut ::core::ffi::c_char,
    size: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuTypeGetClass", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeClass), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeClass, "nvmlVgpuTypeGetClass", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuTypeGetClass", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetName(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    vgpuTypeName: *mut ::core::ffi::c_char,
    size: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuTypeGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeName), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeName, "nvmlVgpuTypeGetName", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuTypeGetName", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetGpuInstanceProfileId(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    gpuInstanceProfileId: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetGpuInstanceProfileId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstanceProfileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstanceProfileId,
        "nvmlVgpuTypeGetGpuInstanceProfileId",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetDeviceID(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    deviceID: *mut ::core::ffi::c_ulonglong,
    subsystemID: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuTypeGetDeviceID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(deviceID), ": ").as_bytes())?;
    crate::CudaDisplay::write(&deviceID, "nvmlVgpuTypeGetDeviceID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(subsystemID), ": ").as_bytes())?;
    crate::CudaDisplay::write(&subsystemID, "nvmlVgpuTypeGetDeviceID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetFramebufferSize(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    fbSize: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetFramebufferSize",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fbSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &fbSize,
        "nvmlVgpuTypeGetFramebufferSize",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetNumDisplayHeads(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    numDisplayHeads: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetNumDisplayHeads",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(numDisplayHeads), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &numDisplayHeads,
        "nvmlVgpuTypeGetNumDisplayHeads",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetResolution(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    displayIndex: ::core::ffi::c_uint,
    xdim: *mut ::core::ffi::c_uint,
    ydim: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetResolution",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(displayIndex), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &displayIndex,
        "nvmlVgpuTypeGetResolution",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(xdim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&xdim, "nvmlVgpuTypeGetResolution", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(ydim), ": ").as_bytes())?;
    crate::CudaDisplay::write(&ydim, "nvmlVgpuTypeGetResolution", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetLicense(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    vgpuTypeLicenseString: *mut ::core::ffi::c_char,
    size: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuTypeGetLicense", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeLicenseString), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeLicenseString,
        "nvmlVgpuTypeGetLicense",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuTypeGetLicense", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetFrameRateLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    frameRateLimit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetFrameRateLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(frameRateLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &frameRateLimit,
        "nvmlVgpuTypeGetFrameRateLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetMaxInstances(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    vgpuInstanceCount: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlVgpuTypeGetMaxInstances", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetMaxInstances",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuInstanceCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstanceCount,
        "nvmlVgpuTypeGetMaxInstances",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetMaxInstancesPerVm(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    vgpuInstanceCountPerVm: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetMaxInstancesPerVm",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuInstanceCountPerVm), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstanceCountPerVm,
        "nvmlVgpuTypeGetMaxInstancesPerVm",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetBAR1Info(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    bar1Info: *mut cuda_types::nvml::nvmlVgpuTypeBar1Info_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuTypeGetBAR1Info", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bar1Info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&bar1Info, "nvmlVgpuTypeGetBAR1Info", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetActiveVgpus(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuCount: *mut ::core::ffi::c_uint,
    vgpuInstances: *mut cuda_types::nvml::nvmlVgpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetActiveVgpus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuCount, "nvmlDeviceGetActiveVgpus", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuInstances), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstances,
        "nvmlDeviceGetActiveVgpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetVmID(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    vmId: *mut ::core::ffi::c_char,
    size: ::core::ffi::c_uint,
    vmIdType: *mut cuda_types::nvml::nvmlVgpuVmIdType_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetVmID",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vmId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vmId, "nvmlVgpuInstanceGetVmID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuInstanceGetVmID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vmIdType), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vmIdType, "nvmlVgpuInstanceGetVmID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetUUID(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    uuid: *mut ::core::ffi::c_char,
    size: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetUUID",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(uuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(&uuid, "nvmlVgpuInstanceGetUUID", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuInstanceGetUUID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetVmDriverVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    version: *mut ::core::ffi::c_char,
    length: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetVmDriverVersion",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(version), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &version,
        "nvmlVgpuInstanceGetVmDriverVersion",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &length,
        "nvmlVgpuInstanceGetVmDriverVersion",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetFbUsage(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    fbUsage: *mut ::core::ffi::c_ulonglong,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetFbUsage",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fbUsage), ": ").as_bytes())?;
    crate::CudaDisplay::write(&fbUsage, "nvmlVgpuInstanceGetFbUsage", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetLicenseStatus(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    licensed: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetLicenseStatus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(licensed), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &licensed,
        "nvmlVgpuInstanceGetLicenseStatus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetType(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    vgpuTypeId: *mut cuda_types::nvml::nvmlVgpuTypeId_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetType",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuTypeId, "nvmlVgpuInstanceGetType", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetFrameRateLimit(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    frameRateLimit: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetFrameRateLimit",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(frameRateLimit), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &frameRateLimit,
        "nvmlVgpuInstanceGetFrameRateLimit",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetEccMode(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    eccMode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetEccMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(eccMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&eccMode, "nvmlVgpuInstanceGetEccMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetEncoderCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    encoderCapacity: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(encoderCapacity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &encoderCapacity,
        "nvmlVgpuInstanceGetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceSetEncoderCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    encoderCapacity: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceSetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(encoderCapacity), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &encoderCapacity,
        "nvmlVgpuInstanceSetEncoderCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetEncoderStats(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    sessionCount: *mut ::core::ffi::c_uint,
    averageFps: *mut ::core::ffi::c_uint,
    averageLatency: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlVgpuInstanceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(averageFps), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &averageFps,
        "nvmlVgpuInstanceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(averageLatency), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &averageLatency,
        "nvmlVgpuInstanceGetEncoderStats",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetEncoderSessions(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    sessionCount: *mut ::core::ffi::c_uint,
    sessionInfo: *mut cuda_types::nvml::nvmlEncoderSessionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetEncoderSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlVgpuInstanceGetEncoderSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionInfo,
        "nvmlVgpuInstanceGetEncoderSessions",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetFBCStats(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    fbcStats: *mut cuda_types::nvml::nvmlFBCStats_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetFBCStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(fbcStats), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &fbcStats,
        "nvmlVgpuInstanceGetFBCStats",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetFBCSessions(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    sessionCount: *mut ::core::ffi::c_uint,
    sessionInfo: *mut cuda_types::nvml::nvmlFBCSessionInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetFBCSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionCount,
        "nvmlVgpuInstanceGetFBCSessions",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sessionInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sessionInfo,
        "nvmlVgpuInstanceGetFBCSessions",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetGpuInstanceId(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    gpuInstanceId: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetGpuInstanceId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstanceId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstanceId,
        "nvmlVgpuInstanceGetGpuInstanceId",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetGpuPciId(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    vgpuPciId: *mut ::core::ffi::c_char,
    length: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetGpuPciId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuPciId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuPciId,
        "nvmlVgpuInstanceGetGpuPciId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(length), ": ").as_bytes())?;
    crate::CudaDisplay::write(&length, "nvmlVgpuInstanceGetGpuPciId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuTypeId: cuda_types::nvml::nvmlVgpuTypeId_t,
    capability: cuda_types::nvml::nvmlVgpuCapability_t,
    capResult: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuTypeId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuTypeId,
        "nvmlVgpuTypeGetCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capability), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capability,
        "nvmlVgpuTypeGetCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(capResult), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &capResult,
        "nvmlVgpuTypeGetCapabilities",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetMdevUUID(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    mdevUuid: *mut ::core::ffi::c_char,
    size: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetMdevUUID",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mdevUuid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mdevUuid,
        "nvmlVgpuInstanceGetMdevUUID",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(size), ": ").as_bytes())?;
    crate::CudaDisplay::write(&size, "nvmlVgpuInstanceGetMdevUUID", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetCreatableVgpus(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pVgpus: *mut cuda_types::nvml::nvmlVgpuTypeIdInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetCreatableVgpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pVgpus), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pVgpus,
        "nvmlGpuInstanceGetCreatableVgpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuTypeGetMaxInstancesPerGpuInstance(
    writer: &mut (impl std::io::Write + ?Sized),
    pMaxInstance: *mut cuda_types::nvml::nvmlVgpuTypeMaxInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pMaxInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pMaxInstance,
        "nvmlVgpuTypeGetMaxInstancesPerGpuInstance",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetActiveVgpus(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pVgpuInstanceInfo: *mut cuda_types::nvml::nvmlActiveVgpuInstanceInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetActiveVgpus",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pVgpuInstanceInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pVgpuInstanceInfo,
        "nvmlGpuInstanceGetActiveVgpus",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceSetVgpuSchedulerState(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pScheduler: *mut cuda_types::nvml::nvmlVgpuSchedulerState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceSetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pScheduler), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pScheduler,
        "nvmlGpuInstanceSetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetVgpuSchedulerState(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pSchedulerStateInfo: *mut cuda_types::nvml::nvmlVgpuSchedulerStateInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSchedulerStateInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSchedulerStateInfo,
        "nvmlGpuInstanceGetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetVgpuSchedulerLog(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pSchedulerLogInfo: *mut cuda_types::nvml::nvmlVgpuSchedulerLogInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetVgpuSchedulerLog",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSchedulerLogInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSchedulerLogInfo,
        "nvmlGpuInstanceGetVgpuSchedulerLog",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetVgpuTypeCreatablePlacements(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pCreatablePlacementInfo: *mut cuda_types::nvml::nvmlVgpuCreatablePlacementInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetVgpuTypeCreatablePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCreatablePlacementInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCreatablePlacementInfo,
        "nvmlGpuInstanceGetVgpuTypeCreatablePlacements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetVgpuHeterogeneousMode(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pHeterogeneousMode: *mut cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pHeterogeneousMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHeterogeneousMode,
        "nvmlGpuInstanceGetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceSetVgpuHeterogeneousMode(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    pHeterogeneousMode: *const cuda_types::nvml::nvmlVgpuHeterogeneousMode_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceSetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pHeterogeneousMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pHeterogeneousMode,
        "nvmlGpuInstanceSetVgpuHeterogeneousMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuVersion_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(minVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.minVersion, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(maxVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.maxVersion, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuMetadata_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(revision), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.revision, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(guestInfoState), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.guestInfoState, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(guestDriverVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.guestDriverVersion, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hostDriverVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hostDriverVersion, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(vgpuVirtualizationCaps), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.vgpuVirtualizationCaps, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(guestVgpuVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.guestVgpuVersion, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(opaqueDataSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.opaqueDataSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(opaqueData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.opaqueData, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPgpuMetadata_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(revision), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.revision, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(hostDriverVersion), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.hostDriverVersion, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(pgpuVirtualizationCaps), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.pgpuVirtualizationCaps, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(hostSupportedVgpuRange), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.hostSupportedVgpuRange, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(opaqueDataSize), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.opaqueDataSize, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(opaqueData), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.opaqueData, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuVmCompatibility_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuVmCompatibility_enum::NVML_VGPU_VM_COMPATIBILITY_NONE => {
                writer.write_all(stringify!(NVML_VGPU_VM_COMPATIBILITY_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuVmCompatibility_enum::NVML_VGPU_VM_COMPATIBILITY_COLD => {
                writer.write_all(stringify!(NVML_VGPU_VM_COMPATIBILITY_COLD).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuVmCompatibility_enum::NVML_VGPU_VM_COMPATIBILITY_HIBERNATE => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_VM_COMPATIBILITY_HIBERNATE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlVgpuVmCompatibility_enum::NVML_VGPU_VM_COMPATIBILITY_SLEEP => {
                writer.write_all(stringify!(NVML_VGPU_VM_COMPATIBILITY_SLEEP).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuVmCompatibility_enum::NVML_VGPU_VM_COMPATIBILITY_LIVE => {
                writer.write_all(stringify!(NVML_VGPU_VM_COMPATIBILITY_LIVE).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum::NVML_VGPU_COMPATIBILITY_LIMIT_NONE => {
                writer
                    .write_all(stringify!(NVML_VGPU_COMPATIBILITY_LIMIT_NONE).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum::NVML_VGPU_COMPATIBILITY_LIMIT_HOST_DRIVER => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_COMPATIBILITY_LIMIT_HOST_DRIVER).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum::NVML_VGPU_COMPATIBILITY_LIMIT_GUEST_DRIVER => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_COMPATIBILITY_LIMIT_GUEST_DRIVER).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum::NVML_VGPU_COMPATIBILITY_LIMIT_GPU => {
                writer
                    .write_all(stringify!(NVML_VGPU_COMPATIBILITY_LIMIT_GPU).as_bytes())
            }
            &cuda_types::nvml::nvmlVgpuPgpuCompatibilityLimitCode_enum::NVML_VGPU_COMPATIBILITY_LIMIT_OTHER => {
                writer
                    .write_all(
                        stringify!(NVML_VGPU_COMPATIBILITY_LIMIT_OTHER).as_bytes(),
                    )
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlVgpuPgpuCompatibility_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer
            .write_all(concat!("{ ", stringify!(vgpuVmCompatibility), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.vgpuVmCompatibility, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(compatibilityLimitCode), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.compatibilityLimitCode, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlVgpuInstanceGetMetadata(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    vgpuMetadata: *mut cuda_types::nvml::nvmlVgpuMetadata_t,
    bufferSize: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetMetadata",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuMetadata), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuMetadata,
        "nvmlVgpuInstanceGetMetadata",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bufferSize,
        "nvmlVgpuInstanceGetMetadata",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuMetadata(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pgpuMetadata: *mut cuda_types::nvml::nvmlVgpuPgpuMetadata_t,
    bufferSize: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetVgpuMetadata", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pgpuMetadata), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pgpuMetadata,
        "nvmlDeviceGetVgpuMetadata",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bufferSize,
        "nvmlDeviceGetVgpuMetadata",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGetVgpuCompatibility(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuMetadata: *mut cuda_types::nvml::nvmlVgpuMetadata_t,
    pgpuMetadata: *mut cuda_types::nvml::nvmlVgpuPgpuMetadata_t,
    compatibilityInfo: *mut cuda_types::nvml::nvmlVgpuPgpuCompatibility_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuMetadata), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuMetadata,
        "nvmlGetVgpuCompatibility",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pgpuMetadata), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pgpuMetadata,
        "nvmlGetVgpuCompatibility",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(compatibilityInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &compatibilityInfo,
        "nvmlGetVgpuCompatibility",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPgpuMetadataString(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pgpuMetadata: *mut ::core::ffi::c_char,
    bufferSize: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetPgpuMetadataString",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pgpuMetadata), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pgpuMetadata,
        "nvmlDeviceGetPgpuMetadataString",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(bufferSize), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &bufferSize,
        "nvmlDeviceGetPgpuMetadataString",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuSchedulerLog(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pSchedulerLog: *mut cuda_types::nvml::nvmlVgpuSchedulerLog_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuSchedulerLog",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSchedulerLog), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSchedulerLog,
        "nvmlDeviceGetVgpuSchedulerLog",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuSchedulerState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pSchedulerState: *mut cuda_types::nvml::nvmlVgpuSchedulerGetState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSchedulerState), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSchedulerState,
        "nvmlDeviceGetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuSchedulerCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pCapabilities: *mut cuda_types::nvml::nvmlVgpuSchedulerCapabilities_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuSchedulerCapabilities",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pCapabilities), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pCapabilities,
        "nvmlDeviceGetVgpuSchedulerCapabilities",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceSetVgpuSchedulerState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pSchedulerState: *mut cuda_types::nvml::nvmlVgpuSchedulerSetState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceSetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pSchedulerState), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pSchedulerState,
        "nvmlDeviceSetVgpuSchedulerState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGetVgpuVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    supported: *mut cuda_types::nvml::nvmlVgpuVersion_t,
    current: *mut cuda_types::nvml::nvmlVgpuVersion_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(supported), ": ").as_bytes())?;
    crate::CudaDisplay::write(&supported, "nvmlGetVgpuVersion", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(&current, "nvmlGetVgpuVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlSetVgpuVersion(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuVersion: *mut cuda_types::nvml::nvmlVgpuVersion_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuVersion), ": ").as_bytes())?;
    crate::CudaDisplay::write(&vgpuVersion, "nvmlSetVgpuVersion", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    sampleValType: *mut cuda_types::nvml::nvmlValueType_t,
    vgpuInstanceSamplesCount: *mut ::core::ffi::c_uint,
    utilizationSamples: *mut cuda_types::nvml::nvmlVgpuInstanceUtilizationSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetVgpuUtilization", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lastSeenTimeStamp,
        "nvmlDeviceGetVgpuUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(sampleValType), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &sampleValType,
        "nvmlDeviceGetVgpuUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuInstanceSamplesCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstanceSamplesCount,
        "nvmlDeviceGetVgpuUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilizationSamples), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilizationSamples,
        "nvmlDeviceGetVgpuUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuInstancesUtilizationInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuUtilInfo: *mut cuda_types::nvml::nvmlVgpuInstancesUtilizationInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuInstancesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuUtilInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuUtilInfo,
        "nvmlDeviceGetVgpuInstancesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuProcessUtilization(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    vgpuProcessSamplesCount: *mut ::core::ffi::c_uint,
    utilizationSamples: *mut cuda_types::nvml::nvmlVgpuProcessUtilizationSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(lastSeenTimeStamp), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &lastSeenTimeStamp,
        "nvmlDeviceGetVgpuProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuProcessSamplesCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuProcessSamplesCount,
        "nvmlDeviceGetVgpuProcessUtilization",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(utilizationSamples), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &utilizationSamples,
        "nvmlDeviceGetVgpuProcessUtilization",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetVgpuProcessesUtilizationInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    vgpuProcUtilInfo: *mut cuda_types::nvml::nvmlVgpuProcessesUtilizationInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetVgpuProcessesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(vgpuProcUtilInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuProcUtilInfo,
        "nvmlDeviceGetVgpuProcessesUtilizationInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetAccountingMode(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    mode: *mut cuda_types::nvml::nvmlEnableState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetAccountingMode",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &mode,
        "nvmlVgpuInstanceGetAccountingMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetAccountingPids(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    count: *mut ::core::ffi::c_uint,
    pids: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetAccountingPids",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlVgpuInstanceGetAccountingPids",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pids), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pids,
        "nvmlVgpuInstanceGetAccountingPids",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetAccountingStats(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    pid: ::core::ffi::c_uint,
    stats: *mut cuda_types::nvml::nvmlAccountingStats_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetAccountingStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pid), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pid,
        "nvmlVgpuInstanceGetAccountingStats",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(stats), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &stats,
        "nvmlVgpuInstanceGetAccountingStats",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceClearAccountingPids(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceClearAccountingPids",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetLicenseInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    licenseInfo: *mut cuda_types::nvml::nvmlVgpuLicenseInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetLicenseInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(licenseInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &licenseInfo,
        "nvmlVgpuInstanceGetLicenseInfo_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlExcludedDeviceInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(pciInfo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.pciInfo, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(uuid), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.uuid, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlGetExcludedDeviceCount(
    writer: &mut (impl std::io::Write + ?Sized),
    deviceCount: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(deviceCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &deviceCount,
        "nvmlGetExcludedDeviceCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGetExcludedDeviceInfoByIndex(
    writer: &mut (impl std::io::Write + ?Sized),
    index: ::core::ffi::c_uint,
    info: *mut cuda_types::nvml::nvmlExcludedDeviceInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &index,
        "nvmlGetExcludedDeviceInfoByIndex",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlGetExcludedDeviceInfoByIndex",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstancePlacement_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(start), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.start, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstanceProfileInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(isP2pSupported), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isP2pSupported, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(copyEngineCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.copyEngineCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(jpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.jpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ofaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ofaCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memorySizeMB), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memorySizeMB, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstanceProfileInfo_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(isP2pSupported), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isP2pSupported, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(copyEngineCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.copyEngineCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(jpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.jpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ofaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ofaCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memorySizeMB), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memorySizeMB, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstanceProfileInfo_v3_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(copyEngineCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.copyEngineCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(decoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.decoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(encoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.encoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(jpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.jpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(ofaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.ofaCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(memorySizeMB), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.memorySizeMB, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(capabilities), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.capabilities, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpuInstanceInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(device), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.device, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(profileId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.profileId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placement), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placement, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstancePlacement_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(start), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.start, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(size), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.size, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstanceProfileInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(sharedCopyEngineCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.sharedCopyEngineCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedDecoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedDecoderCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedEncoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedEncoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedJpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedJpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedOfaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedOfaCount, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstanceProfileInfo_v2_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(sharedCopyEngineCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.sharedCopyEngineCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedDecoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedDecoderCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedEncoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedEncoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedJpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedJpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedOfaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedOfaCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstanceProfileInfo_v3_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sliceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sliceCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(instanceCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.instanceCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(multiprocessorCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.multiprocessorCount, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(sharedCopyEngineCount), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.sharedCopyEngineCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedDecoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedDecoderCount, "", 0, writer)?;
        writer
            .write_all(concat!(", ", stringify!(sharedEncoderCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedEncoderCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedJpegCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedJpegCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sharedOfaCount), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sharedOfaCount, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(name), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.name, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(capabilities), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.capabilities, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstanceInfo_st {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(device), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.device, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(gpuInstance), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.gpuInstance, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(id), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.id, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(profileId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.profileId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(placement), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.placement, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlComputeInstance_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
pub fn write_nvmlDeviceSetMigMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    mode: ::core::ffi::c_uint,
    activationStatus: *mut cuda_types::nvml::nvmlReturn_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceSetMigMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(mode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&mode, "nvmlDeviceSetMigMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(activationStatus), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &activationStatus,
        "nvmlDeviceSetMigMode",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMigMode(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    currentMode: *mut ::core::ffi::c_uint,
    pendingMode: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetMigMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currentMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&currentMode, "nvmlDeviceGetMigMode", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pendingMode), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pendingMode, "nvmlDeviceGetMigMode", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstanceProfileInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profile: ::core::ffi::c_uint,
    info: *mut cuda_types::nvml::nvmlGpuInstanceProfileInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlDeviceGetGpuInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlDeviceGetGpuInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstanceProfileInfoV(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profile: ::core::ffi::c_uint,
    info: *mut cuda_types::nvml::nvmlGpuInstanceProfileInfo_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlDeviceGetGpuInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlDeviceGetGpuInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstancePossiblePlacements_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    placements: *mut cuda_types::nvml::nvmlGpuInstancePlacement_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuInstancePossiblePlacements_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlDeviceGetGpuInstancePossiblePlacements_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(placements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &placements,
        "nvmlDeviceGetGpuInstancePossiblePlacements_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetGpuInstancePossiblePlacements_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstanceRemainingCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlDeviceGetGpuInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetGpuInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceCreateGpuInstance(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceCreateGpuInstance", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlDeviceCreateGpuInstance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlDeviceCreateGpuInstance",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceCreateGpuInstanceWithPlacement(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    placement: *const cuda_types::nvml::nvmlGpuInstancePlacement_t,
    gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceCreateGpuInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlDeviceCreateGpuInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(placement), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &placement,
        "nvmlDeviceCreateGpuInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlDeviceCreateGpuInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpuInstance, "nvmlGpuInstanceDestroy", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstances(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    gpuInstances: *mut cuda_types::nvml::nvmlGpuInstance_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpuInstances", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&profileId, "nvmlDeviceGetGpuInstances", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstances), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstances,
        "nvmlDeviceGetGpuInstances",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(&count, "nvmlDeviceGetGpuInstances", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstanceById(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    id: ::core::ffi::c_uint,
    gpuInstance: *mut cuda_types::nvml::nvmlGpuInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpuInstanceById", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id, "nvmlDeviceGetGpuInstanceById", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlDeviceGetGpuInstanceById",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    info: *mut cuda_types::nvml::nvmlGpuInstanceInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpuInstance, "nvmlGpuInstanceGetInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlGpuInstanceGetInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstanceProfileInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profile: ::core::ffi::c_uint,
    engProfile: ::core::ffi::c_uint,
    info: *mut cuda_types::nvml::nvmlComputeInstanceProfileInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlGpuInstanceGetComputeInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(engProfile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &engProfile,
        "nvmlGpuInstanceGetComputeInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlGpuInstanceGetComputeInstanceProfileInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstanceProfileInfoV(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profile: ::core::ffi::c_uint,
    engProfile: ::core::ffi::c_uint,
    info: *mut cuda_types::nvml::nvmlComputeInstanceProfileInfo_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlGpuInstanceGetComputeInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(engProfile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &engProfile,
        "nvmlGpuInstanceGetComputeInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &info,
        "nvmlGpuInstanceGetComputeInstanceProfileInfoV",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstanceRemainingCapacity(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profileId: ::core::ffi::c_uint,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlGpuInstanceGetComputeInstanceRemainingCapacity",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstancePossiblePlacements(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profileId: ::core::ffi::c_uint,
    placements: *mut cuda_types::nvml::nvmlComputeInstancePlacement_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(placements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &placements,
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlGpuInstanceGetComputeInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceCreateComputeInstance(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profileId: ::core::ffi::c_uint,
    computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceCreateComputeInstance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlGpuInstanceCreateComputeInstance",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlGpuInstanceCreateComputeInstance",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceCreateComputeInstanceWithPlacement(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profileId: ::core::ffi::c_uint,
    placement: *const cuda_types::nvml::nvmlComputeInstancePlacement_t,
    computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceCreateComputeInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlGpuInstanceCreateComputeInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(placement), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &placement,
        "nvmlGpuInstanceCreateComputeInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlGpuInstanceCreateComputeInstanceWithPlacement",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlComputeInstanceDestroy(
    writer: &mut (impl std::io::Write + ?Sized),
    computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlComputeInstanceDestroy",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstances(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    profileId: ::core::ffi::c_uint,
    computeInstances: *mut cuda_types::nvml::nvmlComputeInstance_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstances",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlGpuInstanceGetComputeInstances",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeInstances), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstances,
        "nvmlGpuInstanceGetComputeInstances",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlGpuInstanceGetComputeInstances",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpuInstanceGetComputeInstanceById(
    writer: &mut (impl std::io::Write + ?Sized),
    gpuInstance: cuda_types::nvml::nvmlGpuInstance_t,
    id: ::core::ffi::c_uint,
    computeInstance: *mut cuda_types::nvml::nvmlComputeInstance_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpuInstance,
        "nvmlGpuInstanceGetComputeInstanceById",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &id,
        "nvmlGpuInstanceGetComputeInstanceById",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlGpuInstanceGetComputeInstanceById",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlComputeInstanceGetInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
    info: *mut cuda_types::nvml::nvmlComputeInstanceInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlComputeInstanceGetInfo_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlComputeInstanceGetInfo_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceIsMigDeviceHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    isMigDevice: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceIsMigDeviceHandle", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(isMigDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &isMigDevice,
        "nvmlDeviceIsMigDeviceHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstanceId(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    id: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetGpuInstanceId", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id, "nvmlDeviceGetGpuInstanceId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetComputeInstanceId(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    id: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetComputeInstanceId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(id), ": ").as_bytes())?;
    crate::CudaDisplay::write(&id, "nvmlDeviceGetComputeInstanceId", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMaxMigDeviceCount(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMaxMigDeviceCount",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetMaxMigDeviceCount",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMigDeviceHandleByIndex(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    index: ::core::ffi::c_uint,
    migDevice: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMigDeviceHandleByIndex",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &index,
        "nvmlDeviceGetMigDeviceHandleByIndex",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(migDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &migDevice,
        "nvmlDeviceGetMigDeviceHandleByIndex",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDeviceHandleFromMigDeviceHandle(
    writer: &mut (impl std::io::Write + ?Sized),
    migDevice: cuda_types::nvml::nvmlDevice_t,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(migDevice), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &migDevice,
        "nvmlDeviceGetDeviceHandleFromMigDeviceHandle",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetDeviceHandleFromMigDeviceHandle",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmMetricId_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GRAPHICS_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_GRAPHICS_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_SM_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_SM_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_SM_OCCUPANCY => {
                writer.write_all(stringify!(NVML_GPM_METRIC_SM_OCCUPANCY).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_INTEGER_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_INTEGER_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_ANY_TENSOR_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_ANY_TENSOR_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_DFMA_TENSOR_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_DFMA_TENSOR_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_HMMA_TENSOR_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_HMMA_TENSOR_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_IMMA_TENSOR_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_IMMA_TENSOR_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_DRAM_BW_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_DRAM_BW_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_FP64_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_FP64_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_FP32_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_FP32_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_FP16_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_FP16_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_PCIE_TX_PER_SEC => {
                writer.write_all(stringify!(NVML_GPM_METRIC_PCIE_TX_PER_SEC).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_PCIE_RX_PER_SEC => {
                writer.write_all(stringify!(NVML_GPM_METRIC_PCIE_RX_PER_SEC).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_0_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_0_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_1_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_1_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_2_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_2_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_3_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_3_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_4_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_4_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_5_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_5_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_6_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_6_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVDEC_7_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVDEC_7_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_0_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_0_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_1_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_1_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_2_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_2_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_3_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_3_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_4_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_4_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_5_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_5_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_6_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_6_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVJPG_7_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVJPG_7_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVOFA_0_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVOFA_0_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVOFA_1_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVOFA_1_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK0_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK0_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK0_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK0_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK0_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK0_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK0_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK0_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK1_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK1_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK1_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK1_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK1_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK1_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK1_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK1_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK2_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK2_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK2_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK2_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK2_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK2_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK2_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK2_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK3_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK3_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK3_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK3_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK3_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK3_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK3_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK3_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK4_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK4_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK4_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK4_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK4_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK4_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK4_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK4_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK5_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK5_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK5_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK5_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK5_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK5_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK5_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK5_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK6_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK6_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK6_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK6_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK6_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK6_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK6_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK6_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK7_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK7_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK7_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK7_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK7_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK7_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK7_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK7_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK8_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK8_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK8_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK8_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK8_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK8_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK8_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK8_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK9_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK9_TOTAL_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK9_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK9_TOTAL_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK9_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK9_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK9_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK9_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK10_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK10_TOTAL_TX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK10_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK10_TOTAL_RX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK10_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK10_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK10_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK10_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK11_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK11_TOTAL_TX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK11_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK11_TOTAL_RX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK11_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK11_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK11_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK11_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK12_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK12_TOTAL_TX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK12_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK12_TOTAL_RX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK12_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK12_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK12_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK12_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK13_TOTAL_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK13_TOTAL_TX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK13_TOTAL_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK13_TOTAL_RX_PER_SEC)
                            .as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK13_DATA_TX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK13_DATA_TX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_C2C_LINK13_DATA_RX_PER_SEC => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_C2C_LINK13_DATA_RX_PER_SEC).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_HOSTMEM_CACHE_HIT => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_HOSTMEM_CACHE_HIT).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_HOSTMEM_CACHE_MISS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_HOSTMEM_CACHE_MISS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_PEERMEM_CACHE_HIT => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_PEERMEM_CACHE_HIT).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_PEERMEM_CACHE_MISS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_PEERMEM_CACHE_MISS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_DRAM_CACHE_HIT => {
                writer.write_all(stringify!(NVML_GPM_METRIC_DRAM_CACHE_HIT).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_DRAM_CACHE_MISS => {
                writer.write_all(stringify!(NVML_GPM_METRIC_DRAM_CACHE_MISS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVENC_0_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVENC_0_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVENC_1_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVENC_1_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVENC_2_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVENC_2_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_NVENC_3_UTIL => {
                writer.write_all(stringify!(NVML_GPM_METRIC_NVENC_3_UTIL).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR0_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR0_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR0_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR0_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR0_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR0_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR0_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR1_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR1_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR1_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR1_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR1_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR1_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR1_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR2_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR2_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR2_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR2_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR2_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR2_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR2_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR3_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR3_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR3_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR3_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR3_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR3_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR3_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR4_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR4_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR4_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR4_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR4_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR4_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR4_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR5_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR5_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR5_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR5_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR5_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR5_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR5_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR6_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR6_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR6_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR6_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR6_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR6_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR6_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ELAPSED => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ELAPSED).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ACTIVE => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR7_CTXSW_CYCLES_ACTIVE).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR7_CTXSW_REQUESTS => {
                writer
                    .write_all(stringify!(NVML_GPM_METRIC_GR7_CTXSW_REQUESTS).as_bytes())
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR7_CTXSW_CYCLES_PER_REQ => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR7_CTXSW_CYCLES_PER_REQ).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_GR7_CTXSW_ACTIVE_PCT => {
                writer
                    .write_all(
                        stringify!(NVML_GPM_METRIC_GR7_CTXSW_ACTIVE_PCT).as_bytes(),
                    )
            }
            &cuda_types::nvml::nvmlGpmMetricId_t::NVML_GPM_METRIC_MAX => {
                writer.write_all(stringify!(NVML_GPM_METRIC_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmSample_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        if self.is_null() {
            writer.write_all(b"NULL")
        } else {
            write!(writer, "{:p}", *self)
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmMetric_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(metricId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.metricId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(nvmlReturn), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.nvmlReturn, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(metricInfo), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.metricInfo, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmMetric_t__bindgen_ty_1 {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(shortName), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.shortName, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(longName), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.longName, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(unit), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.unit, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmMetricsGet_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(numMetrics), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.numMetrics, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sample1), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sample1, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(sample2), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.sample2, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(metrics), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.metrics, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlGpmSupport_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(isSupportedDevice), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.isSupportedDevice, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlGpmMetricsGet(
    writer: &mut (impl std::io::Write + ?Sized),
    metricsGet: *mut cuda_types::nvml::nvmlGpmMetricsGet_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(metricsGet), ": ").as_bytes())?;
    crate::CudaDisplay::write(&metricsGet, "nvmlGpmMetricsGet", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmSampleFree(
    writer: &mut (impl std::io::Write + ?Sized),
    gpmSample: cuda_types::nvml::nvmlGpmSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpmSample), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpmSample, "nvmlGpmSampleFree", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmSampleAlloc(
    writer: &mut (impl std::io::Write + ?Sized),
    gpmSample: *mut cuda_types::nvml::nvmlGpmSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(gpmSample), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpmSample, "nvmlGpmSampleAlloc", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmSampleGet(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpmSample: cuda_types::nvml::nvmlGpmSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlGpmSampleGet", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpmSample), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpmSample, "nvmlGpmSampleGet", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmMigSampleGet(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpuInstanceId: ::core::ffi::c_uint,
    gpmSample: cuda_types::nvml::nvmlGpmSample_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlGpmMigSampleGet", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpuInstanceId), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpuInstanceId, "nvmlGpmMigSampleGet", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpmSample), ": ").as_bytes())?;
    crate::CudaDisplay::write(&gpmSample, "nvmlGpmMigSampleGet", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmQueryDeviceSupport(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    gpmSupport: *mut cuda_types::nvml::nvmlGpmSupport_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlGpmQueryDeviceSupport", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(gpmSupport), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &gpmSupport,
        "nvmlGpmQueryDeviceSupport",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmQueryIfStreamingEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    state: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlGpmQueryIfStreamingEnabled",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &state,
        "nvmlGpmQueryIfStreamingEnabled",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlGpmSetStreamingEnabled(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    state: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlGpmSetStreamingEnabled", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(&state, "nvmlGpmSetStreamingEnabled", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlDeviceCapabilities_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(capMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.capMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlDeviceGetCapabilities(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    caps: *mut cuda_types::nvml::nvmlDeviceCapabilities_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetCapabilities", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(caps), ": ").as_bytes())?;
    crate::CudaDisplay::write(&caps, "nvmlDeviceGetCapabilities", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlMask255_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(mask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.mask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPowerProfileType_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_MAX_P => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_MAX_P).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_MAX_Q => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_MAX_Q).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_COMPUTE => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_COMPUTE).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_MEMORY_BOUND => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_MEMORY_BOUND).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_NETWORK => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_NETWORK).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_BALANCED => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_BALANCED).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_LLM_INFERENCE => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_LLM_INFERENCE).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_LLM_TRAINING => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_LLM_TRAINING).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_RBM => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_RBM).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_DCPCIE => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_DCPCIE).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_HMMA_SPARSE => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_HMMA_SPARSE).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_HMMA_DENSE => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_HMMA_DENSE).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_SYNC_BALANCED => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_SYNC_BALANCED).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_HPC => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_HPC).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_MIG => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_MIG).as_bytes())
            }
            &cuda_types::nvml::nvmlPowerProfileType_t::NVML_POWER_PROFILE_MAX => {
                writer.write_all(stringify!(NVML_POWER_PROFILE_MAX).as_bytes())
            }
            _ => write!(writer, "{}", self.0),
        }
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlWorkloadPowerProfileInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(profileId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.profileId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(priority), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.priority, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(conflictingMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.conflictingMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlWorkloadPowerProfileProfilesInfo_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(perfProfilesMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.perfProfilesMask, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(perfProfile), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.perfProfile, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlWorkloadPowerProfileCurrentProfiles_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(perfProfilesMask), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.perfProfilesMask, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(requestedProfilesMask), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.requestedProfilesMask, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(enforcedProfilesMask), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.enforcedProfilesMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay
for cuda_types::nvml::nvmlWorkloadPowerProfileRequestedProfiles_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer
            .write_all(
                concat!(", ", stringify!(requestedProfilesMask), ": ").as_bytes(),
            )?;
        crate::CudaDisplay::write(&self.requestedProfilesMask, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlDeviceWorkloadPowerProfileGetProfilesInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profilesInfo: *mut cuda_types::nvml::nvmlWorkloadPowerProfileProfilesInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceWorkloadPowerProfileGetProfilesInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profilesInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profilesInfo,
        "nvmlDeviceWorkloadPowerProfileGetProfilesInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceWorkloadPowerProfileGetCurrentProfiles(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    currentProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileCurrentProfiles_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceWorkloadPowerProfileGetCurrentProfiles",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(currentProfiles), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &currentProfiles,
        "nvmlDeviceWorkloadPowerProfileGetCurrentProfiles",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceWorkloadPowerProfileSetRequestedProfiles(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    requestedProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceWorkloadPowerProfileSetRequestedProfiles",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedProfiles), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedProfiles,
        "nvmlDeviceWorkloadPowerProfileSetRequestedProfiles",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceWorkloadPowerProfileClearRequestedProfiles(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    requestedProfiles: *mut cuda_types::nvml::nvmlWorkloadPowerProfileRequestedProfiles_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceWorkloadPowerProfileClearRequestedProfiles",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(requestedProfiles), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &requestedProfiles,
        "nvmlDeviceWorkloadPowerProfileClearRequestedProfiles",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPowerSmoothingProfile_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(profileId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.profileId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(paramId), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.paramId, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(value), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.value, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlPowerSmoothingState_v1_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        writer.write_all(concat!("{ ", stringify!(version), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.version, "", 0, writer)?;
        writer.write_all(concat!(", ", stringify!(state), ": ").as_bytes())?;
        crate::CudaDisplay::write(&self.state, "", 0, writer)?;
        writer.write_all(b" }")
    }
}
pub fn write_nvmlDevicePowerSmoothingActivatePresetProfile(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profile: *mut cuda_types::nvml::nvmlPowerSmoothingProfile_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDevicePowerSmoothingActivatePresetProfile",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlDevicePowerSmoothingActivatePresetProfile",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDevicePowerSmoothingUpdatePresetProfileParam(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profile: *mut cuda_types::nvml::nvmlPowerSmoothingProfile_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDevicePowerSmoothingUpdatePresetProfileParam",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profile), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profile,
        "nvmlDevicePowerSmoothingUpdatePresetProfileParam",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDevicePowerSmoothingSetState(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    state: *mut cuda_types::nvml::nvmlPowerSmoothingState_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDevicePowerSmoothingSetState",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(state), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &state,
        "nvmlDevicePowerSmoothingSetState",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlInit(
    writer: &mut (impl std::io::Write + ?Sized),
) -> std::io::Result<()> {
    writer.write_all(b"()")
}
pub fn write_nvmlDeviceGetCount(
    writer: &mut (impl std::io::Write + ?Sized),
    deviceCount: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(deviceCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(&deviceCount, "nvmlDeviceGetCount", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByIndex(
    writer: &mut (impl std::io::Write + ?Sized),
    index: ::core::ffi::c_uint,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(index), ": ").as_bytes())?;
    crate::CudaDisplay::write(&index, "nvmlDeviceGetHandleByIndex", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetHandleByIndex", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetHandleByPciBusId(
    writer: &mut (impl std::io::Write + ?Sized),
    pciBusId: *const ::core::ffi::c_char,
    device: *mut cuda_types::nvml::nvmlDevice_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciBusId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pciBusId,
        "nvmlDeviceGetHandleByPciBusId",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetHandleByPciBusId",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPciInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pci: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPciInfo", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pci, "nvmlDeviceGetPciInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetPciInfo_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pci: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetPciInfo_v2", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pci, "nvmlDeviceGetPciInfo_v2", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetNvLinkRemotePciInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    link: ::core::ffi::c_uint,
    pci: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetNvLinkRemotePciInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(link), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &link,
        "nvmlDeviceGetNvLinkRemotePciInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pci), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pci,
        "nvmlDeviceGetNvLinkRemotePciInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGridLicensableFeatures(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGridLicensableFeatures",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGridLicensableFeatures), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pGridLicensableFeatures,
        "nvmlDeviceGetGridLicensableFeatures",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGridLicensableFeatures_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGridLicensableFeatures_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGridLicensableFeatures), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pGridLicensableFeatures,
        "nvmlDeviceGetGridLicensableFeatures_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGridLicensableFeatures_v3(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    pGridLicensableFeatures: *mut cuda_types::nvml::nvmlGridLicensableFeatures_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGridLicensableFeatures_v3",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pGridLicensableFeatures), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &pGridLicensableFeatures,
        "nvmlDeviceGetGridLicensableFeatures_v3",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceRemoveGpu(
    writer: &mut (impl std::io::Write + ?Sized),
    pciInfo: *mut cuda_types::nvml::nvmlPciInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(pciInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pciInfo, "nvmlDeviceRemoveGpu", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlEventSetWait(
    writer: &mut (impl std::io::Write + ?Sized),
    set: cuda_types::nvml::nvmlEventSet_t,
    data: *mut cuda_types::nvml::nvmlEventData_t,
    timeoutms: ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(set), ": ").as_bytes())?;
    crate::CudaDisplay::write(&set, "nvmlEventSetWait", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(data), ": ").as_bytes())?;
    crate::CudaDisplay::write(&data, "nvmlEventSetWait", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(timeoutms), ": ").as_bytes())?;
    crate::CudaDisplay::write(&timeoutms, "nvmlEventSetWait", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetAttributes(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    attributes: *mut cuda_types::nvml::nvmlDeviceAttributes_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetAttributes", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(attributes), ": ").as_bytes())?;
    crate::CudaDisplay::write(&attributes, "nvmlDeviceGetAttributes", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlComputeInstanceGetInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    computeInstance: cuda_types::nvml::nvmlComputeInstance_t,
    info: *mut cuda_types::nvml::nvmlComputeInstanceInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(computeInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &computeInstance,
        "nvmlComputeInstanceGetInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(info), ": ").as_bytes())?;
    crate::CudaDisplay::write(&info, "nvmlComputeInstanceGetInfo", arg_idx, writer)?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetComputeRunningProcesses(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetComputeRunningProcesses_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGraphicsRunningProcesses(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGraphicsRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetGraphicsRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetGraphicsRunningProcesses",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGraphicsRunningProcesses_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGraphicsRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetGraphicsRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetGraphicsRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMPSComputeRunningProcesses(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v1_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMPSComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetMPSComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetMPSComputeRunningProcesses",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetMPSComputeRunningProcesses_v2(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    infoCount: *mut ::core::ffi::c_uint,
    infos: *mut cuda_types::nvml::nvmlProcessInfo_v2_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetMPSComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infoCount), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infoCount,
        "nvmlDeviceGetMPSComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(infos), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &infos,
        "nvmlDeviceGetMPSComputeRunningProcesses_v2",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetGpuInstancePossiblePlacements(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    profileId: ::core::ffi::c_uint,
    placements: *mut cuda_types::nvml::nvmlGpuInstancePlacement_t,
    count: *mut ::core::ffi::c_uint,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &device,
        "nvmlDeviceGetGpuInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(profileId), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &profileId,
        "nvmlDeviceGetGpuInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(placements), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &placements,
        "nvmlDeviceGetGpuInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(count), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &count,
        "nvmlDeviceGetGpuInstancePossiblePlacements",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlVgpuInstanceGetLicenseInfo(
    writer: &mut (impl std::io::Write + ?Sized),
    vgpuInstance: cuda_types::nvml::nvmlVgpuInstance_t,
    licenseInfo: *mut cuda_types::nvml::nvmlVgpuLicenseInfo_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(vgpuInstance), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &vgpuInstance,
        "nvmlVgpuInstanceGetLicenseInfo",
        arg_idx,
        writer,
    )?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(licenseInfo), ": ").as_bytes())?;
    crate::CudaDisplay::write(
        &licenseInfo,
        "nvmlVgpuInstanceGetLicenseInfo",
        arg_idx,
        writer,
    )?;
    writer.write_all(b")")
}
pub fn write_nvmlDeviceGetDriverModel(
    writer: &mut (impl std::io::Write + ?Sized),
    device: cuda_types::nvml::nvmlDevice_t,
    current: *mut cuda_types::nvml::nvmlDriverModel_t,
    pending: *mut cuda_types::nvml::nvmlDriverModel_t,
) -> std::io::Result<()> {
    let mut arg_idx = 0usize;
    writer.write_all(b"(")?;
    writer.write_all(concat!(stringify!(device), ": ").as_bytes())?;
    crate::CudaDisplay::write(&device, "nvmlDeviceGetDriverModel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(current), ": ").as_bytes())?;
    crate::CudaDisplay::write(&current, "nvmlDeviceGetDriverModel", arg_idx, writer)?;
    arg_idx += 1;
    writer.write_all(b", ")?;
    writer.write_all(concat!(stringify!(pending), ": ").as_bytes())?;
    crate::CudaDisplay::write(&pending, "nvmlDeviceGetDriverModel", arg_idx, writer)?;
    writer.write_all(b")")
}
impl crate::CudaDisplay for cuda_types::nvml::nvmlReturn_t {
    fn write(
        &self,
        _fn_name: &'static str,
        _index: usize,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> std::io::Result<()> {
        match self {
            Ok(()) => writer.write_all(b"NVML_SUCCESS"),
            Err(err) => {
                match err.0.get() {
                    1 => writer.write_all("NVML_ERROR_UNINITIALIZED".as_bytes()),
                    2 => writer.write_all("NVML_ERROR_INVALID_ARGUMENT".as_bytes()),
                    3 => writer.write_all("NVML_ERROR_NOT_SUPPORTED".as_bytes()),
                    4 => writer.write_all("NVML_ERROR_NO_PERMISSION".as_bytes()),
                    5 => writer.write_all("NVML_ERROR_ALREADY_INITIALIZED".as_bytes()),
                    6 => writer.write_all("NVML_ERROR_NOT_FOUND".as_bytes()),
                    7 => writer.write_all("NVML_ERROR_INSUFFICIENT_SIZE".as_bytes()),
                    8 => writer.write_all("NVML_ERROR_INSUFFICIENT_POWER".as_bytes()),
                    9 => writer.write_all("NVML_ERROR_DRIVER_NOT_LOADED".as_bytes()),
                    10 => writer.write_all("NVML_ERROR_TIMEOUT".as_bytes()),
                    11 => writer.write_all("NVML_ERROR_IRQ_ISSUE".as_bytes()),
                    12 => writer.write_all("NVML_ERROR_LIBRARY_NOT_FOUND".as_bytes()),
                    13 => writer.write_all("NVML_ERROR_FUNCTION_NOT_FOUND".as_bytes()),
                    14 => writer.write_all("NVML_ERROR_CORRUPTED_INFOROM".as_bytes()),
                    15 => writer.write_all("NVML_ERROR_GPU_IS_LOST".as_bytes()),
                    16 => writer.write_all("NVML_ERROR_RESET_REQUIRED".as_bytes()),
                    17 => writer.write_all("NVML_ERROR_OPERATING_SYSTEM".as_bytes()),
                    18 => {
                        writer.write_all("NVML_ERROR_LIB_RM_VERSION_MISMATCH".as_bytes())
                    }
                    19 => writer.write_all("NVML_ERROR_IN_USE".as_bytes()),
                    20 => writer.write_all("NVML_ERROR_MEMORY".as_bytes()),
                    21 => writer.write_all("NVML_ERROR_NO_DATA".as_bytes()),
                    22 => {
                        writer.write_all("NVML_ERROR_VGPU_ECC_NOT_SUPPORTED".as_bytes())
                    }
                    23 => {
                        writer.write_all("NVML_ERROR_INSUFFICIENT_RESOURCES".as_bytes())
                    }
                    24 => writer.write_all("NVML_ERROR_FREQ_NOT_SUPPORTED".as_bytes()),
                    25 => {
                        writer
                            .write_all("NVML_ERROR_ARGUMENT_VERSION_MISMATCH".as_bytes())
                    }
                    26 => writer.write_all("NVML_ERROR_DEPRECATED".as_bytes()),
                    27 => writer.write_all("NVML_ERROR_NOT_READY".as_bytes()),
                    28 => writer.write_all("NVML_ERROR_GPU_NOT_FOUND".as_bytes()),
                    29 => writer.write_all("NVML_ERROR_INVALID_STATE".as_bytes()),
                    999 => writer.write_all("NVML_ERROR_UNKNOWN".as_bytes()),
                    err => write!(writer, "{}", err),
                }
            }
        }
    }
}
