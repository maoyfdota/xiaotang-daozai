//! macOS 系统累计计数器读取。调用失败时只返回对应指标的 `None`。

use super::{ByteCounters, CpuCounters, MemoryCounters, RawSample};
use std::ffi::{c_void, CStr};
use std::mem::{size_of, zeroed};
use std::ptr;

pub(super) fn collect() -> RawSample {
    RawSample {
        cpu: cpu_counters(),
        memory: memory_counters(),
        disk: disk_counters(),
        network: network_counters(),
    }
}

fn cpu_counters() -> Option<CpuCounters> {
    unsafe {
        let mut processor_count: libc::natural_t = 0;
        let mut info: libc::processor_info_array_t = ptr::null_mut();
        let mut info_count: libc::mach_msg_type_number_t = 0;
        let result = libc::host_processor_info(
            libc::mach_host_self(),
            libc::PROCESSOR_CPU_LOAD_INFO,
            &mut processor_count,
            &mut info,
            &mut info_count,
        );
        if result != libc::KERN_SUCCESS || info.is_null() {
            return None;
        }

        let slots = info_count as usize;
        // Mach's `natural_t` ticks are unsigned even though the array ABI uses `integer_t`.
        let values = std::slice::from_raw_parts(info.cast::<libc::natural_t>(), slots);
        let counters = (|| {
            let mut busy = 0u64;
            let mut total = 0u64;
            for cpu in values.chunks_exact(libc::CPU_STATE_MAX as usize) {
                let user = cpu[libc::CPU_STATE_USER as usize] as u64;
                let system = cpu[libc::CPU_STATE_SYSTEM as usize] as u64;
                let idle = cpu[libc::CPU_STATE_IDLE as usize] as u64;
                let nice = cpu[libc::CPU_STATE_NICE as usize] as u64;
                busy = busy
                    .checked_add(user)?
                    .checked_add(system)?
                    .checked_add(nice)?;
                total = total
                    .checked_add(user)?
                    .checked_add(system)?
                    .checked_add(idle)?
                    .checked_add(nice)?;
            }
            Some((busy, total))
        })();
        let address = info as libc::vm_address_t;
        let bytes = slots.checked_mul(size_of::<libc::natural_t>())? as libc::vm_size_t;
        let _ = libc::vm_deallocate(libc::mach_task_self(), address, bytes);
        let (busy, total) = counters?;
        (processor_count > 0 && total > 0).then_some(CpuCounters { busy, total })
    }
}

fn memory_counters() -> Option<MemoryCounters> {
    unsafe {
        let mut stats: libc::vm_statistics64 = zeroed();
        let mut count = libc::HOST_VM_INFO64_COUNT;
        let result = libc::host_statistics64(
            libc::mach_host_self(),
            libc::HOST_VM_INFO64,
            &mut stats as *mut _ as libc::host_info64_t,
            &mut count,
        );
        if result != libc::KERN_SUCCESS {
            return None;
        }
        let total = sysctl_u64(c"hw.memsize")?;
        let page_size = libc::sysconf(libc::_SC_PAGESIZE);
        if page_size <= 0 || total == 0 {
            return None;
        }
        let free = (stats.free_count as u64).checked_mul(page_size as u64)?;
        Some(MemoryCounters {
            used: total.saturating_sub(free),
            total,
        })
    }
}

unsafe fn sysctl_u64(name: &CStr) -> Option<u64> {
    let mut value = 0u64;
    let mut length = size_of::<u64>();
    let result = libc::sysctlbyname(
        name.as_ptr(),
        &mut value as *mut _ as *mut c_void,
        &mut length,
        ptr::null_mut(),
        0,
    );
    (result == 0 && length == size_of::<u64>()).then_some(value)
}

fn disk_counters() -> Option<ByteCounters> {
    unsafe {
        let matching_name = c"IOBlockStorageDriver";
        let matching = IOServiceMatching(matching_name.as_ptr());
        if matching.is_null() {
            return None;
        }
        let mut iterator = 0;
        if IOServiceGetMatchingServices(0, matching, &mut iterator) != 0 {
            return None;
        }

        let statistics_key = cf_string(c"Statistics")?;
        let read_key = cf_string(c"Bytes (Read)")?;
        let write_key = cf_string(c"Bytes (Write)")?;
        let mut total = ByteCounters::default();
        let mut found = false;
        loop {
            let service = IOIteratorNext(iterator);
            if service == 0 {
                break;
            }
            let mut properties: *mut c_void = ptr::null_mut();
            if IORegistryEntryCreateCFProperties(service, &mut properties, ptr::null(), 0) == 0
                && !properties.is_null()
            {
                let statistics = CFDictionaryGetValue(properties, statistics_key);
                if !statistics.is_null() {
                    if let (Some(read), Some(write)) = (
                        cf_number(CFDictionaryGetValue(statistics, read_key)),
                        cf_number(CFDictionaryGetValue(statistics, write_key)),
                    ) {
                        if let (Some(next_read), Some(next_write)) =
                            (total.read.checked_add(read), total.write.checked_add(write))
                        {
                            total.read = next_read;
                            total.write = next_write;
                            found = true;
                        }
                    }
                }
                CFRelease(properties);
            }
            let _ = IOObjectRelease(service);
        }
        let _ = IOObjectRelease(iterator);
        CFRelease(statistics_key);
        CFRelease(read_key);
        CFRelease(write_key);
        found.then_some(total)
    }
}

unsafe fn cf_string(value: &CStr) -> Option<*const c_void> {
    let string = CFStringCreateWithCString(ptr::null(), value.as_ptr(), 0x0800_0100);
    (!string.is_null()).then_some(string)
}

unsafe fn cf_number(value: *const c_void) -> Option<u64> {
    if value.is_null() {
        return None;
    }
    let mut number = 0i64;
    let ok = CFNumberGetValue(value, 4, &mut number as *mut _ as *mut c_void);
    (ok && number >= 0).then_some(number as u64)
}

fn network_counters() -> Option<ByteCounters> {
    unsafe {
        let mut mib = [libc::CTL_NET, libc::PF_ROUTE, 0, 0, libc::NET_RT_IFLIST2, 0];
        let mut length = 0usize;
        if libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            ptr::null_mut(),
            &mut length,
            ptr::null_mut(),
            0,
        ) != 0
            || length == 0
        {
            return None;
        }
        let mut bytes = vec![0u8; length];
        if libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            bytes.as_mut_ptr() as *mut c_void,
            &mut length,
            ptr::null_mut(),
            0,
        ) != 0
        {
            return None;
        }

        let mut result = ByteCounters::default();
        let mut found = false;
        let mut offset = 0usize;
        while offset.checked_add(size_of::<libc::if_msghdr2>())? <= length {
            let header = bytes.as_ptr().add(offset) as *const libc::if_msghdr2;
            let message_length = ptr::addr_of!((*header).ifm_msglen).read_unaligned() as usize;
            if message_length < size_of::<libc::if_msghdr2>()
                || offset.checked_add(message_length)? > length
            {
                return None;
            }
            let message_type = ptr::addr_of!((*header).ifm_type).read_unaligned();
            if message_type == libc::RTM_IFINFO2 as u8 {
                let flags = ptr::addr_of!((*header).ifm_flags).read_unaligned();
                if flags & libc::IFF_UP != 0 && flags & libc::IFF_LOOPBACK == 0 {
                    let data = ptr::addr_of!((*header).ifm_data).read_unaligned();
                    let received = ptr::addr_of!(data.ifi_ibytes).read_unaligned();
                    let sent = ptr::addr_of!(data.ifi_obytes).read_unaligned();
                    result.read = result.read.checked_add(received)?;
                    result.write = result.write.checked_add(sent)?;
                    found = true;
                }
            }
            offset += message_length;
        }
        found.then_some(result)
    }
}

type IoObject = u32;

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOServiceMatching(name: *const libc::c_char) -> *mut c_void;
    fn IOServiceGetMatchingServices(
        master_port: u32,
        matching: *mut c_void,
        existing: *mut IoObject,
    ) -> i32;
    fn IOIteratorNext(iterator: IoObject) -> IoObject;
    fn IOObjectRelease(object: IoObject) -> i32;
    fn IORegistryEntryCreateCFProperties(
        entry: IoObject,
        properties: *mut *mut c_void,
        allocator: *const c_void,
        options: u32,
    ) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFStringCreateWithCString(
        allocator: *const c_void,
        c_string: *const libc::c_char,
        encoding: u32,
    ) -> *const c_void;
    fn CFDictionaryGetValue(dictionary: *const c_void, key: *const c_void) -> *const c_void;
    fn CFNumberGetValue(number: *const c_void, number_type: i32, value: *mut c_void) -> bool;
    fn CFRelease(value: *const c_void);
}
