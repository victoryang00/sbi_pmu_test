use crate::ecall::SbiRet;

/// Performance Monitoring Unit Extension 
///
/// The RISC-V hardware performance counters such as `mcycle`, `minstret`, and
/// `mhpmcounterX` CSRs are accessible as read-only from supervisor-mode using
/// `cycle`, `instret`, and `hpmcounterX` CSRs. The SBI performance monitoring
/// unit (PMU) extension is an interface for supervisor-mode to configure and
/// use the RISC-V hardware performance counters with assistance from the
/// machine-mode (or hypervisor-mode). These hardware performance counters
/// can only be started, stopped, or configured from machine-mode using
/// `mcountinhibit` and `mhpmeventX` CSRs. Due to this, a machine-mode SBI
/// implementation may choose to disallow SBI PMU extension if `mcountinhibit`
/// CSR is not implemented by the RISC-V platform.
/// 
/// A RISC-V platform generally supports monitoring of various hardware events
/// using a limited number of hardware performance counters which are up to
/// 64 bits wide. In addition, a SBI implementation can also provide firmware
/// performance counters which can monitor firmware events such as number of
/// misaligned load/store instructions, number of RFENCEs, number of IPIs, etc.
/// The firmware counters are always 64 bits wide.
/// 
/// The SBI PMU extension provides:
/// 
/// 1. An interface for supervisor-mode software to discover and configure
/// per-HART hardware/firmware counters
/// 2. A typical https://en.wikipedia.org/wiki/Perf_(Linux)[perf] compatible
///    interface for hardware/firmware performance counters and events
/// 3. Full access to microarchitecture's raw event encodings
/// 
/// To define SBI PMU extension calls, we first define important entities
/// `counter_idx`, `event_idx`, and `event_data`. The `counter_idx` is a
/// logical number assigned to each hardware/firmware counter. The `event_idx`
/// represents a hardware (or firmware) event whereas the `event_data` is
/// 64 bits wide and represents additional configuration (or parameters) for
/// a hardware (or firmware) event.
/// 
/// The event_idx is a 20 bits wide number encoded as follows:
/// [source, C]
/// ----
///     event_idx[19:16] = type
///     event_idx[15:0] = code
/// ----
/// 
/// Ref: [Section 9, RISC-V Supervisor Binary Interface Specification](https://github.com/riscv/riscv-sbi-doc/blob/master/riscv-sbi.adoc#performance-monitoring-unit-extension-eid-0x504d55-pmu)
pub trait Pmu: Send {
    /// Start or enable a sef of counters on the calling HART with the specified initial value. The counter_idx_base and counter_idx_mask parameters represent the set of counters whereas the initial_value parameter specifies the initial value of the counter.
    /// The bit definitions of the start_flags parameter are shown in the Table  below.
    ///
    /// # Parameters
    /// 
    /// - The `counter_idx_base` parameter specifies the index base for the pmu counter
    /// - The `counter_idx_mask` parameter specifies the idx mask for the counter
    ///
    /// # Return value
    ///
    /// The possible return error codes returned in `SbiRet.error` are shown in the table below:
    ///
    /// | Return code               | Description 
    /// |:--------------------------|:----------------------------------------------
    /// | SBI_SUCCESS               | counter started successfully.
    /// | SBI_ERR_INVALID_PARAM     | `start_addr` is not valid possibly due to following reasons: 1. It is not a valid physical address. 2. The address is prohibited by PMP to run in supervisor mode.
    /// | SBI_ERR_INVALID_PARAM     | `hartid` is not a valid hartid as corresponding hart cannot started in supervisor mode. 
    /// | SBI_ERR_ALREADY_AVAILABLE | The given hartid is already started.
    /// | SBI_ERR_FAILED            | The start request failed for unknown reasons.
    ///
    /// # Flags
    /// 
    ///| Flag Name                    | Bits       | Description
    /// | SBI_PMU_START_SET_INIT_VALUE | 0:0        | Set the value of counters
    /// based on the `initial_value`
    /// parameter
    /// | *RESERVED*                   | 1:(XLEN-1) | All non-zero values are
    /// reserved for future use
    /// 
    /// *NOTE:* When SBI_PMU_START_SET_INIT_VALUE is not set in `start_flags`,
    /// the counter value will not be modified and event counting will start
    /// from current counter value.
    /// 
    /// # Errors
    /// 
    /// | Error code              | Description
    /// | SBI_SUCCESS             | counter started successfully.
    /// | SBI_ERR_INVALID_PARAM   | some of the counters specified in parameters
    ///                             are invalid.
    /// | SBI_ERR_ALREADY_STARTED | some of the counters specified in parameters
    ///                             are already started.
    fn pmu_counter_start(&mut self, counter_idx_base: usize, counter_idx_mask: usize, start_flags: usize, initial_value:u64) -> SbiRet;
    /// Stop or disable a set of counters on the calling HART. The `counter_idx_base`
    ///and `counter_idx_mask` parameters represent the set of counters. The bit
    ///definitions of the `stop_flags` parameter are shown in the below table.
    /// 
    /// # Flags
    /// | Flag Name               | Bits       | Description
    /// | SBI_PMU_STOP_FLAG_RESET | 0:0        | Reset the counter to event mapping.
    /// | *RESERVED*              | 1:(XLEN-1) | All non-zero values are reserved
    ///     
    /// # Errors
    /// 
    /// | Error code              | Description
    /// | SBI_SUCCESS             | counter stopped successfully.
    /// | SBI_ERR_INVALID_PARAM   | some of the counters specified in parameters
    ///                             are invalid.
    /// | SBI_ERR_ALREADY_STOPPED | some of the counters specified in parameters
    ///                             are already stopped.
    fn pmu_counter_stop(&mut self, counter_idx_base: usize, counter_idx_mask: usize, stop_flags: usize) -> SbiRet;
    /// | Function Name                   | SBI Version | FID | EID
    /// | sbi_pmu_num_counters            | 0.3         | 0   | 0x504D55
    /// | sbi_pmu_counter_get_info        | 0.3         | 1   | 0x504D55
    /// | sbi_pmu_counter_config_matching | 0.3         | 2   | 0x504D55
    /// | sbi_pmu_counter_start           | 0.3         | 3   | 0x504D55
    /// | sbi_pmu_counter_stop            | 0.3         | 4   | 0x504D55
    /// | sbi_pmu_counter_fw_read         | 0.3         | 5   | 0x504D55
    /// Low bits is SBI implementation ID. The firmware specific SBI extensions are
    /// for SBI implementations. It provides firmware specific SBI functions which
    /// are defined in the external firmware specification.
    fn pmu_counter_fw_read(&self, counter_idx: usize) -> SbiRet;
}

use alloc::boxed::Box;
use spin::Mutex;

lazy_static::lazy_static! {
    static ref PMU: Mutex<Option<Box<dyn Pmu>>> =
        Mutex::new(None);
}

#[doc(hidden)] // use through a macro or a call from implementation
pub fn init_pmu<T: Pmu + Send + 'static>(pmu: T) {
    *PMU.lock() = Some(Box::new(pmu));
}

#[inline]
pub(crate) fn probe_pmu() -> bool {
    PMU.lock().as_ref().is_some()
}

pub(crate) fn pmu_start(counter_id_base: usize, counter_id_mask: usize, start_flags: usize, initial_value: u64) -> SbiRet {
    if let Some(obj) = &mut *PMU.lock() {
        return obj.pmu_counter_start(counter_id_base, counter_id_mask, start_flags,initial_value);
    }
    SbiRet::not_supported()
}

pub(crate) fn pmu_stop(counter_id_base: usize, counter_id_mask: usize, stop_flags: usize) -> SbiRet {
    if let Some(obj) = &mut *PMU.lock() {
        return obj.pmu_counter_stop(counter_id_base, counter_id_mask, stop_flags);
    }
    SbiRet::not_supported()
}

pub(crate) fn pmu_fw_read(counter_idx: usize) -> SbiRet {
    if let Some(obj) = &mut *PMU.lock() {
        return obj.pmu_counter_fw_read(counter_idx);
    }
    SbiRet::not_supported()
}
