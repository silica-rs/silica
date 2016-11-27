Silica
=========

Objectives
----------
Build a complete OS for bare metal including but not limited to support for :
- Single threading ;
- Preemptive kernel for multi threading ;
- IP stack ;
- file systems...

Short terms
-----------
I will begin with :
- Support for cortex-m3 and stm32f2xx ;
- Fat File system ;
- MQTT-SN ;

---
```nomnoml
#fill: #FFF

[<package> silica-panic]
[<package> silica-allocator|
init()
__rust_allocate(size: usize, _align: usize) -> *mut u8
__rust_deallocate(ptr: *mut u8, _old_size: usize, _align: u8)
__rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize, _align: u8) -> *mut u8
__rust_reallocate_in_place(_ptr: *mut u8, old_size: usize, _size: usize, _align: usize) -> *mut u8
__rust_usable_size(size: usize, align: usize) -> usize]
[<package> silica]
[<package> silica-cortexm]
[<package> silica-stm32f2xx]
[<package> silica-stm32f207]
[<package> silica-olimex-p207]
[<package> silica-chunks]
[<package> applications-libs]
[<package> actual-project]

[silica-chunks]<--[silica-allocator]
[silica-allocator]<--[silica]
[silica-panic]<--[silica]

[silica]<--[silica-cortexm]
[silica-cortexm]<--[silica-stm32f2xx]
[silica-stm32f2xx]<--[silica-stm32f207]
[silica-stm32f207]<--[silica-olimex-p207]

[silica]<--[applications-libs]

[silica]<--[actual-project]
[silica-cortexm]<--[actual-project]
[silica-stm32f2xx]<--[actual-project]
[silica-stm32f207]<--[actual-project]
[silica-olimex-p207]<--[actual-project]
[applications-libs]<--[actual-project]
```
