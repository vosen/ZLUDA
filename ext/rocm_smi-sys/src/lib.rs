// Generated automatically by zluda_bindgen
// DO NOT EDIT MANUALLY
#![allow(warnings)]
pub const RSMI_MAX_NUM_FREQUENCIES: u32 = 33;
pub const RSMI_MAX_FAN_SPEED: u32 = 255;
pub const RSMI_NUM_VOLTAGE_CURVE_POINTS: u32 = 3;
pub const RSMI_NUM_HBM_INSTANCES: u32 = 4;
pub const RSMI_MAX_NUM_VCNS: u32 = 4;
pub const RSMI_MAX_NUM_JPEG_ENGS: u32 = 32;
pub const RSMI_MAX_NUM_CLKS: u32 = 4;
pub const RSMI_MAX_NUM_XGMI_LINKS: u32 = 8;
pub const RSMI_MAX_NUM_GFX_CLKS: u32 = 8;
pub const RSMI_MAX_NUM_XCC: u32 = 8;
pub const RSMI_MAX_NUM_XCP: u32 = 8;
pub const RSMI_DEFAULT_VARIANT: i32 = -1;
impl rsmi_init_flags_t {
    /**!< Attempt to add all GPUs found
!< (including non-AMD) to the list
!< of devices from which SMI
!< information can be retrieved. By
!< default, only AMD devices are
!<  enumerated by RSMI.*/
    pub const RSMI_INIT_FLAG_ALL_GPUS: rsmi_init_flags_t = rsmi_init_flags_t(1);
}
impl rsmi_init_flags_t {
    ///!< The mutex limit to thread
    pub const RSMI_INIT_FLAG_THRAD_ONLY_MUTEX: rsmi_init_flags_t = rsmi_init_flags_t(
        288230376151711744,
    );
}
impl rsmi_init_flags_t {
    ///!< Reserved for test
    pub const RSMI_INIT_FLAG_RESRV_TEST1: rsmi_init_flags_t = rsmi_init_flags_t(
        576460752303423488,
    );
}
#[repr(transparent)]
/** @brief Initialization flags

 Initialization flags may be OR'd together and passed to ::rsmi_init().*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_init_flags_t(pub ::core::ffi::c_ulong);
impl rsmi_dev_perf_level_t {
    ///!< Performance level is "auto"
    pub const RSMI_DEV_PERF_LEVEL_AUTO: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(0);
}
impl rsmi_dev_perf_level_t {
    pub const RSMI_DEV_PERF_LEVEL_FIRST: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        0,
    );
}
impl rsmi_dev_perf_level_t {
    /**!< Keep PowerPlay levels "low",
!< regardless of workload*/
    pub const RSMI_DEV_PERF_LEVEL_LOW: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(1);
}
impl rsmi_dev_perf_level_t {
    /**!< Keep PowerPlay levels "high",
!< regardless of workload*/
    pub const RSMI_DEV_PERF_LEVEL_HIGH: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(2);
}
impl rsmi_dev_perf_level_t {
    /**!< Only use values defined by manually
!< setting the RSMI_CLK_TYPE_SYS speed*/
    pub const RSMI_DEV_PERF_LEVEL_MANUAL: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        3,
    );
}
impl rsmi_dev_perf_level_t {
    /**!< Stable power state with profiling
!< clocks*/
    pub const RSMI_DEV_PERF_LEVEL_STABLE_STD: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        4,
    );
}
impl rsmi_dev_perf_level_t {
    ///!< Stable power state with peak clocks
    pub const RSMI_DEV_PERF_LEVEL_STABLE_PEAK: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        5,
    );
}
impl rsmi_dev_perf_level_t {
    /**!< Stable power state with minimum
!< memory clock*/
    pub const RSMI_DEV_PERF_LEVEL_STABLE_MIN_MCLK: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        6,
    );
}
impl rsmi_dev_perf_level_t {
    /**!< Stable power state with minimum
!< system clock*/
    pub const RSMI_DEV_PERF_LEVEL_STABLE_MIN_SCLK: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        7,
    );
}
impl rsmi_dev_perf_level_t {
    ///!< Performance determinism state
    pub const RSMI_DEV_PERF_LEVEL_DETERMINISM: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        8,
    );
}
impl rsmi_dev_perf_level_t {
    pub const RSMI_DEV_PERF_LEVEL_LAST: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(8);
}
impl rsmi_dev_perf_level_t {
    ///!< Unknown performance level
    pub const RSMI_DEV_PERF_LEVEL_UNKNOWN: rsmi_dev_perf_level_t = rsmi_dev_perf_level_t(
        256,
    );
}
#[repr(transparent)]
/// @brief PowerPlay performance levels
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_dev_perf_level_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_dev_perf_level_t as rsmi_dev_perf_level;
impl rsmi_sw_component_t {
    pub const RSMI_SW_COMP_FIRST: rsmi_sw_component_t = rsmi_sw_component_t(0);
}
impl rsmi_sw_component_t {
    ///!< Driver
    pub const RSMI_SW_COMP_DRIVER: rsmi_sw_component_t = rsmi_sw_component_t(0);
}
impl rsmi_sw_component_t {
    pub const RSMI_SW_COMP_LAST: rsmi_sw_component_t = rsmi_sw_component_t(0);
}
#[repr(transparent)]
/// @brief Software components
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_sw_component_t(pub ::core::ffi::c_uint);
/// @brief Handle to performance event counter
pub type rsmi_event_handle_t = usize;
impl rsmi_event_group_t {
    ///!< Data Fabric (XGMI) related events
    pub const RSMI_EVNT_GRP_XGMI: rsmi_event_group_t = rsmi_event_group_t(0);
}
impl rsmi_event_group_t {
    ///!< XGMI Outbound data
    pub const RSMI_EVNT_GRP_XGMI_DATA_OUT: rsmi_event_group_t = rsmi_event_group_t(10);
}
impl rsmi_event_group_t {
    pub const RSMI_EVNT_GRP_INVALID: rsmi_event_group_t = rsmi_event_group_t(4294967295);
}
#[repr(transparent)]
/** Event Groups

 @brief Enum denoting an event group. The value of the enum is the
 base value for all the event enums in the group.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_event_group_t(pub ::core::ffi::c_uint);
impl rsmi_event_type_t {
    pub const RSMI_EVNT_FIRST: rsmi_event_type_t = rsmi_event_type_t(0);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_XGMI_FIRST: rsmi_event_type_t = rsmi_event_type_t(0);
}
impl rsmi_event_type_t {
    ///!< NOPs sent to neighbor 0
    pub const RSMI_EVNT_XGMI_0_NOP_TX: rsmi_event_type_t = rsmi_event_type_t(0);
}
impl rsmi_event_type_t {
    /**!< Outgoing requests to
!< neighbor 0*/
    pub const RSMI_EVNT_XGMI_0_REQUEST_TX: rsmi_event_type_t = rsmi_event_type_t(1);
}
impl rsmi_event_type_t {
    /**!< Outgoing responses to
!< neighbor 0*/
    pub const RSMI_EVNT_XGMI_0_RESPONSE_TX: rsmi_event_type_t = rsmi_event_type_t(2);
}
impl rsmi_event_type_t {
    /** @brief

 Data beats sent to neighbor 0; Each beat represents 32 bytes.<br><br>

 XGMI throughput can be calculated by multiplying a BEATs event
 such as ::RSMI_EVNT_XGMI_0_BEATS_TX by 32 and dividing by
 the time for which event collection occurred,
 ::rsmi_counter_value_t.time_running (which is in nanoseconds). To get
 bytes per second, multiply this value by 10<sup>9</sup>.<br>
 <br>
 Throughput = BEATS/time_running * 10<sup>9</sup>  (bytes/second)<br>*/
    pub const RSMI_EVNT_XGMI_0_BEATS_TX: rsmi_event_type_t = rsmi_event_type_t(3);
}
impl rsmi_event_type_t {
    ///!< NOPs sent to neighbor 1
    pub const RSMI_EVNT_XGMI_1_NOP_TX: rsmi_event_type_t = rsmi_event_type_t(4);
}
impl rsmi_event_type_t {
    /**!< Outgoing requests to
!< neighbor 1*/
    pub const RSMI_EVNT_XGMI_1_REQUEST_TX: rsmi_event_type_t = rsmi_event_type_t(5);
}
impl rsmi_event_type_t {
    /**!< Outgoing responses to
!< neighbor 1*/
    pub const RSMI_EVNT_XGMI_1_RESPONSE_TX: rsmi_event_type_t = rsmi_event_type_t(6);
}
impl rsmi_event_type_t {
    /**!< Data beats sent to
!< neighbor 1; Each beat
!< represents 32 bytes*/
    pub const RSMI_EVNT_XGMI_1_BEATS_TX: rsmi_event_type_t = rsmi_event_type_t(7);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_XGMI_LAST: rsmi_event_type_t = rsmi_event_type_t(7);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_XGMI_DATA_OUT_FIRST: rsmi_event_type_t = rsmi_event_type_t(10);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_XGMI_DATA_OUT_0: rsmi_event_type_t = rsmi_event_type_t(10);
}
impl rsmi_event_type_t {
    ///!< Outbound beats to neighbor 1
    pub const RSMI_EVNT_XGMI_DATA_OUT_1: rsmi_event_type_t = rsmi_event_type_t(11);
}
impl rsmi_event_type_t {
    ///!< Outbound beats to neighbor 2
    pub const RSMI_EVNT_XGMI_DATA_OUT_2: rsmi_event_type_t = rsmi_event_type_t(12);
}
impl rsmi_event_type_t {
    ///!< Outbound beats to neighbor 3
    pub const RSMI_EVNT_XGMI_DATA_OUT_3: rsmi_event_type_t = rsmi_event_type_t(13);
}
impl rsmi_event_type_t {
    ///!< Outbound beats to neighbor 4
    pub const RSMI_EVNT_XGMI_DATA_OUT_4: rsmi_event_type_t = rsmi_event_type_t(14);
}
impl rsmi_event_type_t {
    ///!< Outbound beats to neighbor 5
    pub const RSMI_EVNT_XGMI_DATA_OUT_5: rsmi_event_type_t = rsmi_event_type_t(15);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_XGMI_DATA_OUT_LAST: rsmi_event_type_t = rsmi_event_type_t(15);
}
impl rsmi_event_type_t {
    pub const RSMI_EVNT_LAST: rsmi_event_type_t = rsmi_event_type_t(15);
}
#[repr(transparent)]
/** Event types
 @brief Event type enum. Events belonging to a particular event group
 ::rsmi_event_group_t should begin enumerating at the ::rsmi_event_group_t
 value for that group.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_event_type_t(pub ::core::ffi::c_uint);
impl rsmi_counter_command_t {
    ///!< Start the counter
    pub const RSMI_CNTR_CMD_START: rsmi_counter_command_t = rsmi_counter_command_t(0);
}
impl rsmi_counter_command_t {
    /**!< Stop the counter; note that this should not
!< be used before reading.*/
    pub const RSMI_CNTR_CMD_STOP: rsmi_counter_command_t = rsmi_counter_command_t(1);
}
#[repr(transparent)]
/// Event counter commands
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_counter_command_t(pub ::core::ffi::c_uint);
/// Counter value
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_counter_value_t {
    ///!< Counter value
    pub value: u64,
    /**!< Time that the counter was enabled
!< (in nanoseconds)*/
    pub time_enabled: u64,
    /**!< Time that the counter was running
!< (in nanoseconds)*/
    pub time_running: u64,
}
impl rsmi_evt_notification_type_t {
    ///!< Unused
    pub const RSMI_EVT_NOTIF_NONE: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        0,
    );
}
impl rsmi_evt_notification_type_t {
    ///!< VM page fault
    pub const RSMI_EVT_NOTIF_VMFAULT: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        1,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_FIRST: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        1,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_THERMAL_THROTTLE: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        2,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_GPU_PRE_RESET: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        3,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_GPU_POST_RESET: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        4,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_MIGRATE_START: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        5,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_MIGRATE_END: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        6,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_PAGE_FAULT_START: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        7,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_PAGE_FAULT_END: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        8,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_QUEUE_EVICTION: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        9,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_QUEUE_RESTORE: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        10,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_UNMAP_FROM_GPU: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        11,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_EVENT_ALL_PROCESS: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        64,
    );
}
impl rsmi_evt_notification_type_t {
    pub const RSMI_EVT_NOTIF_LAST: rsmi_evt_notification_type_t = rsmi_evt_notification_type_t(
        64,
    );
}
#[repr(transparent)]
/// Event notification event types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_evt_notification_type_t(pub ::core::ffi::c_uint);
/// Event notification data returned from event notification API
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_evt_notification_data_t {
    ///!< Index of device that corresponds to the event
    pub dv_ind: u32,
    ///!< Event type
    pub event: rsmi_evt_notification_type_t,
    ///!< Event message
    pub message: [::core::ffi::c_char; 96usize],
}
impl rsmi_clk_type_t {
    ///!< System clock
    pub const RSMI_CLK_TYPE_SYS: rsmi_clk_type_t = rsmi_clk_type_t(0);
}
impl rsmi_clk_type_t {
    pub const RSMI_CLK_TYPE_FIRST: rsmi_clk_type_t = rsmi_clk_type_t(0);
}
impl rsmi_clk_type_t {
    /**!< Data Fabric clock (for ASICs
!< running on a separate clock)*/
    pub const RSMI_CLK_TYPE_DF: rsmi_clk_type_t = rsmi_clk_type_t(1);
}
impl rsmi_clk_type_t {
    ///!< Display Controller Engine clock
    pub const RSMI_CLK_TYPE_DCEF: rsmi_clk_type_t = rsmi_clk_type_t(2);
}
impl rsmi_clk_type_t {
    ///!< SOC clock
    pub const RSMI_CLK_TYPE_SOC: rsmi_clk_type_t = rsmi_clk_type_t(3);
}
impl rsmi_clk_type_t {
    ///!< Memory clock
    pub const RSMI_CLK_TYPE_MEM: rsmi_clk_type_t = rsmi_clk_type_t(4);
}
impl rsmi_clk_type_t {
    ///!< PCIE clock
    pub const RSMI_CLK_TYPE_PCIE: rsmi_clk_type_t = rsmi_clk_type_t(5);
}
impl rsmi_clk_type_t {
    pub const RSMI_CLK_TYPE_LAST: rsmi_clk_type_t = rsmi_clk_type_t(4);
}
impl rsmi_clk_type_t {
    pub const RSMI_CLK_INVALID: rsmi_clk_type_t = rsmi_clk_type_t(4294967295);
}
#[repr(transparent)]
/// Clock types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_clk_type_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_clk_type_t as rsmi_clk_type;
impl rsmi_compute_partition_type_t {
    pub const RSMI_COMPUTE_PARTITION_INVALID: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        0,
    );
}
impl rsmi_compute_partition_type_t {
    /**!< Single GPU mode (SPX)- All XCCs work
!< together with shared memory*/
    pub const RSMI_COMPUTE_PARTITION_SPX: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        1,
    );
}
impl rsmi_compute_partition_type_t {
    /**!< Dual GPU mode (DPX)- Half XCCs work
!< together with shared memory*/
    pub const RSMI_COMPUTE_PARTITION_DPX: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        2,
    );
}
impl rsmi_compute_partition_type_t {
    /**!< Triple GPU mode (TPX)- One-third XCCs
!< work together with shared memory*/
    pub const RSMI_COMPUTE_PARTITION_TPX: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        3,
    );
}
impl rsmi_compute_partition_type_t {
    /**!< Quad GPU mode (QPX)- Quarter XCCs
!< work together with shared memory*/
    pub const RSMI_COMPUTE_PARTITION_QPX: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        4,
    );
}
impl rsmi_compute_partition_type_t {
    /**!< Core mode (CPX)- Per-chip XCC with
!< shared memory*/
    pub const RSMI_COMPUTE_PARTITION_CPX: rsmi_compute_partition_type_t = rsmi_compute_partition_type_t(
        5,
    );
}
#[repr(transparent)]
/** @brief Compute Partition. This enum is used to identify
 various compute partitioning settings.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_compute_partition_type_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_compute_partition_type_t as rsmi_compute_partition_type;
impl rsmi_memory_partition_type_t {
    pub const RSMI_MEMORY_PARTITION_UNKNOWN: rsmi_memory_partition_type_t = rsmi_memory_partition_type_t(
        0,
    );
}
impl rsmi_memory_partition_type_t {
    /**!< NPS1 - All CCD & XCD data is interleaved
!< accross all 8 HBM stacks (all stacks/1).*/
    pub const RSMI_MEMORY_PARTITION_NPS1: rsmi_memory_partition_type_t = rsmi_memory_partition_type_t(
        1,
    );
}
impl rsmi_memory_partition_type_t {
    /**!< NPS2 - 2 sets of CCDs or 4 XCD interleaved
!< accross the 4 HBM stacks per AID pair
!< (8 stacks/2).*/
    pub const RSMI_MEMORY_PARTITION_NPS2: rsmi_memory_partition_type_t = rsmi_memory_partition_type_t(
        2,
    );
}
impl rsmi_memory_partition_type_t {
    /**!< NPS4 - Each XCD data is interleaved accross
!< accross 2 (or single) HBM stacks
!< (8 stacks/8 or 8 stacks/4).*/
    pub const RSMI_MEMORY_PARTITION_NPS4: rsmi_memory_partition_type_t = rsmi_memory_partition_type_t(
        3,
    );
}
impl rsmi_memory_partition_type_t {
    /**!< NPS8 - Each XCD uses a single HBM stack
!< (8 stacks/8). Or each XCD uses a single
!< HBM stack & CCDs share 2 non-interleaved
!< HBM stacks on its AID
!< (AID[1,2,3] = 6 stacks/6).*/
    pub const RSMI_MEMORY_PARTITION_NPS8: rsmi_memory_partition_type_t = rsmi_memory_partition_type_t(
        4,
    );
}
#[repr(transparent)]
/** @brief Memory Partitions. This enum is used to identify various
 memory partition types.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_memory_partition_type_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_memory_partition_type_t as rsmi_memory_partition_type;
impl rsmi_temperature_metric_t {
    ///!< Temperature current value.
    pub const RSMI_TEMP_CURRENT: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        0,
    );
}
impl rsmi_temperature_metric_t {
    pub const RSMI_TEMP_FIRST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(0);
}
impl rsmi_temperature_metric_t {
    ///!< Temperature max value.
    pub const RSMI_TEMP_MAX: rsmi_temperature_metric_t = rsmi_temperature_metric_t(1);
}
impl rsmi_temperature_metric_t {
    ///!< Temperature min value.
    pub const RSMI_TEMP_MIN: rsmi_temperature_metric_t = rsmi_temperature_metric_t(2);
}
impl rsmi_temperature_metric_t {
    /**!< Temperature hysteresis value for max limit.
!< (This is an absolute temperature, not a
!< delta).*/
    pub const RSMI_TEMP_MAX_HYST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        3,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature hysteresis value for min limit.
!< (This is an absolute temperature,
!<  not a delta).*/
    pub const RSMI_TEMP_MIN_HYST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        4,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature critical max value, typically
!<  greater than corresponding temp_max values.*/
    pub const RSMI_TEMP_CRITICAL: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        5,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature hysteresis value for critical
!<  limit. (This is an absolute temperature,
!<  not a delta).*/
    pub const RSMI_TEMP_CRITICAL_HYST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        6,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature emergency max value, for chips
!<  supporting more than two upper temperature
!<  limits. Must be equal or greater than
!<  corresponding temp_crit values.*/
    pub const RSMI_TEMP_EMERGENCY: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        7,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature hysteresis value for emergency
!<  limit. (This is an absolute temperature,
!<  not a delta).*/
    pub const RSMI_TEMP_EMERGENCY_HYST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        8,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature critical min value, typically
!<  lower than corresponding temperature
!<  minimum values.*/
    pub const RSMI_TEMP_CRIT_MIN: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        9,
    );
}
impl rsmi_temperature_metric_t {
    /**!< Temperature hysteresis value for critical
!< minimum limit. (This is an absolute
!< temperature, not a delta).*/
    pub const RSMI_TEMP_CRIT_MIN_HYST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        10,
    );
}
impl rsmi_temperature_metric_t {
    ///!< Temperature offset which is added to the
    pub const RSMI_TEMP_OFFSET: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        11,
    );
}
impl rsmi_temperature_metric_t {
    ///!< Historical minimum temperature.
    pub const RSMI_TEMP_LOWEST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        12,
    );
}
impl rsmi_temperature_metric_t {
    ///!< Historical maximum temperature.
    pub const RSMI_TEMP_HIGHEST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(
        13,
    );
}
impl rsmi_temperature_metric_t {
    pub const RSMI_TEMP_LAST: rsmi_temperature_metric_t = rsmi_temperature_metric_t(13);
}
#[repr(transparent)]
/** @brief Temperature Metrics.  This enum is used to identify various
 temperature metrics. Corresponding values will be in millidegress
 Celcius.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_temperature_metric_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_temperature_metric_t as rsmi_temperature_metric;
impl rsmi_temperature_type_t {
    pub const RSMI_TEMP_TYPE_FIRST: rsmi_temperature_type_t = rsmi_temperature_type_t(0);
}
impl rsmi_temperature_type_t {
    ///!< Edge GPU temperature
    pub const RSMI_TEMP_TYPE_EDGE: rsmi_temperature_type_t = rsmi_temperature_type_t(0);
}
impl rsmi_temperature_type_t {
    /**!< Junction/hotspot
!< temperature*/
    pub const RSMI_TEMP_TYPE_JUNCTION: rsmi_temperature_type_t = rsmi_temperature_type_t(
        1,
    );
}
impl rsmi_temperature_type_t {
    ///!< VRAM temperature
    pub const RSMI_TEMP_TYPE_MEMORY: rsmi_temperature_type_t = rsmi_temperature_type_t(
        2,
    );
}
impl rsmi_temperature_type_t {
    ///!< HBM temperature instance 0
    pub const RSMI_TEMP_TYPE_HBM_0: rsmi_temperature_type_t = rsmi_temperature_type_t(3);
}
impl rsmi_temperature_type_t {
    ///!< HBM temperature instance 1
    pub const RSMI_TEMP_TYPE_HBM_1: rsmi_temperature_type_t = rsmi_temperature_type_t(4);
}
impl rsmi_temperature_type_t {
    ///!< HBM temperature instance 2
    pub const RSMI_TEMP_TYPE_HBM_2: rsmi_temperature_type_t = rsmi_temperature_type_t(5);
}
impl rsmi_temperature_type_t {
    ///!< HBM temperature instance 3
    pub const RSMI_TEMP_TYPE_HBM_3: rsmi_temperature_type_t = rsmi_temperature_type_t(6);
}
impl rsmi_temperature_type_t {
    pub const RSMI_TEMP_TYPE_LAST: rsmi_temperature_type_t = rsmi_temperature_type_t(6);
}
impl rsmi_temperature_type_t {
    ///!< Invalid type
    pub const RSMI_TEMP_TYPE_INVALID: rsmi_temperature_type_t = rsmi_temperature_type_t(
        4294967295,
    );
}
#[repr(transparent)]
/** @brief This enumeration is used to indicate from which part of the device a
 temperature reading should be obtained.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_temperature_type_t(pub ::core::ffi::c_uint);
impl rsmi_activity_metric_t {
    pub const RSMI_ACTIVITY_GFX: rsmi_activity_metric_t = rsmi_activity_metric_t(1);
}
impl rsmi_activity_metric_t {
    ///!< memory controller
    pub const RSMI_ACTIVITY_UMC: rsmi_activity_metric_t = rsmi_activity_metric_t(2);
}
impl rsmi_activity_metric_t {
    ///!< UVD or VCN
    pub const RSMI_ACTIVITY_MM: rsmi_activity_metric_t = rsmi_activity_metric_t(4);
}
#[repr(transparent)]
/** @brief Activity (Utilization) Metrics.  This enum is used to identify
 various activity metrics.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_activity_metric_t(pub ::core::ffi::c_uint);
impl rsmi_voltage_metric_t {
    ///!< Voltage current value.
    pub const RSMI_VOLT_CURRENT: rsmi_voltage_metric_t = rsmi_voltage_metric_t(0);
}
impl rsmi_voltage_metric_t {
    pub const RSMI_VOLT_FIRST: rsmi_voltage_metric_t = rsmi_voltage_metric_t(0);
}
impl rsmi_voltage_metric_t {
    ///!< Voltage max value.
    pub const RSMI_VOLT_MAX: rsmi_voltage_metric_t = rsmi_voltage_metric_t(1);
}
impl rsmi_voltage_metric_t {
    ///!< Voltage critical min value.
    pub const RSMI_VOLT_MIN_CRIT: rsmi_voltage_metric_t = rsmi_voltage_metric_t(2);
}
impl rsmi_voltage_metric_t {
    ///!< Voltage min value.
    pub const RSMI_VOLT_MIN: rsmi_voltage_metric_t = rsmi_voltage_metric_t(3);
}
impl rsmi_voltage_metric_t {
    ///!< Voltage critical max value.
    pub const RSMI_VOLT_MAX_CRIT: rsmi_voltage_metric_t = rsmi_voltage_metric_t(4);
}
impl rsmi_voltage_metric_t {
    ///!< Average voltage.
    pub const RSMI_VOLT_AVERAGE: rsmi_voltage_metric_t = rsmi_voltage_metric_t(5);
}
impl rsmi_voltage_metric_t {
    ///!< Historical minimum voltage.
    pub const RSMI_VOLT_LOWEST: rsmi_voltage_metric_t = rsmi_voltage_metric_t(6);
}
impl rsmi_voltage_metric_t {
    ///!< Historical maximum voltage.
    pub const RSMI_VOLT_HIGHEST: rsmi_voltage_metric_t = rsmi_voltage_metric_t(7);
}
impl rsmi_voltage_metric_t {
    pub const RSMI_VOLT_LAST: rsmi_voltage_metric_t = rsmi_voltage_metric_t(7);
}
#[repr(transparent)]
/** @brief Voltage Metrics.  This enum is used to identify various
 Volatge metrics. Corresponding values will be in millivolt.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_voltage_metric_t(pub ::core::ffi::c_uint);
impl rsmi_voltage_type_t {
    pub const RSMI_VOLT_TYPE_FIRST: rsmi_voltage_type_t = rsmi_voltage_type_t(0);
}
impl rsmi_voltage_type_t {
    /**!< Vddgfx GPU
!< voltage*/
    pub const RSMI_VOLT_TYPE_VDDGFX: rsmi_voltage_type_t = rsmi_voltage_type_t(0);
}
impl rsmi_voltage_type_t {
    pub const RSMI_VOLT_TYPE_LAST: rsmi_voltage_type_t = rsmi_voltage_type_t(0);
}
impl rsmi_voltage_type_t {
    ///!< Invalid type
    pub const RSMI_VOLT_TYPE_INVALID: rsmi_voltage_type_t = rsmi_voltage_type_t(
        4294967295,
    );
}
#[repr(transparent)]
/** @brief This ennumeration is used to indicate which type of
 voltage reading should be obtained.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_voltage_type_t(pub ::core::ffi::c_uint);
impl rsmi_power_profile_preset_masks_t {
    ///!< Custom Power Profile
    pub const RSMI_PWR_PROF_PRST_CUSTOM_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        1,
    );
}
impl rsmi_power_profile_preset_masks_t {
    ///!< Video Power Profile
    pub const RSMI_PWR_PROF_PRST_VIDEO_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        2,
    );
}
impl rsmi_power_profile_preset_masks_t {
    ///!< Power Saving Profile
    pub const RSMI_PWR_PROF_PRST_POWER_SAVING_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        4,
    );
}
impl rsmi_power_profile_preset_masks_t {
    ///!< Compute Saving Profile
    pub const RSMI_PWR_PROF_PRST_COMPUTE_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        8,
    );
}
impl rsmi_power_profile_preset_masks_t {
    ///!< VR Power Profile
    pub const RSMI_PWR_PROF_PRST_VR_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        16,
    );
}
impl rsmi_power_profile_preset_masks_t {
    pub const RSMI_PWR_PROF_PRST_3D_FULL_SCR_MASK: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        32,
    );
}
impl rsmi_power_profile_preset_masks_t {
    ///!< Default Boot Up Profile
    pub const RSMI_PWR_PROF_PRST_BOOTUP_DEFAULT: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        64,
    );
}
impl rsmi_power_profile_preset_masks_t {
    pub const RSMI_PWR_PROF_PRST_LAST: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        64,
    );
}
impl rsmi_power_profile_preset_masks_t {
    pub const RSMI_PWR_PROF_PRST_INVALID: rsmi_power_profile_preset_masks_t = rsmi_power_profile_preset_masks_t(
        18446744073709551615,
    );
}
#[repr(transparent)]
/** @brief Pre-set Profile Selections. These bitmasks can be AND'd with the
 ::rsmi_power_profile_status_t.available_profiles returned from
 ::rsmi_dev_power_profile_presets_get to determine which power profiles
 are supported by the system.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_power_profile_preset_masks_t(pub ::core::ffi::c_ulong);
/// \cond Ignore in docs.
pub use self::rsmi_power_profile_preset_masks_t as rsmi_power_profile_preset_masks;
impl rsmi_gpu_block_t {
    /**!< Used to indicate an
!< invalid block*/
    pub const RSMI_GPU_BLOCK_INVALID: rsmi_gpu_block_t = rsmi_gpu_block_t(0);
}
impl rsmi_gpu_block_t {
    pub const RSMI_GPU_BLOCK_FIRST: rsmi_gpu_block_t = rsmi_gpu_block_t(1);
}
impl rsmi_gpu_block_t {
    ///!< UMC block
    pub const RSMI_GPU_BLOCK_UMC: rsmi_gpu_block_t = rsmi_gpu_block_t(1);
}
impl rsmi_gpu_block_t {
    ///!< SDMA block
    pub const RSMI_GPU_BLOCK_SDMA: rsmi_gpu_block_t = rsmi_gpu_block_t(2);
}
impl rsmi_gpu_block_t {
    ///!< GFX block
    pub const RSMI_GPU_BLOCK_GFX: rsmi_gpu_block_t = rsmi_gpu_block_t(4);
}
impl rsmi_gpu_block_t {
    ///!< MMHUB block
    pub const RSMI_GPU_BLOCK_MMHUB: rsmi_gpu_block_t = rsmi_gpu_block_t(8);
}
impl rsmi_gpu_block_t {
    ///!< ATHUB block
    pub const RSMI_GPU_BLOCK_ATHUB: rsmi_gpu_block_t = rsmi_gpu_block_t(16);
}
impl rsmi_gpu_block_t {
    ///!< PCIE_BIF block
    pub const RSMI_GPU_BLOCK_PCIE_BIF: rsmi_gpu_block_t = rsmi_gpu_block_t(32);
}
impl rsmi_gpu_block_t {
    ///!< HDP block
    pub const RSMI_GPU_BLOCK_HDP: rsmi_gpu_block_t = rsmi_gpu_block_t(64);
}
impl rsmi_gpu_block_t {
    ///!< XGMI block
    pub const RSMI_GPU_BLOCK_XGMI_WAFL: rsmi_gpu_block_t = rsmi_gpu_block_t(128);
}
impl rsmi_gpu_block_t {
    ///!< DF block
    pub const RSMI_GPU_BLOCK_DF: rsmi_gpu_block_t = rsmi_gpu_block_t(256);
}
impl rsmi_gpu_block_t {
    ///!< SMN block
    pub const RSMI_GPU_BLOCK_SMN: rsmi_gpu_block_t = rsmi_gpu_block_t(512);
}
impl rsmi_gpu_block_t {
    ///!< SEM block
    pub const RSMI_GPU_BLOCK_SEM: rsmi_gpu_block_t = rsmi_gpu_block_t(1024);
}
impl rsmi_gpu_block_t {
    ///!< MP0 block
    pub const RSMI_GPU_BLOCK_MP0: rsmi_gpu_block_t = rsmi_gpu_block_t(2048);
}
impl rsmi_gpu_block_t {
    ///!< MP1 block
    pub const RSMI_GPU_BLOCK_MP1: rsmi_gpu_block_t = rsmi_gpu_block_t(4096);
}
impl rsmi_gpu_block_t {
    ///!< Fuse block
    pub const RSMI_GPU_BLOCK_FUSE: rsmi_gpu_block_t = rsmi_gpu_block_t(8192);
}
impl rsmi_gpu_block_t {
    /**!< The highest bit position
!< for supported blocks*/
    pub const RSMI_GPU_BLOCK_LAST: rsmi_gpu_block_t = rsmi_gpu_block_t(8192);
}
impl rsmi_gpu_block_t {
    pub const RSMI_GPU_BLOCK_RESERVED: rsmi_gpu_block_t = rsmi_gpu_block_t(
        9223372036854775808,
    );
}
#[repr(transparent)]
/// @brief This enum is used to identify different GPU blocks.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_gpu_block_t(pub ::core::ffi::c_ulong);
/// \cond Ignore in docs.
pub use self::rsmi_gpu_block_t as rsmi_gpu_block;
impl rsmi_ras_err_state_t {
    ///!< No current errors
    pub const RSMI_RAS_ERR_STATE_NONE: rsmi_ras_err_state_t = rsmi_ras_err_state_t(0);
}
impl rsmi_ras_err_state_t {
    ///!< ECC is disabled
    pub const RSMI_RAS_ERR_STATE_DISABLED: rsmi_ras_err_state_t = rsmi_ras_err_state_t(
        1,
    );
}
impl rsmi_ras_err_state_t {
    ///!< ECC errors present, but type unknown
    pub const RSMI_RAS_ERR_STATE_PARITY: rsmi_ras_err_state_t = rsmi_ras_err_state_t(2);
}
impl rsmi_ras_err_state_t {
    ///!< Single correctable error
    pub const RSMI_RAS_ERR_STATE_SING_C: rsmi_ras_err_state_t = rsmi_ras_err_state_t(3);
}
impl rsmi_ras_err_state_t {
    ///!< Multiple uncorrectable errors
    pub const RSMI_RAS_ERR_STATE_MULT_UC: rsmi_ras_err_state_t = rsmi_ras_err_state_t(4);
}
impl rsmi_ras_err_state_t {
    /**!< Firmware detected error and isolated
!< page. Treat as uncorrectable.*/
    pub const RSMI_RAS_ERR_STATE_POISON: rsmi_ras_err_state_t = rsmi_ras_err_state_t(5);
}
impl rsmi_ras_err_state_t {
    ///!< ECC is enabled
    pub const RSMI_RAS_ERR_STATE_ENABLED: rsmi_ras_err_state_t = rsmi_ras_err_state_t(6);
}
impl rsmi_ras_err_state_t {
    pub const RSMI_RAS_ERR_STATE_LAST: rsmi_ras_err_state_t = rsmi_ras_err_state_t(6);
}
impl rsmi_ras_err_state_t {
    pub const RSMI_RAS_ERR_STATE_INVALID: rsmi_ras_err_state_t = rsmi_ras_err_state_t(
        4294967295,
    );
}
#[repr(transparent)]
/// @brief The current ECC state
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_ras_err_state_t(pub ::core::ffi::c_uint);
impl rsmi_memory_type_t {
    pub const RSMI_MEM_TYPE_FIRST: rsmi_memory_type_t = rsmi_memory_type_t(0);
}
impl rsmi_memory_type_t {
    ///!< VRAM memory
    pub const RSMI_MEM_TYPE_VRAM: rsmi_memory_type_t = rsmi_memory_type_t(0);
}
impl rsmi_memory_type_t {
    ///!< VRAM memory that is visible
    pub const RSMI_MEM_TYPE_VIS_VRAM: rsmi_memory_type_t = rsmi_memory_type_t(1);
}
impl rsmi_memory_type_t {
    ///!< GTT memory
    pub const RSMI_MEM_TYPE_GTT: rsmi_memory_type_t = rsmi_memory_type_t(2);
}
impl rsmi_memory_type_t {
    pub const RSMI_MEM_TYPE_LAST: rsmi_memory_type_t = rsmi_memory_type_t(2);
}
#[repr(transparent)]
/// @brief Types of memory
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_memory_type_t(pub ::core::ffi::c_uint);
impl rsmi_freq_ind_t {
    ///!< Index used for the minimum frequency value
    pub const RSMI_FREQ_IND_MIN: rsmi_freq_ind_t = rsmi_freq_ind_t(0);
}
impl rsmi_freq_ind_t {
    ///!< Index used for the maximum frequency value
    pub const RSMI_FREQ_IND_MAX: rsmi_freq_ind_t = rsmi_freq_ind_t(1);
}
impl rsmi_freq_ind_t {
    ///!< An invalid frequency index
    pub const RSMI_FREQ_IND_INVALID: rsmi_freq_ind_t = rsmi_freq_ind_t(4294967295);
}
#[repr(transparent)]
/// @brief The values of this enum are used as frequency identifiers.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_freq_ind_t(pub ::core::ffi::c_uint);
/// \cond Ignore in docs.
pub use self::rsmi_freq_ind_t as rsmi_freq_ind;
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_FIRST: rsmi_fw_block_t = rsmi_fw_block_t(0);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_ASD: rsmi_fw_block_t = rsmi_fw_block_t(0);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_CE: rsmi_fw_block_t = rsmi_fw_block_t(1);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_DMCU: rsmi_fw_block_t = rsmi_fw_block_t(2);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_MC: rsmi_fw_block_t = rsmi_fw_block_t(3);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_ME: rsmi_fw_block_t = rsmi_fw_block_t(4);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_MEC: rsmi_fw_block_t = rsmi_fw_block_t(5);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_MEC2: rsmi_fw_block_t = rsmi_fw_block_t(6);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_MES: rsmi_fw_block_t = rsmi_fw_block_t(7);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_MES_KIQ: rsmi_fw_block_t = rsmi_fw_block_t(8);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_PFP: rsmi_fw_block_t = rsmi_fw_block_t(9);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_RLC: rsmi_fw_block_t = rsmi_fw_block_t(10);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_RLC_SRLC: rsmi_fw_block_t = rsmi_fw_block_t(11);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_RLC_SRLG: rsmi_fw_block_t = rsmi_fw_block_t(12);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_RLC_SRLS: rsmi_fw_block_t = rsmi_fw_block_t(13);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_SDMA: rsmi_fw_block_t = rsmi_fw_block_t(14);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_SDMA2: rsmi_fw_block_t = rsmi_fw_block_t(15);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_SMC: rsmi_fw_block_t = rsmi_fw_block_t(16);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_SOS: rsmi_fw_block_t = rsmi_fw_block_t(17);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_TA_RAS: rsmi_fw_block_t = rsmi_fw_block_t(18);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_TA_XGMI: rsmi_fw_block_t = rsmi_fw_block_t(19);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_UVD: rsmi_fw_block_t = rsmi_fw_block_t(20);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_VCE: rsmi_fw_block_t = rsmi_fw_block_t(21);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_VCN: rsmi_fw_block_t = rsmi_fw_block_t(22);
}
impl rsmi_fw_block_t {
    pub const RSMI_FW_BLOCK_LAST: rsmi_fw_block_t = rsmi_fw_block_t(22);
}
#[repr(transparent)]
/** @brief The values of this enum are used to identify the various firmware
 blocks.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_fw_block_t(pub ::core::ffi::c_uint);
impl rsmi_xgmi_status_t {
    pub const RSMI_XGMI_STATUS_NO_ERRORS: rsmi_xgmi_status_t = rsmi_xgmi_status_t(0);
}
impl rsmi_xgmi_status_t {
    pub const RSMI_XGMI_STATUS_ERROR: rsmi_xgmi_status_t = rsmi_xgmi_status_t(1);
}
impl rsmi_xgmi_status_t {
    pub const RSMI_XGMI_STATUS_MULTIPLE_ERRORS: rsmi_xgmi_status_t = rsmi_xgmi_status_t(
        2,
    );
}
#[repr(transparent)]
/// @brief XGMI Status
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_xgmi_status_t(pub ::core::ffi::c_uint);
/// @brief Bitfield used in various RSMI calls
pub type rsmi_bit_field_t = u64;
/// \cond Ignore in docs.
pub type rsmi_bit_field = rsmi_bit_field_t;
impl rsmi_memory_page_status_t {
    /**!< Reserved. This gpu page is reserved
!<  and not available for use*/
    pub const RSMI_MEM_PAGE_STATUS_RESERVED: rsmi_memory_page_status_t = rsmi_memory_page_status_t(
        0,
    );
}
impl rsmi_memory_page_status_t {
    /**!< Pending. This gpu page is marked
!<  as bad and will be marked reserved
!<  at the next window.*/
    pub const RSMI_MEM_PAGE_STATUS_PENDING: rsmi_memory_page_status_t = rsmi_memory_page_status_t(
        1,
    );
}
impl rsmi_memory_page_status_t {
    ///!< Unable to reserve this page
    pub const RSMI_MEM_PAGE_STATUS_UNRESERVABLE: rsmi_memory_page_status_t = rsmi_memory_page_status_t(
        2,
    );
}
#[repr(transparent)]
/// @brief Reserved Memory Page States
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_memory_page_status_t(pub ::core::ffi::c_uint);
impl _RSMI_IO_LINK_TYPE {
    ///!< unknown type.
    pub const RSMI_IOLINK_TYPE_UNDEFINED: _RSMI_IO_LINK_TYPE = _RSMI_IO_LINK_TYPE(0);
}
impl _RSMI_IO_LINK_TYPE {
    ///!< PCI Express
    pub const RSMI_IOLINK_TYPE_PCIEXPRESS: _RSMI_IO_LINK_TYPE = _RSMI_IO_LINK_TYPE(1);
}
impl _RSMI_IO_LINK_TYPE {
    ///!< XGMI
    pub const RSMI_IOLINK_TYPE_XGMI: _RSMI_IO_LINK_TYPE = _RSMI_IO_LINK_TYPE(2);
}
impl _RSMI_IO_LINK_TYPE {
    ///!< Number of IO Link types
    pub const RSMI_IOLINK_TYPE_NUMIOLINKTYPES: _RSMI_IO_LINK_TYPE = _RSMI_IO_LINK_TYPE(
        3,
    );
}
impl _RSMI_IO_LINK_TYPE {
    ///!< Max of IO Link types
    pub const RSMI_IOLINK_TYPE_SIZE: _RSMI_IO_LINK_TYPE = _RSMI_IO_LINK_TYPE(4294967295);
}
#[repr(transparent)]
/// @brief Types for IO Link
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct _RSMI_IO_LINK_TYPE(pub ::core::ffi::c_uint);
/// @brief Types for IO Link
pub use self::_RSMI_IO_LINK_TYPE as RSMI_IO_LINK_TYPE;
impl RSMI_UTILIZATION_COUNTER_TYPE {
    pub const RSMI_UTILIZATION_COUNTER_FIRST: RSMI_UTILIZATION_COUNTER_TYPE = RSMI_UTILIZATION_COUNTER_TYPE(
        0,
    );
}
impl RSMI_UTILIZATION_COUNTER_TYPE {
    pub const RSMI_COARSE_GRAIN_GFX_ACTIVITY: RSMI_UTILIZATION_COUNTER_TYPE = RSMI_UTILIZATION_COUNTER_TYPE(
        0,
    );
}
impl RSMI_UTILIZATION_COUNTER_TYPE {
    ///!< Memory Activity
    pub const RSMI_COARSE_GRAIN_MEM_ACTIVITY: RSMI_UTILIZATION_COUNTER_TYPE = RSMI_UTILIZATION_COUNTER_TYPE(
        1,
    );
}
impl RSMI_UTILIZATION_COUNTER_TYPE {
    pub const RSMI_UTILIZATION_COUNTER_LAST: RSMI_UTILIZATION_COUNTER_TYPE = RSMI_UTILIZATION_COUNTER_TYPE(
        1,
    );
}
#[repr(transparent)]
/// @brief The utilization counter type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RSMI_UTILIZATION_COUNTER_TYPE(pub ::core::ffi::c_uint);
impl RSMI_POWER_TYPE {
    ///!< Average Power
    pub const RSMI_AVERAGE_POWER: RSMI_POWER_TYPE = RSMI_POWER_TYPE(0);
}
impl RSMI_POWER_TYPE {
    ///!< Current / Instant Power
    pub const RSMI_CURRENT_POWER: RSMI_POWER_TYPE = RSMI_POWER_TYPE(1);
}
impl RSMI_POWER_TYPE {
    ///!< Invalid / Undetected Power
    pub const RSMI_INVALID_POWER: RSMI_POWER_TYPE = RSMI_POWER_TYPE(4294967295);
}
#[repr(transparent)]
/// @brief Power types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct RSMI_POWER_TYPE(pub ::core::ffi::c_uint);
/// @brief The utilization counter data
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_utilization_counter_t {
    ///!< Utilization counter type
    pub type_: RSMI_UTILIZATION_COUNTER_TYPE,
    ///!< Utilization counter value
    pub value: u64,
}
/// @brief Reserved Memory Page Record
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_retired_page_record_t {
    ///!< Start address of page
    pub page_address: u64,
    ///!< Page size
    pub page_size: u64,
    ///!< Page "reserved" status
    pub status: rsmi_memory_page_status_t,
}
/** @brief This structure contains information about which power profiles are
 supported by the system for a given device, and which power profile is
 currently active.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_power_profile_status_t {
    /// Which profiles are supported by this system
    pub available_profiles: rsmi_bit_field_t,
    /// Which power profile is currently active
    pub current: rsmi_power_profile_preset_masks_t,
    /// How many power profiles are available
    pub num_profiles: u32,
}
/// \cond Ignore in docs.
pub type rsmi_power_profile_status = rsmi_power_profile_status_t;
/// @brief This structure holds information about clock frequencies.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_frequencies_t {
    /// Deep Sleep frequency is only supported by some GPUs
    pub has_deep_sleep: bool,
    /// The number of supported frequencies
    pub num_supported: u32,
    /// The current frequency index
    pub current: u32,
    /** List of frequencies.
 Only the first num_supported frequencies are valid.*/
    pub frequency: [u64; 33usize],
}
/// \cond Ignore in docs.
pub type rsmi_frequencies = rsmi_frequencies_t;
/** @brief This structure holds information about the possible PCIe
 bandwidths. Specifically, the possible transfer rates and their
 associated numbers of lanes are stored here.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_pcie_bandwidth_t {
    /// Transfer rates (T/s) that are possible
    pub transfer_rate: rsmi_frequencies_t,
    /** List of lanes for corresponding transfer rate.
 Only the first num_supported bandwidths are valid.*/
    pub lanes: [u32; 33usize],
}
/// \cond Ignore in docs.
pub type rsmi_pcie_bandwidth = rsmi_pcie_bandwidth_t;
/** @brief This structure holds information about the possible activity
 averages. Specifically, the utilization counters.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_activity_metric_counter_t {
    ///!< Average graphics activity
    pub average_gfx_activity: u16,
    ///!< memory controller
    pub average_umc_activity: u16,
    ///!< UVD or VCN
    pub average_mm_activity: u16,
}
/// @brief This structure holds version information.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_version_t {
    ///!< Major version
    pub major: u32,
    ///!< Minor version
    pub minor: u32,
    ///!< Patch, build  or stepping version
    pub patch: u32,
    ///!< Build string
    pub build: *const ::core::ffi::c_char,
}
/// \cond Ignore in docs.
pub type rsmi_version = rsmi_version_t;
#[doc = " \\endcond\n**\n* @brief This structure represents a range (e.g., frequencies or voltages).\n*/"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_range_t {
    ///!< Lower bound of range
    pub lower_bound: u64,
    ///!< Upper bound of range
    pub upper_bound: u64,
}
/// \cond Ignore in docs.
pub type rsmi_range = rsmi_range_t;
/// @brief This structure represents a point on the frequency-voltage plane.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_od_vddc_point_t {
    ///!< Frequency coordinate (in Hz)
    pub frequency: u64,
    ///!< Voltage coordinate (in mV)
    pub voltage: u64,
}
/// \cond Ignore in docs.
pub type rsmi_od_vddc_point = rsmi_od_vddc_point_t;
/** @brief This structure holds 2 ::rsmi_range_t's, one for frequency and one for
 voltage. These 2 ranges indicate the range of possible values for the
 corresponding ::rsmi_od_vddc_point_t.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_freq_volt_region_t {
    ///!< The frequency range for this VDDC Curve point
    pub freq_range: rsmi_range_t,
    ///!< The voltage range for this VDDC Curve point
    pub volt_range: rsmi_range_t,
}
/// \cond Ignore in docs.
pub type rsmi_freq_volt_region = rsmi_freq_volt_region_t;
/// ::RSMI_NUM_VOLTAGE_CURVE_POINTS number of ::rsmi_od_vddc_point_t's
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_od_volt_curve_t {
    /** Array of ::RSMI_NUM_VOLTAGE_CURVE_POINTS ::rsmi_od_vddc_point_t's that
 make up the voltage frequency curve points.*/
    pub vc_points: [rsmi_od_vddc_point_t; 3usize],
}
/// \cond Ignore in docs.
pub type rsmi_od_volt_curve = rsmi_od_volt_curve_t;
/// @brief This structure holds the frequency-voltage values for a device.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_od_volt_freq_data_t {
    ///!< The current SCLK frequency range
    pub curr_sclk_range: rsmi_range_t,
    /**!< The current MCLK frequency range;
!< (upper bound only)*/
    pub curr_mclk_range: rsmi_range_t,
    ///!< The range possible of SCLK values
    pub sclk_freq_limits: rsmi_range_t,
    ///!< The range possible of MCLK values
    pub mclk_freq_limits: rsmi_range_t,
    /// @brief The current voltage curve
    pub curve: rsmi_od_volt_curve_t,
    ///!< The number of voltage curve regions
    pub num_regions: u32,
}
/// \cond Ignore in docs.
pub type rsmi_od_volt_freq_data = rsmi_od_volt_freq_data_t;
/// @brief Size and version information of metrics data
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct metrics_table_header_t {
    /// \cond Ignore in docs.
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
}
/// @brief The following structures hold the gpu statistics for a device.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct amdgpu_xcp_metrics_t {
    pub gfx_busy_inst: [u32; 8usize],
    pub jpeg_busy: [u16; 32usize],
    pub vcn_busy: [u16; 4usize],
    pub gfx_busy_acc: [u64; 8usize],
    pub gfx_below_host_limit_acc: [u64; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_gpu_metrics_t {
    /// \cond Ignore in docs.
    pub common_header: metrics_table_header_t,
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
    pub average_gfx_activity: u16,
    pub average_umc_activity: u16,
    pub average_mm_activity: u16,
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
    pub system_clock_counter: u64,
    pub average_gfxclk_frequency: u16,
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
    pub current_gfxclk: u16,
    pub current_socclk: u16,
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,
    pub throttle_status: u32,
    pub current_fan_speed: u16,
    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,
    pub gfx_activity_acc: u32,
    pub mem_activity_acc: u32,
    pub temperature_hbm: [u16; 4usize],
    pub firmware_timestamp: u64,
    pub voltage_soc: u16,
    pub voltage_gfx: u16,
    pub voltage_mem: u16,
    pub indep_throttle_status: u64,
    pub current_socket_power: u16,
    pub vcn_activity: [u16; 4usize],
    pub gfxclk_lock_status: u32,
    pub xgmi_link_width: u16,
    pub xgmi_link_speed: u16,
    pub pcie_bandwidth_acc: u64,
    pub pcie_bandwidth_inst: u64,
    pub pcie_l0_to_recov_count_acc: u64,
    pub pcie_replay_count_acc: u64,
    pub pcie_replay_rover_count_acc: u64,
    pub xgmi_read_data_acc: [u64; 8usize],
    pub xgmi_write_data_acc: [u64; 8usize],
    pub current_gfxclks: [u16; 8usize],
    pub current_socclks: [u16; 4usize],
    pub current_vclk0s: [u16; 4usize],
    pub current_dclk0s: [u16; 4usize],
    pub jpeg_activity: [u16; 32usize],
    pub pcie_nak_sent_count_acc: u32,
    pub pcie_nak_rcvd_count_acc: u32,
    pub accumulation_counter: u64,
    /// Accumulated throttler residencies
    pub prochot_residency_acc: u64,
    /** Accumulated throttler residencies

 Prochot (thermal) - PPT (power)
 Package Power Tracking (PPT) violation % (greater than 0% is a violation);
 aka PVIOL

 Ex. PVIOL/TVIOL calculations
 Where A and B are measurments recorded at prior points in time.
 Typically A is the earlier measured value and B is the latest measured value.

 PVIOL % = (PptResidencyAcc (B) - PptResidencyAcc (A)) * 100/ (AccumulationCounter (B) - AccumulationCounter (A))
 TVIOL % = (SocketThmResidencyAcc (B) -  SocketThmResidencyAcc (A)) * 100 / (AccumulationCounter (B) - AccumulationCounter (A))*/
    pub ppt_residency_acc: u64,
    /** Accumulated throttler residencies

 Socket (thermal) -
 Socket thermal violation % (greater than 0% is a violation);
 aka TVIOL

 Ex. PVIOL/TVIOL calculations
 Where A and B are measurments recorded at prior points in time.
 Typically A is the earlier measured value and B is the latest measured value.

 PVIOL % = (PptResidencyAcc (B) - PptResidencyAcc (A)) * 100/ (AccumulationCounter (B) - AccumulationCounter (A))
 TVIOL % = (SocketThmResidencyAcc (B) -  SocketThmResidencyAcc (A)) * 100 / (AccumulationCounter (B) - AccumulationCounter (A))*/
    pub socket_thm_residency_acc: u64,
    pub vr_thm_residency_acc: u64,
    pub hbm_thm_residency_acc: u64,
    pub num_partition: u16,
    pub xcp_stats: [amdgpu_xcp_metrics_t; 8usize],
    pub pcie_lc_perf_other_end_recovery: u32,
    pub vram_max_bandwidth: u64,
    pub xgmi_link_status: [u16; 8usize],
}
/// @brief This structure holds error counts.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_error_count_t {
    ///!< Accumulated correctable errors
    pub correctable_err: u64,
    ///!< Accumulated uncorrectable errors
    pub uncorrectable_err: u64,
}
/// @brief This structure contains information specific to a process.
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct rsmi_process_info_t {
    ///!< Process ID
    pub process_id: u32,
    ///!< PASID
    pub pasid: u32,
    ///!< VRAM usage
    pub vram_usage: u64,
    ///!< SDMA usage in microseconds
    pub sdma_usage: u64,
    ///!< Compute Unit usage in percent
    pub cu_occupancy: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rsmi_func_id_iter_handle {
    _unused: [u8; 0],
}
/// @brief Opaque handle to function-support object
pub type rsmi_func_id_iter_handle_t = *mut rsmi_func_id_iter_handle;
/** @brief This union holds the value of an ::rsmi_func_id_iter_handle_t. The
 value may be a function name, or an ennumerated variant value of types
 such as ::rsmi_memory_type_t, ::rsmi_temperature_metric_t, etc.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub union id {
    ///!< uint64_t representation of value
    pub id: u64,
    ///!< name string (applicable to functions only)
    pub name: *const ::core::ffi::c_char,
    pub __bindgen_anon_1: id__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union id__bindgen_ty_1 {
    /// Used for ::rsmi_memory_type_t variants
    pub memory_type: rsmi_memory_type_t,
    /// Used for ::rsmi_temperature_metric_t variants
    pub temp_metric: rsmi_temperature_metric_t,
    /// Used for ::rsmi_event_type_t variants
    pub evnt_type: rsmi_event_type_t,
    /// Used for ::rsmi_event_group_t variants
    pub evnt_group: rsmi_event_group_t,
    /// Used for ::rsmi_clk_type_t variants
    pub clk_type: rsmi_clk_type_t,
    /// Used for ::rsmi_fw_block_t variants
    pub fw_block: rsmi_fw_block_t,
    /// Used for ::rsmi_gpu_block_t variants
    pub gpu_block_type: rsmi_gpu_block_t,
}
/** @brief This union holds the value of an ::rsmi_func_id_iter_handle_t. The
 value may be a function name, or an ennumerated variant value of types
 such as ::rsmi_memory_type_t, ::rsmi_temperature_metric_t, etc.*/
pub type rsmi_func_id_value_t = id;
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup InitShutAdmin Initialization and Shutdown\n  These functions are used for initialization of ROCm SMI and clean up when\n  done.\n  @{\n/\n/**\n  @brief Initialize ROCm SMI.\n\n  @details When called, this initializes internal data structures,\n  including those corresponding to sources of information that SMI provides.\n\n  @param[in] init_flags Bit flags that tell SMI how to initialze. Values of\n  ::rsmi_init_flags_t may be OR'd together and passed through @p init_flags\n  to modify how RSMI initializes.\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call."]
    pub fn rsmi_init(init_flags: u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Shutdown ROCm SMI.

  @details Do any necessary clean up.*/
    pub fn rsmi_shut_down() -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup IDQuer Identifier Queries\n  These functions provide identification information.\n  @{\n/\n/**\n  @brief Get the number of devices that have monitor information.\n\n  @details The number of devices which have monitors is returned. Monitors\n  are referenced by the index which can be between 0 and @p num_devices - 1.\n\n  @param[inout] num_devices Caller provided pointer to uint32_t. Upon\n  successful call, the value num_devices will contain the number of monitor\n  devices.\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call."]
    pub fn rsmi_num_monitor_devices(num_devices: *mut u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device id associated with the device with provided device
  index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p id,
  this function will write the device id value to the uint64_t pointed to by
  @p id. This ID is an identification of the type of device, so calling this
  function for different devices will give the same value if they are kind
  of device. Consequently, this function should not be used to distinguish
  one device from another. rsmi_dev_pci_id_get() should be used to get a
  unique identifier.

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint64_t to which the device id will be
  written
 If this parameter is nullptr, this function will return
 ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
 arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
 provided arguments.

 @retval ::RSMI_STATUS_SUCCESS call was successful
 @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
 support this function with the given arguments
 @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_id_get(dv_ind: u32, id: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device revision associated with the device

  @details Given a device index @p dv_ind and a pointer to a uint32_t to
  which the revision will be written

  @param[in] dv_ind a device index

  @param[inout] revision a pointer to uint32_t to which the device revision
  will be written

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_revision_get(dv_ind: u32, revision: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the SKU for a desired device associated with the device with
  provided device index.

  @details Given a device index @p dv_ind and a pointer to a char @p sku,
  this function will attempt to obtain the SKU from the Product Information
  FRU chip, present on server ASICs. It will write the sku value to the
  char array pointed to by @p sku.

  @param[in] dv_ind a device index

  @param[inout] sku a pointer to char to which the sku will be written

  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_sku_get(dv_ind: u32, sku: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device vendor id associated with the device with provided
  device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p id,
  this function will write the device vendor id value to the uint64_t pointed
  to by @p id.

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint64_t to which the device vendor id will
  be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_vendor_id_get(dv_ind: u32, id: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the name string of a gpu device.

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p name, and a length of this buffer @p len, this function
  will write the name of the device (up to @p len characters) to the buffer
  @p name.

  If the integer ID associated with the device is not found in one of the
  system files containing device name information (e.g.
  /usr/share/misc/pci.ids), then this function will return the hex device ID
  as a string. Updating the system name files can be accompplished with
  "sudo update-pciids".

  @param[in] dv_ind a device index

  @param[inout] name a pointer to a caller provided char buffer to which the
  name will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len the length of the caller provided buffer @p name.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_name_get(
        dv_ind: u32,
        name: *mut ::core::ffi::c_char,
        len: usize,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the brand string of a gpu device.

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p brand, and a length of this buffer @p len, this function
  will write the brand of the device (up to @p len characters) to the buffer
  @p brand.

  If the sku associated with the device is not found as one of the values
  contained within rsmi_dev_brand_get, then this function will return the
  device marketing name as a string instead of the brand name.

  @param[in] dv_ind a device index

  @param[inout] brand a pointer to a caller provided char buffer to which the
  brand will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len the length of the caller provided buffer @p brand.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_brand_get(
        dv_ind: u32,
        brand: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the name string for a give vendor ID

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p name, and a length of this buffer @p len, this function will
  write the name of the vendor (up to @p len characters) buffer @p name. The
  @p id may be a device vendor or subsystem vendor ID.

  If the integer ID associated with the vendor is not found in one of the
  system files containing device name information (e.g.
  /usr/share/misc/pci.ids), then this function will return the hex vendor ID
  as a string. Updating the system name files can be accompplished with
  "sudo update-pciids".

  @param[in] dv_ind a device index

  @param[inout] name a pointer to a caller provided char buffer to which the
  name will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len the length of the caller provided buffer @p name.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_vendor_name_get(
        dv_ind: u32,
        name: *mut ::core::ffi::c_char,
        len: usize,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device's market name

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p market_name, and a length of this buffer @p len, this function will
  write the name of the market name (up to @p len characters) buffer @p market_name.

  @param[inout] market_name a pointer to a caller provided char buffer to which the
  market name will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_DRM_ERROR if a DRM error occurs

  @param[in] len the length of the caller provided buffer @p name.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_DRM_ERROR if a DRM error occurs
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_market_name_get(
        dv_ind: u32,
        market_name: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the vram vendor string of a gpu device.

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p brand, and a length of this buffer @p len, this function
  will write the vram vendor of the device (up to @p len characters) to the
  buffer @p brand.

  If the vram vendor for the device is not found as one of the values
  contained within rsmi_dev_vram_vendor_get, then this function will return
  the string 'unknown' instead of the vram vendor.

  @param[in] dv_ind a device index

  @param[inout] brand a pointer to a caller provided char buffer to which the
  vram vendor will be written

  @param[in] len the length of the caller provided buffer @p brand.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_vram_vendor_get(
        dv_ind: u32,
        brand: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Get the serial number string for a device

 @details Given a device index @p dv_ind, a pointer to a buffer of chars
 @p serial_num, and the length of the provided buffer @p len, this function
 will write the serial number string (up to @p len characters) to the buffer
 pointed to by @p serial_num.

  @param[in] dv_ind a device index

  @param[inout] serial_num a pointer to caller-provided memory to which the
  serial number will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len the length of the caller provided buffer @p serial_num.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_serial_number_get(
        dv_ind: u32,
        serial_num: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the subsystem device id associated with the device with
  provided device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p id,
  this function will write the subsystem device id value to the uint64_t
  pointed to by @p id.

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint64_t to which the subsystem device id
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_subsystem_id_get(dv_ind: u32, id: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the name string for the device subsytem

  @details Given a device index @p dv_ind, a pointer to a caller provided
  char buffer @p name, and a length of this buffer @p len, this function
  will write the name of the device subsystem (up to @p len characters)
  to the buffer @p name.

  If the integer ID associated with the sub-system is not found in one of the
  system files containing device name information (e.g.
  /usr/share/misc/pci.ids), then this function will return the hex sub-system
  ID as a string. Updating the system name files can be accompplished with
  "sudo update-pciids".

  @param[in] dv_ind a device index

  @param[inout] name a pointer to a caller provided char buffer to which the
  name will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len the length of the caller provided buffer @p name.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.
*/
    pub fn rsmi_dev_subsystem_name_get(
        dv_ind: u32,
        name: *mut ::core::ffi::c_char,
        len: usize,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the drm minor number associated with this device

  @details Given a device index @p dv_ind, find its render device file
  /dev/dri/renderDN where N corresponds to its minor number.

  @param[in] dv_ind a device index

  @param[inout] minor a pointer to a uint32_t into which minor number will
  be copied

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_INIT_ERROR if failed to get minor number during
  initialization.
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_drm_render_minor_get(dv_ind: u32, minor: *mut u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device subsystem vendor id associated with the device with
  provided device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p id,
  this function will write the device subsystem vendor id value to the
  uint64_t pointed to by @p id.

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint64_t to which the device subsystem vendor
  id will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_subsystem_vendor_id_get(dv_ind: u32, id: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get Unique ID

  @details Given a device index @p dv_ind and a pointer to a uint64_t @p
  id, this function will write the unique ID of the GPU pointed to @p
  id.

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint64_t to which the unique ID of the GPU
  is written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_unique_id_get(dv_ind: u32, id: *mut u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the XGMI physical id associated with the device

  @details Given a device index @p dv_ind and a pointer to a uint32_t to
  which the XGMI physical id will be written

  @param[in] dv_ind a device index

  @param[inout] id a pointer to uint32_t to which the XGMI physical id
  will be written

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_xgmi_physical_id_get(dv_ind: u32, id: *mut u16) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the GUID, also known as the GPU device id,
  associated with the provided device index indicated by KFD.

  @details Given a device index @p dv_ind and a pointer to a uint64_t
  @p guid, this function will write the KFD GPU id value to the
  uint64_t pointed to by @p guid.

  @param[in] dv_ind a device index

  @param[inout] guid a pointer to uint64_t to which the KFD gpu id will be
  written. If the @p guid parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS. If the GPU ID is not supported with
  the device index queried, gpu_id will return MAX UINT64 value an
  arguments and ::RSMI_STATUS_NOT_SUPPORTED as a response.

 @retval ::RSMI_STATUS_SUCCESS call was successful
 @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
 support this function with the given arguments
 @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_guid_get(dv_ind: u32, guid: *mut u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the node id associated with the provided device index
  indicated by KFD.

  @details Given a device index @p dv_ind and a pointer to a uint32_t
  @p node_id, this function will write the KFD node id value to the
  uint32_t pointed to by @p node_id.

  @param[in] dv_ind a device index

  @param[inout] node_id a pointer to uint64_t to which the KFD gpu id will be
  written. If the @p node_id parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS. If @p node_id is not supported with
  the device index queried, @p node_id will return MAX UINT64 value as an
  argument and ::RSMI_STATUS_NOT_SUPPORTED as a response.

 @retval ::RSMI_STATUS_SUCCESS call was successful
 @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
 support this function with the given arguments
 @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_node_id_get(dv_ind: u32, node_id: *mut u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup PCIeQuer PCIe Queries\n  These functions provide information about PCIe.\n  @{\n/\n/**\n  @brief Get the list of possible PCIe bandwidths that are available.\n\n  @details Given a device index @p dv_ind and a pointer to a to an\n  ::rsmi_pcie_bandwidth_t structure @p bandwidth, this function will fill in\n  @p bandwidth with the possible T/s values and associated number of lanes,\n  and indication of the current selection.\n\n  @param[in] dv_ind a device index\n\n  @param[inout] bandwidth a pointer to a caller provided\n  ::rsmi_pcie_bandwidth_t structure to which the frequency information will be\n  written\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.\n  @retval ::RSMI_STATUS_UNEXPECTED_DATA Data read or provided was not as\n  expected\n"]
    pub fn rsmi_dev_pci_bandwidth_get(
        dv_ind: u32,
        bandwidth: *mut rsmi_pcie_bandwidth_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the unique PCI device identifier associated for a device

  @details Give a device index @p dv_ind and a pointer to a uint64_t @p
  bdfid, this function will write the Bus/Device/Function PCI identifier
  (BDFID) associated with device @p dv_ind to the value pointed to by
  @p bdfid.

  The format of @p bdfid will be as follows:

      BDFID = ((DOMAIN & 0xFFFFFFFF) << 32) | ((Partition & 0xF) << 28)
              | ((BUS & 0xFF) << 8) | ((DEVICE & 0x1F) <<3 )
              | (FUNCTION & 0x7)

  \code{.unparsed}
  | Name         | Field   | KFD property       KFD -> PCIe ID (uint64_t)
  -------------- | ------- | ---------------- | ---------------------------- |
  | Domain       | [63:32] | "domain"         | (DOMAIN & 0xFFFFFFFF) << 32  |
  | Partition id | [31:28] | "location id"    | (LOCATION & 0xF0000000)      |
  | Reserved     | [27:16] | "location id"    | N/A                          |
  | Bus          | [15: 8] | "location id"    | (LOCATION & 0xFF00)          |
  | Device       | [ 7: 3] | "location id"    | (LOCATION & 0xF8)            |
  | Function     | [ 2: 0] | "location id"    | (LOCATION & 0x7)             |
  \endcode

  Note: In some devices, the partition ID may be stored in the function bits
  BDFID[2:0] instead of BDFID[31:28].

  Note: For MI series devices, the function bits are only used to store the
  partition ID, but this modified BDF is internal to the ROCm stack.
  To the OS, partitions share the same BDF as the unpartitioned device and
  have function bits = 0, which can be verified through lspci.

  @param[in] dv_ind a device index

  @param[inout] bdfid a pointer to uint64_t to which the device bdfid value
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_pci_id_get(dv_ind: u32, bdfid: *mut u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the NUMA node associated with a device

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p
  numa_node, this function will retrieve the NUMA node value associated
  with device @p dv_ind and store the value at location pointed to by
  @p numa_node.

  @param[in] dv_ind a device index

  @param[inout] numa_node pointer to location where NUMA node value will
  be written.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_topo_numa_affinity_get(
        dv_ind: u32,
        numa_node: *mut i32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get PCIe traffic information

  @details Give a device index @p dv_ind and pointers to a uint64_t's, @p
  sent, @p received and @p max_pkt_sz, this function will write the number
  of bytes sent and received in 1 second to @p sent and @p received,
  respectively. The maximum possible packet size will be written to
  @p max_pkt_sz.

  @param[in] dv_ind a device index

  @param[inout] sent a pointer to uint64_t to which the number of bytes sent
  will be written in 1 second. If pointer is NULL, it will be ignored.

  @param[inout] received a pointer to uint64_t to which the number of bytes
  received will be written. If pointer is NULL, it will be ignored.

  @param[inout] max_pkt_sz a pointer to uint64_t to which the maximum packet
  size will be written. If pointer is NULL, it will be ignored.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments*/
    pub fn rsmi_dev_pci_throughput_get(
        dv_ind: u32,
        sent: *mut u64,
        received: *mut u64,
        max_pkt_sz: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get PCIe replay counter

  @details Given a device index @p dv_ind and a pointer to a uint64_t @p
  counter, this function will write the sum of the number of NAK's received
  by the GPU and the NAK's generated by the GPU to memory pointed to by @p
  counter.

  @param[in] dv_ind a device index

  @param[inout] counter a pointer to uint64_t to which the sum of the NAK's
  received and generated by the GPU is written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_pci_replay_counter_get(
        dv_ind: u32,
        counter: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Control the set of allowed PCIe bandwidths that can be used.

  @details Given a device index @p dv_ind and a 64 bit bitmask @p bw_bitmask,
  this function will limit the set of allowable bandwidths. If a bit in @p
  bw_bitmask has a value of 1, then the frequency (as ordered in an
  ::rsmi_frequencies_t returned by ::rsmi_dev_gpu_clk_freq_get()) corresponding
  to that bit index will be allowed.

  This function will change the performance level to
  ::RSMI_DEV_PERF_LEVEL_MANUAL in order to modify the set of allowable
  band_widths. Caller will need to set to ::RSMI_DEV_PERF_LEVEL_AUTO in order
  to get back to default state.

  All bits with indices greater than or equal to the value of the
  ::rsmi_frequencies_t::num_supported field of ::rsmi_pcie_bandwidth_t will be
  ignored.

  @param[in] dv_ind a device index

  @param[in] bw_bitmask A bitmask indicating the indices of the
  bandwidths that are to be enabled (1) and disabled (0). Only the lowest
  ::rsmi_frequencies_t::num_supported (of ::rsmi_pcie_bandwidth_t) bits of
  this mask are relevant.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_pci_bandwidth_set(dv_ind: u32, bw_bitmask: u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup PowerQuer Power Queries\n  These functions provide information about power usage.\n  @{\n/\n/**\n  @brief Get the average power consumption of the device with provided\n  device index.\n\n  @details Given a device index @p dv_ind and a pointer to a uint64_t\n  @p power, this function will write the current average power consumption\n  (in microwatts) to the uint64_t pointed to by @p power.\n\n  @deprecated ::rsmi_dev_power_get() is preferred due to providing\n  backwards compatibility, which looks at both average and current power\n  values. Whereas ::rsmi_dev_power_ave_get only looks for average power\n  consumption. Newer ASICs will support current power only.\n\n  @param[in] dv_ind a device index\n\n  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.\n  If a device has more than one sensor, it could be greater than 0.\n\n  @param[inout] power a pointer to uint64_t to which the average power\n  consumption will be written\n  If this parameter is nullptr, this function will return\n  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,\n  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the\n  provided arguments.\n\n  @retval ::RSMI_STATUS_SUCCESS call was successful\n  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not\n  support this function with the given arguments\n  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid"]
    pub fn rsmi_dev_power_ave_get(
        dv_ind: u32,
        sensor_ind: u32,
        power: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the current socket power (also known as instant
  power) of the device index provided.

  @details Given a device index @p dv_ind and a pointer to a uint64_t
  @p socket_power, this function will write the current socket power
  (in microwatts) to the uint64_t pointed to by @p socket_power.

  @param[in] dv_ind a device index

  @param[inout] socket_power a pointer to uint64_t to which the current
  socket power will be written to. If this parameter is nullptr,
  this function will return ::RSMI_STATUS_INVALID_ARGS if the function is
  supported with the provided, arguments and ::RSMI_STATUS_NOT_SUPPORTED
  if it is not supported with the provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_current_socket_power_get(
        dv_ind: u32,
        socket_power: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief A generic get which attempts to retieve current socket power
  (also known as instant power) of the device index provided, if not
  supported tries to get average power consumed by device. Current
  socket power is typically supported by newer devices, whereas average
  power is generally reported on older devices. This function
  aims to provide backwards compatability depending on device support.

  @details Given a device index @p dv_ind, a pointer to a uint64_t
  @p power, and @p type this function will write the current socket or
  average power (in microwatts) to the uint64_t pointed to by @p power and
  a pointer to its @p type RSMI_POWER_TYPE read.

  @param[in] dv_ind a device index

  @param[inout] power a pointer to uint64_t to which the current or average
  power will be written to. If this parameter is nullptr,
  this function will return ::RSMI_STATUS_INVALID_ARGS if the function is
  supported with the provided, arguments and ::RSMI_STATUS_NOT_SUPPORTED
  if it is not supported with the provided arguments.

  @param[inout] type a pointer to RSMI_POWER_TYPE object. Returns the type
  of power retrieved from the device. Current power is ::RSMI_CURRENT_POWER
  and average power is ::RSMI_AVERAGE_POWER. If an error occurs,
  returns an invalid power type ::RSMI_INVALID_POWER - example device
  neither supports average power or current power.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_power_get(
        dv_ind: u32,
        power: *mut u64,
        type_: *mut RSMI_POWER_TYPE,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the energy accumulator counter of the device with provided
  device index.

  @details Given a device index @p dv_ind, a pointer to a uint64_t
  @p power, and a pointer to a uint64_t @p timestamp, this function will write
  amount of energy consumed to the uint64_t pointed to by @p power,
  and the timestamp to the uint64_t pointed to by @p timestamp.
  The rsmi_dev_power_ave_get() is an average of a short time. This function
  accumulates all energy consumed.

  @param[in] dv_ind a device index
  @param[inout] counter_resolution resolution of the counter @p power in
  micro Joules

  @param[inout] power a pointer to uint64_t to which the energy
  counter will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[inout] timestamp a pointer to uint64_t to which the timestamp
  will be written. Resolution: 1 ns.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_energy_count_get(
        dv_ind: u32,
        power: *mut u64,
        counter_resolution: *mut f32,
        timestamp: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the cap on power which, when reached, causes the system to take
  action to reduce power.

  @details When power use rises above the value @p power, the system will
  take action to reduce power use. The power level returned through
  @p power will be in microWatts.

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[inout] cap a pointer to a uint64_t that indicates the power cap,
  in microwatts
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_power_cap_get(
        dv_ind: u32,
        sensor_ind: u32,
        cap: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the default power cap for the device specified by @p dv_ind.

  @details The maximum power cap be temporarily changed by the user. However,
  this function always returns the default reset power cap. The power level
  returned through @p power will be in microWatts.

  @param[in] dv_ind a device index

  @param[inout] default_cap a pointer to a uint64_t that indicates the default
  power cap, in microwatts
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_power_cap_default_get(
        dv_ind: u32,
        default_cap: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the range of valid values for the power cap

  @details This function will return the maximum possible valid power cap
  @p max and the minimum possible valid power cap @p min

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[inout] max a pointer to a uint64_t that indicates the maximum
  possible power cap, in microwatts
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[inout] min a pointer to a uint64_t that indicates the minimum
  possible power cap, in microwatts
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_power_cap_range_get(
        dv_ind: u32,
        sensor_ind: u32,
        max: *mut u64,
        min: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup PowerCont Power Control\n  These functions provide ways to control power usage.\n  @{\n/\n/**\n  @brief Set the power cap value\n\n  @details This function will set the power cap to the provided value @p cap.\n  @p cap must be between the minimum and maximum power cap values set by the\n  system, which can be obtained from ::rsmi_dev_power_cap_range_get.\n\n  @param[in] dv_ind a device index\n\n  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.\n  If a device has more than one sensor, it could be greater than 0.\n\n  @param[in] cap a uint64_t that indicates the desired power cap, in\n  microwatts\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.\n  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid\n  @retval ::RSMI_STATUS_PERMISSION function requires root access\n"]
    pub fn rsmi_dev_power_cap_set(
        dv_ind: u32,
        sensor_ind: u32,
        cap: u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Set the power profile

  @details Given a device index @p dv_ind and a @p profile, this function will
  attempt to set the current profile to the provided profile. The provided
  profile must be one of the currently supported profiles, as indicated by a
  call to ::rsmi_dev_power_profile_presets_get()

  @param[in] dv_ind a device index

  @param[in] reserved Not currently used. Set to 0.

  @param[in] profile a ::rsmi_power_profile_preset_masks_t that hold the mask
  of the desired new power profile

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_power_profile_set(
        dv_ind: u32,
        reserved: u32,
        profile: rsmi_power_profile_preset_masks_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the total amount of memory that exists

  @details Given a device index @p dv_ind, a type of memory @p mem_type, and
  a pointer to a uint64_t @p total, this function will write the total amount
  of @p mem_type memory that exists to the location pointed to by @p total.

  @param[in] dv_ind a device index

  @param[in] mem_type The type of memory for which the total amount will be
  found

  @param[inout] total a pointer to uint64_t to which the total amount of
  memory will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_memory_total_get(
        dv_ind: u32,
        mem_type: rsmi_memory_type_t,
        total: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the current memory usage

  @details Given a device index @p dv_ind, a type of memory @p mem_type, and
  a pointer to a uint64_t @p usage, this function will write the amount of
  @p mem_type memory that that is currently being used to the location
  pointed to by @p used.

  @param[in] dv_ind a device index

  @param[in] mem_type The type of memory for which the amount being used will
  be found

  @param[inout] used a pointer to uint64_t to which the amount of memory
  currently being used will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_memory_usage_get(
        dv_ind: u32,
        mem_type: rsmi_memory_type_t,
        used: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get percentage of time any device memory is being used

  @details Given a device index @p dv_ind, this function returns the
  percentage of time that any device memory is being used for the specified
  device.

  @param[in] dv_ind a device index

  @param[inout] busy_percent a pointer to the uint32_t to which the busy
  percent will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_memory_busy_percent_get(
        dv_ind: u32,
        busy_percent: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get information about reserved ("retired") memory pages

  @details Given a device index @p dv_ind, this function returns retired page
  information @p records corresponding to the device with the provided device
  index @p dv_ind. The number of retired page records is returned through @p
  num_pages. @p records may be NULL on input. In this case, the number of
  records available for retrieval will be returned through @p num_pages.

  @param[in] dv_ind a device index

  @param[inout] num_pages a pointer to a uint32. As input, the value passed
  through this parameter is the number of ::rsmi_retired_page_record_t's that
  may be safely written to the memory pointed to by @p records. This is the
  limit on how many records will be written to @p records. On return, @p
  num_pages will contain the number of records written to @p records, or the
  number of records that could have been written if enough memory had been
  provided.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[inout] records A pointer to a block of memory to which the
  ::rsmi_retired_page_record_t values will be written. This value may be NULL.
  In this case, this function can be used to query how many records are
  available to read.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if more records were available
  than allowed by the provided, allocated memory.*/
    pub fn rsmi_dev_memory_reserved_pages_get(
        dv_ind: u32,
        num_pages: *mut u32,
        records: *mut rsmi_retired_page_record_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = " @defgroup PhysQuer Physical State Queries\n  These functions provide information about the physical characteristics of\n  the device.\n  @{\n/\n/**\n  @brief Get the fan speed in RPMs of the device with the specified device\n  index and 0-based sensor index.\n\n  @details Given a device index @p dv_ind and a pointer to a uint32_t\n  @p speed, this function will write the current fan speed in RPMs to the\n  uint32_t pointed to by @p speed\n\n  @param[in] dv_ind a device index\n\n  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.\n  If a device has more than one sensor, it could be greater than 0.\n\n  @param[inout] speed a pointer to uint32_t to which the speed will be\n  written\n  If this parameter is nullptr, this function will return\n  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,\n  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the\n  provided arguments.\n\n  @retval ::RSMI_STATUS_SUCCESS call was successful\n  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not\n  support this function with the given arguments\n  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid\n"]
    pub fn rsmi_dev_fan_rpms_get(
        dv_ind: u32,
        sensor_ind: u32,
        speed: *mut i64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the fan speed for the specified device as a value relative to
  ::RSMI_MAX_FAN_SPEED

  @details Given a device index @p dv_ind and a pointer to a uint32_t
  @p speed, this function will write the current fan speed (a value
  between 0 and the maximum fan speed, ::RSMI_MAX_FAN_SPEED) to the uint32_t
  pointed to by @p speed

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[inout] speed a pointer to uint32_t to which the speed will be
  written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_fan_speed_get(
        dv_ind: u32,
        sensor_ind: u32,
        speed: *mut i64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the max. fan speed of the device with provided device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t
  @p max_speed, this function will write the maximum fan speed possible to
  the uint32_t pointed to by @p max_speed

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[inout] max_speed a pointer to uint32_t to which the maximum speed
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_fan_speed_max_get(
        dv_ind: u32,
        sensor_ind: u32,
        max_speed: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the temperature metric value for the specified metric, from the
  specified temperature sensor on the specified device.

  @details Given a device index @p dv_ind, a sensor type @p sensor_type, a
  ::rsmi_temperature_metric_t @p metric and a pointer to an int64_t @p
  temperature, this function will write the value of the metric indicated by
  @p metric and @p sensor_type to the memory location @p temperature.

  @param[in] dv_ind a device index

  @param[in] sensor_type part of device from which temperature should be
  obtained. This should come from the enum ::rsmi_temperature_type_t

  @param[in] metric enum indicated which temperature value should be
  retrieved

  @param[inout] temperature a pointer to int64_t to which the temperature
  will be written, in millidegrees Celcius.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_temp_metric_get(
        dv_ind: u32,
        sensor_type: u32,
        metric: rsmi_temperature_metric_t,
        temperature: *mut i64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the voltage metric value for the specified metric, from the
  specified voltage sensor on the specified device.

  @details Given a device index @p dv_ind, a sensor type @p sensor_type, a
  ::rsmi_voltage_metric_t @p metric and a pointer to an int64_t @p
  voltage, this function will write the value of the metric indicated by
  @p metric and @p sensor_type to the memory location @p voltage.

  @param[in] dv_ind a device index

  @param[in] sensor_type part of device from which voltage should be
  obtained. This should come from the enum ::rsmi_voltage_type_t

  @param[in] metric enum indicated which voltage value should be
  retrieved

  @param[inout] voltage a pointer to int64_t to which the voltage
  will be written, in millivolts.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_volt_metric_get(
        dv_ind: u32,
        sensor_type: rsmi_voltage_type_t,
        metric: rsmi_voltage_metric_t,
        voltage: *mut i64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = "/\n/** @defgroup PhysCont Physical State Control\n  These functions provide control over the physical state of a device.\n  @{\n/\n/**\n  @brief Reset the fan to automatic driver control\n\n  @details This function returns control of the fan to the system\n\n  @param[in] dv_ind a device index\n\n  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.\n  If a device has more than one sensor, it could be greater than 0.\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.\n  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not\n  support this function with the given arguments\n"]
    pub fn rsmi_dev_fan_reset(dv_ind: u32, sensor_ind: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Set the fan speed for the specified device with the provided speed,
  in RPMs.

  @details Given a device index @p dv_ind and a integer value indicating
  speed @p speed, this function will attempt to set the fan speed to @p speed.
  An error will be returned if the specified speed is outside the allowable
  range for the device. The maximum value is 255 and the minimum is 0.

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[in] speed the speed to which the function will attempt to set the fan

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_fan_speed_set(
        dv_ind: u32,
        sensor_ind: u32,
        speed: u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get percentage of time device is busy doing any processing

  @details Given a device index @p dv_ind, this function returns the
  percentage of time that the specified device is busy. The device is
  considered busy if any one or more of its sub-blocks are working, and idle
  if none of the sub-blocks are working.

  @param[in] dv_ind a device index

  @param[inout] busy_percent a pointer to the uint32_t to which the busy
  percent will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_busy_percent_get(
        dv_ind: u32,
        busy_percent: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get coarse grain utilization counter of the specified device

  @details Given a device index @p dv_ind, the array of the utilization counters,
  the size of the array, this function returns the coarse grain utilization counters
  and timestamp.
  The counter is the accumulated percentages. Every milliseconds the firmware calculates
  % busy count and then accumulates that value in the counter. This provides minimally
  invasive coarse grain GPU usage information.

  @param[in] dv_ind a device index

  @param[inout] utilization_counters Multiple utilization counters can be retreived with a single
  call. The caller must allocate enough space to the utilization_counters array. The caller also
  needs to set valid RSMI_UTILIZATION_COUNTER_TYPE type for each element of the array.
  ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the provided arguments.

  If the function reutrns RSMI_STATUS_SUCCESS, the counter will be set in the value field of
  the rsmi_utilization_counter_t.

  @param[in] count The size of utilization_counters array.

  @param[inout] timestamp The timestamp when the counter is retreived. Resolution: 1 ns.
  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_utilization_count_get(
        dv_ind: u32,
        utilization_counters: *mut rsmi_utilization_counter_t,
        count: u32,
        timestamp: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get activity metric average utilization counter of the specified device

  @details Given a device index @p dv_ind, the activity metric type,
  this function returns the requested utilization counters

  @param[in] dv_ind a device index

  @param[in] activity_metric_type a metric type

  @param[inout] activity_metric_counter Multiple utilization counters can be retrieved with a single
  call. The caller must allocate enough space to the rsmi_activity_metric_counter_t structure.

  If the function returns RSMI_STATUS_SUCCESS, the requested type will be set in the corresponding
  field of the counter will be set in the value field of
  the activity_metric_counter_t.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_activity_metric_get(
        dv_ind: u32,
        activity_metric_type: rsmi_activity_metric_t,
        activity_metric_counter: *mut rsmi_activity_metric_counter_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get activity metric bandwidth average utilization counter of the specified device

  @details Given a device index @p dv_ind, the activity metric type,
  this function returns the requested utilization counters

  @param[in] dv_ind a device index

  @param[inout] avg_activity average bandwidth utilization counters can be retrieved

  If the function returns RSMI_STATUS_SUCCESS, the requested type will be set in the corresponding
  field of the counter will be set in the value field of
  the activity_metric_counter_t.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_activity_avg_mm_get(
        dv_ind: u32,
        avg_activity: *mut u16,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the performance level of the device with provided
  device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p
  perf, this function will write the ::rsmi_dev_perf_level_t to the uint32_t
  pointed to by @p perf

  @param[in] dv_ind a device index

  @param[inout] perf a pointer to ::rsmi_dev_perf_level_t to which the
  performance level will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_perf_level_get(
        dv_ind: u32,
        perf: *mut rsmi_dev_perf_level_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Enter performance determinism mode with provided device index.

  @details Given a device index @p dv_ind and @p clkvalue this function
  will enable performance determinism mode, which enforces a GFXCLK frequency
  SoftMax limit per GPU set by the user. This prevents the GFXCLK PLL from
  stretching when running the same workload on different GPUS, making
  performance variation minimal. This call will result in the performance
  level ::rsmi_dev_perf_level_t of the device being
  ::RSMI_DEV_PERF_LEVEL_DETERMINISM.

  @param[in] dv_ind a device index

  @param[in] clkvalue Softmax value for GFXCLK in MHz.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_perf_determinism_mode_set(dv_ind: u32, clkvalue: u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the overdrive percent associated with the device with provided
  device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p od,
  this function will write the overdrive percentage to the uint32_t pointed
  to by @p od

  @param[in] dv_ind a device index

  @param[inout] od a pointer to uint32_t to which the overdrive percentage
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_overdrive_level_get(dv_ind: u32, od: *mut u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the memory clock overdrive percent associated with the device
  with provided device index.

  @details Given a device index @p dv_ind and a pointer to a uint32_t @p od,
  this function will write the memory overdrive percentage to the uint32_t
  pointed to by @p od

  @param[in] dv_ind a device index

  @param[inout] od a pointer to uint32_t to which the overdrive percentage
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_mem_overdrive_level_get(dv_ind: u32, od: *mut u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the list of possible system clock speeds of device for a
  specified clock type.

  @details Given a device index @p dv_ind, a clock type @p clk_type, and a
  pointer to a to an ::rsmi_frequencies_t structure @p f, this function will
  fill in @p f with the possible clock speeds, and indication of the current
  clock speed selection.

  @param[in] dv_ind a device index

  @param[in] clk_type the type of clock for which the frequency is desired

  @param[inout] f a pointer to a caller provided ::rsmi_frequencies_t structure
  to which the frequency information will be written. Frequency values are in
  Hz.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.
  If multiple current frequencies are found, a warning is shown. If no
  current frequency is found, it is reflected as -1. If frequencies are not
  read from low to high a warning is shown as well.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_UNEXPECTED_DATA Data read or provided was not as
  expected
*/
    pub fn rsmi_dev_gpu_clk_freq_get(
        dv_ind: u32,
        clk_type: rsmi_clk_type_t,
        f: *mut rsmi_frequencies_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Reset the gpu associated with the device with provided device index

  @details Given a device index @p dv_ind, this function will reset the GPU

  @param[in] dv_ind a device index

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_gpu_reset(dv_ind: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function retrieves the voltage/frequency curve information

  @details Given a device index @p dv_ind and a pointer to a
  ::rsmi_od_volt_freq_data_t structure @p odv, this function will populate @p
  odv. See ::rsmi_od_volt_freq_data_t for more details.

  @param[in] dv_ind a device index

  @param[inout] odv a pointer to an ::rsmi_od_volt_freq_data_t structure
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_od_volt_info_get(
        dv_ind: u32,
        odv: *mut rsmi_od_volt_freq_data_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function retrieves the gpu metrics information

  @details Given a device index @p dv_ind and a pointer to a
  ::rsmi_gpu_metrics_t structure @p pgpu_metrics, this function will populate
  @p pgpu_metrics. See ::rsmi_gpu_metrics_t for more details.

  @param[in] dv_ind a device index

  @param[inout] pgpu_metrics a pointer to an ::rsmi_gpu_metrics_t structure
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_gpu_metrics_info_get(
        dv_ind: u32,
        pgpu_metrics: *mut rsmi_gpu_metrics_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function sets the clock range information

  @details Given a device index @p dv_ind, a minimum clock value @p minclkvalue,
  a maximum clock value @p maxclkvalue and a clock type @p clkType this function
  will set the sclk|mclk range

  @param[in] dv_ind a device index

  @param[in] minclkvalue value to apply to the clock range. Frequency values
  are in MHz.

  @param[in] maxclkvalue value to apply to the clock range. Frequency values
  are in MHz.

  @param[in] clkType RSMI_CLK_TYPE_SYS | RSMI_CLK_TYPE_MEM range type

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_clk_range_set(
        dv_ind: u32,
        minclkvalue: u64,
        maxclkvalue: u64,
        clkType: rsmi_clk_type_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function sets the clock min/max level

  @details Given a device index @p dv_ind, a clock value @p minclkvalue,
  a maximum clock value @p maxclkvalue and a clock type @p clkType this function
  will set the sclk|mclk range

  @param[in] dv_ind a device index

  @param[in] level RSMI_FREQ_IND_MIN|RSMI_FREQ_IND_MAX

  @param[in] clkvalue value to apply to the clock level. Frequency values
  are in MHz.

  @param[in] clkType RSMI_CLK_TYPE_SYS | RSMI_CLK_TYPE_MEM level type

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_clk_extremum_set(
        dv_ind: u32,
        level: rsmi_freq_ind_t,
        clkvalue: u64,
        clkType: rsmi_clk_type_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function sets the clock frequency information

  @details Given a device index @p dv_ind, a frequency level @p level,
  a clock value @p clkvalue and a clock type @p clkType this function
  will set the sclk|mclk range

  @param[in] dv_ind a device index

  @param[in] level RSMI_FREQ_IND_MIN|RSMI_FREQ_IND_MAX to set the
  minimum (0) or maximum (1) speed.

  @param[in] clkvalue value to apply to the clock range. Frequency values
  are in MHz.

  @param[in] clkType RSMI_CLK_TYPE_SYS | RSMI_CLK_TYPE_MEM range type

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_od_clk_info_set(
        dv_ind: u32,
        level: rsmi_freq_ind_t,
        clkvalue: u64,
        clkType: rsmi_clk_type_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function sets  1 of the 3 voltage curve points.

  @details Given a device index @p dv_ind, a voltage point @p vpoint
  and a voltage value @p voltvalue this function will set voltage curve point

  @param[in] dv_ind a device index

  @param[in] vpoint voltage point [0|1|2] on the voltage curve

  @param[in] clkvalue clock value component of voltage curve point.
  Frequency values are in MHz.

  @param[in] voltvalue voltage value component of voltage curve point.
  Voltage is in mV.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_od_volt_info_set(
        dv_ind: u32,
        vpoint: u32,
        clkvalue: u64,
        voltvalue: u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief This function will retrieve the current valid regions in the
  frequency/voltage space.

  @details Given a device index @p dv_ind, a pointer to an unsigned integer
  @p num_regions and a buffer of ::rsmi_freq_volt_region_t structures, @p
  buffer, this function will populate @p buffer with the current
  frequency-volt space regions. The caller should assign @p buffer to memory
  that can be written to by this function. The caller should also
  indicate the number of ::rsmi_freq_volt_region_t structures that can safely
  be written to @p buffer in @p num_regions.

  The number of regions to expect this function provide (@p num_regions) can
  be obtained by calling ::rsmi_dev_od_volt_info_get().

  @param[in] dv_ind a device index

  @param[inout] num_regions As input, this is the number of
  ::rsmi_freq_volt_region_t structures that can be written to @p buffer. As
  output, this is the number of ::rsmi_freq_volt_region_t structures that were
  actually written.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[inout] buffer a caller provided buffer to which
  ::rsmi_freq_volt_region_t structures will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_od_volt_curve_regions_get(
        dv_ind: u32,
        num_regions: *mut u32,
        buffer: *mut rsmi_freq_volt_region_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the list of available preset power profiles and an indication of
  which profile is currently active.

  @details Given a device index @p dv_ind and a pointer to a
  ::rsmi_power_profile_status_t @p status, this function will set the bits of
  the ::rsmi_power_profile_status_t.available_profiles bit field of @p status to
  1 if the profile corresponding to the respective
  ::rsmi_power_profile_preset_masks_t profiles are enabled. For example, if both
  the VIDEO and VR power profiles are available selections, then
  ::RSMI_PWR_PROF_PRST_VIDEO_MASK AND'ed with
  ::rsmi_power_profile_status_t.available_profiles will be non-zero as will
  ::RSMI_PWR_PROF_PRST_VR_MASK AND'ed with
  ::rsmi_power_profile_status_t.available_profiles. Additionally,
  ::rsmi_power_profile_status_t.current will be set to the
  ::rsmi_power_profile_preset_masks_t of the profile that is currently active.

  @param[in] dv_ind a device index

  @param[in] sensor_ind a 0-based sensor index. Normally, this will be 0.
  If a device has more than one sensor, it could be greater than 0.

  @param[inout] status a pointer to ::rsmi_power_profile_status_t that will be
  populated by a call to this function
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_power_profile_presets_get(
        dv_ind: u32,
        sensor_ind: u32,
        status: *mut rsmi_power_profile_status_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    #[doc = " @defgroup PerfCont Clock, Power and Performance Control\n  These functions provide control over clock frequencies, power and\n  performance.\n  @{\n/\n/**\n  @brief Set the PowerPlay performance level associated with the device with\n  provided device index with the provided value.\n\n  @deprecated ::rsmi_dev_perf_level_set_v1() is preferred, with an\n  interface that more closely  matches the rest of the rocm_smi API.\n\n  @details Given a device index @p dv_ind and an ::rsmi_dev_perf_level_t @p\n  perf_level, this function will set the PowerPlay performance level for the\n  device to the value @p perf_lvl.\n\n  @param[in] dv_ind a device index\n\n  @param[in] perf_lvl the value to which the performance level should be set\n\n  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.\n  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not\n  support this function with the given arguments\n  @retval ::RSMI_STATUS_PERMISSION function requires root access\n"]
    pub fn rsmi_dev_perf_level_set(
        dv_ind: u32,
        perf_lvl: rsmi_dev_perf_level_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Set the PowerPlay performance level associated with the device with
  provided device index with the provided value.

  @details Given a device index @p dv_ind and an ::rsmi_dev_perf_level_t @p
  perf_level, this function will set the PowerPlay performance level for the
  device to the value @p perf_lvl.

  @param[in] dv_ind a device index

  @param[in] perf_lvl the value to which the performance level should be set

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_perf_level_set_v1(
        dv_ind: u32,
        perf_lvl: rsmi_dev_perf_level_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Set the overdrive percent associated with the device with provided
  device index with the provided value. See details for WARNING.

  @deprecated This function is deprecated. ::rsmi_dev_overdrive_level_set_v1
  has the same functionaltiy, with an interface that more closely
  matches the rest of the rocm_smi API.

  @details Given a device index @p dv_ind and an overdrive level @p od,
  this function will set the overdrive level for the device to the value
  @p od. The overdrive level is an integer value between 0 and 20, inclusive,
  which represents the overdrive percentage; e.g., a value of 5 specifies
  an overclocking of 5%.

  The overdrive level is specific to the gpu system clock.

  The overdrive level is the percentage above the maximum Performance Level
  to which overclocking will be limited. The overclocking percentage does
  not apply to clock speeds other than the maximum. This percentage is
  limited to 20%.

   ******WARNING******
  Operating your AMD GPU outside of official AMD specifications or outside of
  factory settings, including but not limited to the conducting of
  overclocking (including use of this overclocking software, even if such
  software has been directly or indirectly provided by AMD or otherwise
  affiliated in any way with AMD), may cause damage to your AMD GPU, system
  components and/or result in system failure, as well as cause other problems.
  DAMAGES CAUSED BY USE OF YOUR AMD GPU OUTSIDE OF OFFICIAL AMD SPECIFICATIONS
  OR OUTSIDE OF FACTORY SETTINGS ARE NOT COVERED UNDER ANY AMD PRODUCT
  WARRANTY AND MAY NOT BE COVERED BY YOUR BOARD OR SYSTEM MANUFACTURER'S
  WARRANTY. Please use this utility with caution.

  @param[in] dv_ind a device index

  @param[in] od the value to which the overdrive level should be set

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_overdrive_level_set(dv_ind: u32, od: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Set the overdrive percent associated with the device with provided
  device index with the provided value. See details for WARNING.

  @details Given a device index @p dv_ind and an overdrive level @p od,
  this function will set the overdrive level for the device to the value
  @p od. The overdrive level is an integer value between 0 and 20, inclusive,
  which represents the overdrive percentage; e.g., a value of 5 specifies
  an overclocking of 5%.

  The overdrive level is specific to the gpu system clock.

  The overdrive level is the percentage above the maximum Performance Level
  to which overclocking will be limited. The overclocking percentage does
  not apply to clock speeds other than the maximum. This percentage is
  limited to 20%.

   ******WARNING******
  Operating your AMD GPU outside of official AMD specifications or outside of
  factory settings, including but not limited to the conducting of
  overclocking (including use of this overclocking software, even if such
  software has been directly or indirectly provided by AMD or otherwise
  affiliated in any way with AMD), may cause damage to your AMD GPU, system
  components and/or result in system failure, as well as cause other problems.
  DAMAGES CAUSED BY USE OF YOUR AMD GPU OUTSIDE OF OFFICIAL AMD SPECIFICATIONS
  OR OUTSIDE OF FACTORY SETTINGS ARE NOT COVERED UNDER ANY AMD PRODUCT
  WARRANTY AND MAY NOT BE COVERED BY YOUR BOARD OR SYSTEM MANUFACTURER'S
  WARRANTY. Please use this utility with caution.

  @param[in] dv_ind a device index

  @param[in] od the value to which the overdrive level should be set

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_overdrive_level_set_v1(dv_ind: u32, od: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Control the set of allowed frequencies that can be used for the
 specified clock.

 @details Given a device index @p dv_ind, a clock type @p clk_type, and a
 64 bit bitmask @p freq_bitmask, this function will limit the set of
 allowable frequencies. If a bit in @p freq_bitmask has a value of 1, then
 the frequency (as ordered in an ::rsmi_frequencies_t returned by
 rsmi_dev_gpu_clk_freq_get()) corresponding to that bit index will be
 allowed.

 This function will change the performance level to
 ::RSMI_DEV_PERF_LEVEL_MANUAL in order to modify the set of allowable
 frequencies. Caller will need to set to ::RSMI_DEV_PERF_LEVEL_AUTO in order
 to get back to default state.

 All bits with indices greater than or equal to
 ::rsmi_frequencies_t::num_supported will be ignored.

  @param[in] dv_ind a device index

  @param[in] clk_type the type of clock for which the set of frequencies
  will be modified

  @param[in] freq_bitmask A bitmask indicating the indices of the
  frequencies that are to be enabled (1) and disabled (0). Only the lowest
  ::rsmi_frequencies_t.num_supported bits of this mask are relevant.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_gpu_clk_freq_set(
        dv_ind: u32,
        clk_type: rsmi_clk_type_t,
        freq_bitmask: u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Get the build version information for the currently running build of
 RSMI.

 @details  Get the major, minor, patch and build string for RSMI build
 currently in use through @p version

 @param[inout] version A pointer to an ::rsmi_version_t structure that will
 be updated with the version information upon return.

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
*/
    pub fn rsmi_version_get(version: *mut rsmi_version_t) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the driver version string for the current system.

  @details Given a software component @p component, a pointer to a char
  buffer, @p ver_str, this function will write the driver version string
  (up to @p len characters) for the current system to @p ver_str. The caller
  must ensure that it is safe to write at least @p len characters to @p
  ver_str.

  @param[in] component The component for which the version string is being
  requested

  @param[inout] ver_str A pointer to a buffer of char's to which the version
  of @p component will be written

  @param[in] len the length of the caller provided buffer @p name.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire name. In this case, only @p len bytes will
  be written.*/
    pub fn rsmi_version_str_get(
        component: rsmi_sw_component_t,
        ver_str: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the VBIOS identifer string

  @details Given a device ID @p dv_ind, and a pointer to a char buffer,
  @p vbios, this function will write the VBIOS string (up to @p len
  characters) for device @p dv_ind to @p vbios. The caller must ensure that
  it is safe to write at least @p len characters to @p vbios.

  @param[in] dv_ind a device index

  @param[inout] vbios A pointer to a buffer of char's to which the VBIOS name
  will be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @param[in] len The number of char's pointed to by @p vbios which can safely
  be written to by this function.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_vbios_version_get(
        dv_ind: u32,
        vbios: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the firmware versions for a device

  @details Given a device ID @p dv_ind, and a pointer to a uint64_t,
  @p fw_version, this function will write the FW Versions as a string (up to @p len
  characters) for device @p dv_ind to @p vbios. The caller must ensure that
  it is safe to write at least @p len characters to @p vbios.

  @param[in] dv_ind a device index

  @param[in] block The firmware block for which the version is being requested

  @param[inout] fw_version The version for the firmware block
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_firmware_version_get(
        dv_ind: u32,
        block: rsmi_fw_block_t,
        fw_version: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the target graphics version for a GPU device

  @details Given a device ID @p dv_ind and a uint64_t pointer
  @p gfx_version, this function will write the graphics version.

  @param[in] dv_ind a device index

  @param[inout] gfx_version The device graphics version number indicated by
  KFD. If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS. If device does not support this value,
  will return ::RSMI_STATUS_NOT_SUPPORTED and a maximum UINT64 value as
  @p gfx_version.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_target_graphics_version_get(
        dv_ind: u32,
        gfx_version: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the error counts for a GPU block

  @details Given a device index @p dv_ind, an ::rsmi_gpu_block_t @p block and a
  pointer to an ::rsmi_error_count_t @p ec, this function will write the error
  count values for the GPU block indicated by @p block to memory pointed to by
  @p ec.

  @param[in] dv_ind a device index

  @param[in] block The block for which error counts should be retrieved

  @param[inout] ec A pointer to an ::rsmi_error_count_t to which the error
  counts should be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_ecc_count_get(
        dv_ind: u32,
        block: rsmi_gpu_block_t,
        ec: *mut rsmi_error_count_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the enabled ECC bit-mask

  @details Given a device index @p dv_ind, and a pointer to a uint64_t @p
  enabled_mask, this function will write bits to memory pointed to by
  @p enabled_blocks. Upon a successful call, @p enabled_blocks can then be
  AND'd with elements of the ::rsmi_gpu_block_t ennumeration to determine if
  the corresponding block has ECC enabled. Note that whether a block has ECC
  enabled or not in the device is independent of whether there is kernel
  support for error counting for that block. Although a block may be enabled,
  but there may not be kernel support for reading error counters for that
  block.

  @param[in] dv_ind a device index

  @param[inout] enabled_blocks A pointer to a uint64_t to which the enabled
  blocks bits will be written.
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_dev_ecc_enabled_get(
        dv_ind: u32,
        enabled_blocks: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the ECC status for a GPU block

  @details Given a device index @p dv_ind, an ::rsmi_gpu_block_t @p block and
  a pointer to an ::rsmi_ras_err_state_t @p state, this function will write
  the current state for the GPU block indicated by @p block to memory pointed
  to by @p state.

  @param[in] dv_ind a device index

  @param[in] block The block for which error counts should be retrieved

  @param[inout] state A pointer to an ::rsmi_ras_err_state_t to which the
  ECC state should be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_ecc_status_get(
        dv_ind: u32,
        block: rsmi_gpu_block_t,
        state: *mut rsmi_ras_err_state_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get a description of a provided RSMI error status

  @details Set the provided pointer to a const char *, @p status_string, to
  a string containing a description of the provided error code @p status.

  @param[in] status The error status for which a description is desired

  @param[inout] status_string A pointer to a const char * which will be made
  to point to a description of the provided error code

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
*/
    pub fn rsmi_status_string(
        status: rsmi_status_t,
        status_string: *mut *const ::core::ffi::c_char,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Tell if an event group is supported by a given device

  @details Given a device index @p dv_ind and an event group specifier @p
  group, tell if @p group type events are supported by the device associated
  with @p dv_ind

  @param[in] dv_ind device index of device being queried

  @param[in] group ::rsmi_event_group_t identifier of group for which support
  is being queried

  @retval ::RSMI_STATUS_SUCCESS if the device associatee with @p dv_ind
  support counting events of the type indicated by @p group.
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  group
*/
    pub fn rsmi_dev_counter_group_supported(
        dv_ind: u32,
        group: rsmi_event_group_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Create a performance counter object

  @details Create a performance counter object of type @p type for the device
  with a device index of @p dv_ind, and write a handle to the object to the
  memory location pointed to by @p evnt_handle. @p evnt_handle can be used
  with other performance event operations. The handle should be deallocated
  with ::rsmi_dev_counter_destroy() when no longer needed.

  @param[in] dv_ind a device index

  @param[in] type the ::rsmi_event_type_t of performance event to create

  @param[inout] evnt_handle A pointer to a ::rsmi_event_handle_t which will be
  associated with a newly allocated counter
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_OUT_OF_RESOURCES unable to allocate memory for counter
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_counter_create(
        dv_ind: u32,
        type_: rsmi_event_type_t,
        evnt_handle: *mut rsmi_event_handle_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Deallocate a performance counter object

  @details Deallocate the performance counter object with the provided
  ::rsmi_event_handle_t @p evnt_handle

  @param[in] evnt_handle handle to event object to be deallocated

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_dev_counter_destroy(evnt_handle: rsmi_event_handle_t) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Issue performance counter control commands

  @details Issue a command @p cmd on the event counter associated with the
  provided handle @p evt_handle.

  @param[in] evt_handle an event handle

  @param[in] cmd The event counter command to be issued

  @param[inout] cmd_args Currently not used. Should be set to NULL.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_counter_control(
        evt_handle: rsmi_event_handle_t,
        cmd: rsmi_counter_command_t,
        cmd_args: *mut ::core::ffi::c_void,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Read the current value of a performance counter

  @details Read the current counter value of the counter associated with the
  provided handle @p evt_handle and write the value to the location pointed
  to by @p value.

  @param[in] evt_handle an event handle

  @param[inout] value pointer to memory of size of ::rsmi_counter_value_t to
  which the counter value will be written

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_PERMISSION function requires root access
*/
    pub fn rsmi_counter_read(
        evt_handle: rsmi_event_handle_t,
        value: *mut rsmi_counter_value_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the number of currently available counters

  @details Given a device index @p dv_ind, a performance event group @p grp,
  and a pointer to a uint32_t @p available, this function will write the
  number of @p grp type counters that are available on the device with index
  @p dv_ind to the memory that @p available points to.

  @param[in] dv_ind a device index

  @param[in] grp an event device group

  @param[inout] available A pointer to a uint32_t to which the number of
  available counters will be written

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_counter_available_counters_get(
        dv_ind: u32,
        grp: rsmi_event_group_t,
        available: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get process information about processes currently using GPU

  @details Given a non-NULL pointer to an array @p procs of
  ::rsmi_process_info_t's, of length *@p num_items, this function will write
  up to *@p num_items instances of ::rsmi_process_info_t to the memory pointed
  to by @p procs. These instances contain information about each GPU compute
  process and their PASID for further analysis or monitoring via
  ::rsmi_compute_process_info_by_pid_get().
  If @p procs is not NULL, @p num_items will be updated with
  the number of processes actually written. If @p procs is NULL, @p num_items
  will be updated with the number of processes for which there is current
  process information. Calling this function with @p procs being NULL is a way
  to determine how much memory should be allocated for when @p procs is not
  NULL.

  @param[inout] procs a pointer to memory provided by the caller to which
  process information will be written. This may be NULL in which case only @p
  num_items will be updated with the number of processes found.

  @param[inout] num_items A pointer to a uint32_t, which on input, should
  contain the amount of memory in ::rsmi_process_info_t's which have been
  provided by the @p procs argument. On output, if @p procs is non-NULL, this
  will be updated with the number ::rsmi_process_info_t structs actually
  written. If @p procs is NULL, this argument will be updated with the number
  processes for which there is information.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if there were more
  processes for which information was available, but not enough space was
  provided as indicated by @p procs and @p num_items, on input.*/
    pub fn rsmi_compute_process_info_get(
        procs: *mut rsmi_process_info_t,
        num_items: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get process information about a specific process

  @details Given a pointer to an ::rsmi_process_info_t @p proc and a process
  id
  @p pid, this function will write the process information for @p pid, if
  available, to the memory pointed to by @p proc.

  @param[in] pid The process ID for which process information is being
  requested

  @param[inout] proc a pointer to a ::rsmi_process_info_t to which
  process information for @p pid will be written if it is found.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_NOT_FOUND is returned if there was no process
  information
  found for the provided @p pid
*/
    pub fn rsmi_compute_process_info_by_pid_get(
        pid: u32,
        proc_: *mut rsmi_process_info_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the device indices currently being used by a process

  @details Given a process id @p pid, a non-NULL pointer to an array of
  uint32_t's @p dv_indices of length *@p num_devices, this function will
  write up to @p num_devices device indices to the memory pointed to by
  @p dv_indices. If @p dv_indices is not NULL, @p num_devices will be
  updated with the number of gpu's currently being used by process @p pid.
  If @p dv_indices is NULL, @p dv_indices will be updated with the number of
  gpus currently being used by @p pid. Calling this function with @p
  dv_indices being NULL is a way to determine how much memory is required
  for when @p dv_indices is not NULL.

  @param[in] pid The process id of the process for which the number of gpus
  currently being used is requested

  @param[inout] dv_indices a pointer to memory provided by the caller to
  which indices of devices currently being used by the process will be
  written. This may be NULL in which case only @p num_devices will be
  updated with the number of devices being used.

  @param[inout] num_devices A pointer to a uint32_t, which on input, should
  contain the amount of memory in uint32_t's which have been provided by the
  @p dv_indices argument. On output, if @p dv_indices is non-NULL, this will
  be updated with the number uint32_t's actually written. If @p dv_indices is
  NULL, this argument will be updated with the number devices being used.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if there were more
  gpu indices that could have been written, but not enough space was
  provided as indicated by @p dv_indices and @p num_devices, on input.
*/
    pub fn rsmi_compute_process_gpus_get(
        pid: u32,
        dv_indices: *mut u32,
        num_devices: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the info of a process on a specific device.

  @details Given a process id @p pid, a @p dv_ind, this function will
  write the process information for pid on the device, if available, to
  the memory pointed to by @p proc.

  @param[in] pid The process id of the process for which the gpu
  currently being used is requested.

  @param[in] dv_ind a device index where the process running on.

  @param[inout] proc a pointer to memory provided by the caller to which
  process information will be written.

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_compute_process_info_by_device_get(
        pid: u32,
        dv_ind: u32,
        proc_: *mut rsmi_process_info_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the XGMI error status for a device

  @details Given a device index @p dv_ind, and a pointer to an
  ::rsmi_xgmi_status_t @p status, this function will write the current XGMI
  error state ::rsmi_xgmi_status_t for the device @p dv_ind to the memory
  pointed to by @p status.

  @param[in] dv_ind a device index

  @param[inout] status A pointer to an ::rsmi_xgmi_status_t to which the
  XGMI error state should be written
  If this parameter is nullptr, this function will return
  ::RSMI_STATUS_INVALID_ARGS if the function is supported with the provided,
  arguments and ::RSMI_STATUS_NOT_SUPPORTED if it is not supported with the
  provided arguments.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_xgmi_error_status(
        dv_ind: u32,
        status: *mut rsmi_xgmi_status_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Reset the XGMI error status for a device

 @details Given a device index @p dv_ind, this function will reset the
 current XGMI error state ::rsmi_xgmi_status_t for the device @p dv_ind to
 rsmi_xgmi_status_t::RSMI_XGMI_STATUS_NO_ERRORS

 @param[in] dv_ind a device index

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_xgmi_error_reset(dv_ind: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the XGMI hive id for a device

  @details Given a device index @p dv_ind, and a pointer to an
  uint64_t @p hive_id, this function will write the current XGMI
  hive id for the device @p dv_ind to the memory pointed to by @p hive_id.

  @param[in] dv_ind a device index

  @param[inout] hive_id A pointer to an uint64_t to which the XGMI hive id
  should be written

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function with the given arguments
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_dev_xgmi_hive_id_get(dv_ind: u32, hive_id: *mut u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the NUMA CPU node number for a device

  @details Given a device index @p dv_ind, and a pointer to an
  uint32_t @p numa_node, this function will write the
  node number of NUMA CPU for the device @p dv_ind to the memory
  pointed to by @p numa_node.

  @param[in] dv_ind a device index

  @param[inout] numa_node A pointer to an uint32_t to which the
  numa node number should be written.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_topo_get_numa_node_number(
        dv_ind: u32,
        numa_node: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the weight for a connection between 2 GPUs

  @details Given a source device index @p dv_ind_src and
  a destination device index @p dv_ind_dst, and a pointer to an
  uint64_t @p weight, this function will write the
  weight for the connection between the device @p dv_ind_src
  and @p dv_ind_dst to the memory pointed to by @p weight.

  @param[in] dv_ind_src the source device index

  @param[in] dv_ind_dst the destination device index

  @param[inout] weight A pointer to an uint64_t to which the
  weight for the connection should be written.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_topo_get_link_weight(
        dv_ind_src: u32,
        dv_ind_dst: u32,
        weight: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retreive minimal and maximal io link bandwidth between 2 GPUs

  @details Given a source device index @p dv_ind_src and
  a destination device index @p dv_ind_dst,  pointer to an
  uint64_t @p min_bandwidth, and a pointer to uint64_t @p max_bandiwidth,
  this function will write theoretical minimal and maximal bandwidth limits.
  API works if src and dst are connected via xgmi and have 1 hop distance.

  @param[in] dv_ind_src the source device index

  @param[in] dv_ind_dst the destination device index

  @param[inout] min_bandwidth A pointer to an uint64_t to which the
  minimal bandwidth for the connection should be written.

  @param[inout] max_bandwidth A pointer to an uint64_t to which the
  maximal bandwidth for the connection should be written.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid*/
    pub fn rsmi_minmax_bandwidth_get(
        dv_ind_src: u32,
        dv_ind_dst: u32,
        min_bandwidth: *mut u64,
        max_bandwidth: *mut u64,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieve the hops and the connection type between GPU to GPU/CPU

  @details Given a source device index @p dv_ind_src and
  a destination device index @p dv_ind_dst, and a pointer to an
  uint64_t @p hops and a pointer to an RSMI_IO_LINK_TYPE @p type,
  this function will write the number of hops and the connection type
  between the device @p dv_ind_src and @p dv_ind_dst to the memory
  pointed to by @p hops and @p type.

  To query the link type between GPU and CPU, given a source GPU index
  @p dev_ind_srcc and a destination device index @p dv_ind_dst
  CPU_NODE_INDEX(0xFFFFFFFF), a pointer to an
  uint64_t @p hops and a pointer to an RSMI_IO_LINK_TYPE @p type,
  this function will write the number of hops and the connection type
  between the device @p dv_ind_src and CPU to the memory
  pointed to by @p hops and @p type.

  @param[in] dv_ind_src the source device index

  @param[in] dv_ind_dst the destination device index

  @param[inout] hops A pointer to an uint64_t to which the
  hops for the connection should be written.

  @param[inout] type A pointer to an ::RSMI_IO_LINK_TYPE to which the
  type for the connection should be written.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_topo_get_link_type(
        dv_ind_src: u32,
        dv_ind_dst: u32,
        hops: *mut u64,
        type_: *mut RSMI_IO_LINK_TYPE,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Return P2P availability status between 2 GPUs

  @details Given a source device index @p dv_ind_src and
  a destination device index @p dv_ind_dst, and a pointer to a
  bool @p accessible, this function will write the P2P connection status
  between the device @p dv_ind_src and @p dv_ind_dst to the memory
  pointed to by @p accessible.

  @param[in] dv_ind_src the source device index

  @param[in] dv_ind_dst the destination device index

  @param[inout] accessible A pointer to a bool to which the status for
  the P2P connection availablity should be written.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
*/
    pub fn rsmi_is_P2P_accessible(
        dv_ind_src: u32,
        dv_ind_dst: u32,
        accessible: *mut bool,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieves the current compute partitioning for a desired device

  @details
  Given a device index @p dv_ind and a string @p compute_partition ,
  and uint32 @p len , this function will attempt to obtain the device's
  current compute partition setting string. Upon successful retreival,
  the obtained device's compute partition settings string shall be stored in
  the passed @p compute_partition char string variable.

  @param[in] dv_ind a device index

  @param[inout] compute_partition a pointer to a char string variable,
  which the device's current compute partition will be written to.

  @param[in] len the length of the caller provided buffer @p compute_partition
  , suggested length is 4 or greater.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_UNEXPECTED_DATA data provided to function is not valid
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire compute partition value. In this case,
  only @p len bytes will be written.
*/
    pub fn rsmi_dev_compute_partition_get(
        dv_ind: u32,
        compute_partition: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Modifies a selected device's compute partition setting.

  @details Given a device index @p dv_ind, a type of compute partition
  @p compute_partition, this function will attempt to update the selected
  device's compute partition setting.

  @param[in] dv_ind a device index

  @param[in] compute_partition using enum ::rsmi_compute_partition_type_t,
  define what the selected device's compute partition setting should be
  updated to.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_PERMISSION function requires root access
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_SETTING_UNAVAILABLE the provided setting is
  unavailable for current device
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
  @retval ::RSMI_STATUS_BUSY A resource or mutex could not be acquired
  because it is already being used - device is busy
*/
    pub fn rsmi_dev_compute_partition_set(
        dv_ind: u32,
        compute_partition: rsmi_compute_partition_type_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieves the partition_id for a desired device

  @details
  Given a device index @p dv_ind and a uint32_t pointer @p partition_id ,
  this function will attempt to obtain the device's partition ID.
  Upon successful retreival, the obtained device's partition will be stored
  in the passed @p partition_id uint32_t variable. If device does
  not support partitions or is in SPX, a @p partition_id ID of 0 shall
  be returned.

  @param[in] dv_ind a device index

  @param[inout] partition_id a uint32_t variable,
  which the device's partition_id will be written to.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
*/
    pub fn rsmi_dev_partition_id_get(
        dv_ind: u32,
        partition_id: *mut u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieves the current memory partition for a desired device

  @details
  Given a device index @p dv_ind and a string @p memory_partition ,
  and uint32 @p len , this function will attempt to obtain the device's
  memory partition string. Upon successful retreival, the obtained device's
  memory partition string shall be stored in the passed @p memory_partition
  char string variable.

  @param[in] dv_ind a device index

  @param[inout] memory_partition a pointer to a char string variable,
  which the device's memory partition will be written to.

  @param[in] len the length of the caller provided buffer @p memory_partition ,
  suggested length is 5 or greater.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_UNEXPECTED_DATA data provided to function is not valid
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire memory partition value. In this case,
  only @p len bytes will be written.
*/
    pub fn rsmi_dev_memory_partition_get(
        dv_ind: u32,
        memory_partition: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Retrieves the available memory partition capabilities
  for a desired device

  @details
  Given a device index @p dv_ind and a string @p memory_partition_caps ,
  and uint32 @p len , this function will attempt to obtain the device's
  available memory partition capabilities string. Upon successful
  retreival, the obtained device's available memory partition capablilities
  string shall be stored in the passed @p memory_partition_caps
  char string variable.

  @param[in] dv_ind a device index

  @param[inout] memory_partition_caps a pointer to a char string variable,
  which the device's available memory partition capabilities will be written to.

  @param[in] len the length of the caller provided buffer @p len ,
  suggested length is 30 or greater.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_UNEXPECTED_DATA data provided to function is not valid
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
  @retval ::RSMI_STATUS_INSUFFICIENT_SIZE is returned if @p len bytes is not
  large enough to hold the entire memory partition value. In this case,
  only @p len bytes will be written.
*/
    pub fn rsmi_dev_memory_partition_capabilities_get(
        dv_ind: u32,
        memory_partition_caps: *mut ::core::ffi::c_char,
        len: u32,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Modifies a selected device's current memory partition setting.

  @details Given a device index @p dv_ind and a type of memory partition
  @p memory_partition, this function will attempt to update the selected
  device's memory partition setting.

  @param[in] dv_ind a device index

  @param[in] memory_partition using enum ::rsmi_memory_partition_type_t,
  define what the selected device's current mode setting should be updated to.

  @retval ::RSMI_STATUS_SUCCESS call was successful
  @retval ::RSMI_STATUS_PERMISSION function requires root access
  @retval ::RSMI_STATUS_INVALID_ARGS the provided arguments are not valid
  @retval ::RSMI_STATUS_NOT_SUPPORTED installed software or hardware does not
  support this function
  @retval ::RSMI_STATUS_AMDGPU_RESTART_ERR could not successfully restart
  the amdgpu driver
  @retval ::RSMI_STATUS_BUSY A resource or mutex could not be acquired
  because it is already being used - device is busy
*/
    pub fn rsmi_dev_memory_partition_set(
        dv_ind: u32,
        memory_partition: rsmi_memory_partition_type_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Get a function name iterator of supported RSMI functions for a device

 @details Given a device index @p dv_ind, this function will write a function
 iterator handle to the caller-provided memory pointed to by @p handle. This
 handle can be used to iterate through all the supported functions.

 Note that although this function takes in @p dv_ind as an argument,
 ::rsmi_dev_supported_func_iterator_open itself will not be among the
 functions listed as supported. This is because
 ::rsmi_dev_supported_func_iterator_open does not depend on hardware or
 driver support and should always be supported.

 @param[in] dv_ind a device index of device for which support information is
 requested

 @param[inout] handle A pointer to caller-provided memory to which the
 function iterator will be written.

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_supported_func_iterator_open(
        dv_ind: u32,
        handle: *mut rsmi_func_id_iter_handle_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Get a variant iterator for a given handle

 @details Given a ::rsmi_func_id_iter_handle_t @p obj_h, this function will
 write a function iterator handle to the caller-provided memory pointed to
 by @p var_iter. This handle can be used to iterate through all the supported
 variants of the provided handle. @p obj_h may be a handle to a function
 object, as provided by a call to ::rsmi_dev_supported_func_iterator_open, or
 it may be a variant itself (from a call to
 ::rsmi_dev_supported_variant_iterator_open), it which case @p var_iter will
 be an iterator of the sub-variants of @p obj_h (e.g., monitors).

 This call allocates a small amount of memory to @p var_iter. To free this memory
 ::rsmi_dev_supported_func_iterator_close should be called on the returned
 iterator handle @p var_iter when it is no longer needed.

 @param[in] obj_h an iterator handle for which the variants are being requested

 @param[inout] var_iter A pointer to caller-provided memory to which the
 sub-variant iterator will be written.

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_supported_variant_iterator_open(
        obj_h: rsmi_func_id_iter_handle_t,
        var_iter: *mut rsmi_func_id_iter_handle_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Advance a function identifer iterator

 @details Given a function id iterator handle (::rsmi_func_id_iter_handle_t)
 @p handle, this function will increment the iterator to point to the next
 identifier. After a successful call to this function, obtaining the value
 of the iterator @p handle will provide the value of the next item in the
 list of functions/variants.

 If there are no more items in the list, ::RSMI_STATUS_NO_DATA is returned.

 @param[in] handle A pointer to an iterator handle to be incremented

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
 @retval ::RSMI_STATUS_NO_DATA is returned when list of identifiers has been
 exhausted
*/
    pub fn rsmi_func_iter_next(handle: rsmi_func_id_iter_handle_t) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Close a variant iterator handle

 @details Given a pointer to an ::rsmi_func_id_iter_handle_t @p handle, this
 function will free the resources being used by the handle

 @param[in] handle A pointer to an iterator handle to be closed

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_supported_func_iterator_close(
        handle: *mut rsmi_func_id_iter_handle_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Get the value associated with a function/variant iterator

 @details Given an ::rsmi_func_id_iter_handle_t @p handle, this function
 will write the identifier of the function/variant to the user provided
 memory pointed to by @p value.

 @p value may point to a function name, a variant id, or a monitor/sensor
 index, depending on what kind of iterator @p handle is

 @param[in] handle An iterator for which the value is being requested

 @param[inout] value A pointer to an ::rsmi_func_id_value_t provided by the
 caller to which this function will write the value assocaited with @p handle

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_func_iter_value_get(
        handle: rsmi_func_id_iter_handle_t,
        value: *mut rsmi_func_id_value_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Prepare to collect event notifications for a GPU

 @details This function prepares to collect events for the GPU with device
 ID @p dv_ind, by initializing any required system parameters. This call
 may open files which will remain open until ::rsmi_event_notification_stop()
 is called.

 @param dv_ind a device index corresponding to the device on which to
 listen for events

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.*/
    pub fn rsmi_event_notification_init(dv_ind: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Specify which events to collect for a device

 @details Given a device index @p dv_ind and a @p mask consisting of
 elements of ::rsmi_evt_notification_type_t OR'd together, this function
 will listen for the events specified in @p mask on the device
 corresponding to @p dv_ind.

 @param dv_ind a device index corresponding to the device on which to
 listen for events

 @param mask Bitmask generated by OR'ing 1 or more elements of
 ::rsmi_evt_notification_type_t indicating which event types to listen for,
 where the rsmi_evt_notification_type_t value indicates the bit field, with
 bit position starting from 1.
 For example, if the mask field is 0x0000000000000003, which means first bit,
 bit 1 (bit position start from 1) and bit 2 are set, which indicate interest
 in receiving RSMI_EVT_NOTIF_VMFAULT (which has a value of 1) and
 RSMI_EVT_NOTIF_THERMAL_THROTTLE event (which has a value of 2).

 @retval ::RSMI_STATUS_INIT_ERROR is returned if
 ::rsmi_event_notification_init() has not been called before a call to this
 function

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call*/
    pub fn rsmi_event_notification_mask_set(dv_ind: u32, mask: u64) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Collect event notifications, waiting a specified amount of time

 @details Given a time period @p timeout_ms in milliseconds and a caller-
 provided buffer of ::rsmi_evt_notification_data_t's @p data with a length
 (in ::rsmi_evt_notification_data_t's, also specified by the caller) in the
 memory location pointed to by @p num_elem, this function will collect
 ::rsmi_evt_notification_type_t events for up to @p timeout_ms milliseconds,
 and write up to *@p num_elem event items to @p data. Upon return @p num_elem
 is updated with the number of events that were actually written. If events
 are already present when this function is called, it will write the events
 to the buffer then poll for new events if there is still caller-provided
 buffer available to write any new events that would be found.

 This function requires prior calls to ::rsmi_event_notification_init() and
 ::rsmi_event_notification_mask_set(). This function polls for the
 occurrance of the events on the respective devices that were previously
 specified by ::rsmi_event_notification_mask_set().

 @param[in] timeout_ms number of milliseconds to wait for an event
 to occur

 @param[inout] num_elem pointer to uint32_t, provided by the caller. On
 input, this value tells how many ::rsmi_evt_notification_data_t elements
 are being provided by the caller with @p data. On output, the location
 pointed to by @p num_elem will contain the number of items written to
 the provided buffer.

 @param[out] data pointer to a caller-provided memory buffer of size
 @p num_elem ::rsmi_evt_notification_data_t to which this function may safely
 write. If there are events found, up to @p num_elem event items will be
 written to @p data.

 @retval ::RSMI_STATUS_SUCCESS The function ran successfully. The events
 that were found are written to @p data and @p num_elems is updated
 with the number of elements that were written.

 @retval ::RSMI_STATUS_NO_DATA No events were found to collect.
*/
    pub fn rsmi_event_notification_get(
        timeout_ms: ::core::ffi::c_int,
        num_elem: *mut u32,
        data: *mut rsmi_evt_notification_data_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /** @brief Close any file handles and free any resources used by event
 notification for a GPU

 @details Any resources used by event notification for the GPU with
 device index @p dv_ind will be free with this
 function. This includes freeing any memory and closing file handles. This
 should be called for every call to ::rsmi_event_notification_init()

 @param[in] dv_ind The device index of the GPU for which event
 notification resources will be free

 @retval ::RSMI_STATUS_INVALID_ARGS resources for the given device have
 either already been freed, or were never allocated by
 ::rsmi_event_notification_init()

 @retval ::RSMI_STATUS_SUCCESS is returned upon successful call*/
    pub fn rsmi_event_notification_stop(dv_ind: u32) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the 'metrics_header_info' from the GPU metrics associated with the device

  @details Given a device index @p dv_ind and a pointer to a metrics_table_header_t in which
  the 'metrics_header_info' will stored

  @param[in] dv_ind a device index

  @param[inout] header_value a pointer to metrics_table_header_t to which the device gpu
  metric unit will be stored

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
          ::RSMI_STATUS_NOT_SUPPORTED is returned in case the metric unit
            does not exist for the given device
*/
    pub fn rsmi_dev_metrics_header_info_get(
        dv_ind: u32,
        header_value: *mut metrics_table_header_t,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the 'xcd_counter' from the GPU metrics associated with the device

  @details Given a device index @p dv_ind and a pointer to a uint16_t in which
  the 'xcd_counter' will stored

  @param[in] dv_ind a device index

  @param[inout] xcd_counter_value a pointer to uint16_t to which the device gpu
  metric unit will be stored

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
          ::RSMI_STATUS_NOT_SUPPORTED is returned in case the metric unit
            does not exist for the given device
*/
    pub fn rsmi_dev_metrics_xcd_counter_get(
        dv_ind: u32,
        xcd_counter_value: *mut u16,
    ) -> rsmi_status_t;
}
extern "C" {
    #[must_use]
    /**  @brief Get the log from the GPU metrics associated with the device

  @details Given a device index @p dv_ind it will log all the gpu metric info
  related to the device. The 'logging' feature must be on.

  @param[in] dv_ind a device index

  @retval ::RSMI_STATUS_SUCCESS is returned upon successful call.
*/
    pub fn rsmi_dev_metrics_log_get(dv_ind: u32) -> rsmi_status_t;
}
impl rsmi_error {
    pub const r#INVALID_ARGS: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const r#NOT_SUPPORTED: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const r#FILE_ERROR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const r#PERMISSION: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const r#OUT_OF_RESOURCES: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const r#INTERNAL_EXCEPTION: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const r#INPUT_OUT_OF_BOUNDS: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const r#INIT_ERROR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#IZATION_ERROR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const r#NOT_YET_IMPLEMENTED: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(9)
    });
    pub const r#NOT_FOUND: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(10)
    });
    pub const r#INSUFFICIENT_SIZE: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(11)
    });
    pub const r#INTERRUPT: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(12)
    });
    pub const r#UNEXPECTED_SIZE: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(13)
    });
    pub const r#NO_DATA: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(14)
    });
    pub const r#UNEXPECTED_DATA: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(15)
    });
    pub const r#BUSY: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(16)
    });
    pub const r#REFCOUNT_OVERFLOW: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(17)
    });
    pub const r#SETTING_UNAVAILABLE: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(18)
    });
    pub const r#AMDGPU_RESTART_ERR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(19)
    });
    pub const r#DRM_ERROR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(20)
    });
    pub const r#UNKNOWN_ERROR: rsmi_error = rsmi_error(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4294967295)
    });
}
#[repr(transparent)]
#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct rsmi_error(pub ::core::num::NonZeroU32);
pub trait rsmi_status_tConsts {
    const SUCCESS: rsmi_status_t = rsmi_status_t::Ok(());
    const ERROR_INVALID_ARGS: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#INVALID_ARGS,
    );
    const ERROR_NOT_SUPPORTED: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#NOT_SUPPORTED,
    );
    const ERROR_FILE_ERROR: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#FILE_ERROR);
    const ERROR_PERMISSION: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#PERMISSION);
    const ERROR_OUT_OF_RESOURCES: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#OUT_OF_RESOURCES,
    );
    const ERROR_INTERNAL_EXCEPTION: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#INTERNAL_EXCEPTION,
    );
    const ERROR_INPUT_OUT_OF_BOUNDS: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#INPUT_OUT_OF_BOUNDS,
    );
    const ERROR_INIT_ERROR: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#INIT_ERROR);
    const ERROR_IZATION_ERROR: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#IZATION_ERROR,
    );
    const ERROR_NOT_YET_IMPLEMENTED: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#NOT_YET_IMPLEMENTED,
    );
    const ERROR_NOT_FOUND: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#NOT_FOUND);
    const ERROR_INSUFFICIENT_SIZE: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#INSUFFICIENT_SIZE,
    );
    const ERROR_INTERRUPT: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#INTERRUPT);
    const ERROR_UNEXPECTED_SIZE: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#UNEXPECTED_SIZE,
    );
    const ERROR_NO_DATA: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#NO_DATA);
    const ERROR_UNEXPECTED_DATA: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#UNEXPECTED_DATA,
    );
    const ERROR_BUSY: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#BUSY);
    const ERROR_REFCOUNT_OVERFLOW: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#REFCOUNT_OVERFLOW,
    );
    const ERROR_SETTING_UNAVAILABLE: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#SETTING_UNAVAILABLE,
    );
    const ERROR_AMDGPU_RESTART_ERR: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#AMDGPU_RESTART_ERR,
    );
    const ERROR_DRM_ERROR: rsmi_status_t = rsmi_status_t::Err(rsmi_error::r#DRM_ERROR);
    const ERROR_UNKNOWN_ERROR: rsmi_status_t = rsmi_status_t::Err(
        rsmi_error::r#UNKNOWN_ERROR,
    );
}
impl rsmi_status_tConsts for rsmi_status_t {}
#[must_use]
pub type rsmi_status_t = ::core::result::Result<(), rsmi_error>;
const _: fn() = || {
    let _ = std::mem::transmute::<rsmi_status_t, u32>;
};
