// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const NVML_API_VERSION: u32 = 12;
pub const NVML_API_VERSION_STR: &[u8; 3] = b"12\0";
pub const NVML_VALUE_NOT_AVAILABLE: i32 = -1;
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_SIZE: u32 = 32;
pub const NVML_DEVICE_PCI_BUS_ID_BUFFER_V2_SIZE: u32 = 16;
pub const NVML_DEVICE_PCI_BUS_ID_LEGACY_FMT: &[u8; 17] = b"%04X:%02X:%02X.0\0";
pub const NVML_DEVICE_PCI_BUS_ID_FMT: &[u8; 17] = b"%08X:%02X:%02X.0\0";
pub const NVML_NVLINK_MAX_LINKS: u32 = 18;
pub const NVML_MAX_PHYSICAL_BRIDGE: u32 = 128;
pub const NVML_MAX_THERMAL_SENSORS_PER_GPU: u32 = 3;
pub const NVML_MAX_GPU_PERF_PSTATES: u32 = 16;
pub const NVML_GRID_LICENSE_EXPIRY_NOT_AVAILABLE: u32 = 0;
pub const NVML_GRID_LICENSE_EXPIRY_INVALID: u32 = 1;
pub const NVML_GRID_LICENSE_EXPIRY_VALID: u32 = 2;
pub const NVML_GRID_LICENSE_EXPIRY_NOT_APPLICABLE: u32 = 3;
pub const NVML_GRID_LICENSE_EXPIRY_PERMANENT: u32 = 4;
pub const NVML_GRID_LICENSE_BUFFER_SIZE: u32 = 128;
pub const NVML_VGPU_NAME_BUFFER_SIZE: u32 = 64;
pub const NVML_GRID_LICENSE_FEATURE_MAX_COUNT: u32 = 3;
pub const NVML_INVALID_VGPU_PLACEMENT_ID: u32 = 65535;
pub const NVML_VGPU_VIRTUALIZATION_CAP_MIGRATION_NO: u32 = 0;
pub const NVML_VGPU_VIRTUALIZATION_CAP_MIGRATION_YES: u32 = 1;
pub const NVML_VGPU_PGPU_VIRTUALIZATION_CAP_MIGRATION_NO: u32 = 0;
pub const NVML_VGPU_PGPU_VIRTUALIZATION_CAP_MIGRATION_YES: u32 = 1;
pub const NVML_VGPU_SCHEDULER_POLICY_UNKNOWN: u32 = 0;
pub const NVML_VGPU_SCHEDULER_POLICY_BEST_EFFORT: u32 = 1;
pub const NVML_VGPU_SCHEDULER_POLICY_EQUAL_SHARE: u32 = 2;
pub const NVML_VGPU_SCHEDULER_POLICY_FIXED_SHARE: u32 = 3;
pub const NVML_SUPPORTED_VGPU_SCHEDULER_POLICY_COUNT: u32 = 3;
pub const NVML_SCHEDULER_SW_MAX_LOG_ENTRIES: u32 = 200;
pub const NVML_VGPU_SCHEDULER_ARR_DEFAULT: u32 = 0;
pub const NVML_VGPU_SCHEDULER_ARR_DISABLE: u32 = 1;
pub const NVML_VGPU_SCHEDULER_ARR_ENABLE: u32 = 2;
pub const NVML_GRID_LICENSE_STATE_UNKNOWN: u32 = 0;
pub const NVML_GRID_LICENSE_STATE_UNINITIALIZED: u32 = 1;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED_UNRESTRICTED: u32 = 2;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED_RESTRICTED: u32 = 3;
pub const NVML_GRID_LICENSE_STATE_UNLICENSED: u32 = 4;
pub const NVML_GRID_LICENSE_STATE_LICENSED: u32 = 5;
pub const NVML_GSP_FIRMWARE_VERSION_BUF_SIZE: u32 = 64;
pub const NVML_DEVICE_ARCH_KEPLER: u32 = 2;
pub const NVML_DEVICE_ARCH_MAXWELL: u32 = 3;
pub const NVML_DEVICE_ARCH_PASCAL: u32 = 4;
pub const NVML_DEVICE_ARCH_VOLTA: u32 = 5;
pub const NVML_DEVICE_ARCH_TURING: u32 = 6;
pub const NVML_DEVICE_ARCH_AMPERE: u32 = 7;
pub const NVML_DEVICE_ARCH_ADA: u32 = 8;
pub const NVML_DEVICE_ARCH_HOPPER: u32 = 9;
pub const NVML_DEVICE_ARCH_UNKNOWN: u32 = 4294967295;
pub const NVML_BUS_TYPE_UNKNOWN: u32 = 0;
pub const NVML_BUS_TYPE_PCI: u32 = 1;
pub const NVML_BUS_TYPE_PCIE: u32 = 2;
pub const NVML_BUS_TYPE_FPCI: u32 = 3;
pub const NVML_BUS_TYPE_AGP: u32 = 4;
pub const NVML_FAN_POLICY_TEMPERATURE_CONTINOUS_SW: u32 = 0;
pub const NVML_FAN_POLICY_MANUAL: u32 = 1;
pub const NVML_POWER_SOURCE_AC: u32 = 0;
pub const NVML_POWER_SOURCE_BATTERY: u32 = 1;
pub const NVML_POWER_SOURCE_UNDERSIZED: u32 = 2;
pub const NVML_PCIE_LINK_MAX_SPEED_INVALID: u32 = 0;
pub const NVML_PCIE_LINK_MAX_SPEED_2500MBPS: u32 = 1;
pub const NVML_PCIE_LINK_MAX_SPEED_5000MBPS: u32 = 2;
pub const NVML_PCIE_LINK_MAX_SPEED_8000MBPS: u32 = 3;
pub const NVML_PCIE_LINK_MAX_SPEED_16000MBPS: u32 = 4;
pub const NVML_PCIE_LINK_MAX_SPEED_32000MBPS: u32 = 5;
pub const NVML_PCIE_LINK_MAX_SPEED_64000MBPS: u32 = 6;
pub const NVML_ADAPTIVE_CLOCKING_INFO_STATUS_DISABLED: u32 = 0;
pub const NVML_ADAPTIVE_CLOCKING_INFO_STATUS_ENABLED: u32 = 1;
pub const NVML_MAX_GPU_UTILIZATIONS: u32 = 8;
pub const NVML_FI_DEV_ECC_CURRENT: u32 = 1;
pub const NVML_FI_DEV_ECC_PENDING: u32 = 2;
pub const NVML_FI_DEV_ECC_SBE_VOL_TOTAL: u32 = 3;
pub const NVML_FI_DEV_ECC_DBE_VOL_TOTAL: u32 = 4;
pub const NVML_FI_DEV_ECC_SBE_AGG_TOTAL: u32 = 5;
pub const NVML_FI_DEV_ECC_DBE_AGG_TOTAL: u32 = 6;
pub const NVML_FI_DEV_ECC_SBE_VOL_L1: u32 = 7;
pub const NVML_FI_DEV_ECC_DBE_VOL_L1: u32 = 8;
pub const NVML_FI_DEV_ECC_SBE_VOL_L2: u32 = 9;
pub const NVML_FI_DEV_ECC_DBE_VOL_L2: u32 = 10;
pub const NVML_FI_DEV_ECC_SBE_VOL_DEV: u32 = 11;
pub const NVML_FI_DEV_ECC_DBE_VOL_DEV: u32 = 12;
pub const NVML_FI_DEV_ECC_SBE_VOL_REG: u32 = 13;
pub const NVML_FI_DEV_ECC_DBE_VOL_REG: u32 = 14;
pub const NVML_FI_DEV_ECC_SBE_VOL_TEX: u32 = 15;
pub const NVML_FI_DEV_ECC_DBE_VOL_TEX: u32 = 16;
pub const NVML_FI_DEV_ECC_DBE_VOL_CBU: u32 = 17;
pub const NVML_FI_DEV_ECC_SBE_AGG_L1: u32 = 18;
pub const NVML_FI_DEV_ECC_DBE_AGG_L1: u32 = 19;
pub const NVML_FI_DEV_ECC_SBE_AGG_L2: u32 = 20;
pub const NVML_FI_DEV_ECC_DBE_AGG_L2: u32 = 21;
pub const NVML_FI_DEV_ECC_SBE_AGG_DEV: u32 = 22;
pub const NVML_FI_DEV_ECC_DBE_AGG_DEV: u32 = 23;
pub const NVML_FI_DEV_ECC_SBE_AGG_REG: u32 = 24;
pub const NVML_FI_DEV_ECC_DBE_AGG_REG: u32 = 25;
pub const NVML_FI_DEV_ECC_SBE_AGG_TEX: u32 = 26;
pub const NVML_FI_DEV_ECC_DBE_AGG_TEX: u32 = 27;
pub const NVML_FI_DEV_ECC_DBE_AGG_CBU: u32 = 28;
pub const NVML_FI_DEV_RETIRED_SBE: u32 = 29;
pub const NVML_FI_DEV_RETIRED_DBE: u32 = 30;
pub const NVML_FI_DEV_RETIRED_PENDING: u32 = 31;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L0: u32 = 32;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L1: u32 = 33;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L2: u32 = 34;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L3: u32 = 35;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L4: u32 = 36;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L5: u32 = 37;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_TOTAL: u32 = 38;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L0: u32 = 39;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L1: u32 = 40;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L2: u32 = 41;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L3: u32 = 42;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L4: u32 = 43;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L5: u32 = 44;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_TOTAL: u32 = 45;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L0: u32 = 46;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L1: u32 = 47;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L2: u32 = 48;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L3: u32 = 49;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L4: u32 = 50;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L5: u32 = 51;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_TOTAL: u32 = 52;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L0: u32 = 53;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L1: u32 = 54;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L2: u32 = 55;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L3: u32 = 56;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L4: u32 = 57;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L5: u32 = 58;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_TOTAL: u32 = 59;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L0: u32 = 60;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L1: u32 = 61;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L2: u32 = 62;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L3: u32 = 63;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L4: u32 = 64;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L5: u32 = 65;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_TOTAL: u32 = 66;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L0: u32 = 67;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L1: u32 = 68;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L2: u32 = 69;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L3: u32 = 70;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L4: u32 = 71;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L5: u32 = 72;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_TOTAL: u32 = 73;
pub const NVML_FI_DEV_PERF_POLICY_POWER: u32 = 74;
pub const NVML_FI_DEV_PERF_POLICY_THERMAL: u32 = 75;
pub const NVML_FI_DEV_PERF_POLICY_SYNC_BOOST: u32 = 76;
pub const NVML_FI_DEV_PERF_POLICY_BOARD_LIMIT: u32 = 77;
pub const NVML_FI_DEV_PERF_POLICY_LOW_UTILIZATION: u32 = 78;
pub const NVML_FI_DEV_PERF_POLICY_RELIABILITY: u32 = 79;
pub const NVML_FI_DEV_PERF_POLICY_TOTAL_APP_CLOCKS: u32 = 80;
pub const NVML_FI_DEV_PERF_POLICY_TOTAL_BASE_CLOCKS: u32 = 81;
pub const NVML_FI_DEV_MEMORY_TEMP: u32 = 82;
pub const NVML_FI_DEV_TOTAL_ENERGY_CONSUMPTION: u32 = 83;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L0: u32 = 84;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L1: u32 = 85;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L2: u32 = 86;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L3: u32 = 87;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L4: u32 = 88;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L5: u32 = 89;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_COMMON: u32 = 90;
pub const NVML_FI_DEV_NVLINK_LINK_COUNT: u32 = 91;
pub const NVML_FI_DEV_RETIRED_PENDING_SBE: u32 = 92;
pub const NVML_FI_DEV_RETIRED_PENDING_DBE: u32 = 93;
pub const NVML_FI_DEV_PCIE_REPLAY_COUNTER: u32 = 94;
pub const NVML_FI_DEV_PCIE_REPLAY_ROLLOVER_COUNTER: u32 = 95;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L6: u32 = 96;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L7: u32 = 97;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L8: u32 = 98;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L9: u32 = 99;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L10: u32 = 100;
pub const NVML_FI_DEV_NVLINK_CRC_FLIT_ERROR_COUNT_L11: u32 = 101;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L6: u32 = 102;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L7: u32 = 103;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L8: u32 = 104;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L9: u32 = 105;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L10: u32 = 106;
pub const NVML_FI_DEV_NVLINK_CRC_DATA_ERROR_COUNT_L11: u32 = 107;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L6: u32 = 108;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L7: u32 = 109;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L8: u32 = 110;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L9: u32 = 111;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L10: u32 = 112;
pub const NVML_FI_DEV_NVLINK_REPLAY_ERROR_COUNT_L11: u32 = 113;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L6: u32 = 114;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L7: u32 = 115;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L8: u32 = 116;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L9: u32 = 117;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L10: u32 = 118;
pub const NVML_FI_DEV_NVLINK_RECOVERY_ERROR_COUNT_L11: u32 = 119;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L6: u32 = 120;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L7: u32 = 121;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L8: u32 = 122;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L9: u32 = 123;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L10: u32 = 124;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C0_L11: u32 = 125;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L6: u32 = 126;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L7: u32 = 127;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L8: u32 = 128;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L9: u32 = 129;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L10: u32 = 130;
pub const NVML_FI_DEV_NVLINK_BANDWIDTH_C1_L11: u32 = 131;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L6: u32 = 132;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L7: u32 = 133;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L8: u32 = 134;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L9: u32 = 135;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L10: u32 = 136;
pub const NVML_FI_DEV_NVLINK_SPEED_MBPS_L11: u32 = 137;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_DATA_TX: u32 = 138;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_DATA_RX: u32 = 139;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_RAW_TX: u32 = 140;
pub const NVML_FI_DEV_NVLINK_THROUGHPUT_RAW_RX: u32 = 141;
pub const NVML_FI_DEV_REMAPPED_COR: u32 = 142;
pub const NVML_FI_DEV_REMAPPED_UNC: u32 = 143;
pub const NVML_FI_DEV_REMAPPED_PENDING: u32 = 144;
pub const NVML_FI_DEV_REMAPPED_FAILURE: u32 = 145;
pub const NVML_FI_DEV_NVLINK_REMOTE_NVLINK_ID: u32 = 146;
pub const NVML_FI_DEV_NVSWITCH_CONNECTED_LINK_COUNT: u32 = 147;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L0: u32 = 148;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L1: u32 = 149;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L2: u32 = 150;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L3: u32 = 151;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L4: u32 = 152;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L5: u32 = 153;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L6: u32 = 154;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L7: u32 = 155;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L8: u32 = 156;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L9: u32 = 157;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L10: u32 = 158;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_L11: u32 = 159;
pub const NVML_FI_DEV_NVLINK_ECC_DATA_ERROR_COUNT_TOTAL: u32 = 160;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_REPLAY: u32 = 161;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_RECOVERY: u32 = 162;
pub const NVML_FI_DEV_NVLINK_ERROR_DL_CRC: u32 = 163;
pub const NVML_FI_DEV_NVLINK_GET_SPEED: u32 = 164;
pub const NVML_FI_DEV_NVLINK_GET_STATE: u32 = 165;
pub const NVML_FI_DEV_NVLINK_GET_VERSION: u32 = 166;
pub const NVML_FI_DEV_NVLINK_GET_POWER_STATE: u32 = 167;
pub const NVML_FI_DEV_NVLINK_GET_POWER_THRESHOLD: u32 = 168;
pub const NVML_FI_DEV_PCIE_L0_TO_RECOVERY_COUNTER: u32 = 169;
pub const NVML_FI_DEV_C2C_LINK_COUNT: u32 = 170;
pub const NVML_FI_DEV_C2C_LINK_GET_STATUS: u32 = 171;
pub const NVML_FI_DEV_C2C_LINK_GET_MAX_BW: u32 = 172;
pub const NVML_FI_DEV_PCIE_COUNT_CORRECTABLE_ERRORS: u32 = 173;
pub const NVML_FI_DEV_PCIE_COUNT_NAKS_RECEIVED: u32 = 174;
pub const NVML_FI_DEV_PCIE_COUNT_RECEIVER_ERROR: u32 = 175;
pub const NVML_FI_DEV_PCIE_COUNT_BAD_TLP: u32 = 176;
pub const NVML_FI_DEV_PCIE_COUNT_NAKS_SENT: u32 = 177;
pub const NVML_FI_DEV_PCIE_COUNT_BAD_DLLP: u32 = 178;
pub const NVML_FI_DEV_PCIE_COUNT_NON_FATAL_ERROR: u32 = 179;
pub const NVML_FI_DEV_PCIE_COUNT_FATAL_ERROR: u32 = 180;
pub const NVML_FI_DEV_PCIE_COUNT_UNSUPPORTED_REQ: u32 = 181;
pub const NVML_FI_DEV_PCIE_COUNT_LCRC_ERROR: u32 = 182;
pub const NVML_FI_DEV_PCIE_COUNT_LANE_ERROR: u32 = 183;
pub const NVML_FI_DEV_IS_RESETLESS_MIG_SUPPORTED: u32 = 184;
pub const NVML_FI_DEV_POWER_AVERAGE: u32 = 185;
pub const NVML_FI_DEV_POWER_INSTANT: u32 = 186;
pub const NVML_FI_DEV_POWER_MIN_LIMIT: u32 = 187;
pub const NVML_FI_DEV_POWER_MAX_LIMIT: u32 = 188;
pub const NVML_FI_DEV_POWER_DEFAULT_LIMIT: u32 = 189;
pub const NVML_FI_DEV_POWER_CURRENT_LIMIT: u32 = 190;
pub const NVML_FI_DEV_ENERGY: u32 = 191;
pub const NVML_FI_DEV_POWER_REQUESTED_LIMIT: u32 = 192;
pub const NVML_FI_DEV_TEMPERATURE_SHUTDOWN_TLIMIT: u32 = 193;
pub const NVML_FI_DEV_TEMPERATURE_SLOWDOWN_TLIMIT: u32 = 194;
pub const NVML_FI_DEV_TEMPERATURE_MEM_MAX_TLIMIT: u32 = 195;
pub const NVML_FI_DEV_TEMPERATURE_GPU_MAX_TLIMIT: u32 = 196;
pub const NVML_FI_DEV_IS_MIG_MODE_INDEPENDENT_MIG_QUERY_CAPABLE: u32 = 199;
pub const NVML_FI_MAX: u32 = 200;
pub const NVML_NVFBC_SESSION_FLAG_DIFFMAP_ENABLED: u32 = 1;
pub const NVML_NVFBC_SESSION_FLAG_CLASSIFICATIONMAP_ENABLED: u32 = 2;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_NO_WAIT: u32 = 4;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_INFINITE: u32 = 8;
pub const NVML_NVFBC_SESSION_FLAG_CAPTURE_WITH_WAIT_TIMEOUT: u32 = 16;
pub const NVML_CC_SYSTEM_CPU_CAPS_NONE: u32 = 0;
pub const NVML_CC_SYSTEM_CPU_CAPS_AMD_SEV: u32 = 1;
pub const NVML_CC_SYSTEM_CPU_CAPS_INTEL_TDX: u32 = 2;
pub const NVML_CC_SYSTEM_GPUS_CC_NOT_CAPABLE: u32 = 0;
pub const NVML_CC_SYSTEM_GPUS_CC_CAPABLE: u32 = 1;
pub const NVML_CC_SYSTEM_DEVTOOLS_MODE_OFF: u32 = 0;
pub const NVML_CC_SYSTEM_DEVTOOLS_MODE_ON: u32 = 1;
pub const NVML_CC_SYSTEM_ENVIRONMENT_UNAVAILABLE: u32 = 0;
pub const NVML_CC_SYSTEM_ENVIRONMENT_SIM: u32 = 1;
pub const NVML_CC_SYSTEM_ENVIRONMENT_PROD: u32 = 2;
pub const NVML_CC_SYSTEM_FEATURE_DISABLED: u32 = 0;
pub const NVML_CC_SYSTEM_FEATURE_ENABLED: u32 = 1;
pub const NVML_CC_SYSTEM_MULTIGPU_NONE: u32 = 0;
pub const NVML_CC_SYSTEM_MULTIGPU_PROTECTED_PCIE: u32 = 1;
pub const NVML_CC_ACCEPTING_CLIENT_REQUESTS_FALSE: u32 = 0;
pub const NVML_CC_ACCEPTING_CLIENT_REQUESTS_TRUE: u32 = 1;
pub const NVML_GPU_CERT_CHAIN_SIZE: u32 = 4096;
pub const NVML_GPU_ATTESTATION_CERT_CHAIN_SIZE: u32 = 5120;
pub const NVML_CC_GPU_CEC_NONCE_SIZE: u32 = 32;
pub const NVML_CC_GPU_ATTESTATION_REPORT_SIZE: u32 = 8192;
pub const NVML_CC_GPU_CEC_ATTESTATION_REPORT_SIZE: u32 = 4096;
pub const NVML_CC_CEC_ATTESTATION_REPORT_NOT_PRESENT: u32 = 0;
pub const NVML_CC_CEC_ATTESTATION_REPORT_PRESENT: u32 = 1;
pub const NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MIN: u32 = 50;
pub const NVML_CC_KEY_ROTATION_THRESHOLD_ATTACKER_ADVANTAGE_MAX: u32 = 75;
pub const NVML_GPU_FABRIC_UUID_LEN: u32 = 16;
pub const NVML_GPU_FABRIC_STATE_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_STATE_NOT_STARTED: u32 = 1;
pub const NVML_GPU_FABRIC_STATE_IN_PROGRESS: u32 = 2;
pub const NVML_GPU_FABRIC_STATE_COMPLETED: u32 = 3;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_NOT_SUPPORTED: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_TRUE: u32 = 1;
pub const NVML_GPU_FABRIC_HEALTH_MASK_DEGRADED_BW_FALSE: u32 = 2;
pub const NVML_GPU_FABRIC_HEALTH_MASK_SHIFT_DEGRADED_BW: u32 = 0;
pub const NVML_GPU_FABRIC_HEALTH_MASK_WIDTH_DEGRADED_BW: u32 = 17;
pub const NVML_POWER_SCOPE_GPU: u32 = 0;
pub const NVML_POWER_SCOPE_MODULE: u32 = 1;
pub const NVML_POWER_SCOPE_MEMORY: u32 = 2;
pub const NVML_INIT_FLAG_NO_GPUS: u32 = 1;
pub const NVML_INIT_FLAG_NO_ATTACH: u32 = 2;
pub const NVML_DEVICE_INFOROM_VERSION_BUFFER_SIZE: u32 = 16;
pub const NVML_DEVICE_UUID_BUFFER_SIZE: u32 = 80;
pub const NVML_DEVICE_UUID_V2_BUFFER_SIZE: u32 = 96;
pub const NVML_DEVICE_PART_NUMBER_BUFFER_SIZE: u32 = 80;
pub const NVML_SYSTEM_DRIVER_VERSION_BUFFER_SIZE: u32 = 80;
pub const NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE: u32 = 80;
pub const NVML_DEVICE_NAME_BUFFER_SIZE: u32 = 64;
pub const NVML_DEVICE_NAME_V2_BUFFER_SIZE: u32 = 96;
pub const NVML_DEVICE_SERIAL_BUFFER_SIZE: u32 = 30;
pub const NVML_DEVICE_VBIOS_VERSION_BUFFER_SIZE: u32 = 32;
pub const NVML_AFFINITY_SCOPE_NODE: u32 = 0;
pub const NVML_AFFINITY_SCOPE_SOCKET: u32 = 1;
pub const NVML_DEVICE_MIG_DISABLE: u32 = 0;
pub const NVML_DEVICE_MIG_ENABLE: u32 = 1;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE: u32 = 0;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE: u32 = 1;
pub const NVML_GPU_INSTANCE_PROFILE_3_SLICE: u32 = 2;
pub const NVML_GPU_INSTANCE_PROFILE_4_SLICE: u32 = 3;
pub const NVML_GPU_INSTANCE_PROFILE_7_SLICE: u32 = 4;
pub const NVML_GPU_INSTANCE_PROFILE_8_SLICE: u32 = 5;
pub const NVML_GPU_INSTANCE_PROFILE_6_SLICE: u32 = 6;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_REV1: u32 = 7;
pub const NVML_GPU_INSTANCE_PROFILE_2_SLICE_REV1: u32 = 8;
pub const NVML_GPU_INSTANCE_PROFILE_1_SLICE_REV2: u32 = 9;
pub const NVML_GPU_INSTANCE_PROFILE_COUNT: u32 = 10;
pub const NVML_GPU_INTSTANCE_PROFILE_CAPS_P2P: u32 = 1;
pub const NVML_COMPUTE_INSTANCE_PROFILE_1_SLICE: u32 = 0;
pub const NVML_COMPUTE_INSTANCE_PROFILE_2_SLICE: u32 = 1;
pub const NVML_COMPUTE_INSTANCE_PROFILE_3_SLICE: u32 = 2;
pub const NVML_COMPUTE_INSTANCE_PROFILE_4_SLICE: u32 = 3;
pub const NVML_COMPUTE_INSTANCE_PROFILE_7_SLICE: u32 = 4;
pub const NVML_COMPUTE_INSTANCE_PROFILE_8_SLICE: u32 = 5;
pub const NVML_COMPUTE_INSTANCE_PROFILE_6_SLICE: u32 = 6;
pub const NVML_COMPUTE_INSTANCE_PROFILE_1_SLICE_REV1: u32 = 7;
pub const NVML_COMPUTE_INSTANCE_PROFILE_COUNT: u32 = 8;
pub const NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_SHARED: u32 = 0;
pub const NVML_COMPUTE_INSTANCE_ENGINE_PROFILE_COUNT: u32 = 1;
pub const NVML_GPM_METRICS_GET_VERSION: u32 = 1;
pub const NVML_GPM_SUPPORT_VERSION: u32 = 1;
pub const NVML_NVLINK_POWER_STATE_HIGH_SPEED: u32 = 0;
pub const NVML_NVLINK_POWER_STATE_LOW: u32 = 1;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_MIN: u32 = 1;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_MAX: u32 = 8191;
pub const NVML_NVLINK_LOW_POWER_THRESHOLD_RESET: u32 = 4294967295;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlDevice_st {
    _unused: [u8; 0],
}
pub type nvmlDevice_t = *mut nvmlDevice_st;
/// PCI information about a GPU device.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPciInfoExt_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< The PCI domain on which the device's bus resides, 0 to 0xffffffff
    pub domain: ::core::ffi::c_uint,
    ///!< The bus on which the device resides, 0 to 0xff
    pub bus: ::core::ffi::c_uint,
    ///!< The device's id on the bus, 0 to 31
    pub device: ::core::ffi::c_uint,
    ///!< The combined 16-bit device id and 16-bit vendor id
    pub pciDeviceId: ::core::ffi::c_uint,
    ///!< The 32-bit Sub System Device ID
    pub pciSubSystemId: ::core::ffi::c_uint,
    ///!< The 8-bit PCI base class code
    pub baseClass: ::core::ffi::c_uint,
    ///!< The 8-bit PCI sub class code
    pub subClass: ::core::ffi::c_uint,
    ///!< The tuple domain:bus:device.function PCI identifier (&amp; NULL terminator)
    pub busId: [::core::ffi::c_char; 32usize],
}
/// PCI information about a GPU device.
pub type nvmlPciInfoExt_t = nvmlPciInfoExt_v1_t;
/// PCI information about a GPU device.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPciInfo_st {
    ///!< The legacy tuple domain:bus:device.function PCI identifier (&amp; NULL terminator)
    pub busIdLegacy: [::core::ffi::c_char; 16usize],
    ///!< The PCI domain on which the device's bus resides, 0 to 0xffffffff
    pub domain: ::core::ffi::c_uint,
    ///!< The bus on which the device resides, 0 to 0xff
    pub bus: ::core::ffi::c_uint,
    ///!< The device's id on the bus, 0 to 31
    pub device: ::core::ffi::c_uint,
    ///!< The combined 16-bit device id and 16-bit vendor id
    pub pciDeviceId: ::core::ffi::c_uint,
    ///!< The 32-bit Sub System Device ID
    pub pciSubSystemId: ::core::ffi::c_uint,
    ///!< The tuple domain:bus:device.function PCI identifier (&amp; NULL terminator)
    pub busId: [::core::ffi::c_char; 32usize],
}
/// PCI information about a GPU device.
pub type nvmlPciInfo_t = nvmlPciInfo_st;
/** Detailed ECC error counts for a device.

 @deprecated  Different GPU families can have different memory error counters
              See \ref nvmlDeviceGetMemoryErrorCounter*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEccErrorCounts_st {
    ///!< L1 cache errors
    pub l1Cache: ::core::ffi::c_ulonglong,
    ///!< L2 cache errors
    pub l2Cache: ::core::ffi::c_ulonglong,
    ///!< Device memory errors
    pub deviceMemory: ::core::ffi::c_ulonglong,
    ///!< Register file errors
    pub registerFile: ::core::ffi::c_ulonglong,
}
/** Detailed ECC error counts for a device.

 @deprecated  Different GPU families can have different memory error counters
              See \ref nvmlDeviceGetMemoryErrorCounter*/
pub type nvmlEccErrorCounts_t = nvmlEccErrorCounts_st;
/** Utilization information for a device.
 Each sample period may be between 1 second and 1/6 second, depending on the product being queried.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlUtilization_st {
    ///!< Percent of time over the past sample period during which one or more kernels was executing on the GPU
    pub gpu: ::core::ffi::c_uint,
    ///!< Percent of time over the past sample period during which global (device) memory was being read or written
    pub memory: ::core::ffi::c_uint,
}
/** Utilization information for a device.
 Each sample period may be between 1 second and 1/6 second, depending on the product being queried.*/
pub type nvmlUtilization_t = nvmlUtilization_st;
/** Memory allocation information for a device (v1).
 The total amount is equal to the sum of the amounts of free and used memory.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlMemory_st {
    ///!< Total physical device memory (in bytes)
    pub total: ::core::ffi::c_ulonglong,
    ///!< Unallocated device memory (in bytes)
    pub free: ::core::ffi::c_ulonglong,
    /**!< Sum of Reserved and Allocated device memory (in bytes).
!< Note that the driver/GPU always sets aside a small amount of memory for bookkeeping*/
    pub used: ::core::ffi::c_ulonglong,
}
/** Memory allocation information for a device (v1).
 The total amount is equal to the sum of the amounts of free and used memory.*/
pub type nvmlMemory_t = nvmlMemory_st;
/** Memory allocation information for a device (v2).

 Version 2 adds versioning for the struct and the amount of system-reserved memory as an output.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlMemory_v2_st {
    ///!< Structure format version (must be 2)
    pub version: ::core::ffi::c_uint,
    ///!< Total physical device memory (in bytes)
    pub total: ::core::ffi::c_ulonglong,
    ///!< Device memory (in bytes) reserved for system use (driver or firmware)
    pub reserved: ::core::ffi::c_ulonglong,
    ///!< Unallocated device memory (in bytes)
    pub free: ::core::ffi::c_ulonglong,
    ///!< Allocated device memory (in bytes).
    pub used: ::core::ffi::c_ulonglong,
}
/** Memory allocation information for a device (v2).

 Version 2 adds versioning for the struct and the amount of system-reserved memory as an output.*/
pub type nvmlMemory_v2_t = nvmlMemory_v2_st;
/// BAR1 Memory allocation Information for a device
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlBAR1Memory_st {
    ///!< Total BAR1 Memory (in bytes)
    pub bar1Total: ::core::ffi::c_ulonglong,
    ///!< Unallocated BAR1 Memory (in bytes)
    pub bar1Free: ::core::ffi::c_ulonglong,
    ///!< Allocated Used Memory (in bytes)
    pub bar1Used: ::core::ffi::c_ulonglong,
}
/// BAR1 Memory allocation Information for a device
pub type nvmlBAR1Memory_t = nvmlBAR1Memory_st;
/** Information about running compute processes on the GPU, legacy version
 for older versions of the API.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessInfo_v1_st {
    ///!< Process ID
    pub pid: ::core::ffi::c_uint,
    ///!< Amount of used GPU memory in bytes.
    pub usedGpuMemory: ::core::ffi::c_ulonglong,
}
/** Information about running compute processes on the GPU, legacy version
 for older versions of the API.*/
pub type nvmlProcessInfo_v1_t = nvmlProcessInfo_v1_st;
/// Information about running compute processes on the GPU
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessInfo_v2_st {
    ///!< Process ID
    pub pid: ::core::ffi::c_uint,
    ///!< Amount of used GPU memory in bytes.
    pub usedGpuMemory: ::core::ffi::c_ulonglong,
    ///!< If MIG is enabled, stores a valid GPU instance ID. gpuInstanceId is set to
    pub gpuInstanceId: ::core::ffi::c_uint,
    ///!< If MIG is enabled, stores a valid compute instance ID. computeInstanceId is set to
    pub computeInstanceId: ::core::ffi::c_uint,
}
/// Information about running compute processes on the GPU
pub type nvmlProcessInfo_v2_t = nvmlProcessInfo_v2_st;
/// Information about running compute processes on the GPU
pub type nvmlProcessInfo_t = nvmlProcessInfo_v2_st;
/// Information about running process on the GPU with protected memory
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessDetail_v1_t {
    ///!< Process ID
    pub pid: ::core::ffi::c_uint,
    ///!< Amount of used GPU memory in bytes.
    pub usedGpuMemory: ::core::ffi::c_ulonglong,
    ///!< If MIG is enabled, stores a valid GPU instance ID. gpuInstanceId is
    pub gpuInstanceId: ::core::ffi::c_uint,
    ///!< If MIG is enabled, stores a valid compute instance ID. computeInstanceId
    pub computeInstanceId: ::core::ffi::c_uint,
    ///!< Amount of used GPU conf compute protected memory in bytes.
    pub usedGpuCcProtectedMemory: ::core::ffi::c_ulonglong,
}
/// Information about all running processes on the GPU for the given mode
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessDetailList_v1_t {
    ///!< Struct version, MUST be nvmlProcessDetailList_v1
    pub version: ::core::ffi::c_uint,
    ///!< Process mode(Compute/Graphics/MPSCompute)
    pub mode: ::core::ffi::c_uint,
    ///!< Number of process entries in procArray
    pub numProcArrayEntries: ::core::ffi::c_uint,
    ///!< Process array
    pub procArray: *mut nvmlProcessDetail_v1_t,
}
/// Information about all running processes on the GPU for the given mode
pub type nvmlProcessDetailList_t = nvmlProcessDetailList_v1_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlDeviceAttributes_st {
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Shared Copy Engine count
    pub sharedCopyEngineCount: ::core::ffi::c_uint,
    ///!< Shared Decoder Engine count
    pub sharedDecoderCount: ::core::ffi::c_uint,
    ///!< Shared Encoder Engine count
    pub sharedEncoderCount: ::core::ffi::c_uint,
    ///!< Shared JPEG Engine count
    pub sharedJpegCount: ::core::ffi::c_uint,
    ///!< Shared OFA Engine count
    pub sharedOfaCount: ::core::ffi::c_uint,
    ///!< GPU instance slice count
    pub gpuInstanceSliceCount: ::core::ffi::c_uint,
    ///!< Compute instance slice count
    pub computeInstanceSliceCount: ::core::ffi::c_uint,
    ///!< Device memory size (in MiB)
    pub memorySizeMB: ::core::ffi::c_ulonglong,
}
pub type nvmlDeviceAttributes_t = nvmlDeviceAttributes_st;
/// C2C Mode information for a device
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlC2cModeInfo_v1_t {
    pub isC2cEnabled: ::core::ffi::c_uint,
}
/** Possible values that classify the remap availability for each bank. The max
 field will contain the number of banks that have maximum remap availability
 (all reserved rows are available). None means that there are no reserved
 rows available.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlRowRemapperHistogramValues_st {
    pub max: ::core::ffi::c_uint,
    pub high: ::core::ffi::c_uint,
    pub partial: ::core::ffi::c_uint,
    pub low: ::core::ffi::c_uint,
    pub none: ::core::ffi::c_uint,
}
/** Possible values that classify the remap availability for each bank. The max
 field will contain the number of banks that have maximum remap availability
 (all reserved rows are available). None means that there are no reserved
 rows available.*/
pub type nvmlRowRemapperHistogramValues_t = nvmlRowRemapperHistogramValues_st;
impl nvmlBridgeChipType_enum {
    pub const NVML_BRIDGE_CHIP_PLX: nvmlBridgeChipType_enum = nvmlBridgeChipType_enum(0);
}
impl nvmlBridgeChipType_enum {
    pub const NVML_BRIDGE_CHIP_BRO4: nvmlBridgeChipType_enum = nvmlBridgeChipType_enum(
        1,
    );
}
#[repr(transparent)]
/// Enum to represent type of bridge chip
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlBridgeChipType_enum(pub ::core::ffi::c_uint);
/// Enum to represent type of bridge chip
pub use self::nvmlBridgeChipType_enum as nvmlBridgeChipType_t;
impl nvmlNvLinkUtilizationCountUnits_enum {
    pub const NVML_NVLINK_COUNTER_UNIT_CYCLES: nvmlNvLinkUtilizationCountUnits_enum = nvmlNvLinkUtilizationCountUnits_enum(
        0,
    );
}
impl nvmlNvLinkUtilizationCountUnits_enum {
    pub const NVML_NVLINK_COUNTER_UNIT_PACKETS: nvmlNvLinkUtilizationCountUnits_enum = nvmlNvLinkUtilizationCountUnits_enum(
        1,
    );
}
impl nvmlNvLinkUtilizationCountUnits_enum {
    pub const NVML_NVLINK_COUNTER_UNIT_BYTES: nvmlNvLinkUtilizationCountUnits_enum = nvmlNvLinkUtilizationCountUnits_enum(
        2,
    );
}
impl nvmlNvLinkUtilizationCountUnits_enum {
    pub const NVML_NVLINK_COUNTER_UNIT_RESERVED: nvmlNvLinkUtilizationCountUnits_enum = nvmlNvLinkUtilizationCountUnits_enum(
        3,
    );
}
impl nvmlNvLinkUtilizationCountUnits_enum {
    pub const NVML_NVLINK_COUNTER_UNIT_COUNT: nvmlNvLinkUtilizationCountUnits_enum = nvmlNvLinkUtilizationCountUnits_enum(
        4,
    );
}
#[repr(transparent)]
/// Enum to represent the NvLink utilization counter packet units
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkUtilizationCountUnits_enum(pub ::core::ffi::c_uint);
/// Enum to represent the NvLink utilization counter packet units
pub use self::nvmlNvLinkUtilizationCountUnits_enum as nvmlNvLinkUtilizationCountUnits_t;
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_NOP: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        1,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_READ: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        2,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_WRITE: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        4,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_RATOM: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        8,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_NRATOM: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        16,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_FLUSH: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        32,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_RESPDATA: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        64,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_RESPNODATA: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        128,
    );
}
impl nvmlNvLinkUtilizationCountPktTypes_enum {
    pub const NVML_NVLINK_COUNTER_PKTFILTER_ALL: nvmlNvLinkUtilizationCountPktTypes_enum = nvmlNvLinkUtilizationCountPktTypes_enum(
        255,
    );
}
#[repr(transparent)]
/** Enum to represent the NvLink utilization counter packet types to count
  ** this is ONLY applicable with the units as packets or bytes
  ** as specified in \a nvmlNvLinkUtilizationCountUnits_t
  ** all packet filter descriptions are target GPU centric
  ** these can be "OR'd" together*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkUtilizationCountPktTypes_enum(pub ::core::ffi::c_uint);
/** Enum to represent the NvLink utilization counter packet types to count
  ** this is ONLY applicable with the units as packets or bytes
  ** as specified in \a nvmlNvLinkUtilizationCountUnits_t
  ** all packet filter descriptions are target GPU centric
  ** these can be "OR'd" together*/
pub use self::nvmlNvLinkUtilizationCountPktTypes_enum as nvmlNvLinkUtilizationCountPktTypes_t;
/// Struct to define the NVLINK counter controls
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkUtilizationControl_st {
    pub units: nvmlNvLinkUtilizationCountUnits_t,
    pub pktfilter: nvmlNvLinkUtilizationCountPktTypes_t,
}
/// Struct to define the NVLINK counter controls
pub type nvmlNvLinkUtilizationControl_t = nvmlNvLinkUtilizationControl_st;
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_P2P_SUPPORTED: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        0,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_SYSMEM_ACCESS: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        1,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_P2P_ATOMICS: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        2,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_SYSMEM_ATOMICS: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        3,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_SLI_BRIDGE: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        4,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_VALID: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        5,
    );
}
impl nvmlNvLinkCapability_enum {
    pub const NVML_NVLINK_CAP_COUNT: nvmlNvLinkCapability_enum = nvmlNvLinkCapability_enum(
        6,
    );
}
#[repr(transparent)]
/// Enum to represent NvLink queryable capabilities
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkCapability_enum(pub ::core::ffi::c_uint);
/// Enum to represent NvLink queryable capabilities
pub use self::nvmlNvLinkCapability_enum as nvmlNvLinkCapability_t;
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_DL_REPLAY: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        0,
    );
}
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_DL_RECOVERY: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        1,
    );
}
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_DL_CRC_FLIT: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        2,
    );
}
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_DL_CRC_DATA: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        3,
    );
}
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_DL_ECC_DATA: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        4,
    );
}
impl nvmlNvLinkErrorCounter_enum {
    pub const NVML_NVLINK_ERROR_COUNT: nvmlNvLinkErrorCounter_enum = nvmlNvLinkErrorCounter_enum(
        5,
    );
}
#[repr(transparent)]
/// Enum to represent NvLink queryable error counters
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkErrorCounter_enum(pub ::core::ffi::c_uint);
/// Enum to represent NvLink queryable error counters
pub use self::nvmlNvLinkErrorCounter_enum as nvmlNvLinkErrorCounter_t;
impl nvmlIntNvLinkDeviceType_enum {
    pub const NVML_NVLINK_DEVICE_TYPE_GPU: nvmlIntNvLinkDeviceType_enum = nvmlIntNvLinkDeviceType_enum(
        0,
    );
}
impl nvmlIntNvLinkDeviceType_enum {
    pub const NVML_NVLINK_DEVICE_TYPE_IBMNPU: nvmlIntNvLinkDeviceType_enum = nvmlIntNvLinkDeviceType_enum(
        1,
    );
}
impl nvmlIntNvLinkDeviceType_enum {
    pub const NVML_NVLINK_DEVICE_TYPE_SWITCH: nvmlIntNvLinkDeviceType_enum = nvmlIntNvLinkDeviceType_enum(
        2,
    );
}
impl nvmlIntNvLinkDeviceType_enum {
    pub const NVML_NVLINK_DEVICE_TYPE_UNKNOWN: nvmlIntNvLinkDeviceType_enum = nvmlIntNvLinkDeviceType_enum(
        255,
    );
}
#[repr(transparent)]
/// Enum to represent NvLink's remote device type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlIntNvLinkDeviceType_enum(pub ::core::ffi::c_uint);
/// Enum to represent NvLink's remote device type
pub use self::nvmlIntNvLinkDeviceType_enum as nvmlIntNvLinkDeviceType_t;
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_INTERNAL: nvmlGpuLevel_enum = nvmlGpuLevel_enum(0);
}
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_SINGLE: nvmlGpuLevel_enum = nvmlGpuLevel_enum(10);
}
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_MULTIPLE: nvmlGpuLevel_enum = nvmlGpuLevel_enum(20);
}
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_HOSTBRIDGE: nvmlGpuLevel_enum = nvmlGpuLevel_enum(30);
}
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_NODE: nvmlGpuLevel_enum = nvmlGpuLevel_enum(40);
}
impl nvmlGpuLevel_enum {
    pub const NVML_TOPOLOGY_SYSTEM: nvmlGpuLevel_enum = nvmlGpuLevel_enum(50);
}
#[repr(transparent)]
/** Represents level relationships within a system between two GPUs
 The enums are spaced to allow for future relationships*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuLevel_enum(pub ::core::ffi::c_uint);
/** Represents level relationships within a system between two GPUs
 The enums are spaced to allow for future relationships*/
pub use self::nvmlGpuLevel_enum as nvmlGpuTopologyLevel_t;
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_OK: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(0);
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_CHIPSET_NOT_SUPPORED: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        1,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_CHIPSET_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        1,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_GPU_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        2,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_IOH_TOPOLOGY_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        3,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_DISABLED_BY_REGKEY: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        4,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_NOT_SUPPORTED: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(
        5,
    );
}
impl nvmlGpuP2PStatus_enum {
    pub const NVML_P2P_STATUS_UNKNOWN: nvmlGpuP2PStatus_enum = nvmlGpuP2PStatus_enum(6);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuP2PStatus_enum(pub ::core::ffi::c_uint);
pub use self::nvmlGpuP2PStatus_enum as nvmlGpuP2PStatus_t;
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_READ: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        0,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_WRITE: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        1,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_NVLINK: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        2,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_ATOMICS: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        3,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_PCI: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        4,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_PROP: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        4,
    );
}
impl nvmlGpuP2PCapsIndex_enum {
    pub const NVML_P2P_CAPS_INDEX_UNKNOWN: nvmlGpuP2PCapsIndex_enum = nvmlGpuP2PCapsIndex_enum(
        5,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuP2PCapsIndex_enum(pub ::core::ffi::c_uint);
pub use self::nvmlGpuP2PCapsIndex_enum as nvmlGpuP2PCapsIndex_t;
/// Information about the Bridge Chip Firmware
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlBridgeChipInfo_st {
    ///!< Type of Bridge Chip
    pub type_: nvmlBridgeChipType_t,
    ///!< Firmware Version. 0=Version is unavailable
    pub fwVersion: ::core::ffi::c_uint,
}
/// Information about the Bridge Chip Firmware
pub type nvmlBridgeChipInfo_t = nvmlBridgeChipInfo_st;
/** This structure stores the complete Hierarchy of the Bridge Chip within the board. The immediate
 bridge is stored at index 0 of bridgeInfoList, parent to immediate bridge is at index 1 and so forth.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlBridgeChipHierarchy_st {
    ///!< Number of Bridge Chips on the Board
    pub bridgeCount: ::core::ffi::c_uchar,
    ///!< Hierarchy of Bridge Chips on the board
    pub bridgeChipInfo: [nvmlBridgeChipInfo_t; 128usize],
}
/** This structure stores the complete Hierarchy of the Bridge Chip within the board. The immediate
 bridge is stored at index 0 of bridgeInfoList, parent to immediate bridge is at index 1 and so forth.*/
pub type nvmlBridgeChipHierarchy_t = nvmlBridgeChipHierarchy_st;
impl nvmlSamplingType_enum {
    ///!< To represent total power drawn by GPU
    pub const NVML_TOTAL_POWER_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(0);
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which one or more kernels was executing on the GPU
    pub const NVML_GPU_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        1,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which global (device) memory was being read or written
    pub const NVML_MEMORY_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        2,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which NVENC remains busy
    pub const NVML_ENC_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        3,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which NVDEC remains busy
    pub const NVML_DEC_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        4,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent processor clock samples
    pub const NVML_PROCESSOR_CLK_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        5,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent memory clock samples
    pub const NVML_MEMORY_CLK_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(6);
}
impl nvmlSamplingType_enum {
    ///!< To represent module power samples for total module starting Grace Hopper
    pub const NVML_MODULE_POWER_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        7,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which NVJPG remains busy
    pub const NVML_JPG_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        8,
    );
}
impl nvmlSamplingType_enum {
    ///!< To represent percent of time during which NVOFA remains busy
    pub const NVML_OFA_UTILIZATION_SAMPLES: nvmlSamplingType_enum = nvmlSamplingType_enum(
        9,
    );
}
impl nvmlSamplingType_enum {
    pub const NVML_SAMPLINGTYPE_COUNT: nvmlSamplingType_enum = nvmlSamplingType_enum(10);
}
#[repr(transparent)]
///  Represents Type of Sampling Event
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlSamplingType_enum(pub ::core::ffi::c_uint);
///  Represents Type of Sampling Event
pub use self::nvmlSamplingType_enum as nvmlSamplingType_t;
impl nvmlPcieUtilCounter_enum {
    pub const NVML_PCIE_UTIL_TX_BYTES: nvmlPcieUtilCounter_enum = nvmlPcieUtilCounter_enum(
        0,
    );
}
impl nvmlPcieUtilCounter_enum {
    pub const NVML_PCIE_UTIL_RX_BYTES: nvmlPcieUtilCounter_enum = nvmlPcieUtilCounter_enum(
        1,
    );
}
impl nvmlPcieUtilCounter_enum {
    pub const NVML_PCIE_UTIL_COUNT: nvmlPcieUtilCounter_enum = nvmlPcieUtilCounter_enum(
        2,
    );
}
#[repr(transparent)]
/// Represents the queryable PCIe utilization counters
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPcieUtilCounter_enum(pub ::core::ffi::c_uint);
/// Represents the queryable PCIe utilization counters
pub use self::nvmlPcieUtilCounter_enum as nvmlPcieUtilCounter_t;
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_DOUBLE: nvmlValueType_enum = nvmlValueType_enum(0);
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_UNSIGNED_INT: nvmlValueType_enum = nvmlValueType_enum(1);
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_UNSIGNED_LONG: nvmlValueType_enum = nvmlValueType_enum(2);
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_UNSIGNED_LONG_LONG: nvmlValueType_enum = nvmlValueType_enum(
        3,
    );
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_SIGNED_LONG_LONG: nvmlValueType_enum = nvmlValueType_enum(
        4,
    );
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_SIGNED_INT: nvmlValueType_enum = nvmlValueType_enum(5);
}
impl nvmlValueType_enum {
    pub const NVML_VALUE_TYPE_COUNT: nvmlValueType_enum = nvmlValueType_enum(6);
}
#[repr(transparent)]
/// Represents the type for sample value returned
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlValueType_enum(pub ::core::ffi::c_uint);
/// Represents the type for sample value returned
pub use self::nvmlValueType_enum as nvmlValueType_t;
/// Union to represent different types of Value
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlValue_st {
    ///!< If the value is double
    pub dVal: f64,
    ///!< If the value is signed int
    pub siVal: ::core::ffi::c_int,
    ///!< If the value is unsigned int
    pub uiVal: ::core::ffi::c_uint,
    ///!< If the value is unsigned long
    pub ulVal: ::core::ffi::c_ulong,
    ///!< If the value is unsigned long long
    pub ullVal: ::core::ffi::c_ulonglong,
    ///!< If the value is signed long long
    pub sllVal: ::core::ffi::c_longlong,
}
/// Union to represent different types of Value
pub type nvmlValue_t = nvmlValue_st;
/// Information for Sample
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlSample_st {
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< Sample Value
    pub sampleValue: nvmlValue_t,
}
/// Information for Sample
pub type nvmlSample_t = nvmlSample_st;
impl nvmlPerfPolicyType_enum {
    ///!< How long did power violations cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_POWER: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        0,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< How long did thermal violations cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_THERMAL: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        1,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< How long did sync boost cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_SYNC_BOOST: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        2,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< How long did the board limit cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_BOARD_LIMIT: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        3,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< How long did low utilization cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_LOW_UTILIZATION: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        4,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< How long did the board reliability limit cause the GPU to be below application clocks
    pub const NVML_PERF_POLICY_RELIABILITY: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        5,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< Total time the GPU was held below application clocks by any limiter (0 - 5 above)
    pub const NVML_PERF_POLICY_TOTAL_APP_CLOCKS: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        10,
    );
}
impl nvmlPerfPolicyType_enum {
    ///!< Total time the GPU was held below base clocks
    pub const NVML_PERF_POLICY_TOTAL_BASE_CLOCKS: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        11,
    );
}
impl nvmlPerfPolicyType_enum {
    pub const NVML_PERF_POLICY_COUNT: nvmlPerfPolicyType_enum = nvmlPerfPolicyType_enum(
        12,
    );
}
#[repr(transparent)]
/// Represents type of perf policy for which violation times can be queried
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPerfPolicyType_enum(pub ::core::ffi::c_uint);
/// Represents type of perf policy for which violation times can be queried
pub use self::nvmlPerfPolicyType_enum as nvmlPerfPolicyType_t;
/// Struct to hold perf policy violation status data
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlViolationTime_st {
    ///!< referenceTime represents CPU timestamp in microseconds
    pub referenceTime: ::core::ffi::c_ulonglong,
    ///!< violationTime in Nanoseconds
    pub violationTime: ::core::ffi::c_ulonglong,
}
/// Struct to hold perf policy violation status data
pub type nvmlViolationTime_t = nvmlViolationTime_st;
impl nvmlThermalTarget_t {
    pub const NVML_THERMAL_TARGET_NONE: nvmlThermalTarget_t = nvmlThermalTarget_t(0);
}
impl nvmlThermalTarget_t {
    ///!< GPU core temperature requires NvPhysicalGpuHandle
    pub const NVML_THERMAL_TARGET_GPU: nvmlThermalTarget_t = nvmlThermalTarget_t(1);
}
impl nvmlThermalTarget_t {
    ///!< GPU memory temperature requires NvPhysicalGpuHandle
    pub const NVML_THERMAL_TARGET_MEMORY: nvmlThermalTarget_t = nvmlThermalTarget_t(2);
}
impl nvmlThermalTarget_t {
    ///!< GPU power supply temperature requires NvPhysicalGpuHandle
    pub const NVML_THERMAL_TARGET_POWER_SUPPLY: nvmlThermalTarget_t = nvmlThermalTarget_t(
        4,
    );
}
impl nvmlThermalTarget_t {
    ///!< GPU board ambient temperature requires NvPhysicalGpuHandle
    pub const NVML_THERMAL_TARGET_BOARD: nvmlThermalTarget_t = nvmlThermalTarget_t(8);
}
impl nvmlThermalTarget_t {
    ///!< Visual Computing Device Board temperature requires NvVisualComputingDeviceHandle
    pub const NVML_THERMAL_TARGET_VCD_BOARD: nvmlThermalTarget_t = nvmlThermalTarget_t(
        9,
    );
}
impl nvmlThermalTarget_t {
    ///!< Visual Computing Device Inlet temperature requires NvVisualComputingDeviceHandle
    pub const NVML_THERMAL_TARGET_VCD_INLET: nvmlThermalTarget_t = nvmlThermalTarget_t(
        10,
    );
}
impl nvmlThermalTarget_t {
    ///!< Visual Computing Device Outlet temperature requires NvVisualComputingDeviceHandle
    pub const NVML_THERMAL_TARGET_VCD_OUTLET: nvmlThermalTarget_t = nvmlThermalTarget_t(
        11,
    );
}
impl nvmlThermalTarget_t {
    pub const NVML_THERMAL_TARGET_ALL: nvmlThermalTarget_t = nvmlThermalTarget_t(15);
}
impl nvmlThermalTarget_t {
    pub const NVML_THERMAL_TARGET_UNKNOWN: nvmlThermalTarget_t = nvmlThermalTarget_t(-1);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlThermalTarget_t(pub ::core::ffi::c_int);
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_NONE: nvmlThermalController_t = nvmlThermalController_t(
        0,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_GPU_INTERNAL: nvmlThermalController_t = nvmlThermalController_t(
        1,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_ADM1032: nvmlThermalController_t = nvmlThermalController_t(
        2,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_ADT7461: nvmlThermalController_t = nvmlThermalController_t(
        3,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_MAX6649: nvmlThermalController_t = nvmlThermalController_t(
        4,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_MAX1617: nvmlThermalController_t = nvmlThermalController_t(
        5,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_LM99: nvmlThermalController_t = nvmlThermalController_t(
        6,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_LM89: nvmlThermalController_t = nvmlThermalController_t(
        7,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_LM64: nvmlThermalController_t = nvmlThermalController_t(
        8,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_G781: nvmlThermalController_t = nvmlThermalController_t(
        9,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_ADT7473: nvmlThermalController_t = nvmlThermalController_t(
        10,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_SBMAX6649: nvmlThermalController_t = nvmlThermalController_t(
        11,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_VBIOSEVT: nvmlThermalController_t = nvmlThermalController_t(
        12,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_OS: nvmlThermalController_t = nvmlThermalController_t(
        13,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_NVSYSCON_CANOAS: nvmlThermalController_t = nvmlThermalController_t(
        14,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_NVSYSCON_E551: nvmlThermalController_t = nvmlThermalController_t(
        15,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_MAX6649R: nvmlThermalController_t = nvmlThermalController_t(
        16,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_ADT7473S: nvmlThermalController_t = nvmlThermalController_t(
        17,
    );
}
impl nvmlThermalController_t {
    pub const NVML_THERMAL_CONTROLLER_UNKNOWN: nvmlThermalController_t = nvmlThermalController_t(
        -1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlThermalController_t(pub ::core::ffi::c_int);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuThermalSettings_t {
    pub count: ::core::ffi::c_uint,
    pub sensor: [nvmlGpuThermalSettings_t__bindgen_ty_1; 3usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuThermalSettings_t__bindgen_ty_1 {
    pub controller: nvmlThermalController_t,
    pub defaultMinTemp: ::core::ffi::c_int,
    pub defaultMaxTemp: ::core::ffi::c_int,
    pub currentTemp: ::core::ffi::c_int,
    pub target: nvmlThermalTarget_t,
}
impl nvmlEnableState_enum {
    ///!< Feature disabled
    pub const NVML_FEATURE_DISABLED: nvmlEnableState_enum = nvmlEnableState_enum(0);
}
impl nvmlEnableState_enum {
    ///!< Feature enabled
    pub const NVML_FEATURE_ENABLED: nvmlEnableState_enum = nvmlEnableState_enum(1);
}
#[repr(transparent)]
/// Generic enable/disable enum.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEnableState_enum(pub ::core::ffi::c_uint);
/// Generic enable/disable enum.
pub use self::nvmlEnableState_enum as nvmlEnableState_t;
impl nvmlBrandType_enum {
    pub const NVML_BRAND_UNKNOWN: nvmlBrandType_enum = nvmlBrandType_enum(0);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_QUADRO: nvmlBrandType_enum = nvmlBrandType_enum(1);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_TESLA: nvmlBrandType_enum = nvmlBrandType_enum(2);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVS: nvmlBrandType_enum = nvmlBrandType_enum(3);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_GRID: nvmlBrandType_enum = nvmlBrandType_enum(4);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_GEFORCE: nvmlBrandType_enum = nvmlBrandType_enum(5);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_TITAN: nvmlBrandType_enum = nvmlBrandType_enum(6);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_VAPPS: nvmlBrandType_enum = nvmlBrandType_enum(7);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_VPC: nvmlBrandType_enum = nvmlBrandType_enum(8);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_VCS: nvmlBrandType_enum = nvmlBrandType_enum(9);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_VWS: nvmlBrandType_enum = nvmlBrandType_enum(10);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_CLOUD_GAMING: nvmlBrandType_enum = nvmlBrandType_enum(
        11,
    );
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_VGAMING: nvmlBrandType_enum = nvmlBrandType_enum(11);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_QUADRO_RTX: nvmlBrandType_enum = nvmlBrandType_enum(12);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA_RTX: nvmlBrandType_enum = nvmlBrandType_enum(13);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_NVIDIA: nvmlBrandType_enum = nvmlBrandType_enum(14);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_GEFORCE_RTX: nvmlBrandType_enum = nvmlBrandType_enum(15);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_TITAN_RTX: nvmlBrandType_enum = nvmlBrandType_enum(16);
}
impl nvmlBrandType_enum {
    pub const NVML_BRAND_COUNT: nvmlBrandType_enum = nvmlBrandType_enum(17);
}
#[repr(transparent)]
///  * The Brand of the GPU
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlBrandType_enum(pub ::core::ffi::c_uint);
///  * The Brand of the GPU
pub use self::nvmlBrandType_enum as nvmlBrandType_t;
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_SHUTDOWN: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        0,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_SLOWDOWN: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        1,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_MEM_MAX: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        2,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_GPU_MAX: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        3,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MIN: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        4,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_CURR: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        5,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_ACOUSTIC_MAX: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        6,
    );
}
impl nvmlTemperatureThresholds_enum {
    pub const NVML_TEMPERATURE_THRESHOLD_COUNT: nvmlTemperatureThresholds_enum = nvmlTemperatureThresholds_enum(
        7,
    );
}
#[repr(transparent)]
/// Temperature thresholds.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlTemperatureThresholds_enum(pub ::core::ffi::c_uint);
/// Temperature thresholds.
pub use self::nvmlTemperatureThresholds_enum as nvmlTemperatureThresholds_t;
impl nvmlTemperatureSensors_enum {
    ///!< Temperature sensor for the GPU die
    pub const NVML_TEMPERATURE_GPU: nvmlTemperatureSensors_enum = nvmlTemperatureSensors_enum(
        0,
    );
}
impl nvmlTemperatureSensors_enum {
    pub const NVML_TEMPERATURE_COUNT: nvmlTemperatureSensors_enum = nvmlTemperatureSensors_enum(
        1,
    );
}
#[repr(transparent)]
/// Temperature sensors.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlTemperatureSensors_enum(pub ::core::ffi::c_uint);
/// Temperature sensors.
pub use self::nvmlTemperatureSensors_enum as nvmlTemperatureSensors_t;
impl nvmlComputeMode_enum {
    ///!< Default compute mode -- multiple contexts per device
    pub const NVML_COMPUTEMODE_DEFAULT: nvmlComputeMode_enum = nvmlComputeMode_enum(0);
}
impl nvmlComputeMode_enum {
    ///!< Support Removed
    pub const NVML_COMPUTEMODE_EXCLUSIVE_THREAD: nvmlComputeMode_enum = nvmlComputeMode_enum(
        1,
    );
}
impl nvmlComputeMode_enum {
    ///!< Compute-prohibited mode -- no contexts per device
    pub const NVML_COMPUTEMODE_PROHIBITED: nvmlComputeMode_enum = nvmlComputeMode_enum(
        2,
    );
}
impl nvmlComputeMode_enum {
    ///!< Compute-exclusive-process mode -- only one context per device, usable from multiple threads at a time
    pub const NVML_COMPUTEMODE_EXCLUSIVE_PROCESS: nvmlComputeMode_enum = nvmlComputeMode_enum(
        3,
    );
}
impl nvmlComputeMode_enum {
    pub const NVML_COMPUTEMODE_COUNT: nvmlComputeMode_enum = nvmlComputeMode_enum(4);
}
#[repr(transparent)]
/** Compute mode.

 NVML_COMPUTEMODE_EXCLUSIVE_PROCESS was added in CUDA 4.0.
 Earlier CUDA versions supported a single exclusive mode,
 which is equivalent to NVML_COMPUTEMODE_EXCLUSIVE_THREAD in CUDA 4.0 and beyond.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeMode_enum(pub ::core::ffi::c_uint);
/** Compute mode.

 NVML_COMPUTEMODE_EXCLUSIVE_PROCESS was added in CUDA 4.0.
 Earlier CUDA versions supported a single exclusive mode,
 which is equivalent to NVML_COMPUTEMODE_EXCLUSIVE_THREAD in CUDA 4.0 and beyond.*/
pub use self::nvmlComputeMode_enum as nvmlComputeMode_t;
/// Clock Monitor error types
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlClkMonFaultInfo_struct {
    /// The Domain which faulted
    pub clkApiDomain: ::core::ffi::c_uint,
    /// Faults Information
    pub clkDomainFaultMask: ::core::ffi::c_uint,
}
/// Clock Monitor error types
pub type nvmlClkMonFaultInfo_t = nvmlClkMonFaultInfo_struct;
/// Clock Monitor Status
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlClkMonStatus_status {
    /// Fault status Indicator
    pub bGlobalStatus: ::core::ffi::c_uint,
    /// Total faulted domain numbers
    pub clkMonListSize: ::core::ffi::c_uint,
    /// The fault Information structure
    pub clkMonList: [nvmlClkMonFaultInfo_t; 32usize],
}
/// Clock Monitor Status
pub type nvmlClkMonStatus_t = nvmlClkMonStatus_status;
impl nvmlMemoryErrorType_enum {
    /** A memory error that was corrected

 For ECC errors, these are single bit errors
 For Texture memory, these are errors fixed by resend*/
    pub const NVML_MEMORY_ERROR_TYPE_CORRECTED: nvmlMemoryErrorType_enum = nvmlMemoryErrorType_enum(
        0,
    );
}
impl nvmlMemoryErrorType_enum {
    /** A memory error that was not corrected

 For ECC errors, these are double bit errors
 For Texture memory, these are errors where the resend fails*/
    pub const NVML_MEMORY_ERROR_TYPE_UNCORRECTED: nvmlMemoryErrorType_enum = nvmlMemoryErrorType_enum(
        1,
    );
}
impl nvmlMemoryErrorType_enum {
    ///!< Count of memory error types
    pub const NVML_MEMORY_ERROR_TYPE_COUNT: nvmlMemoryErrorType_enum = nvmlMemoryErrorType_enum(
        2,
    );
}
#[repr(transparent)]
/// Memory error types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlMemoryErrorType_enum(pub ::core::ffi::c_uint);
/// Memory error types
pub use self::nvmlMemoryErrorType_enum as nvmlMemoryErrorType_t;
impl nvmlEccCounterType_enum {
    ///!< Volatile counts are reset each time the driver loads.
    pub const NVML_VOLATILE_ECC: nvmlEccCounterType_enum = nvmlEccCounterType_enum(0);
}
impl nvmlEccCounterType_enum {
    ///!< Aggregate counts persist across reboots (i.e. for the lifetime of the device)
    pub const NVML_AGGREGATE_ECC: nvmlEccCounterType_enum = nvmlEccCounterType_enum(1);
}
impl nvmlEccCounterType_enum {
    ///!< Count of memory counter types
    pub const NVML_ECC_COUNTER_TYPE_COUNT: nvmlEccCounterType_enum = nvmlEccCounterType_enum(
        2,
    );
}
#[repr(transparent)]
/** ECC counter types.

 Note: Volatile counts are reset each time the driver loads. On Windows this is once per boot. On Linux this can be more frequent.
       On Linux the driver unloads when no active clients exist. If persistence mode is enabled or there is always a driver
       client active (e.g. X11), then Linux also sees per-boot behavior. If not, volatile counts are reset each time a compute app
       is run.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEccCounterType_enum(pub ::core::ffi::c_uint);
/** ECC counter types.

 Note: Volatile counts are reset each time the driver loads. On Windows this is once per boot. On Linux this can be more frequent.
       On Linux the driver unloads when no active clients exist. If persistence mode is enabled or there is always a driver
       client active (e.g. X11), then Linux also sees per-boot behavior. If not, volatile counts are reset each time a compute app
       is run.*/
pub use self::nvmlEccCounterType_enum as nvmlEccCounterType_t;
impl nvmlClockType_enum {
    ///!< Graphics clock domain
    pub const NVML_CLOCK_GRAPHICS: nvmlClockType_enum = nvmlClockType_enum(0);
}
impl nvmlClockType_enum {
    ///!< SM clock domain
    pub const NVML_CLOCK_SM: nvmlClockType_enum = nvmlClockType_enum(1);
}
impl nvmlClockType_enum {
    ///!< Memory clock domain
    pub const NVML_CLOCK_MEM: nvmlClockType_enum = nvmlClockType_enum(2);
}
impl nvmlClockType_enum {
    ///!< Video encoder/decoder clock domain
    pub const NVML_CLOCK_VIDEO: nvmlClockType_enum = nvmlClockType_enum(3);
}
impl nvmlClockType_enum {
    ///!< Count of clock types
    pub const NVML_CLOCK_COUNT: nvmlClockType_enum = nvmlClockType_enum(4);
}
#[repr(transparent)]
/** Clock types.

 All speeds are in Mhz.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlClockType_enum(pub ::core::ffi::c_uint);
/** Clock types.

 All speeds are in Mhz.*/
pub use self::nvmlClockType_enum as nvmlClockType_t;
impl nvmlClockId_enum {
    ///!< Current actual clock value
    pub const NVML_CLOCK_ID_CURRENT: nvmlClockId_enum = nvmlClockId_enum(0);
}
impl nvmlClockId_enum {
    ///!< Target application clock
    pub const NVML_CLOCK_ID_APP_CLOCK_TARGET: nvmlClockId_enum = nvmlClockId_enum(1);
}
impl nvmlClockId_enum {
    ///!< Default application clock target
    pub const NVML_CLOCK_ID_APP_CLOCK_DEFAULT: nvmlClockId_enum = nvmlClockId_enum(2);
}
impl nvmlClockId_enum {
    ///!< OEM-defined maximum clock rate
    pub const NVML_CLOCK_ID_CUSTOMER_BOOST_MAX: nvmlClockId_enum = nvmlClockId_enum(3);
}
impl nvmlClockId_enum {
    ///!< Count of Clock Ids.
    pub const NVML_CLOCK_ID_COUNT: nvmlClockId_enum = nvmlClockId_enum(4);
}
#[repr(transparent)]
/** Clock Ids.  These are used in combination with nvmlClockType_t
 to specify a single clock value.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlClockId_enum(pub ::core::ffi::c_uint);
/** Clock Ids.  These are used in combination with nvmlClockType_t
 to specify a single clock value.*/
pub use self::nvmlClockId_enum as nvmlClockId_t;
impl nvmlDriverModel_enum {
    ///!< WDDM driver model -- GPU treated as a display device
    pub const NVML_DRIVER_WDDM: nvmlDriverModel_enum = nvmlDriverModel_enum(0);
}
impl nvmlDriverModel_enum {
    ///!< WDM (TCC) model (recommended) -- GPU treated as a generic device
    pub const NVML_DRIVER_WDM: nvmlDriverModel_enum = nvmlDriverModel_enum(1);
}
#[repr(transparent)]
/** Driver models.

 Windows only.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlDriverModel_enum(pub ::core::ffi::c_uint);
/** Driver models.

 Windows only.*/
pub use self::nvmlDriverModel_enum as nvmlDriverModel_t;
impl nvmlPStates_enum {
    ///!< Performance state 0 -- Maximum Performance
    pub const NVML_PSTATE_0: nvmlPStates_enum = nvmlPStates_enum(0);
}
impl nvmlPStates_enum {
    ///!< Performance state 1
    pub const NVML_PSTATE_1: nvmlPStates_enum = nvmlPStates_enum(1);
}
impl nvmlPStates_enum {
    ///!< Performance state 2
    pub const NVML_PSTATE_2: nvmlPStates_enum = nvmlPStates_enum(2);
}
impl nvmlPStates_enum {
    ///!< Performance state 3
    pub const NVML_PSTATE_3: nvmlPStates_enum = nvmlPStates_enum(3);
}
impl nvmlPStates_enum {
    ///!< Performance state 4
    pub const NVML_PSTATE_4: nvmlPStates_enum = nvmlPStates_enum(4);
}
impl nvmlPStates_enum {
    ///!< Performance state 5
    pub const NVML_PSTATE_5: nvmlPStates_enum = nvmlPStates_enum(5);
}
impl nvmlPStates_enum {
    ///!< Performance state 6
    pub const NVML_PSTATE_6: nvmlPStates_enum = nvmlPStates_enum(6);
}
impl nvmlPStates_enum {
    ///!< Performance state 7
    pub const NVML_PSTATE_7: nvmlPStates_enum = nvmlPStates_enum(7);
}
impl nvmlPStates_enum {
    ///!< Performance state 8
    pub const NVML_PSTATE_8: nvmlPStates_enum = nvmlPStates_enum(8);
}
impl nvmlPStates_enum {
    ///!< Performance state 9
    pub const NVML_PSTATE_9: nvmlPStates_enum = nvmlPStates_enum(9);
}
impl nvmlPStates_enum {
    ///!< Performance state 10
    pub const NVML_PSTATE_10: nvmlPStates_enum = nvmlPStates_enum(10);
}
impl nvmlPStates_enum {
    ///!< Performance state 11
    pub const NVML_PSTATE_11: nvmlPStates_enum = nvmlPStates_enum(11);
}
impl nvmlPStates_enum {
    ///!< Performance state 12
    pub const NVML_PSTATE_12: nvmlPStates_enum = nvmlPStates_enum(12);
}
impl nvmlPStates_enum {
    ///!< Performance state 13
    pub const NVML_PSTATE_13: nvmlPStates_enum = nvmlPStates_enum(13);
}
impl nvmlPStates_enum {
    ///!< Performance state 14
    pub const NVML_PSTATE_14: nvmlPStates_enum = nvmlPStates_enum(14);
}
impl nvmlPStates_enum {
    ///!< Performance state 15 -- Minimum Performance
    pub const NVML_PSTATE_15: nvmlPStates_enum = nvmlPStates_enum(15);
}
impl nvmlPStates_enum {
    ///!< Unknown performance state
    pub const NVML_PSTATE_UNKNOWN: nvmlPStates_enum = nvmlPStates_enum(32);
}
#[repr(transparent)]
/// Allowed PStates.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPStates_enum(pub ::core::ffi::c_uint);
/// Allowed PStates.
pub use self::nvmlPStates_enum as nvmlPstates_t;
impl nvmlGom_enum {
    ///!< Everything is enabled and running at full speed
    pub const NVML_GOM_ALL_ON: nvmlGom_enum = nvmlGom_enum(0);
}
impl nvmlGom_enum {
    /**!< Designed for running only compute tasks. Graphics operations
!< are not allowed*/
    pub const NVML_GOM_COMPUTE: nvmlGom_enum = nvmlGom_enum(1);
}
impl nvmlGom_enum {
    /**!< Designed for running graphics applications that don't require
!< high bandwidth double precision*/
    pub const NVML_GOM_LOW_DP: nvmlGom_enum = nvmlGom_enum(2);
}
#[repr(transparent)]
/** GPU Operation Mode

 GOM allows to reduce power usage and optimize GPU throughput by disabling GPU features.

 Each GOM is designed to meet specific user needs.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGom_enum(pub ::core::ffi::c_uint);
/** GPU Operation Mode

 GOM allows to reduce power usage and optimize GPU throughput by disabling GPU features.

 Each GOM is designed to meet specific user needs.*/
pub use self::nvmlGom_enum as nvmlGpuOperationMode_t;
impl nvmlInforomObject_enum {
    ///!< An object defined by OEM
    pub const NVML_INFOROM_OEM: nvmlInforomObject_enum = nvmlInforomObject_enum(0);
}
impl nvmlInforomObject_enum {
    ///!< The ECC object determining the level of ECC support
    pub const NVML_INFOROM_ECC: nvmlInforomObject_enum = nvmlInforomObject_enum(1);
}
impl nvmlInforomObject_enum {
    ///!< The power management object
    pub const NVML_INFOROM_POWER: nvmlInforomObject_enum = nvmlInforomObject_enum(2);
}
impl nvmlInforomObject_enum {
    ///!< This counts the number of infoROM objects the driver knows about
    pub const NVML_INFOROM_COUNT: nvmlInforomObject_enum = nvmlInforomObject_enum(3);
}
#[repr(transparent)]
/// Available infoROM objects.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlInforomObject_enum(pub ::core::ffi::c_uint);
/// Available infoROM objects.
pub use self::nvmlInforomObject_enum as nvmlInforomObject_t;
/// Return values for NVML API calls.
pub type nvmlReturn_enum = ::core::ffi::c_uint;
impl nvmlMemoryLocation_enum {
    ///!< GPU L1 Cache
    pub const NVML_MEMORY_LOCATION_L1_CACHE: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        0,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< GPU L2 Cache
    pub const NVML_MEMORY_LOCATION_L2_CACHE: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        1,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< Turing+ DRAM
    pub const NVML_MEMORY_LOCATION_DRAM: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        2,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< GPU Device Memory
    pub const NVML_MEMORY_LOCATION_DEVICE_MEMORY: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        2,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< GPU Register File
    pub const NVML_MEMORY_LOCATION_REGISTER_FILE: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        3,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< GPU Texture Memory
    pub const NVML_MEMORY_LOCATION_TEXTURE_MEMORY: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        4,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< Shared memory
    pub const NVML_MEMORY_LOCATION_TEXTURE_SHM: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        5,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< CBU
    pub const NVML_MEMORY_LOCATION_CBU: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        6,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< Turing+ SRAM
    pub const NVML_MEMORY_LOCATION_SRAM: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        7,
    );
}
impl nvmlMemoryLocation_enum {
    ///!< This counts the number of memory locations the driver knows about
    pub const NVML_MEMORY_LOCATION_COUNT: nvmlMemoryLocation_enum = nvmlMemoryLocation_enum(
        8,
    );
}
#[repr(transparent)]
/// See \ref nvmlDeviceGetMemoryErrorCounter
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlMemoryLocation_enum(pub ::core::ffi::c_uint);
/// See \ref nvmlDeviceGetMemoryErrorCounter
pub use self::nvmlMemoryLocation_enum as nvmlMemoryLocation_t;
impl nvmlPageRetirementCause_enum {
    ///!< Page was retired due to multiple single bit ECC error
    pub const NVML_PAGE_RETIREMENT_CAUSE_MULTIPLE_SINGLE_BIT_ECC_ERRORS: nvmlPageRetirementCause_enum = nvmlPageRetirementCause_enum(
        0,
    );
}
impl nvmlPageRetirementCause_enum {
    ///!< Page was retired due to double bit ECC error
    pub const NVML_PAGE_RETIREMENT_CAUSE_DOUBLE_BIT_ECC_ERROR: nvmlPageRetirementCause_enum = nvmlPageRetirementCause_enum(
        1,
    );
}
impl nvmlPageRetirementCause_enum {
    pub const NVML_PAGE_RETIREMENT_CAUSE_COUNT: nvmlPageRetirementCause_enum = nvmlPageRetirementCause_enum(
        2,
    );
}
#[repr(transparent)]
/// Causes for page retirement
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPageRetirementCause_enum(pub ::core::ffi::c_uint);
/// Causes for page retirement
pub use self::nvmlPageRetirementCause_enum as nvmlPageRetirementCause_t;
impl nvmlRestrictedAPI_enum {
    /**!< APIs that change application clocks, see nvmlDeviceSetApplicationsClocks
!< and see nvmlDeviceResetApplicationsClocks*/
    pub const NVML_RESTRICTED_API_SET_APPLICATION_CLOCKS: nvmlRestrictedAPI_enum = nvmlRestrictedAPI_enum(
        0,
    );
}
impl nvmlRestrictedAPI_enum {
    /**!< APIs that enable/disable Auto Boosted clocks
!< see nvmlDeviceSetAutoBoostedClocksEnabled*/
    pub const NVML_RESTRICTED_API_SET_AUTO_BOOSTED_CLOCKS: nvmlRestrictedAPI_enum = nvmlRestrictedAPI_enum(
        1,
    );
}
impl nvmlRestrictedAPI_enum {
    pub const NVML_RESTRICTED_API_COUNT: nvmlRestrictedAPI_enum = nvmlRestrictedAPI_enum(
        2,
    );
}
#[repr(transparent)]
/// API types that allow changes to default permission restrictions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlRestrictedAPI_enum(pub ::core::ffi::c_uint);
/// API types that allow changes to default permission restrictions
pub use self::nvmlRestrictedAPI_enum as nvmlRestrictedAPI_t;
impl nvmlGpuVirtualizationMode {
    ///!< Represents Bare Metal GPU
    pub const NVML_GPU_VIRTUALIZATION_MODE_NONE: nvmlGpuVirtualizationMode = nvmlGpuVirtualizationMode(
        0,
    );
}
impl nvmlGpuVirtualizationMode {
    ///!< Device is associated with GPU-Passthorugh
    pub const NVML_GPU_VIRTUALIZATION_MODE_PASSTHROUGH: nvmlGpuVirtualizationMode = nvmlGpuVirtualizationMode(
        1,
    );
}
impl nvmlGpuVirtualizationMode {
    ///!< Device is associated with vGPU inside virtual machine.
    pub const NVML_GPU_VIRTUALIZATION_MODE_VGPU: nvmlGpuVirtualizationMode = nvmlGpuVirtualizationMode(
        2,
    );
}
impl nvmlGpuVirtualizationMode {
    ///!< Device is associated with VGX hypervisor in vGPU mode
    pub const NVML_GPU_VIRTUALIZATION_MODE_HOST_VGPU: nvmlGpuVirtualizationMode = nvmlGpuVirtualizationMode(
        3,
    );
}
impl nvmlGpuVirtualizationMode {
    ///!< Device is associated with VGX hypervisor in vSGA mode
    pub const NVML_GPU_VIRTUALIZATION_MODE_HOST_VSGA: nvmlGpuVirtualizationMode = nvmlGpuVirtualizationMode(
        4,
    );
}
#[repr(transparent)]
/// GPU virtualization mode types.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuVirtualizationMode(pub ::core::ffi::c_uint);
/// GPU virtualization mode types.
pub use self::nvmlGpuVirtualizationMode as nvmlGpuVirtualizationMode_t;
impl nvmlHostVgpuMode_enum {
    ///!< Non SR-IOV mode
    pub const NVML_HOST_VGPU_MODE_NON_SRIOV: nvmlHostVgpuMode_enum = nvmlHostVgpuMode_enum(
        0,
    );
}
impl nvmlHostVgpuMode_enum {
    ///!< SR-IOV mode
    pub const NVML_HOST_VGPU_MODE_SRIOV: nvmlHostVgpuMode_enum = nvmlHostVgpuMode_enum(
        1,
    );
}
#[repr(transparent)]
/// Host vGPU modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlHostVgpuMode_enum(pub ::core::ffi::c_uint);
/// Host vGPU modes
pub use self::nvmlHostVgpuMode_enum as nvmlHostVgpuMode_t;
impl nvmlVgpuVmIdType {
    ///!< VM ID represents DOMAIN ID
    pub const NVML_VGPU_VM_ID_DOMAIN_ID: nvmlVgpuVmIdType = nvmlVgpuVmIdType(0);
}
impl nvmlVgpuVmIdType {
    ///!< VM ID represents UUID
    pub const NVML_VGPU_VM_ID_UUID: nvmlVgpuVmIdType = nvmlVgpuVmIdType(1);
}
#[repr(transparent)]
/// Types of VM identifiers
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuVmIdType(pub ::core::ffi::c_uint);
/// Types of VM identifiers
pub use self::nvmlVgpuVmIdType as nvmlVgpuVmIdType_t;
impl nvmlVgpuGuestInfoState_enum {
    ///!< Guest-dependent fields uninitialized
    pub const NVML_VGPU_INSTANCE_GUEST_INFO_STATE_UNINITIALIZED: nvmlVgpuGuestInfoState_enum = nvmlVgpuGuestInfoState_enum(
        0,
    );
}
impl nvmlVgpuGuestInfoState_enum {
    ///!< Guest-dependent fields initialized
    pub const NVML_VGPU_INSTANCE_GUEST_INFO_STATE_INITIALIZED: nvmlVgpuGuestInfoState_enum = nvmlVgpuGuestInfoState_enum(
        1,
    );
}
#[repr(transparent)]
/// vGPU GUEST info state
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuGuestInfoState_enum(pub ::core::ffi::c_uint);
/// vGPU GUEST info state
pub use self::nvmlVgpuGuestInfoState_enum as nvmlVgpuGuestInfoState_t;
impl nvmlGridLicenseFeatureCode_t {
    ///!< Unknown
    pub const NVML_GRID_LICENSE_FEATURE_CODE_UNKNOWN: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        0,
    );
}
impl nvmlGridLicenseFeatureCode_t {
    ///!< Virtual GPU
    pub const NVML_GRID_LICENSE_FEATURE_CODE_VGPU: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        1,
    );
}
impl nvmlGridLicenseFeatureCode_t {
    ///!< Nvidia RTX
    pub const NVML_GRID_LICENSE_FEATURE_CODE_NVIDIA_RTX: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        2,
    );
}
impl nvmlGridLicenseFeatureCode_t {
    ///!< Deprecated, do not use.
    pub const NVML_GRID_LICENSE_FEATURE_CODE_VWORKSTATION: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        2,
    );
}
impl nvmlGridLicenseFeatureCode_t {
    ///!< Gaming
    pub const NVML_GRID_LICENSE_FEATURE_CODE_GAMING: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        3,
    );
}
impl nvmlGridLicenseFeatureCode_t {
    ///!< Compute
    pub const NVML_GRID_LICENSE_FEATURE_CODE_COMPUTE: nvmlGridLicenseFeatureCode_t = nvmlGridLicenseFeatureCode_t(
        4,
    );
}
#[repr(transparent)]
/// vGPU software licensable features
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGridLicenseFeatureCode_t(pub ::core::ffi::c_uint);
impl nvmlVgpuCapability_enum {
    ///!< P2P over NVLink is supported
    pub const NVML_VGPU_CAP_NVLINK_P2P: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(
        0,
    );
}
impl nvmlVgpuCapability_enum {
    ///!< GPUDirect capability is supported
    pub const NVML_VGPU_CAP_GPUDIRECT: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(
        1,
    );
}
impl nvmlVgpuCapability_enum {
    ///!< vGPU profile cannot be mixed with other vGPU profiles in same VM
    pub const NVML_VGPU_CAP_MULTI_VGPU_EXCLUSIVE: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(
        2,
    );
}
impl nvmlVgpuCapability_enum {
    ///!< vGPU profile cannot run on a GPU alongside other profiles of different type
    pub const NVML_VGPU_CAP_EXCLUSIVE_TYPE: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(
        3,
    );
}
impl nvmlVgpuCapability_enum {
    ///!< vGPU profile cannot run on a GPU alongside other profiles of different size
    pub const NVML_VGPU_CAP_EXCLUSIVE_SIZE: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(
        4,
    );
}
impl nvmlVgpuCapability_enum {
    pub const NVML_VGPU_CAP_COUNT: nvmlVgpuCapability_enum = nvmlVgpuCapability_enum(5);
}
#[repr(transparent)]
/// vGPU queryable capabilities
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuCapability_enum(pub ::core::ffi::c_uint);
/// vGPU queryable capabilities
pub use self::nvmlVgpuCapability_enum as nvmlVgpuCapability_t;
impl nvmlVgpuDriverCapability_enum {
    ///!< Supports mixing of different vGPU profiles within one guest VM
    pub const NVML_VGPU_DRIVER_CAP_HETEROGENEOUS_MULTI_VGPU: nvmlVgpuDriverCapability_enum = nvmlVgpuDriverCapability_enum(
        0,
    );
}
impl nvmlVgpuDriverCapability_enum {
    pub const NVML_VGPU_DRIVER_CAP_COUNT: nvmlVgpuDriverCapability_enum = nvmlVgpuDriverCapability_enum(
        1,
    );
}
#[repr(transparent)]
/// vGPU driver queryable capabilities
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuDriverCapability_enum(pub ::core::ffi::c_uint);
/// vGPU driver queryable capabilities
pub use self::nvmlVgpuDriverCapability_enum as nvmlVgpuDriverCapability_t;
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query if the fractional vGPU profiles on this GPU can be used in multi-vGPU configurations
    pub const NVML_DEVICE_VGPU_CAP_FRACTIONAL_MULTI_VGPU: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        0,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query if the GPU support concurrent execution of timesliced vGPU profiles of differing types
    pub const NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_PROFILES: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        1,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query if the GPU support concurrent execution of timesliced vGPU profiles of differing framebuffer sizes
    pub const NVML_DEVICE_VGPU_CAP_HETEROGENEOUS_TIMESLICE_SIZES: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        2,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query the GPU's read_device_buffer expected bandwidth capacity in megabytes per second
    pub const NVML_DEVICE_VGPU_CAP_READ_DEVICE_BUFFER_BW: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        3,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query the GPU's write_device_buffer expected bandwidth capacity in megabytes per second
    pub const NVML_DEVICE_VGPU_CAP_WRITE_DEVICE_BUFFER_BW: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        4,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Query if vGPU profiles on the GPU supports migration data streaming
    pub const NVML_DEVICE_VGPU_CAP_DEVICE_STREAMING: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        5,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Set/Get support for mini-quarter vGPU profiles
    pub const NVML_DEVICE_VGPU_CAP_MINI_QUARTER_GPU: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        6,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    ///!< Set/Get support for compute media engine vGPU profiles
    pub const NVML_DEVICE_VGPU_CAP_COMPUTE_MEDIA_ENGINE_GPU: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        7,
    );
}
impl nvmlDeviceVgpuCapability_enum {
    pub const NVML_DEVICE_VGPU_CAP_COUNT: nvmlDeviceVgpuCapability_enum = nvmlDeviceVgpuCapability_enum(
        8,
    );
}
#[repr(transparent)]
/// Device vGPU queryable capabilities
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlDeviceVgpuCapability_enum(pub ::core::ffi::c_uint);
/// Device vGPU queryable capabilities
pub use self::nvmlDeviceVgpuCapability_enum as nvmlDeviceVgpuCapability_t;
#[doc = "/\n/** @defgroup nvmlVgpuStructs vGPU Structs\n  @{\n/\n/"]
pub type nvmlVgpuTypeId_t = ::core::ffi::c_uint;
pub type nvmlVgpuInstance_t = ::core::ffi::c_uint;
/// Structure to store the vGPU heterogeneous mode of device -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuHeterogeneousMode_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< The vGPU heterogeneous mode
    pub mode: ::core::ffi::c_uint,
}
/// Structure to store the vGPU heterogeneous mode of device -- version 1
pub type nvmlVgpuHeterogeneousMode_t = nvmlVgpuHeterogeneousMode_v1_t;
/// Structure to store the placement ID of vGPU instance -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuPlacementId_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< Placement ID of the active vGPU instance
    pub placementId: ::core::ffi::c_uint,
}
/// Structure to store the placement ID of vGPU instance -- version 1
pub type nvmlVgpuPlacementId_t = nvmlVgpuPlacementId_v1_t;
/// Structure to store the list of vGPU placements -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuPlacementList_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< The number of slots occupied by the vGPU type
    pub placementSize: ::core::ffi::c_uint,
    ///!< Count of placement IDs fetched
    pub count: ::core::ffi::c_uint,
    ///!< Placement IDs for the vGPU type
    pub placementIds: *mut ::core::ffi::c_uint,
}
/// Structure to store the list of vGPU placements -- version 1
pub type nvmlVgpuPlacementList_t = nvmlVgpuPlacementList_v1_t;
/// Structure to store Utilization Value and vgpuInstance
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuInstanceUtilizationSample_st {
    ///!< vGPU Instance
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: nvmlValue_t,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: nvmlValue_t,
    ///!< Encoder Util Value
    pub encUtil: nvmlValue_t,
    ///!< Decoder Util Value
    pub decUtil: nvmlValue_t,
}
/// Structure to store Utilization Value and vgpuInstance
pub type nvmlVgpuInstanceUtilizationSample_t = nvmlVgpuInstanceUtilizationSample_st;
/// Structure to store Utilization Value and vgpuInstance Info -- Version 1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuInstanceUtilizationInfo_v1_t {
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< vGPU Instance
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: nvmlValue_t,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: nvmlValue_t,
    ///!< Encoder Util Value
    pub encUtil: nvmlValue_t,
    ///!< Decoder Util Value
    pub decUtil: nvmlValue_t,
    ///!< Jpeg Util Value
    pub jpgUtil: nvmlValue_t,
    ///!< Ofa Util Value
    pub ofaUtil: nvmlValue_t,
}
/// Structure to store recent utilization for vGPU instances running on a device -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuInstancesUtilizationInfo_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< Hold the type of returned sample values
    pub sampleValType: nvmlValueType_t,
    ///!< Hold the number of vGPU instances
    pub vgpuInstanceCount: ::core::ffi::c_uint,
    ///!< Return only samples with timestamp greater than lastSeenTimeStamp
    pub lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    ///!< The array (allocated by caller) in which vGPU utilization are returned
    pub vgpuUtilArray: *mut nvmlVgpuInstanceUtilizationInfo_v1_t,
}
/// Structure to store recent utilization for vGPU instances running on a device -- version 1
pub type nvmlVgpuInstancesUtilizationInfo_t = nvmlVgpuInstancesUtilizationInfo_v1_t;
/// Structure to store Utilization Value, vgpuInstance and subprocess information
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuProcessUtilizationSample_st {
    ///!< vGPU Instance
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< PID of process running within the vGPU VM
    pub pid: ::core::ffi::c_uint,
    ///!< Name of process running within the vGPU VM
    pub processName: [::core::ffi::c_char; 64usize],
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: ::core::ffi::c_uint,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: ::core::ffi::c_uint,
    ///!< Encoder Util Value
    pub encUtil: ::core::ffi::c_uint,
    ///!< Decoder Util Value
    pub decUtil: ::core::ffi::c_uint,
}
/// Structure to store Utilization Value, vgpuInstance and subprocess information
pub type nvmlVgpuProcessUtilizationSample_t = nvmlVgpuProcessUtilizationSample_st;
/// Structure to store Utilization Value, vgpuInstance and subprocess information for process running on vGPU instance -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuProcessUtilizationInfo_v1_t {
    ///!< Name of process running within the vGPU VM
    pub processName: [::core::ffi::c_char; 64usize],
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< vGPU Instance
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< PID of process running within the vGPU VM
    pub pid: ::core::ffi::c_uint,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: ::core::ffi::c_uint,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: ::core::ffi::c_uint,
    ///!< Encoder Util Value
    pub encUtil: ::core::ffi::c_uint,
    ///!< Decoder Util Value
    pub decUtil: ::core::ffi::c_uint,
    ///!< Jpeg Util Value
    pub jpgUtil: ::core::ffi::c_uint,
    ///!< Ofa Util Value
    pub ofaUtil: ::core::ffi::c_uint,
}
/// Structure to store recent utilization, vgpuInstance and subprocess information for processes running on vGPU instances active on a device -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuProcessesUtilizationInfo_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< Hold the number of processes running on vGPU instances
    pub vgpuProcessCount: ::core::ffi::c_uint,
    ///!< Return only samples with timestamp greater than lastSeenTimeStamp
    pub lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    ///!< The array (allocated by caller) in which utilization of processes running on vGPU instances are returned
    pub vgpuProcUtilArray: *mut nvmlVgpuProcessUtilizationInfo_v1_t,
}
/// Structure to store recent utilization, vgpuInstance and subprocess information for processes running on vGPU instances active on a device -- version 1
pub type nvmlVgpuProcessesUtilizationInfo_t = nvmlVgpuProcessesUtilizationInfo_v1_t;
/// Union to represent the vGPU Scheduler Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlVgpuSchedulerParams_t {
    pub vgpuSchedDataWithARR: nvmlVgpuSchedulerParams_t__bindgen_ty_1,
    pub vgpuSchedData: nvmlVgpuSchedulerParams_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerParams_t__bindgen_ty_1 {
    ///!< Average factor in compensating the timeslice for Adaptive Round Robin mode
    pub avgFactor: ::core::ffi::c_uint,
    ///!< The timeslice in ns for each software run list as configured, or the default value otherwise
    pub timeslice: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerParams_t__bindgen_ty_2 {
    ///!< The timeslice in ns for each software run list as configured, or the default value otherwise
    pub timeslice: ::core::ffi::c_uint,
}
/// Structure to store the state and logs of a software runlist
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerLogEntries_st {
    ///!< Timestamp in ns when this software runlist was preeempted
    pub timestamp: ::core::ffi::c_ulonglong,
    ///!< Total time in ns this software runlist has run
    pub timeRunTotal: ::core::ffi::c_ulonglong,
    ///!< Time in ns this software runlist ran before preemption
    pub timeRun: ::core::ffi::c_ulonglong,
    ///!< Software runlist Id
    pub swRunlistId: ::core::ffi::c_uint,
    ///!< The actual timeslice after deduction
    pub targetTimeSlice: ::core::ffi::c_ulonglong,
    ///!< Preemption time in ns for this SW runlist
    pub cumulativePreemptionTime: ::core::ffi::c_ulonglong,
}
/// Structure to store the state and logs of a software runlist
pub type nvmlVgpuSchedulerLogEntry_t = nvmlVgpuSchedulerLogEntries_st;
/// Structure to store a vGPU software scheduler log
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerLog_st {
    ///!< Engine whose software runlist log entries are fetched
    pub engineId: ::core::ffi::c_uint,
    ///!< Scheduler policy
    pub schedulerPolicy: ::core::ffi::c_uint,
    ///!< Adaptive Round Robin scheduler mode. One of the NVML_VGPU_SCHEDULER_ARR_*.
    pub arrMode: ::core::ffi::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
    ///!< Count of log entries fetched
    pub entriesCount: ::core::ffi::c_uint,
    pub logEntries: [nvmlVgpuSchedulerLogEntry_t; 200usize],
}
/// Structure to store a vGPU software scheduler log
pub type nvmlVgpuSchedulerLog_t = nvmlVgpuSchedulerLog_st;
/// Structure to store the vGPU scheduler state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerGetState_st {
    ///!< Scheduler policy
    pub schedulerPolicy: ::core::ffi::c_uint,
    ///!< Adaptive Round Robin scheduler mode. One of the NVML_VGPU_SCHEDULER_ARR_*.
    pub arrMode: ::core::ffi::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerParams_t,
}
/// Structure to store the vGPU scheduler state
pub type nvmlVgpuSchedulerGetState_t = nvmlVgpuSchedulerGetState_st;
/// Union to represent the vGPU Scheduler set Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub union nvmlVgpuSchedulerSetParams_t {
    pub vgpuSchedDataWithARR: nvmlVgpuSchedulerSetParams_t__bindgen_ty_1,
    pub vgpuSchedData: nvmlVgpuSchedulerSetParams_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerSetParams_t__bindgen_ty_1 {
    ///!< Average factor in compensating the timeslice for Adaptive Round Robin mode
    pub avgFactor: ::core::ffi::c_uint,
    ///!< Frequency for Adaptive Round Robin mode
    pub frequency: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerSetParams_t__bindgen_ty_2 {
    ///!< The timeslice in ns(Nanoseconds) for each software run list as configured, or the default value otherwise
    pub timeslice: ::core::ffi::c_uint,
}
/// Structure to set the vGPU scheduler state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlVgpuSchedulerSetState_st {
    ///!< Scheduler policy
    pub schedulerPolicy: ::core::ffi::c_uint,
    ///!< Adaptive Round Robin scheduler
    pub enableARRMode: ::core::ffi::c_uint,
    pub schedulerParams: nvmlVgpuSchedulerSetParams_t,
}
/// Structure to set the vGPU scheduler state
pub type nvmlVgpuSchedulerSetState_t = nvmlVgpuSchedulerSetState_st;
/// Structure to store the vGPU scheduler capabilities
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuSchedulerCapabilities_st {
    ///!< List the supported vGPU schedulers on the device
    pub supportedSchedulers: [::core::ffi::c_uint; 3usize],
    ///!< Maximum timeslice value in ns
    pub maxTimeslice: ::core::ffi::c_uint,
    ///!< Minimum timeslice value in ns
    pub minTimeslice: ::core::ffi::c_uint,
    ///!< Flag to check Adaptive Round Robin mode enabled/disabled.
    pub isArrModeSupported: ::core::ffi::c_uint,
    ///!< Maximum frequency for Adaptive Round Robin mode
    pub maxFrequencyForARR: ::core::ffi::c_uint,
    ///!< Minimum frequency for Adaptive Round Robin mode
    pub minFrequencyForARR: ::core::ffi::c_uint,
    ///!< Maximum averaging factor for Adaptive Round Robin mode
    pub maxAvgFactorForARR: ::core::ffi::c_uint,
    ///!< Minimum averaging factor for Adaptive Round Robin mode
    pub minAvgFactorForARR: ::core::ffi::c_uint,
}
/// Structure to store the vGPU scheduler capabilities
pub type nvmlVgpuSchedulerCapabilities_t = nvmlVgpuSchedulerCapabilities_st;
/// Structure to store the vGPU license expiry details
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuLicenseExpiry_st {
    ///!< Year of license expiry
    pub year: ::core::ffi::c_uint,
    ///!< Month of license expiry
    pub month: ::core::ffi::c_ushort,
    ///!< Day of license expiry
    pub day: ::core::ffi::c_ushort,
    ///!< Hour of license expiry
    pub hour: ::core::ffi::c_ushort,
    ///!< Minutes of license expiry
    pub min: ::core::ffi::c_ushort,
    ///!< Seconds of license expiry
    pub sec: ::core::ffi::c_ushort,
    ///!< License expiry status
    pub status: ::core::ffi::c_uchar,
}
/// Structure to store the vGPU license expiry details
pub type nvmlVgpuLicenseExpiry_t = nvmlVgpuLicenseExpiry_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuLicenseInfo_st {
    ///!< License status
    pub isLicensed: ::core::ffi::c_uchar,
    ///!< License expiry information
    pub licenseExpiry: nvmlVgpuLicenseExpiry_t,
    ///!< Current license state
    pub currentState: ::core::ffi::c_uint,
}
pub type nvmlVgpuLicenseInfo_t = nvmlVgpuLicenseInfo_st;
/// Structure to store utilization value and process Id
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessUtilizationSample_st {
    ///!< PID of process
    pub pid: ::core::ffi::c_uint,
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: ::core::ffi::c_uint,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: ::core::ffi::c_uint,
    ///!< Encoder Util Value
    pub encUtil: ::core::ffi::c_uint,
    ///!< Decoder Util Value
    pub decUtil: ::core::ffi::c_uint,
}
/// Structure to store utilization value and process Id
pub type nvmlProcessUtilizationSample_t = nvmlProcessUtilizationSample_st;
/// Structure to store utilization value and process Id -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessUtilizationInfo_v1_t {
    ///!< CPU Timestamp in microseconds
    pub timeStamp: ::core::ffi::c_ulonglong,
    ///!< PID of process
    pub pid: ::core::ffi::c_uint,
    ///!< SM (3D/Compute) Util Value
    pub smUtil: ::core::ffi::c_uint,
    ///!< Frame Buffer Memory Util Value
    pub memUtil: ::core::ffi::c_uint,
    ///!< Encoder Util Value
    pub encUtil: ::core::ffi::c_uint,
    ///!< Decoder Util Value
    pub decUtil: ::core::ffi::c_uint,
    ///!< Jpeg Util Value
    pub jpgUtil: ::core::ffi::c_uint,
    ///!< Ofa Util Value
    pub ofaUtil: ::core::ffi::c_uint,
}
/// Structure to store utilization and process ID for each running process -- version 1
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlProcessesUtilizationInfo_v1_t {
    ///!< The version number of this struct
    pub version: ::core::ffi::c_uint,
    ///!< Caller-supplied array size, and returns number of processes running
    pub processSamplesCount: ::core::ffi::c_uint,
    ///!< Return only samples with timestamp greater than lastSeenTimeStamp
    pub lastSeenTimeStamp: ::core::ffi::c_ulonglong,
    ///!< The array (allocated by caller) of the utilization of GPU SM, framebuffer, video encoder, video decoder, JPEG, and OFA
    pub procUtilArray: *mut nvmlProcessUtilizationInfo_v1_t,
}
/// Structure to store utilization and process ID for each running process -- version 1
pub type nvmlProcessesUtilizationInfo_t = nvmlProcessesUtilizationInfo_v1_t;
/// Structure to store license expiry date and time values
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGridLicenseExpiry_st {
    ///!< Year value of license expiry
    pub year: ::core::ffi::c_uint,
    ///!< Month value of license expiry
    pub month: ::core::ffi::c_ushort,
    ///!< Day value of license expiry
    pub day: ::core::ffi::c_ushort,
    ///!< Hour value of license expiry
    pub hour: ::core::ffi::c_ushort,
    ///!< Minutes value of license expiry
    pub min: ::core::ffi::c_ushort,
    ///!< Seconds value of license expiry
    pub sec: ::core::ffi::c_ushort,
    ///!< License expiry status
    pub status: ::core::ffi::c_uchar,
}
/// Structure to store license expiry date and time values
pub type nvmlGridLicenseExpiry_t = nvmlGridLicenseExpiry_st;
/// Structure containing vGPU software licensable feature information
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGridLicensableFeature_st {
    ///!< Licensed feature code
    pub featureCode: nvmlGridLicenseFeatureCode_t,
    ///!< Non-zero if feature is currently licensed, otherwise zero
    pub featureState: ::core::ffi::c_uint,
    ///!< Deprecated.
    pub licenseInfo: [::core::ffi::c_char; 128usize],
    ///!< Product name of feature
    pub productName: [::core::ffi::c_char; 128usize],
    ///!< Non-zero if feature is enabled, otherwise zero
    pub featureEnabled: ::core::ffi::c_uint,
    ///!< License expiry structure containing date and time
    pub licenseExpiry: nvmlGridLicenseExpiry_t,
}
/// Structure containing vGPU software licensable feature information
pub type nvmlGridLicensableFeature_t = nvmlGridLicensableFeature_st;
/// Structure to store vGPU software licensable features
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGridLicensableFeatures_st {
    ///!< Non-zero if vGPU Software Licensing is supported on the system, otherwise zero
    pub isGridLicenseSupported: ::core::ffi::c_int,
    ///!< Entries returned in \a gridLicensableFeatures array
    pub licensableFeaturesCount: ::core::ffi::c_uint,
    ///!< Array of vGPU software licensable features.
    pub gridLicensableFeatures: [nvmlGridLicensableFeature_t; 3usize],
}
/// Structure to store vGPU software licensable features
pub type nvmlGridLicensableFeatures_t = nvmlGridLicensableFeatures_st;
/// Structure to store SRAM uncorrectable error counters
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEccSramErrorStatus_v1_t {
    ///!< the API version number
    pub version: ::core::ffi::c_uint,
    ///!< aggregate uncorrectable parity error count
    pub aggregateUncParity: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable SEC-DED error count
    pub aggregateUncSecDed: ::core::ffi::c_ulonglong,
    ///!< aggregate correctable error count
    pub aggregateCor: ::core::ffi::c_ulonglong,
    ///!< volatile uncorrectable parity error count
    pub volatileUncParity: ::core::ffi::c_ulonglong,
    ///!< volatile uncorrectable SEC-DED error count
    pub volatileUncSecDed: ::core::ffi::c_ulonglong,
    ///!< volatile correctable error count
    pub volatileCor: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable error count for L2 cache bucket
    pub aggregateUncBucketL2: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable error count for SM bucket
    pub aggregateUncBucketSm: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable error count for PCIE bucket
    pub aggregateUncBucketPcie: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable error count for Microcontroller bucket
    pub aggregateUncBucketMcu: ::core::ffi::c_ulonglong,
    ///!< aggregate uncorrectable error count for Other bucket
    pub aggregateUncBucketOther: ::core::ffi::c_ulonglong,
    ///!< if the error threshold of field diag is exceeded
    pub bThresholdExceeded: ::core::ffi::c_uint,
}
/// Structure to store SRAM uncorrectable error counters
pub type nvmlEccSramErrorStatus_t = nvmlEccSramErrorStatus_v1_t;
pub type nvmlDeviceArchitecture_t = ::core::ffi::c_uint;
pub type nvmlBusType_t = ::core::ffi::c_uint;
pub type nvmlFanControlPolicy_t = ::core::ffi::c_uint;
pub type nvmlPowerSource_t = ::core::ffi::c_uint;
impl nvmlGpuUtilizationDomainId_t {
    ///!< Graphics engine domain
    pub const NVML_GPU_UTILIZATION_DOMAIN_GPU: nvmlGpuUtilizationDomainId_t = nvmlGpuUtilizationDomainId_t(
        0,
    );
}
impl nvmlGpuUtilizationDomainId_t {
    ///!< Frame buffer domain
    pub const NVML_GPU_UTILIZATION_DOMAIN_FB: nvmlGpuUtilizationDomainId_t = nvmlGpuUtilizationDomainId_t(
        1,
    );
}
impl nvmlGpuUtilizationDomainId_t {
    ///!< Video engine domain
    pub const NVML_GPU_UTILIZATION_DOMAIN_VID: nvmlGpuUtilizationDomainId_t = nvmlGpuUtilizationDomainId_t(
        2,
    );
}
impl nvmlGpuUtilizationDomainId_t {
    ///!< Bus interface domain
    pub const NVML_GPU_UTILIZATION_DOMAIN_BUS: nvmlGpuUtilizationDomainId_t = nvmlGpuUtilizationDomainId_t(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuUtilizationDomainId_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuDynamicPstatesInfo_st {
    ///!< Reserved for future use
    pub flags: ::core::ffi::c_uint,
    pub utilization: [nvmlGpuDynamicPstatesInfo_st__bindgen_ty_1; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuDynamicPstatesInfo_st__bindgen_ty_1 {
    ///!< Set if this utilization domain is present on this GPU
    pub bIsPresent: ::core::ffi::c_uint,
    ///!< Percentage of time where the domain is considered busy in the last 1-second interval
    pub percentage: ::core::ffi::c_uint,
    ///!< Utilization threshold that can trigger a perf-increasing P-State change when crossed
    pub incThreshold: ::core::ffi::c_uint,
    ///!< Utilization threshold that can trigger a perf-decreasing P-State change when crossed
    pub decThreshold: ::core::ffi::c_uint,
}
pub type nvmlGpuDynamicPstatesInfo_t = nvmlGpuDynamicPstatesInfo_st;
/// Information for a Field Value Sample
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvmlFieldValue_st {
    ///!< ID of the NVML field to retrieve. This must be set before any call that uses this struct. See the constants starting with NVML_FI_ above.
    pub fieldId: ::core::ffi::c_uint,
    ///!< Scope ID can represent data used by NVML depending on fieldId's context. For example, for NVLink throughput counter data, scopeId can represent linkId.
    pub scopeId: ::core::ffi::c_uint,
    ///!< CPU Timestamp of this value in microseconds since 1970
    pub timestamp: ::core::ffi::c_longlong,
    ///!< How long this field value took to update (in usec) within NVML. This may be averaged across several fields that are serviced by the same driver call.
    pub latencyUsec: ::core::ffi::c_longlong,
    ///!< Type of the value stored in value
    pub valueType: nvmlValueType_t,
    ///!< Return code for retrieving this value. This must be checked before looking at value, as value is undefined if nvmlReturn != NVML_SUCCESS
    pub nvmlReturn: nvmlReturn_t,
    ///!< Value for this field. This is only valid if nvmlReturn == NVML_SUCCESS
    pub value: nvmlValue_t,
}
/// Information for a Field Value Sample
pub type nvmlFieldValue_t = nvmlFieldValue_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlUnit_st {
    _unused: [u8; 0],
}
#[doc = "/\n/** @defgroup nvmlUnitStructs Unit Structs\n  @{\n/\n/"]
pub type nvmlUnit_t = *mut nvmlUnit_st;
/// Description of HWBC entry
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlHwbcEntry_st {
    pub hwbcId: ::core::ffi::c_uint,
    pub firmwareVersion: [::core::ffi::c_char; 32usize],
}
/// Description of HWBC entry
pub type nvmlHwbcEntry_t = nvmlHwbcEntry_st;
impl nvmlFanState_enum {
    ///!< Fan is working properly
    pub const NVML_FAN_NORMAL: nvmlFanState_enum = nvmlFanState_enum(0);
}
impl nvmlFanState_enum {
    ///!< Fan has failed
    pub const NVML_FAN_FAILED: nvmlFanState_enum = nvmlFanState_enum(1);
}
#[repr(transparent)]
/// Fan state enum.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlFanState_enum(pub ::core::ffi::c_uint);
/// Fan state enum.
pub use self::nvmlFanState_enum as nvmlFanState_t;
impl nvmlLedColor_enum {
    ///!< GREEN, indicates good health
    pub const NVML_LED_COLOR_GREEN: nvmlLedColor_enum = nvmlLedColor_enum(0);
}
impl nvmlLedColor_enum {
    ///!< AMBER, indicates problem
    pub const NVML_LED_COLOR_AMBER: nvmlLedColor_enum = nvmlLedColor_enum(1);
}
#[repr(transparent)]
/// Led color enum.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlLedColor_enum(pub ::core::ffi::c_uint);
/// Led color enum.
pub use self::nvmlLedColor_enum as nvmlLedColor_t;
/// LED states for an S-class unit.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlLedState_st {
    ///!< If amber, a text description of the cause
    pub cause: [::core::ffi::c_char; 256usize],
    ///!< GREEN or AMBER
    pub color: nvmlLedColor_t,
}
/// LED states for an S-class unit.
pub type nvmlLedState_t = nvmlLedState_st;
/// Static S-class unit info.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlUnitInfo_st {
    ///!< Product name
    pub name: [::core::ffi::c_char; 96usize],
    ///!< Product identifier
    pub id: [::core::ffi::c_char; 96usize],
    ///!< Product serial number
    pub serial: [::core::ffi::c_char; 96usize],
    ///!< Firmware version
    pub firmwareVersion: [::core::ffi::c_char; 96usize],
}
/// Static S-class unit info.
pub type nvmlUnitInfo_t = nvmlUnitInfo_st;
/** Power usage information for an S-class unit.
 The power supply state is a human readable string that equals "Normal" or contains
 a combination of "Abnormal" plus one or more of the following:

    - High voltage
    - Fan failure
    - Heatsink temperature
    - Current limit
    - Voltage below UV alarm threshold
    - Low-voltage
    - SI2C remote off command
    - MOD_DISABLE input
    - Short pin transition*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPSUInfo_st {
    ///!< The power supply state
    pub state: [::core::ffi::c_char; 256usize],
    ///!< PSU current (A)
    pub current: ::core::ffi::c_uint,
    ///!< PSU voltage (V)
    pub voltage: ::core::ffi::c_uint,
    ///!< PSU power draw (W)
    pub power: ::core::ffi::c_uint,
}
/** Power usage information for an S-class unit.
 The power supply state is a human readable string that equals "Normal" or contains
 a combination of "Abnormal" plus one or more of the following:

    - High voltage
    - Fan failure
    - Heatsink temperature
    - Current limit
    - Voltage below UV alarm threshold
    - Low-voltage
    - SI2C remote off command
    - MOD_DISABLE input
    - Short pin transition*/
pub type nvmlPSUInfo_t = nvmlPSUInfo_st;
/// Fan speed reading for a single fan in an S-class unit.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlUnitFanInfo_st {
    ///!< Fan speed (RPM)
    pub speed: ::core::ffi::c_uint,
    ///!< Flag that indicates whether fan is working properly
    pub state: nvmlFanState_t,
}
/// Fan speed reading for a single fan in an S-class unit.
pub type nvmlUnitFanInfo_t = nvmlUnitFanInfo_st;
/// Fan speed readings for an entire S-class unit.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlUnitFanSpeeds_st {
    ///!< Fan speed data for each fan
    pub fans: [nvmlUnitFanInfo_t; 24usize],
    ///!< Number of fans in unit
    pub count: ::core::ffi::c_uint,
}
/// Fan speed readings for an entire S-class unit.
pub type nvmlUnitFanSpeeds_t = nvmlUnitFanSpeeds_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlEventSet_st {
    _unused: [u8; 0],
}
/// Handle to an event set
pub type nvmlEventSet_t = *mut nvmlEventSet_st;
/// Information about occurred event
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEventData_st {
    ///!< Specific device where the event occurred
    pub device: nvmlDevice_t,
    ///!< Information about what specific event occurred
    pub eventType: ::core::ffi::c_ulonglong,
    ///!< Stores XID error for the device in the event of nvmlEventTypeXidCriticalError,
    pub eventData: ::core::ffi::c_ulonglong,
    ///!< If MIG is enabled and nvmlEventTypeXidCriticalError event is attributable to a GPU
    pub gpuInstanceId: ::core::ffi::c_uint,
    ///!< If MIG is enabled and nvmlEventTypeXidCriticalError event is attributable to a
    pub computeInstanceId: ::core::ffi::c_uint,
}
/// Information about occurred event
pub type nvmlEventData_t = nvmlEventData_st;
/// Describes accounting statistics of a process.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlAccountingStats_st {
    ///!< Percent of time over the process's lifetime during which one or more kernels was executing on the GPU.
    pub gpuUtilization: ::core::ffi::c_uint,
    ///!< Percent of time over the process's lifetime during which global (device) memory was being read or written.
    pub memoryUtilization: ::core::ffi::c_uint,
    ///!< Maximum total memory in bytes that was ever allocated by the process.
    pub maxMemoryUsage: ::core::ffi::c_ulonglong,
    /**!< Amount of time in ms during which the compute context was active. The time is reported as 0 if
!< the process is not terminated*/
    pub time: ::core::ffi::c_ulonglong,
    ///!< CPU Timestamp in usec representing start time for the process
    pub startTime: ::core::ffi::c_ulonglong,
    ///!< Flag to represent if the process is running (1 for running, 0 for terminated)
    pub isRunning: ::core::ffi::c_uint,
    ///!< Reserved for future use
    pub reserved: [::core::ffi::c_uint; 5usize],
}
/// Describes accounting statistics of a process.
pub type nvmlAccountingStats_t = nvmlAccountingStats_st;
impl nvmlEncoderQueryType_enum {
    ///!< H264 encoder
    pub const NVML_ENCODER_QUERY_H264: nvmlEncoderQueryType_enum = nvmlEncoderQueryType_enum(
        0,
    );
}
impl nvmlEncoderQueryType_enum {
    ///!< HEVC encoder
    pub const NVML_ENCODER_QUERY_HEVC: nvmlEncoderQueryType_enum = nvmlEncoderQueryType_enum(
        1,
    );
}
impl nvmlEncoderQueryType_enum {
    ///!< AV1 encoder
    pub const NVML_ENCODER_QUERY_AV1: nvmlEncoderQueryType_enum = nvmlEncoderQueryType_enum(
        2,
    );
}
impl nvmlEncoderQueryType_enum {
    ///!< Unknown encoder
    pub const NVML_ENCODER_QUERY_UNKNOWN: nvmlEncoderQueryType_enum = nvmlEncoderQueryType_enum(
        255,
    );
}
#[repr(transparent)]
/// Represents type of encoder for capacity can be queried
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEncoderQueryType_enum(pub ::core::ffi::c_uint);
/// Represents type of encoder for capacity can be queried
pub use self::nvmlEncoderQueryType_enum as nvmlEncoderType_t;
/// Structure to hold encoder session data
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlEncoderSessionInfo_st {
    ///!< Unique session ID
    pub sessionId: ::core::ffi::c_uint,
    ///!< Owning process ID
    pub pid: ::core::ffi::c_uint,
    ///!< Owning vGPU instance ID (only valid on vGPU hosts, otherwise zero)
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< Video encoder type
    pub codecType: nvmlEncoderType_t,
    ///!< Current encode horizontal resolution
    pub hResolution: ::core::ffi::c_uint,
    ///!< Current encode vertical resolution
    pub vResolution: ::core::ffi::c_uint,
    ///!< Moving average encode frames per second
    pub averageFps: ::core::ffi::c_uint,
    ///!< Moving average encode latency in microseconds
    pub averageLatency: ::core::ffi::c_uint,
}
/// Structure to hold encoder session data
pub type nvmlEncoderSessionInfo_t = nvmlEncoderSessionInfo_st;
impl nvmlFBCSessionType_enum {
    ///!< Unknown
    pub const NVML_FBC_SESSION_TYPE_UNKNOWN: nvmlFBCSessionType_enum = nvmlFBCSessionType_enum(
        0,
    );
}
impl nvmlFBCSessionType_enum {
    ///!< ToSys
    pub const NVML_FBC_SESSION_TYPE_TOSYS: nvmlFBCSessionType_enum = nvmlFBCSessionType_enum(
        1,
    );
}
impl nvmlFBCSessionType_enum {
    ///!< Cuda
    pub const NVML_FBC_SESSION_TYPE_CUDA: nvmlFBCSessionType_enum = nvmlFBCSessionType_enum(
        2,
    );
}
impl nvmlFBCSessionType_enum {
    ///!< Vid
    pub const NVML_FBC_SESSION_TYPE_VID: nvmlFBCSessionType_enum = nvmlFBCSessionType_enum(
        3,
    );
}
impl nvmlFBCSessionType_enum {
    ///!< HEnc
    pub const NVML_FBC_SESSION_TYPE_HWENC: nvmlFBCSessionType_enum = nvmlFBCSessionType_enum(
        4,
    );
}
#[repr(transparent)]
/// Represents frame buffer capture session type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlFBCSessionType_enum(pub ::core::ffi::c_uint);
/// Represents frame buffer capture session type
pub use self::nvmlFBCSessionType_enum as nvmlFBCSessionType_t;
/// Structure to hold frame buffer capture sessions stats
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlFBCStats_st {
    ///!< Total no of sessions
    pub sessionsCount: ::core::ffi::c_uint,
    ///!< Moving average new frames captured per second
    pub averageFPS: ::core::ffi::c_uint,
    ///!< Moving average new frame capture latency in microseconds
    pub averageLatency: ::core::ffi::c_uint,
}
/// Structure to hold frame buffer capture sessions stats
pub type nvmlFBCStats_t = nvmlFBCStats_st;
/// Structure to hold FBC session data
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlFBCSessionInfo_st {
    ///!< Unique session ID
    pub sessionId: ::core::ffi::c_uint,
    ///!< Owning process ID
    pub pid: ::core::ffi::c_uint,
    ///!< Owning vGPU instance ID (only valid on vGPU hosts, otherwise zero)
    pub vgpuInstance: nvmlVgpuInstance_t,
    ///!< Display identifier
    pub displayOrdinal: ::core::ffi::c_uint,
    ///!< Type of frame buffer capture session
    pub sessionType: nvmlFBCSessionType_t,
    ///!< Session flags (one or more of NVML_NVFBC_SESSION_FLAG_XXX).
    pub sessionFlags: ::core::ffi::c_uint,
    ///!< Max horizontal resolution supported by the capture session
    pub hMaxResolution: ::core::ffi::c_uint,
    ///!< Max vertical resolution supported by the capture session
    pub vMaxResolution: ::core::ffi::c_uint,
    ///!< Horizontal resolution requested by caller in capture call
    pub hResolution: ::core::ffi::c_uint,
    ///!< Vertical resolution requested by caller in capture call
    pub vResolution: ::core::ffi::c_uint,
    ///!< Moving average new frames captured per second
    pub averageFPS: ::core::ffi::c_uint,
    ///!< Moving average new frame capture latency in microseconds
    pub averageLatency: ::core::ffi::c_uint,
}
/// Structure to hold FBC session data
pub type nvmlFBCSessionInfo_t = nvmlFBCSessionInfo_st;
impl nvmlDetachGpuState_enum {
    pub const NVML_DETACH_GPU_KEEP: nvmlDetachGpuState_enum = nvmlDetachGpuState_enum(0);
}
impl nvmlDetachGpuState_enum {
    pub const NVML_DETACH_GPU_REMOVE: nvmlDetachGpuState_enum = nvmlDetachGpuState_enum(
        1,
    );
}
#[repr(transparent)]
///  Is the GPU device to be removed from the kernel by nvmlDeviceRemoveGpu()
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlDetachGpuState_enum(pub ::core::ffi::c_uint);
///  Is the GPU device to be removed from the kernel by nvmlDeviceRemoveGpu()
pub use self::nvmlDetachGpuState_enum as nvmlDetachGpuState_t;
impl nvmlPcieLinkState_enum {
    pub const NVML_PCIE_LINK_KEEP: nvmlPcieLinkState_enum = nvmlPcieLinkState_enum(0);
}
impl nvmlPcieLinkState_enum {
    pub const NVML_PCIE_LINK_SHUT_DOWN: nvmlPcieLinkState_enum = nvmlPcieLinkState_enum(
        1,
    );
}
#[repr(transparent)]
///  Parent bridge PCIe link state requested by nvmlDeviceRemoveGpu()
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPcieLinkState_enum(pub ::core::ffi::c_uint);
///  Parent bridge PCIe link state requested by nvmlDeviceRemoveGpu()
pub use self::nvmlPcieLinkState_enum as nvmlPcieLinkState_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeSystemCaps_st {
    pub cpuCaps: ::core::ffi::c_uint,
    pub gpusCaps: ::core::ffi::c_uint,
}
pub type nvmlConfComputeSystemCaps_t = nvmlConfComputeSystemCaps_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeSystemState_st {
    pub environment: ::core::ffi::c_uint,
    pub ccFeature: ::core::ffi::c_uint,
    pub devToolsMode: ::core::ffi::c_uint,
}
pub type nvmlConfComputeSystemState_t = nvmlConfComputeSystemState_st;
/// Confidential Compute System settings
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlSystemConfComputeSettings_v1_t {
    pub version: ::core::ffi::c_uint,
    pub environment: ::core::ffi::c_uint,
    pub ccFeature: ::core::ffi::c_uint,
    pub devToolsMode: ::core::ffi::c_uint,
    pub multiGpuMode: ::core::ffi::c_uint,
}
/// Confidential Compute System settings
pub type nvmlSystemConfComputeSettings_t = nvmlSystemConfComputeSettings_v1_t;
/// Protected memory size
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeMemSizeInfo_st {
    pub protectedMemSizeKib: ::core::ffi::c_ulonglong,
    pub unprotectedMemSizeKib: ::core::ffi::c_ulonglong,
}
/// Protected memory size
pub type nvmlConfComputeMemSizeInfo_t = nvmlConfComputeMemSizeInfo_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeGpuCertificate_st {
    pub certChainSize: ::core::ffi::c_uint,
    pub attestationCertChainSize: ::core::ffi::c_uint,
    pub certChain: [::core::ffi::c_uchar; 4096usize],
    pub attestationCertChain: [::core::ffi::c_uchar; 5120usize],
}
pub type nvmlConfComputeGpuCertificate_t = nvmlConfComputeGpuCertificate_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeGpuAttestationReport_st {
    pub isCecAttestationReportPresent: ::core::ffi::c_uint,
    pub attestationReportSize: ::core::ffi::c_uint,
    pub cecAttestationReportSize: ::core::ffi::c_uint,
    pub nonce: [::core::ffi::c_uchar; 32usize],
    pub attestationReport: [::core::ffi::c_uchar; 8192usize],
    pub cecAttestationReport: [::core::ffi::c_uchar; 4096usize],
}
pub type nvmlConfComputeGpuAttestationReport_t = nvmlConfComputeGpuAttestationReport_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeSetKeyRotationThresholdInfo_st {
    pub version: ::core::ffi::c_uint,
    pub maxAttackerAdvantage: ::core::ffi::c_ulonglong,
}
pub type nvmlConfComputeSetKeyRotationThresholdInfo_v1_t = nvmlConfComputeSetKeyRotationThresholdInfo_st;
pub type nvmlConfComputeSetKeyRotationThresholdInfo_t = nvmlConfComputeSetKeyRotationThresholdInfo_v1_t;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlConfComputeGetKeyRotationThresholdInfo_st {
    pub version: ::core::ffi::c_uint,
    pub attackerAdvantage: ::core::ffi::c_ulonglong,
}
pub type nvmlConfComputeGetKeyRotationThresholdInfo_v1_t = nvmlConfComputeGetKeyRotationThresholdInfo_st;
pub type nvmlConfComputeGetKeyRotationThresholdInfo_t = nvmlConfComputeGetKeyRotationThresholdInfo_v1_t;
pub type nvmlGpuFabricState_t = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuFabricInfo_t {
    ///!< Uuid of the cluster to which this GPU belongs
    pub clusterUuid: [::core::ffi::c_uchar; 16usize],
    ///!< Error status, if any. Must be checked only if state returns "complete".
    pub status: nvmlReturn_t,
    ///!< ID of the fabric clique to which this GPU belongs
    pub cliqueId: ::core::ffi::c_uint,
    ///!< Current state of GPU registration process
    pub state: nvmlGpuFabricState_t,
}
/** GPU Fabric information (v2).

 Version 2 adds the \ref nvmlGpuFabricInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlGpuFabricInfo_v2_t.healthMask
 field to the end. This structure is not backwards-compatible with
 \ref nvmlGpuFabricInfo_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuFabricInfo_v2_t {
    ///!< Structure version identifier (set to \ref nvmlGpuFabricInfo_v2)
    pub version: ::core::ffi::c_uint,
    ///!< Uuid of the cluster to which this GPU belongs
    pub clusterUuid: [::core::ffi::c_uchar; 16usize],
    ///!< Error status, if any. Must be checked only if state returns "complete".
    pub status: nvmlReturn_t,
    ///!< ID of the fabric clique to which this GPU belongs
    pub cliqueId: ::core::ffi::c_uint,
    ///!< Current state of GPU registration process
    pub state: nvmlGpuFabricState_t,
    ///!< GPU Fabric health Status Mask
    pub healthMask: ::core::ffi::c_uint,
}
/** GPU Fabric information (v2).

 Version 2 adds the \ref nvmlGpuFabricInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlGpuFabricInfo_v2_t.healthMask
 field to the end. This structure is not backwards-compatible with
 \ref nvmlGpuFabricInfo_t.*/
pub type nvmlGpuFabricInfoV_t = nvmlGpuFabricInfo_v2_t;
pub type nvmlPowerScopeType_t = ::core::ffi::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlPowerValue_v2_t {
    ///!< Structure format version (must be 1)
    pub version: ::core::ffi::c_uint,
    ///!< [in]  Device type: GPU or Total Module
    pub powerScope: nvmlPowerScopeType_t,
    ///!< [out] Power value to retrieve or set in milliwatts
    pub powerValueMw: ::core::ffi::c_uint,
}
pub type nvmlAffinityScope_t = ::core::ffi::c_uint;
impl nvmlClockLimitId_enum {
    pub const NVML_CLOCK_LIMIT_ID_RANGE_START: nvmlClockLimitId_enum = nvmlClockLimitId_enum(
        4294967040,
    );
}
impl nvmlClockLimitId_enum {
    pub const NVML_CLOCK_LIMIT_ID_TDP: nvmlClockLimitId_enum = nvmlClockLimitId_enum(
        4294967041,
    );
}
impl nvmlClockLimitId_enum {
    pub const NVML_CLOCK_LIMIT_ID_UNLIMITED: nvmlClockLimitId_enum = nvmlClockLimitId_enum(
        4294967042,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlClockLimitId_enum(pub ::core::ffi::c_uint);
pub use self::nvmlClockLimitId_enum as nvmlClockLimitId_t;
/// Structure representing range of vGPU versions.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuVersion_st {
    ///!< Minimum vGPU version.
    pub minVersion: ::core::ffi::c_uint,
    ///!< Maximum vGPU version.
    pub maxVersion: ::core::ffi::c_uint,
}
/// Structure representing range of vGPU versions.
pub type nvmlVgpuVersion_t = nvmlVgpuVersion_st;
/// vGPU metadata structure.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuMetadata_st {
    ///!< Current version of the structure
    pub version: ::core::ffi::c_uint,
    ///!< Current revision of the structure
    pub revision: ::core::ffi::c_uint,
    ///!< Current state of Guest-dependent fields
    pub guestInfoState: nvmlVgpuGuestInfoState_t,
    ///!< Version of driver installed in guest
    pub guestDriverVersion: [::core::ffi::c_char; 80usize],
    ///!< Version of driver installed in host
    pub hostDriverVersion: [::core::ffi::c_char; 80usize],
    ///!< Reserved for internal use
    pub reserved: [::core::ffi::c_uint; 6usize],
    ///!< vGPU virtualization capabilities bitfield
    pub vgpuVirtualizationCaps: ::core::ffi::c_uint,
    ///!< vGPU version of guest driver
    pub guestVgpuVersion: ::core::ffi::c_uint,
    ///!< Size of opaque data field in bytes
    pub opaqueDataSize: ::core::ffi::c_uint,
    ///!< Opaque data
    pub opaqueData: [::core::ffi::c_char; 4usize],
}
/// vGPU metadata structure.
pub type nvmlVgpuMetadata_t = nvmlVgpuMetadata_st;
/// Physical GPU metadata structure
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuPgpuMetadata_st {
    ///!< Current version of the structure
    pub version: ::core::ffi::c_uint,
    ///!< Current revision of the structure
    pub revision: ::core::ffi::c_uint,
    ///!< Host driver version
    pub hostDriverVersion: [::core::ffi::c_char; 80usize],
    ///!< Pgpu virtualization capabilities bitfield
    pub pgpuVirtualizationCaps: ::core::ffi::c_uint,
    ///!< Reserved for internal use
    pub reserved: [::core::ffi::c_uint; 5usize],
    ///!< vGPU version range supported by host driver
    pub hostSupportedVgpuRange: nvmlVgpuVersion_t,
    ///!< Size of opaque data field in bytes
    pub opaqueDataSize: ::core::ffi::c_uint,
    ///!< Opaque data
    pub opaqueData: [::core::ffi::c_char; 4usize],
}
/// Physical GPU metadata structure
pub type nvmlVgpuPgpuMetadata_t = nvmlVgpuPgpuMetadata_st;
impl nvmlVgpuVmCompatibility_enum {
    ///!< vGPU is not runnable
    pub const NVML_VGPU_VM_COMPATIBILITY_NONE: nvmlVgpuVmCompatibility_enum = nvmlVgpuVmCompatibility_enum(
        0,
    );
}
impl nvmlVgpuVmCompatibility_enum {
    ///!< vGPU is runnable from a cold / powered-off state (ACPI S5)
    pub const NVML_VGPU_VM_COMPATIBILITY_COLD: nvmlVgpuVmCompatibility_enum = nvmlVgpuVmCompatibility_enum(
        1,
    );
}
impl nvmlVgpuVmCompatibility_enum {
    ///!< vGPU is runnable from a hibernated state (ACPI S4)
    pub const NVML_VGPU_VM_COMPATIBILITY_HIBERNATE: nvmlVgpuVmCompatibility_enum = nvmlVgpuVmCompatibility_enum(
        2,
    );
}
impl nvmlVgpuVmCompatibility_enum {
    ///!< vGPU is runnable from a sleeped state (ACPI S3)
    pub const NVML_VGPU_VM_COMPATIBILITY_SLEEP: nvmlVgpuVmCompatibility_enum = nvmlVgpuVmCompatibility_enum(
        4,
    );
}
impl nvmlVgpuVmCompatibility_enum {
    ///!< vGPU is runnable from a live/paused (ACPI S0)
    pub const NVML_VGPU_VM_COMPATIBILITY_LIVE: nvmlVgpuVmCompatibility_enum = nvmlVgpuVmCompatibility_enum(
        8,
    );
}
#[repr(transparent)]
/// vGPU VM compatibility codes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuVmCompatibility_enum(pub ::core::ffi::c_uint);
/// vGPU VM compatibility codes
pub use self::nvmlVgpuVmCompatibility_enum as nvmlVgpuVmCompatibility_t;
impl nvmlVgpuPgpuCompatibilityLimitCode_enum {
    ///!< Compatibility is not limited.
    pub const NVML_VGPU_COMPATIBILITY_LIMIT_NONE: nvmlVgpuPgpuCompatibilityLimitCode_enum = nvmlVgpuPgpuCompatibilityLimitCode_enum(
        0,
    );
}
impl nvmlVgpuPgpuCompatibilityLimitCode_enum {
    ///!< ompatibility is limited by host driver version.
    pub const NVML_VGPU_COMPATIBILITY_LIMIT_HOST_DRIVER: nvmlVgpuPgpuCompatibilityLimitCode_enum = nvmlVgpuPgpuCompatibilityLimitCode_enum(
        1,
    );
}
impl nvmlVgpuPgpuCompatibilityLimitCode_enum {
    ///!< Compatibility is limited by guest driver version.
    pub const NVML_VGPU_COMPATIBILITY_LIMIT_GUEST_DRIVER: nvmlVgpuPgpuCompatibilityLimitCode_enum = nvmlVgpuPgpuCompatibilityLimitCode_enum(
        2,
    );
}
impl nvmlVgpuPgpuCompatibilityLimitCode_enum {
    ///!< Compatibility is limited by GPU hardware.
    pub const NVML_VGPU_COMPATIBILITY_LIMIT_GPU: nvmlVgpuPgpuCompatibilityLimitCode_enum = nvmlVgpuPgpuCompatibilityLimitCode_enum(
        4,
    );
}
impl nvmlVgpuPgpuCompatibilityLimitCode_enum {
    ///!< Compatibility is limited by an undefined factor.
    pub const NVML_VGPU_COMPATIBILITY_LIMIT_OTHER: nvmlVgpuPgpuCompatibilityLimitCode_enum = nvmlVgpuPgpuCompatibilityLimitCode_enum(
        2147483648,
    );
}
#[repr(transparent)]
///  vGPU-pGPU compatibility limit codes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuPgpuCompatibilityLimitCode_enum(pub ::core::ffi::c_uint);
///  vGPU-pGPU compatibility limit codes
pub use self::nvmlVgpuPgpuCompatibilityLimitCode_enum as nvmlVgpuPgpuCompatibilityLimitCode_t;
/// vGPU-pGPU compatibility structure
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlVgpuPgpuCompatibility_st {
    ///!< Compatibility of vGPU VM. See \ref nvmlVgpuVmCompatibility_t
    pub vgpuVmCompatibility: nvmlVgpuVmCompatibility_t,
    ///!< Limiting factor for vGPU-pGPU compatibility. See \ref nvmlVgpuPgpuCompatibilityLimitCode_t
    pub compatibilityLimitCode: nvmlVgpuPgpuCompatibilityLimitCode_t,
}
/// vGPU-pGPU compatibility structure
pub type nvmlVgpuPgpuCompatibility_t = nvmlVgpuPgpuCompatibility_st;
/// Excluded GPU device information
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlExcludedDeviceInfo_st {
    ///!< The PCI information for the excluded GPU
    pub pciInfo: nvmlPciInfo_t,
    ///!< The ASCII string UUID for the excluded GPU
    pub uuid: [::core::ffi::c_char; 80usize],
}
/// Excluded GPU device information
pub type nvmlExcludedDeviceInfo_t = nvmlExcludedDeviceInfo_st;
/** MIG compute instance profile capability.

 Bit field values representing MIG profile capabilities
 \ref nvmlComputeInstanceProfileInfo_v3_t.capabilities*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuInstancePlacement_st {
    ///!< Index of first occupied memory slice
    pub start: ::core::ffi::c_uint,
    ///!< Number of memory slices occupied
    pub size: ::core::ffi::c_uint,
}
/** MIG compute instance profile capability.

 Bit field values representing MIG profile capabilities
 \ref nvmlComputeInstanceProfileInfo_v3_t.capabilities*/
pub type nvmlGpuInstancePlacement_t = nvmlGpuInstancePlacement_st;
/// GPU instance profile information.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuInstanceProfileInfo_st {
    ///!< Unique profile ID within the device
    pub id: ::core::ffi::c_uint,
    ///!< Peer-to-Peer support
    pub isP2pSupported: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< GPU instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Copy Engine count
    pub copyEngineCount: ::core::ffi::c_uint,
    ///!< Decoder Engine count
    pub decoderCount: ::core::ffi::c_uint,
    ///!< Encoder Engine count
    pub encoderCount: ::core::ffi::c_uint,
    ///!< JPEG Engine count
    pub jpegCount: ::core::ffi::c_uint,
    ///!< OFA Engine count
    pub ofaCount: ::core::ffi::c_uint,
    ///!< Memory size in MBytes
    pub memorySizeMB: ::core::ffi::c_ulonglong,
}
/// GPU instance profile information.
pub type nvmlGpuInstanceProfileInfo_t = nvmlGpuInstanceProfileInfo_st;
/** GPU instance profile information (v2).

 Version 2 adds the \ref nvmlGpuInstanceProfileInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlGpuInstanceProfileInfo_v2_t.name
 field to the end. This structure is not backwards-compatible with
 \ref nvmlGpuInstanceProfileInfo_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuInstanceProfileInfo_v2_st {
    ///!< Structure version identifier (set to \ref nvmlGpuInstanceProfileInfo_v2)
    pub version: ::core::ffi::c_uint,
    ///!< Unique profile ID within the device
    pub id: ::core::ffi::c_uint,
    ///!< Peer-to-Peer support
    pub isP2pSupported: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< GPU instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Copy Engine count
    pub copyEngineCount: ::core::ffi::c_uint,
    ///!< Decoder Engine count
    pub decoderCount: ::core::ffi::c_uint,
    ///!< Encoder Engine count
    pub encoderCount: ::core::ffi::c_uint,
    ///!< JPEG Engine count
    pub jpegCount: ::core::ffi::c_uint,
    ///!< OFA Engine count
    pub ofaCount: ::core::ffi::c_uint,
    ///!< Memory size in MBytes
    pub memorySizeMB: ::core::ffi::c_ulonglong,
    ///!< Profile name
    pub name: [::core::ffi::c_char; 96usize],
}
/** GPU instance profile information (v2).

 Version 2 adds the \ref nvmlGpuInstanceProfileInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlGpuInstanceProfileInfo_v2_t.name
 field to the end. This structure is not backwards-compatible with
 \ref nvmlGpuInstanceProfileInfo_t.*/
pub type nvmlGpuInstanceProfileInfo_v2_t = nvmlGpuInstanceProfileInfo_v2_st;
/** GPU instance profile information (v3).

 Version 3 removes isP2pSupported field and adds the \ref nvmlGpuInstanceProfileInfo_v3_t.capabilities
 field \ref nvmlGpuInstanceProfileInfo_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuInstanceProfileInfo_v3_st {
    ///!< Structure version identifier (set to \ref nvmlGpuInstanceProfileInfo_v3)
    pub version: ::core::ffi::c_uint,
    ///!< Unique profile ID within the device
    pub id: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< GPU instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Copy Engine count
    pub copyEngineCount: ::core::ffi::c_uint,
    ///!< Decoder Engine count
    pub decoderCount: ::core::ffi::c_uint,
    ///!< Encoder Engine count
    pub encoderCount: ::core::ffi::c_uint,
    ///!< JPEG Engine count
    pub jpegCount: ::core::ffi::c_uint,
    ///!< OFA Engine count
    pub ofaCount: ::core::ffi::c_uint,
    ///!< Memory size in MBytes
    pub memorySizeMB: ::core::ffi::c_ulonglong,
    ///!< Profile name
    pub name: [::core::ffi::c_char; 96usize],
    ///!< Additional capabilities
    pub capabilities: ::core::ffi::c_uint,
}
/** GPU instance profile information (v3).

 Version 3 removes isP2pSupported field and adds the \ref nvmlGpuInstanceProfileInfo_v3_t.capabilities
 field \ref nvmlGpuInstanceProfileInfo_t.*/
pub type nvmlGpuInstanceProfileInfo_v3_t = nvmlGpuInstanceProfileInfo_v3_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpuInstanceInfo_st {
    ///!< Parent device
    pub device: nvmlDevice_t,
    ///!< Unique instance ID within the device
    pub id: ::core::ffi::c_uint,
    ///!< Unique profile ID within the device
    pub profileId: ::core::ffi::c_uint,
    ///!< Placement for this instance
    pub placement: nvmlGpuInstancePlacement_t,
}
pub type nvmlGpuInstanceInfo_t = nvmlGpuInstanceInfo_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpuInstance_st {
    _unused: [u8; 0],
}
pub type nvmlGpuInstance_t = *mut nvmlGpuInstance_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeInstancePlacement_st {
    ///!< Index of first occupied compute slice
    pub start: ::core::ffi::c_uint,
    ///!< Number of compute slices occupied
    pub size: ::core::ffi::c_uint,
}
pub type nvmlComputeInstancePlacement_t = nvmlComputeInstancePlacement_st;
/// Compute instance profile information.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeInstanceProfileInfo_st {
    ///!< Unique profile ID within the GPU instance
    pub id: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< Compute instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Shared Copy Engine count
    pub sharedCopyEngineCount: ::core::ffi::c_uint,
    ///!< Shared Decoder Engine count
    pub sharedDecoderCount: ::core::ffi::c_uint,
    ///!< Shared Encoder Engine count
    pub sharedEncoderCount: ::core::ffi::c_uint,
    ///!< Shared JPEG Engine count
    pub sharedJpegCount: ::core::ffi::c_uint,
    ///!< Shared OFA Engine count
    pub sharedOfaCount: ::core::ffi::c_uint,
}
/// Compute instance profile information.
pub type nvmlComputeInstanceProfileInfo_t = nvmlComputeInstanceProfileInfo_st;
/** Compute instance profile information (v2).

 Version 2 adds the \ref nvmlComputeInstanceProfileInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlComputeInstanceProfileInfo_v2_t.name
 field to the end. This structure is not backwards-compatible with
 \ref nvmlComputeInstanceProfileInfo_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeInstanceProfileInfo_v2_st {
    ///!< Structure version identifier (set to \ref nvmlComputeInstanceProfileInfo_v2)
    pub version: ::core::ffi::c_uint,
    ///!< Unique profile ID within the GPU instance
    pub id: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< Compute instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Shared Copy Engine count
    pub sharedCopyEngineCount: ::core::ffi::c_uint,
    ///!< Shared Decoder Engine count
    pub sharedDecoderCount: ::core::ffi::c_uint,
    ///!< Shared Encoder Engine count
    pub sharedEncoderCount: ::core::ffi::c_uint,
    ///!< Shared JPEG Engine count
    pub sharedJpegCount: ::core::ffi::c_uint,
    ///!< Shared OFA Engine count
    pub sharedOfaCount: ::core::ffi::c_uint,
    ///!< Profile name
    pub name: [::core::ffi::c_char; 96usize],
}
/** Compute instance profile information (v2).

 Version 2 adds the \ref nvmlComputeInstanceProfileInfo_v2_t.version field
 to the start of the structure, and the \ref nvmlComputeInstanceProfileInfo_v2_t.name
 field to the end. This structure is not backwards-compatible with
 \ref nvmlComputeInstanceProfileInfo_t.*/
pub type nvmlComputeInstanceProfileInfo_v2_t = nvmlComputeInstanceProfileInfo_v2_st;
/** Compute instance profile information (v3).

 Version 3 adds the \ref nvmlComputeInstanceProfileInfo_v3_t.capabilities field
 \ref nvmlComputeInstanceProfileInfo_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeInstanceProfileInfo_v3_st {
    ///!< Structure version identifier (set to \ref nvmlComputeInstanceProfileInfo_v3)
    pub version: ::core::ffi::c_uint,
    ///!< Unique profile ID within the GPU instance
    pub id: ::core::ffi::c_uint,
    ///!< GPU Slice count
    pub sliceCount: ::core::ffi::c_uint,
    ///!< Compute instance count
    pub instanceCount: ::core::ffi::c_uint,
    ///!< Streaming Multiprocessor count
    pub multiprocessorCount: ::core::ffi::c_uint,
    ///!< Shared Copy Engine count
    pub sharedCopyEngineCount: ::core::ffi::c_uint,
    ///!< Shared Decoder Engine count
    pub sharedDecoderCount: ::core::ffi::c_uint,
    ///!< Shared Encoder Engine count
    pub sharedEncoderCount: ::core::ffi::c_uint,
    ///!< Shared JPEG Engine count
    pub sharedJpegCount: ::core::ffi::c_uint,
    ///!< Shared OFA Engine count
    pub sharedOfaCount: ::core::ffi::c_uint,
    ///!< Profile name
    pub name: [::core::ffi::c_char; 96usize],
    ///!< Additional capabilities
    pub capabilities: ::core::ffi::c_uint,
}
/** Compute instance profile information (v3).

 Version 3 adds the \ref nvmlComputeInstanceProfileInfo_v3_t.capabilities field
 \ref nvmlComputeInstanceProfileInfo_t.*/
pub type nvmlComputeInstanceProfileInfo_v3_t = nvmlComputeInstanceProfileInfo_v3_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlComputeInstanceInfo_st {
    ///!< Parent device
    pub device: nvmlDevice_t,
    ///!< Parent GPU instance
    pub gpuInstance: nvmlGpuInstance_t,
    ///!< Unique instance ID within the GPU instance
    pub id: ::core::ffi::c_uint,
    ///!< Unique profile ID within the GPU instance
    pub profileId: ::core::ffi::c_uint,
    ///!< Placement for this instance within the GPU instance's compute slice range {0, sliceCount}
    pub placement: nvmlComputeInstancePlacement_t,
}
pub type nvmlComputeInstanceInfo_t = nvmlComputeInstanceInfo_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlComputeInstance_st {
    _unused: [u8; 0],
}
pub type nvmlComputeInstance_t = *mut nvmlComputeInstance_st;
impl nvmlGpmMetricId_t {
    ///!< Percentage of time any compute/graphics app was active on the GPU. 0.0 - 100.0
    pub const NVML_GPM_METRIC_GRAPHICS_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(1);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of SMs that were busy. 0.0 - 100.0
    pub const NVML_GPM_METRIC_SM_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(2);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of warps that were active vs theoretical maximum. 0.0 - 100.0
    pub const NVML_GPM_METRIC_SM_OCCUPANCY: nvmlGpmMetricId_t = nvmlGpmMetricId_t(3);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing integer operations. 0.0 - 100.0
    pub const NVML_GPM_METRIC_INTEGER_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(4);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing ANY tensor operations. 0.0 - 100.0
    pub const NVML_GPM_METRIC_ANY_TENSOR_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(5);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing DFMA tensor operations. 0.0 - 100.0
    pub const NVML_GPM_METRIC_DFMA_TENSOR_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(6);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing HMMA tensor operations. 0.0 - 100.0
    pub const NVML_GPM_METRIC_HMMA_TENSOR_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(7);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing IMMA tensor operations. 0.0 - 100.0
    pub const NVML_GPM_METRIC_IMMA_TENSOR_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(9);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of DRAM bw used vs theoretical maximum. 0.0 - 100.0 */
    pub const NVML_GPM_METRIC_DRAM_BW_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(10);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing non-tensor FP64 math. 0.0 - 100.0
    pub const NVML_GPM_METRIC_FP64_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(11);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing non-tensor FP32 math. 0.0 - 100.0
    pub const NVML_GPM_METRIC_FP32_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(12);
}
impl nvmlGpmMetricId_t {
    ///!< Percentage of time the GPU's SMs were doing non-tensor FP16 math. 0.0 - 100.0
    pub const NVML_GPM_METRIC_FP16_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(13);
}
impl nvmlGpmMetricId_t {
    ///!< PCIe traffic from this GPU in MiB/sec
    pub const NVML_GPM_METRIC_PCIE_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(20);
}
impl nvmlGpmMetricId_t {
    ///!< PCIe traffic to this GPU in MiB/sec
    pub const NVML_GPM_METRIC_PCIE_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(21);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 0. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_0_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(30);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 1. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_1_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(31);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 2. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_2_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(32);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 3. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_3_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(33);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 4. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_4_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(34);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 5. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_5_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(35);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 6. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_6_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(36);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVDEC 7. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVDEC_7_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(37);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 0. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_0_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(40);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 1. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_1_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(41);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 2. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_2_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(42);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 3. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_3_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(43);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 4. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_4_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(44);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 5. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_5_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(45);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 6. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_6_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(46);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVJPG 7. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVJPG_7_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(47);
}
impl nvmlGpmMetricId_t {
    ///!< Percent utilization of NVOFA 0. 0.0 - 100.0
    pub const NVML_GPM_METRIC_NVOFA_0_UTIL: nvmlGpmMetricId_t = nvmlGpmMetricId_t(50);
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for all links in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        60,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for all links in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        61,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 0 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        62,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 0 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        63,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 1 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        64,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 1 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        65,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 2 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        66,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 2 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        67,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 3 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        68,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 3 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        69,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 4 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        70,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 4 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        71,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 5 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        72,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 5 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        73,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 6 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        74,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 6 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        75,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 7 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        76,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 7 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        77,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 8 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        78,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 8 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        79,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 9 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        80,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 9 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        81,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 10 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        82,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 10 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        83,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 11 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        84,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 11 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        85,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 12 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        86,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 12 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        87,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 13 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        88,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 13 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        89,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 14 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        90,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 14 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        91,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 15 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        92,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 15 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        93,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 16 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        94,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 16 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        95,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink read bandwidth for link 17 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        96,
    );
}
impl nvmlGpmMetricId_t {
    ///!< NvLink write bandwidth for link 17 in MiB/sec
    pub const NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC: nvmlGpmMetricId_t = nvmlGpmMetricId_t(
        97,
    );
}
impl nvmlGpmMetricId_t {
    ///!< Maximum value above +1. Note that changing this should also change NVML_GPM_METRICS_GET_VERSION due to struct size change
    pub const NVML_GPM_METRIC_MAX: nvmlGpmMetricId_t = nvmlGpmMetricId_t(98);
}
#[repr(transparent)]
/// GPM Metric Identifiers
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpmMetricId_t(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nvmlGpmSample_st {
    _unused: [u8; 0],
}
/// Handle to an allocated GPM sample allocated with nvmlGpmSampleAlloc(). Free this with nvmlGpmSampleFree().
pub type nvmlGpmSample_t = *mut nvmlGpmSample_st;
/// GPM metric information.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct nvmlGpmMetric_t {
    ///!<  IN: NVML_GPM_METRIC_? #define of which metric to retrieve
    pub metricId: ::core::ffi::c_uint,
    ///!<  OUT: Status of this metric. If this is nonzero, then value is not valid
    pub nvmlReturn: nvmlReturn_t,
    ///!<  OUT: Value of this metric. Is only valid if nvmlReturn is 0 (NVML_SUCCESS)
    pub value: f64,
    ///!< OUT: Metric name and unit. Those can be NULL if not defined
    pub metricInfo: nvmlGpmMetric_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpmMetric_t__bindgen_ty_1 {
    pub shortName: *mut ::core::ffi::c_char,
    pub longName: *mut ::core::ffi::c_char,
    pub unit: *mut ::core::ffi::c_char,
}
/// GPM buffer information.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct nvmlGpmMetricsGet_t {
    ///!< IN: Set to NVML_GPM_METRICS_GET_VERSION
    pub version: ::core::ffi::c_uint,
    ///!< IN: How many metrics to retrieve in metrics[]
    pub numMetrics: ::core::ffi::c_uint,
    ///!< IN: Sample buffer
    pub sample1: nvmlGpmSample_t,
    ///!< IN: Sample buffer
    pub sample2: nvmlGpmSample_t,
    ///!< IN/OUT: Array of metrics. Set metricId on call. See nvmlReturn and value on return
    pub metrics: [nvmlGpmMetric_t; 98usize],
}
/// GPM device information.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlGpmSupport_t {
    ///!< IN: Set to NVML_GPM_SUPPORT_VERSION
    pub version: ::core::ffi::c_uint,
    ///!< OUT: Indicates device support
    pub isSupportedDevice: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct nvmlNvLinkPowerThres_st {
    ///!< Low power threshold (in units of 100us)
    pub lowPwrThreshold: ::core::ffi::c_uint,
}
pub type nvmlNvLinkPowerThres_t = nvmlNvLinkPowerThres_st;
impl nvmlError_t {
    pub const UNINITIALIZED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const INVALID_ARGUMENT: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const NOT_SUPPORTED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const NO_PERMISSION: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const ALREADY_INITIALIZED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const NOT_FOUND: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const INSUFFICIENT_SIZE: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const INSUFFICIENT_POWER: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const DRIVER_NOT_LOADED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const TIMEOUT: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const IRQ_ISSUE: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
    pub const LIBRARY_NOT_FOUND: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(12)
    });
    pub const FUNCTION_NOT_FOUND: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const CORRUPTED_INFOROM: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(14)
    });
    pub const GPU_IS_LOST: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(15)
    });
    pub const RESET_REQUIRED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(16)
    });
    pub const OPERATING_SYSTEM: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(17)
    });
    pub const LIB_RM_VERSION_MISMATCH: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(18)
    });
    pub const IN_USE: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(19)
    });
    pub const MEMORY: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(20)
    });
    pub const NO_DATA: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(21)
    });
    pub const VGPU_ECC_NOT_SUPPORTED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(22)
    });
    pub const INSUFFICIENT_RESOURCES: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(23)
    });
    pub const FREQ_NOT_SUPPORTED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(24)
    });
    pub const ARGUMENT_VERSION_MISMATCH: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(25)
    });
    pub const DEPRECATED: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(26)
    });
    pub const NOT_READY: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(27)
    });
    pub const GPU_NOT_FOUND: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(28)
    });
    pub const INVALID_STATE: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(29)
    });
    pub const UNKNOWN: nvmlError_t = nvmlError_t(unsafe {
        ::core::num::NonZeroU32::new_unchecked(999)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct nvmlError_t(pub ::core::num::NonZeroU32);
pub trait nvmlReturn_tConsts {
    const SUCCESS: nvmlReturn_t = nvmlReturn_t::Ok(());
    const ERROR_UNINITIALIZED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::UNINITIALIZED,
    );
    const ERROR_INVALID_ARGUMENT: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::INVALID_ARGUMENT,
    );
    const ERROR_NOT_SUPPORTED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::NOT_SUPPORTED,
    );
    const ERROR_NO_PERMISSION: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::NO_PERMISSION,
    );
    const ERROR_ALREADY_INITIALIZED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::ALREADY_INITIALIZED,
    );
    const ERROR_NOT_FOUND: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::NOT_FOUND);
    const ERROR_INSUFFICIENT_SIZE: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::INSUFFICIENT_SIZE,
    );
    const ERROR_INSUFFICIENT_POWER: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::INSUFFICIENT_POWER,
    );
    const ERROR_DRIVER_NOT_LOADED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::DRIVER_NOT_LOADED,
    );
    const ERROR_TIMEOUT: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::TIMEOUT);
    const ERROR_IRQ_ISSUE: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::IRQ_ISSUE);
    const ERROR_LIBRARY_NOT_FOUND: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::LIBRARY_NOT_FOUND,
    );
    const ERROR_FUNCTION_NOT_FOUND: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::FUNCTION_NOT_FOUND,
    );
    const ERROR_CORRUPTED_INFOROM: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::CORRUPTED_INFOROM,
    );
    const ERROR_GPU_IS_LOST: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::GPU_IS_LOST);
    const ERROR_RESET_REQUIRED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::RESET_REQUIRED,
    );
    const ERROR_OPERATING_SYSTEM: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::OPERATING_SYSTEM,
    );
    const ERROR_LIB_RM_VERSION_MISMATCH: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::LIB_RM_VERSION_MISMATCH,
    );
    const ERROR_IN_USE: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::IN_USE);
    const ERROR_MEMORY: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::MEMORY);
    const ERROR_NO_DATA: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::NO_DATA);
    const ERROR_VGPU_ECC_NOT_SUPPORTED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::VGPU_ECC_NOT_SUPPORTED,
    );
    const ERROR_INSUFFICIENT_RESOURCES: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::INSUFFICIENT_RESOURCES,
    );
    const ERROR_FREQ_NOT_SUPPORTED: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::FREQ_NOT_SUPPORTED,
    );
    const ERROR_ARGUMENT_VERSION_MISMATCH: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::ARGUMENT_VERSION_MISMATCH,
    );
    const ERROR_DEPRECATED: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::DEPRECATED);
    const ERROR_NOT_READY: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::NOT_READY);
    const ERROR_GPU_NOT_FOUND: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::GPU_NOT_FOUND,
    );
    const ERROR_INVALID_STATE: nvmlReturn_t = nvmlReturn_t::Err(
        nvmlError_t::INVALID_STATE,
    );
    const ERROR_UNKNOWN: nvmlReturn_t = nvmlReturn_t::Err(nvmlError_t::UNKNOWN);
}
impl nvmlReturn_tConsts for nvmlReturn_t {}
#[must_use]
pub type nvmlReturn_t = ::core::result::Result<(), nvmlError_t>;
const _: fn() = || {
    let _ = std::mem::transmute::<nvmlReturn_t, u32>;
};
