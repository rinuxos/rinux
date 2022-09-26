(function() {var implementors = {};
implementors["enderpearl"] = [{"text":"impl Sync for <a class=\"struct\" href=\"enderpearl/struct.Command.html\" title=\"struct enderpearl::Command\">Command</a>","synthetic":true,"types":["enderpearl::Command"]},{"text":"impl Sync for <a class=\"struct\" href=\"enderpearl/struct.Operation.html\" title=\"struct enderpearl::Operation\">Operation</a>","synthetic":true,"types":["enderpearl::Operation"]},{"text":"impl Sync for <a class=\"struct\" href=\"enderpearl/struct.EnderPearl.html\" title=\"struct enderpearl::EnderPearl\">EnderPearl</a>","synthetic":true,"types":["enderpearl::EnderPearl"]}];
implementors["lazy_static"] = [{"text":"impl&lt;T&gt; Sync for <a class=\"struct\" href=\"lazy_static/lazy/struct.Lazy.html\" title=\"struct lazy_static::lazy::Lazy\">Lazy</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send,&nbsp;</span>","synthetic":true,"types":["lazy_static::lazy::Lazy"]}];
implementors["rinuxcore"] = [{"text":"impl Sync for <a class=\"enum\" href=\"rinuxcore/conf/enum.ConfigType.html\" title=\"enum rinuxcore::conf::ConfigType\">ConfigType</a>","synthetic":true,"types":["rinuxcore::conf::ConfigType"]},{"text":"impl Sync for <a class=\"struct\" href=\"rinuxcore/conf/struct.Config.html\" title=\"struct rinuxcore::conf::Config\">Config</a>","synthetic":true,"types":["rinuxcore::conf::Config"]},{"text":"impl !Sync for <a class=\"struct\" href=\"rinuxcore/task/executor/struct.Executor.html\" title=\"struct rinuxcore::task::executor::Executor\">Executor</a>","synthetic":true,"types":["rinuxcore::task::executor::Executor"]},{"text":"impl Sync for <a class=\"struct\" href=\"rinuxcore/task/keyboard/struct.ScancodeStream.html\" title=\"struct rinuxcore::task::keyboard::ScancodeStream\">ScancodeStream</a>","synthetic":true,"types":["rinuxcore::task::keyboard::ScancodeStream"]},{"text":"impl !Sync for <a class=\"struct\" href=\"rinuxcore/task/simple_executor/struct.SimpleExecutor.html\" title=\"struct rinuxcore::task::simple_executor::SimpleExecutor\">SimpleExecutor</a>","synthetic":true,"types":["rinuxcore::task::simple_executor::SimpleExecutor"]},{"text":"impl !Sync for <a class=\"struct\" href=\"rinuxcore/task/struct.Task.html\" title=\"struct rinuxcore::task::Task\">Task</a>","synthetic":true,"types":["rinuxcore::task::Task"]},{"text":"impl Sync for <a class=\"enum\" href=\"rinuxcore/enum.ConfigType.html\" title=\"enum rinuxcore::ConfigType\">ConfigType</a>","synthetic":true,"types":["rinuxcore::ConfigType"]},{"text":"impl Sync for <a class=\"enum\" href=\"rinuxcore/enum.QemuExitCode.html\" title=\"enum rinuxcore::QemuExitCode\">QemuExitCode</a>","synthetic":true,"types":["rinuxcore::QemuExitCode"]}];
implementors["spin"] = [{"text":"impl&lt;R&gt; Sync for <a class=\"struct\" href=\"spin/barrier/struct.Barrier.html\" title=\"struct spin::barrier::Barrier\">Barrier</a>&lt;R&gt;","synthetic":true,"types":["spin::barrier::Barrier"]},{"text":"impl Sync for <a class=\"struct\" href=\"spin/barrier/struct.BarrierWaitResult.html\" title=\"struct spin::barrier::BarrierWaitResult\">BarrierWaitResult</a>","synthetic":true,"types":["spin::barrier::BarrierWaitResult"]},{"text":"impl&lt;T, F&nbsp;=&nbsp;fn() -&gt; T, R&nbsp;=&nbsp;<a class=\"struct\" href=\"spin/relax/struct.Spin.html\" title=\"struct spin::relax::Spin\">Spin</a>&gt; !Sync for <a class=\"struct\" href=\"spin/lazy/struct.Lazy.html\" title=\"struct spin::lazy::Lazy\">Lazy</a>&lt;T, F, R&gt;","synthetic":true,"types":["spin::lazy::Lazy"]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Sync for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutexGuard.html\" title=\"struct spin::mutex::spin::SpinMutexGuard\">SpinMutexGuard</a>&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":["spin::mutex::spin::SpinMutexGuard"]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Sync for <a class=\"struct\" href=\"spin/mutex/struct.MutexGuard.html\" title=\"struct spin::mutex::MutexGuard\">MutexGuard</a>&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":["spin::mutex::MutexGuard"]},{"text":"impl&lt;'a, T:&nbsp;?Sized&gt; Sync for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockReadGuard.html\" title=\"struct spin::rwlock::RwLockReadGuard\">RwLockReadGuard</a>&lt;'a, T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":["spin::rwlock::RwLockReadGuard"]},{"text":"impl&lt;'a, T:&nbsp;?Sized, R&gt; Sync for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockWriteGuard.html\" title=\"struct spin::rwlock::RwLockWriteGuard\">RwLockWriteGuard</a>&lt;'a, T, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send + Sync,&nbsp;</span>","synthetic":true,"types":["spin::rwlock::RwLockWriteGuard"]},{"text":"impl&lt;'a, T:&nbsp;?Sized, R&gt; Sync for <a class=\"struct\" href=\"spin/rwlock/struct.RwLockUpgradableGuard.html\" title=\"struct spin::rwlock::RwLockUpgradableGuard\">RwLockUpgradableGuard</a>&lt;'a, T, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Send + Sync,&nbsp;</span>","synthetic":true,"types":["spin::rwlock::RwLockUpgradableGuard"]},{"text":"impl Sync for <a class=\"struct\" href=\"spin/relax/struct.Spin.html\" title=\"struct spin::relax::Spin\">Spin</a>","synthetic":true,"types":["spin::relax::Spin"]},{"text":"impl Sync for <a class=\"struct\" href=\"spin/relax/struct.Loop.html\" title=\"struct spin::relax::Loop\">Loop</a>","synthetic":true,"types":["spin::relax::Loop"]},{"text":"impl&lt;T, F:&nbsp;Send&gt; Sync for <a class=\"struct\" href=\"spin/lazy/struct.Lazy.html\" title=\"struct spin::lazy::Lazy\">Lazy</a>&lt;T, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;<a class=\"struct\" href=\"spin/once/struct.Once.html\" title=\"struct spin::once::Once\">Once</a>&lt;T&gt;: Sync,&nbsp;</span>","synthetic":false,"types":["spin::lazy::Lazy"]},{"text":"impl&lt;T:&nbsp;?Sized + Send, R&gt; Sync for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutex.html\" title=\"struct spin::mutex::spin::SpinMutex\">SpinMutex</a>&lt;T, R&gt;","synthetic":false,"types":["spin::mutex::spin::SpinMutex"]},{"text":"impl&lt;T:&nbsp;?Sized + Send, R&gt; Sync for <a class=\"struct\" href=\"spin/mutex/struct.Mutex.html\" title=\"struct spin::mutex::Mutex\">Mutex</a>&lt;T, R&gt;","synthetic":false,"types":["spin::mutex::Mutex"]},{"text":"impl&lt;T:&nbsp;Send + Sync, R&gt; Sync for <a class=\"struct\" href=\"spin/once/struct.Once.html\" title=\"struct spin::once::Once\">Once</a>&lt;T, R&gt;","synthetic":false,"types":["spin::once::Once"]},{"text":"impl&lt;T:&nbsp;?Sized + Send + Sync, R&gt; Sync for <a class=\"struct\" href=\"spin/rwlock/struct.RwLock.html\" title=\"struct spin::rwlock::RwLock\">RwLock</a>&lt;T, R&gt;","synthetic":false,"types":["spin::rwlock::RwLock"]}];
implementors["std3"] = [];
implementors["uart_16550"] = [{"text":"impl Sync for <a class=\"struct\" href=\"uart_16550/struct.MmioSerialPort.html\" title=\"struct uart_16550::MmioSerialPort\">MmioSerialPort</a>","synthetic":true,"types":["uart_16550::mmio::MmioSerialPort"]},{"text":"impl Sync for <a class=\"struct\" href=\"uart_16550/struct.SerialPort.html\" title=\"struct uart_16550::SerialPort\">SerialPort</a>","synthetic":true,"types":["uart_16550::port::SerialPort"]}];
implementors["vga_buffer"] = [{"text":"impl Sync for <a class=\"enum\" href=\"vga_buffer/enum.Color.html\" title=\"enum vga_buffer::Color\">Color</a>","synthetic":true,"types":["vga_buffer::writers::Color"]},{"text":"impl Sync for <a class=\"struct\" href=\"vga_buffer/struct.Writer.html\" title=\"struct vga_buffer::Writer\">Writer</a>","synthetic":true,"types":["vga_buffer::writers::Writer"]}];
implementors["volatile"] = [{"text":"impl Sync for <a class=\"struct\" href=\"volatile/access/struct.ReadWrite.html\" title=\"struct volatile::access::ReadWrite\">ReadWrite</a>","synthetic":true,"types":["volatile::access::ReadWrite"]},{"text":"impl Sync for <a class=\"struct\" href=\"volatile/access/struct.ReadOnly.html\" title=\"struct volatile::access::ReadOnly\">ReadOnly</a>","synthetic":true,"types":["volatile::access::ReadOnly"]},{"text":"impl Sync for <a class=\"struct\" href=\"volatile/access/struct.WriteOnly.html\" title=\"struct volatile::access::WriteOnly\">WriteOnly</a>","synthetic":true,"types":["volatile::access::WriteOnly"]},{"text":"impl&lt;R, A&gt; Sync for <a class=\"struct\" href=\"volatile/struct.Volatile.html\" title=\"struct volatile::Volatile\">Volatile</a>&lt;R, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Sync,&nbsp;</span>","synthetic":true,"types":["volatile::Volatile"]}];
implementors["x86_64"] = [{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/addr/struct.VirtAddr.html\" title=\"struct x86_64::addr::VirtAddr\">VirtAddr</a>","synthetic":true,"types":["x86_64::addr::VirtAddr"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/addr/struct.PhysAddr.html\" title=\"struct x86_64::addr::PhysAddr\">PhysAddr</a>","synthetic":true,"types":["x86_64::addr::PhysAddr"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/addr/struct.VirtAddrNotValid.html\" title=\"struct x86_64::addr::VirtAddrNotValid\">VirtAddrNotValid</a>","synthetic":true,"types":["x86_64::addr::VirtAddrNotValid"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/addr/struct.PhysAddrNotValid.html\" title=\"struct x86_64::addr::PhysAddrNotValid\">PhysAddrNotValid</a>","synthetic":true,"types":["x86_64::addr::PhysAddrNotValid"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/instructions/port/struct.ReadOnlyAccess.html\" title=\"struct x86_64::instructions::port::ReadOnlyAccess\">ReadOnlyAccess</a>","synthetic":true,"types":["x86_64::instructions::port::ReadOnlyAccess"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/instructions/port/struct.WriteOnlyAccess.html\" title=\"struct x86_64::instructions::port::WriteOnlyAccess\">WriteOnlyAccess</a>","synthetic":true,"types":["x86_64::instructions::port::WriteOnlyAccess"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/instructions/port/struct.ReadWriteAccess.html\" title=\"struct x86_64::instructions::port::ReadWriteAccess\">ReadWriteAccess</a>","synthetic":true,"types":["x86_64::instructions::port::ReadWriteAccess"]},{"text":"impl&lt;T, A&gt; Sync for <a class=\"struct\" href=\"x86_64/instructions/port/struct.PortGeneric.html\" title=\"struct x86_64::instructions::port::PortGeneric\">PortGeneric</a>&lt;T, A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: Sync,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::instructions::port::PortGeneric"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/instructions/random/struct.RdRand.html\" title=\"struct x86_64::instructions::random::RdRand\">RdRand</a>","synthetic":true,"types":["x86_64::instructions::random::RdRand"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/instructions/tlb/enum.InvPicdCommand.html\" title=\"enum x86_64::instructions::tlb::InvPicdCommand\">InvPicdCommand</a>","synthetic":true,"types":["x86_64::instructions::tlb::InvPicdCommand"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/instructions/tlb/struct.Pcid.html\" title=\"struct x86_64::instructions::tlb::Pcid\">Pcid</a>","synthetic":true,"types":["x86_64::instructions::tlb::Pcid"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr0.html\" title=\"struct x86_64::registers::control::Cr0\">Cr0</a>","synthetic":true,"types":["x86_64::registers::control::Cr0"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr0Flags.html\" title=\"struct x86_64::registers::control::Cr0Flags\">Cr0Flags</a>","synthetic":true,"types":["x86_64::registers::control::Cr0Flags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr2.html\" title=\"struct x86_64::registers::control::Cr2\">Cr2</a>","synthetic":true,"types":["x86_64::registers::control::Cr2"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr3.html\" title=\"struct x86_64::registers::control::Cr3\">Cr3</a>","synthetic":true,"types":["x86_64::registers::control::Cr3"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr3Flags.html\" title=\"struct x86_64::registers::control::Cr3Flags\">Cr3Flags</a>","synthetic":true,"types":["x86_64::registers::control::Cr3Flags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr4.html\" title=\"struct x86_64::registers::control::Cr4\">Cr4</a>","synthetic":true,"types":["x86_64::registers::control::Cr4"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/control/struct.Cr4Flags.html\" title=\"struct x86_64::registers::control::Cr4Flags\">Cr4Flags</a>","synthetic":true,"types":["x86_64::registers::control::Cr4Flags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr0.html\" title=\"struct x86_64::registers::debug::Dr0\">Dr0</a>","synthetic":true,"types":["x86_64::registers::debug::Dr0"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr1.html\" title=\"struct x86_64::registers::debug::Dr1\">Dr1</a>","synthetic":true,"types":["x86_64::registers::debug::Dr1"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr2.html\" title=\"struct x86_64::registers::debug::Dr2\">Dr2</a>","synthetic":true,"types":["x86_64::registers::debug::Dr2"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr3.html\" title=\"struct x86_64::registers::debug::Dr3\">Dr3</a>","synthetic":true,"types":["x86_64::registers::debug::Dr3"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/registers/debug/enum.DebugAddressRegisterNumber.html\" title=\"enum x86_64::registers::debug::DebugAddressRegisterNumber\">DebugAddressRegisterNumber</a>","synthetic":true,"types":["x86_64::registers::debug::DebugAddressRegisterNumber"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr6.html\" title=\"struct x86_64::registers::debug::Dr6\">Dr6</a>","synthetic":true,"types":["x86_64::registers::debug::Dr6"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr6Flags.html\" title=\"struct x86_64::registers::debug::Dr6Flags\">Dr6Flags</a>","synthetic":true,"types":["x86_64::registers::debug::Dr6Flags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr7Flags.html\" title=\"struct x86_64::registers::debug::Dr7Flags\">Dr7Flags</a>","synthetic":true,"types":["x86_64::registers::debug::Dr7Flags"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/registers/debug/enum.BreakpointCondition.html\" title=\"enum x86_64::registers::debug::BreakpointCondition\">BreakpointCondition</a>","synthetic":true,"types":["x86_64::registers::debug::BreakpointCondition"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/registers/debug/enum.BreakpointSize.html\" title=\"enum x86_64::registers::debug::BreakpointSize\">BreakpointSize</a>","synthetic":true,"types":["x86_64::registers::debug::BreakpointSize"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr7Value.html\" title=\"struct x86_64::registers::debug::Dr7Value\">Dr7Value</a>","synthetic":true,"types":["x86_64::registers::debug::Dr7Value"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/debug/struct.Dr7.html\" title=\"struct x86_64::registers::debug::Dr7\">Dr7</a>","synthetic":true,"types":["x86_64::registers::debug::Dr7"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.Msr.html\" title=\"struct x86_64::registers::model_specific::Msr\">Msr</a>","synthetic":true,"types":["x86_64::registers::model_specific::Msr"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.Efer.html\" title=\"struct x86_64::registers::model_specific::Efer\">Efer</a>","synthetic":true,"types":["x86_64::registers::model_specific::Efer"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.FsBase.html\" title=\"struct x86_64::registers::model_specific::FsBase\">FsBase</a>","synthetic":true,"types":["x86_64::registers::model_specific::FsBase"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.GsBase.html\" title=\"struct x86_64::registers::model_specific::GsBase\">GsBase</a>","synthetic":true,"types":["x86_64::registers::model_specific::GsBase"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.KernelGsBase.html\" title=\"struct x86_64::registers::model_specific::KernelGsBase\">KernelGsBase</a>","synthetic":true,"types":["x86_64::registers::model_specific::KernelGsBase"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.Star.html\" title=\"struct x86_64::registers::model_specific::Star\">Star</a>","synthetic":true,"types":["x86_64::registers::model_specific::Star"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.LStar.html\" title=\"struct x86_64::registers::model_specific::LStar\">LStar</a>","synthetic":true,"types":["x86_64::registers::model_specific::LStar"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.SFMask.html\" title=\"struct x86_64::registers::model_specific::SFMask\">SFMask</a>","synthetic":true,"types":["x86_64::registers::model_specific::SFMask"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.UCet.html\" title=\"struct x86_64::registers::model_specific::UCet\">UCet</a>","synthetic":true,"types":["x86_64::registers::model_specific::UCet"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.SCet.html\" title=\"struct x86_64::registers::model_specific::SCet\">SCet</a>","synthetic":true,"types":["x86_64::registers::model_specific::SCet"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.EferFlags.html\" title=\"struct x86_64::registers::model_specific::EferFlags\">EferFlags</a>","synthetic":true,"types":["x86_64::registers::model_specific::EferFlags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/model_specific/struct.CetFlags.html\" title=\"struct x86_64::registers::model_specific::CetFlags\">CetFlags</a>","synthetic":true,"types":["x86_64::registers::model_specific::CetFlags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/mxcsr/struct.MxCsr.html\" title=\"struct x86_64::registers::mxcsr::MxCsr\">MxCsr</a>","synthetic":true,"types":["x86_64::registers::mxcsr::MxCsr"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/rflags/struct.RFlags.html\" title=\"struct x86_64::registers::rflags::RFlags\">RFlags</a>","synthetic":true,"types":["x86_64::registers::rflags::RFlags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.SegmentSelector.html\" title=\"struct x86_64::registers::segmentation::SegmentSelector\">SegmentSelector</a>","synthetic":true,"types":["x86_64::registers::segmentation::SegmentSelector"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.CS.html\" title=\"struct x86_64::registers::segmentation::CS\">CS</a>","synthetic":true,"types":["x86_64::registers::segmentation::CS"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.SS.html\" title=\"struct x86_64::registers::segmentation::SS\">SS</a>","synthetic":true,"types":["x86_64::registers::segmentation::SS"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.DS.html\" title=\"struct x86_64::registers::segmentation::DS\">DS</a>","synthetic":true,"types":["x86_64::registers::segmentation::DS"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.ES.html\" title=\"struct x86_64::registers::segmentation::ES\">ES</a>","synthetic":true,"types":["x86_64::registers::segmentation::ES"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.FS.html\" title=\"struct x86_64::registers::segmentation::FS\">FS</a>","synthetic":true,"types":["x86_64::registers::segmentation::FS"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/segmentation/struct.GS.html\" title=\"struct x86_64::registers::segmentation::GS\">GS</a>","synthetic":true,"types":["x86_64::registers::segmentation::GS"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/xcontrol/struct.XCr0.html\" title=\"struct x86_64::registers::xcontrol::XCr0\">XCr0</a>","synthetic":true,"types":["x86_64::registers::xcontrol::XCr0"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/registers/xcontrol/struct.XCr0Flags.html\" title=\"struct x86_64::registers::xcontrol::XCr0Flags\">XCr0Flags</a>","synthetic":true,"types":["x86_64::registers::xcontrol::XCr0Flags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/gdt/struct.GlobalDescriptorTable.html\" title=\"struct x86_64::structures::gdt::GlobalDescriptorTable\">GlobalDescriptorTable</a>","synthetic":true,"types":["x86_64::structures::gdt::GlobalDescriptorTable"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/gdt/enum.Descriptor.html\" title=\"enum x86_64::structures::gdt::Descriptor\">Descriptor</a>","synthetic":true,"types":["x86_64::structures::gdt::Descriptor"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/gdt/struct.DescriptorFlags.html\" title=\"struct x86_64::structures::gdt::DescriptorFlags\">DescriptorFlags</a>","synthetic":true,"types":["x86_64::structures::gdt::DescriptorFlags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.InterruptDescriptorTable.html\" title=\"struct x86_64::structures::idt::InterruptDescriptorTable\">InterruptDescriptorTable</a>","synthetic":true,"types":["x86_64::structures::idt::InterruptDescriptorTable"]},{"text":"impl&lt;F&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.Entry.html\" title=\"struct x86_64::structures::idt::Entry\">Entry</a>&lt;F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::idt::Entry"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.EntryOptions.html\" title=\"struct x86_64::structures::idt::EntryOptions\">EntryOptions</a>","synthetic":true,"types":["x86_64::structures::idt::EntryOptions"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.InterruptStackFrame.html\" title=\"struct x86_64::structures::idt::InterruptStackFrame\">InterruptStackFrame</a>","synthetic":true,"types":["x86_64::structures::idt::InterruptStackFrame"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.InterruptStackFrameValue.html\" title=\"struct x86_64::structures::idt::InterruptStackFrameValue\">InterruptStackFrameValue</a>","synthetic":true,"types":["x86_64::structures::idt::InterruptStackFrameValue"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.PageFaultErrorCode.html\" title=\"struct x86_64::structures::idt::PageFaultErrorCode\">PageFaultErrorCode</a>","synthetic":true,"types":["x86_64::structures::idt::PageFaultErrorCode"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/idt/struct.SelectorErrorCode.html\" title=\"struct x86_64::structures::idt::SelectorErrorCode\">SelectorErrorCode</a>","synthetic":true,"types":["x86_64::structures::idt::SelectorErrorCode"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/idt/enum.DescriptorTable.html\" title=\"enum x86_64::structures::idt::DescriptorTable\">DescriptorTable</a>","synthetic":true,"types":["x86_64::structures::idt::DescriptorTable"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/idt/enum.ExceptionVector.html\" title=\"enum x86_64::structures::idt::ExceptionVector\">ExceptionVector</a>","synthetic":true,"types":["x86_64::structures::idt::ExceptionVector"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/frame/struct.PhysFrame.html\" title=\"struct x86_64::structures::paging::frame::PhysFrame\">PhysFrame</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::frame::PhysFrame"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/frame/struct.PhysFrameRange.html\" title=\"struct x86_64::structures::paging::frame::PhysFrameRange\">PhysFrameRange</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::frame::PhysFrameRange"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/frame/struct.PhysFrameRangeInclusive.html\" title=\"struct x86_64::structures::paging::frame::PhysFrameRangeInclusive\">PhysFrameRangeInclusive</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::frame::PhysFrameRangeInclusive"]},{"text":"impl&lt;'a, P&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/mapper/struct.MappedPageTable.html\" title=\"struct x86_64::structures::paging::mapper::MappedPageTable\">MappedPageTable</a>&lt;'a, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::mapper::mapped_page_table::MappedPageTable"]},{"text":"impl&lt;'a&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/mapper/struct.OffsetPageTable.html\" title=\"struct x86_64::structures::paging::mapper::OffsetPageTable\">OffsetPageTable</a>&lt;'a&gt;","synthetic":true,"types":["x86_64::structures::paging::mapper::offset_page_table::OffsetPageTable"]},{"text":"impl&lt;'a&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/mapper/struct.RecursivePageTable.html\" title=\"struct x86_64::structures::paging::mapper::RecursivePageTable\">RecursivePageTable</a>&lt;'a&gt;","synthetic":true,"types":["x86_64::structures::paging::mapper::recursive_page_table::RecursivePageTable"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.InvalidPageTable.html\" title=\"enum x86_64::structures::paging::mapper::InvalidPageTable\">InvalidPageTable</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::recursive_page_table::InvalidPageTable"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.TranslateResult.html\" title=\"enum x86_64::structures::paging::mapper::TranslateResult\">TranslateResult</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::TranslateResult"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.MappedFrame.html\" title=\"enum x86_64::structures::paging::mapper::MappedFrame\">MappedFrame</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::MappedFrame"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/mapper/struct.MapperFlush.html\" title=\"struct x86_64::structures::paging::mapper::MapperFlush\">MapperFlush</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::mapper::MapperFlush"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/mapper/struct.MapperFlushAll.html\" title=\"struct x86_64::structures::paging::mapper::MapperFlushAll\">MapperFlushAll</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::MapperFlushAll"]},{"text":"impl&lt;S&gt; Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.MapToError.html\" title=\"enum x86_64::structures::paging::mapper::MapToError\">MapToError</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::mapper::MapToError"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.UnmapError.html\" title=\"enum x86_64::structures::paging::mapper::UnmapError\">UnmapError</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::UnmapError"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.FlagUpdateError.html\" title=\"enum x86_64::structures::paging::mapper::FlagUpdateError\">FlagUpdateError</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::FlagUpdateError"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/mapper/enum.TranslateError.html\" title=\"enum x86_64::structures::paging::mapper::TranslateError\">TranslateError</a>","synthetic":true,"types":["x86_64::structures::paging::mapper::TranslateError"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/page/enum.Size4KiB.html\" title=\"enum x86_64::structures::paging::page::Size4KiB\">Size4KiB</a>","synthetic":true,"types":["x86_64::structures::paging::page::Size4KiB"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/page/enum.Size2MiB.html\" title=\"enum x86_64::structures::paging::page::Size2MiB\">Size2MiB</a>","synthetic":true,"types":["x86_64::structures::paging::page::Size2MiB"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/page/enum.Size1GiB.html\" title=\"enum x86_64::structures::paging::page::Size1GiB\">Size1GiB</a>","synthetic":true,"types":["x86_64::structures::paging::page::Size1GiB"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page/struct.Page.html\" title=\"struct x86_64::structures::paging::page::Page\">Page</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::page::Page"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page/struct.PageRange.html\" title=\"struct x86_64::structures::paging::page::PageRange\">PageRange</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::page::PageRange"]},{"text":"impl&lt;S&gt; Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page/struct.PageRangeInclusive.html\" title=\"struct x86_64::structures::paging::page::PageRangeInclusive\">PageRangeInclusive</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Sync,&nbsp;</span>","synthetic":true,"types":["x86_64::structures::paging::page::PageRangeInclusive"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page/struct.AddressNotAligned.html\" title=\"struct x86_64::structures::paging::page::AddressNotAligned\">AddressNotAligned</a>","synthetic":true,"types":["x86_64::structures::paging::page::AddressNotAligned"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/page_table/enum.FrameError.html\" title=\"enum x86_64::structures::paging::page_table::FrameError\">FrameError</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::FrameError"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page_table/struct.PageTableEntry.html\" title=\"struct x86_64::structures::paging::page_table::PageTableEntry\">PageTableEntry</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageTableEntry"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page_table/struct.PageTableFlags.html\" title=\"struct x86_64::structures::paging::page_table::PageTableFlags\">PageTableFlags</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageTableFlags"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page_table/struct.PageTable.html\" title=\"struct x86_64::structures::paging::page_table::PageTable\">PageTable</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageTable"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page_table/struct.PageTableIndex.html\" title=\"struct x86_64::structures::paging::page_table::PageTableIndex\">PageTableIndex</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageTableIndex"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/paging/page_table/struct.PageOffset.html\" title=\"struct x86_64::structures::paging::page_table::PageOffset\">PageOffset</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageOffset"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/structures/paging/page_table/enum.PageTableLevel.html\" title=\"enum x86_64::structures::paging::page_table::PageTableLevel\">PageTableLevel</a>","synthetic":true,"types":["x86_64::structures::paging::page_table::PageTableLevel"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/tss/struct.TaskStateSegment.html\" title=\"struct x86_64::structures::tss::TaskStateSegment\">TaskStateSegment</a>","synthetic":true,"types":["x86_64::structures::tss::TaskStateSegment"]},{"text":"impl Sync for <a class=\"struct\" href=\"x86_64/structures/struct.DescriptorTablePointer.html\" title=\"struct x86_64::structures::DescriptorTablePointer\">DescriptorTablePointer</a>","synthetic":true,"types":["x86_64::structures::DescriptorTablePointer"]},{"text":"impl Sync for <a class=\"enum\" href=\"x86_64/enum.PrivilegeLevel.html\" title=\"enum x86_64::PrivilegeLevel\">PrivilegeLevel</a>","synthetic":true,"types":["x86_64::PrivilegeLevel"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()