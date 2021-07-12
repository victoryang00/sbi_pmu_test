
use super::SbiRet;

use riscv::register;

const FUNCTION_PMU_NUM_COUNTERS:usize =	0x0;
const FUNCTION_PMU_COUNTER_GET_INFO:usize =	0x1;
const FUNCTION_PMU_COUNTER_CFG_MATCH:usize =	0x2;
const FUNCTION_PMU_COUNTER_START:usize =	0x3;
const FUNCTION_PMU_COUNTER_STOP:usize =	0x4;
const FUNCTION_PMU_COUNTER_FW_READ:usize =	0x5;
/**
 * Generalized hardware cache events:
 *
 *       { L1-D, L1-I, LLC, ITLB, DTLB, BPU, NODE } x
 *       { read, write, prefetch } x
 *       { accesses, misses }
 */
const SBI_PMU_HW_CACHE_L1D:usize = 0x0;
const SBI_PMU_HW_CACHE_L1I:usize = 0x1;
const SBI_PMU_HW_CACHE_LL:usize = 0x2;
const SBI_PMU_HW_CACHE_DTLB:usize = 0x3;
const SBI_PMU_HW_CACHE_ITLB:usize = 0x4;
const SBI_PMU_HW_CACHE_BPU:usize = 0x5;
const SBI_PMU_HW_CACHE_NODE:usize = 0x6;
const SBI_PMU_HW_CACHE_MAX:usize =core::usize::MAX;



const SBI_PMU_START_SET_INIT_VALUE:usize = 0x0;

#[inline]
pub fn handle_ecall_pmu(function: usize, param0: usize, param1: usize, param2: usize, param3: usize, param4: u64) -> SbiRet {
    match function {
        FUNCTION_PMU_NUM_COUNTERS=>pmu_num_counters(),
        FUNCTION_PMU_COUNTER_GET_INFO=>pmu_counter_get_info(),
        FUNCTION_PMU_COUNTER_CFG_MATCH =>pmu_counter_cfg_map(),
        FUNCTION_PMU_COUNTER_START => pmu_start(param1,param2,param3,param4),
        FUNCTION_PMU_COUNTER_STOP => pmu_stop(param1,param2,param3),
        FUNCTION_PMU_COUNTER_FW_READ => pmu_read(param1),
        _ => SbiRet::not_supported(),
    }
}

#[inline]
fn pmu_start(counter_id_base: usize, counter_id_mask: usize, start_flags: usize, initial_value: u64) -> SbiRet{
    crate::pmu::pmu_start(counter_id_base, counter_id_mask, start_flags, initial_value)
}

#[inline]
fn pmu_stop(counter_id_base: usize, counter_id_mask: usize, stop_flags: usize) -> SbiRet{
    crate::pmu::pmu_stop(counter_id_base, counter_id_mask, stop_flags)
}

#[inline]
fn pmu_read(counter_idx: usize) -> SbiRet {
    crate::pmu::pmu_fw_read(counter_idx)
}

#[inline]
fn pmu_num_counters() ->SbiRet{
    SbiRet::not_supported()
}

#[inline]
fn pmu_counter_get_info() ->SbiRet{
    SbiRet::not_supported()
}

#[inline]
fn pmu_counter_cfg_map() ->SbiRet{
    SbiRet::not_supported()
}


