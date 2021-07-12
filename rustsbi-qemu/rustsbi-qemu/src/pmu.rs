pub struct Pmu;

const TEST_FAIL: u32 = 0x3333;
const TEST_PASS: u32 = 0x5555;
const TEST_RESET: u32 = 0x7777;

impl rustsbi::Pmu for Pmu {
    fn pmu_counter_start(&mut self, counter_idx_base: usize, counter_idx_mask: usize, start_flags: usize, initial_value:u64) -> rustsbi::SbiRet{
        rustsbi::SbiRet {
            error: 0,
            value: 0,
        }
    }
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
    fn pmu_counter_stop(&mut self, counter_idx_base: usize, counter_idx_mask: usize, stop_flags: usize) -> rustsbi::SbiRet{
        rustsbi::SbiRet {
            error: 0,
            value: 0,
        }
    }
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
    fn pmu_counter_fw_read(&self, counter_idx: usize) -> rustsbi::SbiRet{
        rustsbi::SbiRet {
            error: 0,
            value: 0,
        }
    }
}
