	.text
	.file	"MAKEODD_RUST.83b8fc49-cgu.0"
	.section	".text._ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE,@function
_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE:
	.cfi_startproc
	movabsq	$9147559743429524724, %rax
	retq
.Lfunc_end0:
	.size	_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE, .Lfunc_end0-_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE
	.cfi_endproc

	.section	.text._ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E,@function
_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rax
	movq	8(%rdi), %rsi
	movq	16(%rdi), %rdx
	movq	%rax, %rdi
	callq	_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E
	ud2
.Lfunc_end1:
	.size	_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E, .Lfunc_end1-_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E
	.cfi_endproc

	.section	.text._ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE,@function
_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	callq	*%rdi
	#APP
	#NO_APP
	popq	%rax
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end2:
	.size	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE, .Lfunc_end2-_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE
	.cfi_endproc

	.section	.text._ZN3std2rt10lang_start17hb57675e5e366c7bbE,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17hb57675e5e366c7bbE
	.globl	_ZN3std2rt10lang_start17hb57675e5e366c7bbE
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start17hb57675e5e366c7bbE,@function
_ZN3std2rt10lang_start17hb57675e5e366c7bbE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rdx, %rcx
	movq	%rsi, %rdx
	movq	%rdi, (%rsp)
	leaq	.L__unnamed_1(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17hc4dd8cd3ec4518c2E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end3:
	.size	_ZN3std2rt10lang_start17hb57675e5e366c7bbE, .Lfunc_end3-_ZN3std2rt10lang_start17hb57675e5e366c7bbE
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE, .Lfunc_end4-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE
	.cfi_endproc

	.section	.text.unlikely._ZN3std9panicking11begin_panic17h36ade0ca250eea5fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std9panicking11begin_panic17h36ade0ca250eea5fE,@function
_ZN3std9panicking11begin_panic17h36ade0ca250eea5fE:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	leaq	.L__unnamed_2(%rip), %rdi
	callq	*_ZN4core5panic8location8Location6caller17h0be74926b1dd1461E@GOTPCREL(%rip)
	leaq	.L__unnamed_3(%rip), %rcx
	movq	%rcx, (%rsp)
	movq	$34, 8(%rsp)
	movq	%rax, 16(%rsp)
	movq	%rsp, %rdi
	callq	_ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hc32dfd0573e5e780E
	ud2
.Lfunc_end5:
	.size	_ZN3std9panicking11begin_panic17h36ade0ca250eea5fE, .Lfunc_end5-_ZN3std9panicking11begin_panic17h36ade0ca250eea5fE
	.cfi_endproc

	.section	".text._ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E,@function
_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdx, %rcx
	movq	%rdi, 8(%rsp)
	movq	%rsi, 16(%rsp)
	leaq	.L__unnamed_4(%rip), %rsi
	leaq	8(%rsp), %rdi
	xorl	%edx, %edx
	callq	*_ZN3std9panicking20rust_panic_with_hook17ha677a669fb275654E@GOTPCREL(%rip)
	ud2
.Lfunc_end6:
	.size	_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E, .Lfunc_end6-_ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17h233b07da7c66edc4E
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rsi, %rbx
	movq	(%rdi), %r14
	movq	%rsi, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_lower_hex17hf5c481e4db26ca7fE@GOTPCREL(%rip)
	testb	%al, %al
	je	.LBB7_1
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$i128$GT$3fmt17h90c54e4d7d2c9b9cE@GOTPCREL(%rip)
.LBB7_1:
	.cfi_def_cfa_offset 32
	movq	%rbx, %rdi
	callq	*_ZN4core3fmt9Formatter15debug_upper_hex17h2f36c694673318faE@GOTPCREL(%rip)
	movq	%r14, %rdi
	movq	%rbx, %rsi
	addq	$8, %rsp
	testb	%al, %al
	je	.LBB7_4
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num54_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$i128$GT$3fmt17h9a9068ae3a19904bE@GOTPCREL(%rip)
.LBB7_4:
	.cfi_def_cfa_offset 32
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*_ZN4core3fmt3num53_$LT$impl$u20$core..fmt..Display$u20$for$u20$i128$GT$3fmt17hf71ad2866088d9ecE@GOTPCREL(%rip)
.Lfunc_end7:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE, .Lfunc_end7-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE
	.cfi_endproc

	.section	".text._ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE,@function
_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE:
	.cfi_startproc
	movq	%rsi, %rdx
	movq	(%rdi), %rax
	movq	8(%rdi), %rsi
	movq	%rax, %rdi
	jmpq	*_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h47f03611ff5c17afE@GOTPCREL(%rip)
.Lfunc_end8:
	.size	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE, .Lfunc_end8-_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	(%rdi), %rdi
	callq	_ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17h56350748e0a933abE
	xorl	%eax, %eax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end9:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E, .Lfunc_end9-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E
	.cfi_endproc

	.section	".text._ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E,@function
_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E:
	.cfi_startproc
	retq
.Lfunc_end10:
	.size	_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E, .Lfunc_end10-_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E
	.cfi_endproc

	.section	".text._ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E,@function
_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception0
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	cmpb	$3, (%rdi)
	jne	.LBB11_5
	movq	%rdi, %r15
	movq	8(%rdi), %rbx
	movq	(%rbx), %rdi
	movq	8(%rbx), %rax
.Ltmp0:
	callq	*(%rax)
.Ltmp1:
	movq	8(%rbx), %rax
	movq	8(%rax), %rsi
	testq	%rsi, %rsi
	je	.LBB11_4
	movq	(%rbx), %rdi
	movq	16(%rax), %rdx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB11_4:
	movq	8(%r15), %rdi
	movl	$24, %esi
	movl	$8, %edx
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB11_5:
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB11_6:
	.cfi_def_cfa_offset 32
.Ltmp2:
	movq	%rax, %r14
	movq	(%rbx), %rdi
	movq	8(%rbx), %rsi
	callq	_ZN5alloc5alloc8box_free17hf6e3599485ec0caaE
	movq	8(%r15), %rdi
	callq	_ZN5alloc5alloc8box_free17h2133c8013c6e75a6E
	movq	%r14, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end11:
	.size	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E, .Lfunc_end11-_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E","a",@progbits
	.p2align	2
GCC_except_table11:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp1-.Ltmp0
	.uleb128 .Ltmp2-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp1-.Lfunc_begin0
	.uleb128 .Lfunc_end11-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2

	.section	".text._ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E,@function
_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E:
	.cfi_startproc
	movq	8(%rdi), %rsi
	testq	%rsi, %rsi
	je	.LBB12_2
	movq	(%rdi), %rdi
	testq	%rdi, %rdi
	je	.LBB12_2
	movl	$1, %edx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB12_2:
	retq
.Lfunc_end12:
	.size	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E, .Lfunc_end12-_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E
	.cfi_endproc

	.section	".text._ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE,@function
_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE:
	.cfi_startproc
	movq	8(%rdi), %rsi
	testq	%rsi, %rsi
	je	.LBB13_3
	movq	(%rdi), %rdi
	testq	%rdi, %rdi
	je	.LBB13_3
	shlq	$3, %rsi
	testq	%rsi, %rsi
	je	.LBB13_3
	movl	$8, %edx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB13_3:
	retq
.Lfunc_end13:
	.size	_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE, .Lfunc_end13-_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE
	.cfi_endproc

	.section	".text._ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E,@function
_ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	movl	40(%rdi), %edi
.Ltmp3:
	callq	*close@GOTPCREL(%rip)
.Ltmp4:
	movq	8(%rbx), %rsi
	testq	%rsi, %rsi
	je	.LBB14_3
	movq	(%rbx), %rdi
	testq	%rdi, %rdi
	je	.LBB14_3
	movl	$1, %edx
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB14_3:
	.cfi_def_cfa_offset 32
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB14_4:
	.cfi_def_cfa_offset 32
.Ltmp5:
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E
	movq	%r14, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end14:
	.size	_ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E, .Lfunc_end14-_ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E","a",@progbits
	.p2align	2
GCC_except_table14:
.Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Ltmp3-.Lfunc_begin1
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp4-.Lfunc_begin1
	.uleb128 .Lfunc_end14-.Ltmp4
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2

	.section	.text._ZN5alloc5alloc8box_free17h2133c8013c6e75a6E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc8box_free17h2133c8013c6e75a6E,@function
_ZN5alloc5alloc8box_free17h2133c8013c6e75a6E:
	.cfi_startproc
	movl	$24, %esi
	movl	$8, %edx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.Lfunc_end15:
	.size	_ZN5alloc5alloc8box_free17h2133c8013c6e75a6E, .Lfunc_end15-_ZN5alloc5alloc8box_free17h2133c8013c6e75a6E
	.cfi_endproc

	.section	.text._ZN5alloc5alloc8box_free17hf6e3599485ec0caaE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc8box_free17hf6e3599485ec0caaE,@function
_ZN5alloc5alloc8box_free17hf6e3599485ec0caaE:
	.cfi_startproc
	movq	%rsi, %rax
	movq	8(%rsi), %rsi
	testq	%rsi, %rsi
	je	.LBB16_1
	movq	16(%rax), %rdx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB16_1:
	retq
.Lfunc_end16:
	.size	_ZN5alloc5alloc8box_free17hf6e3599485ec0caaE, .Lfunc_end16-_ZN5alloc5alloc8box_free17hf6e3599485ec0caaE
	.cfi_endproc

	.section	.text._ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE,@function
_ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rsi, %rbx
	movq	%rdi, %r14
	testq	%rdx, %rdx
	je	.LBB17_5
	movq	%rdx, %r15
	movq	(%rcx), %rdi
	testq	%rdi, %rdi
	je	.LBB17_6
	movq	8(%rcx), %rsi
	testq	%rsi, %rsi
	je	.LBB17_6
	movq	%r15, %rdx
	movq	%rbx, %rcx
	callq	*__rust_realloc@GOTPCREL(%rip)
	testq	%rax, %rax
	jne	.LBB17_10
.LBB17_4:
	movq	%rbx, 8(%r14)
	movl	$1, %eax
	movq	%r15, %rbx
	jmp	.LBB17_11
.LBB17_6:
	testq	%rbx, %rbx
	je	.LBB17_7
	movq	%rbx, %rdi
	movq	%r15, %rsi
	callq	*__rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB17_4
.LBB17_10:
	movq	%rax, 8(%r14)
	xorl	%eax, %eax
	jmp	.LBB17_11
.LBB17_5:
	movq	%rbx, 8(%r14)
	movl	$1, %eax
	xorl	%ebx, %ebx
.LBB17_11:
	movq	%rbx, 16(%r14)
	movq	%rax, (%r14)
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB17_7:
	.cfi_def_cfa_offset 32
	movq	%r15, %rax
	testq	%rax, %rax
	jne	.LBB17_10
	jmp	.LBB17_4
.Lfunc_end17:
	.size	_ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE, .Lfunc_end17-_ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE
	.cfi_endproc

	.section	".text.unlikely._ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E,@function
_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	subq	$56, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	incq	%rsi
	je	.LBB18_8
	movq	%rdi, %r14
	movq	8(%rdi), %rcx
	leaq	(%rcx,%rcx), %rax
	cmpq	%rsi, %rax
	cmovaq	%rax, %rsi
	cmpq	$5, %rsi
	movl	$4, %eax
	cmovaeq	%rsi, %rax
	movl	$8, %edx
	xorl	%ebx, %ebx
	mulq	%rdx
	setno	%bl
	shlq	$3, %rbx
	testq	%rcx, %rcx
	je	.LBB18_3
	movq	(%r14), %rdx
	shlq	$3, %rcx
	movq	%rdx, 32(%rsp)
	movq	%rcx, 40(%rsp)
	movq	$8, 48(%rsp)
	jmp	.LBB18_4
.LBB18_3:
	movq	$0, 32(%rsp)
.LBB18_4:
	leaq	8(%rsp), %rdi
	leaq	32(%rsp), %rcx
	movq	%rax, %rsi
	movq	%rbx, %rdx
	callq	_ZN5alloc7raw_vec11finish_grow17h373885933d5dd92fE
	cmpl	$1, 8(%rsp)
	je	.LBB18_5
	movq	16(%rsp), %rax
	movq	24(%rsp), %rcx
	movq	%rax, (%r14)
	shrq	$3, %rcx
	movq	%rcx, 8(%r14)
	addq	$56, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB18_5:
	.cfi_def_cfa_offset 80
	movq	24(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB18_6
.LBB18_8:
	callq	*_ZN5alloc7raw_vec17capacity_overflow17h4b3e814645d8e64dE@GOTPCREL(%rip)
	ud2
.LBB18_6:
	movq	16(%rsp), %rdi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.Lfunc_end18:
	.size	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E, .Lfunc_end18-_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E
	.cfi_endproc

	.section	".text._ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE,@function
_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$48, %rsp
	.cfi_def_cfa_offset 96
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	%rdi, %rbx
	leaq	40(%rdi), %r14
	movq	24(%rdi), %r8
	movq	32(%rdi), %rdi
	leaq	8(%rsp), %r15
	movq	_ZN47_$LT$std..fs..File$u20$as$u20$std..io..Read$GT$4read17h93627641821ed849E@GOTPCREL(%rip), %r13
	cmpq	%r8, %rdi
	jb	.LBB19_9
	.p2align	4, 0x90
.LBB19_2:
	movq	(%rbx), %rdx
	movq	16(%rbx), %rcx
	movq	%r15, %rdi
	movq	%r14, %rsi
	callq	*%r13
	cmpl	$1, 8(%rsp)
	je	.LBB19_3
	movq	16(%rsp), %r8
	movq	%r8, 24(%rbx)
	movq	$0, 32(%rbx)
	testq	%r8, %r8
	je	.LBB19_24
	xorl	%edi, %edi
.LBB19_9:
	movq	16(%rbx), %rcx
	cmpq	%rcx, %rdi
	jae	.LBB19_22
	movq	(%rbx), %rdx
	movzbl	(%rdx,%rdi), %edx
	addl	$-48, %edx
	cmpl	$9, %edx
	jbe	.LBB19_11
	addq	$1, %rdi
	movq	%rdi, 32(%rbx)
	cmpq	%r8, %rdi
	jae	.LBB19_2
	jmp	.LBB19_9
.LBB19_11:
	cmpq	%rcx, %rdi
	jae	.LBB19_22
	xorl	%r15d, %r15d
	leaq	8(%rsp), %r12
	jmp	.LBB19_13
	.p2align	4, 0x90
.LBB19_20:
	xorl	%edi, %edi
.LBB19_21:
	movq	16(%rbx), %rcx
	cmpq	%rcx, %rdi
	jae	.LBB19_22
.LBB19_13:
	movq	(%rbx), %rdx
	movzbl	(%rdx,%rdi), %esi
	leal	-48(%rsi), %eax
	cmpl	$9, %eax
	ja	.LBB19_23
	leaq	(%r15,%r15,4), %rax
	addq	$1, %rdi
	movq	%rdi, 32(%rbx)
	leaq	(%rsi,%rax,2), %r15
	addq	$-48, %r15
	cmpq	%r8, %rdi
	jb	.LBB19_21
	movq	%r12, %rdi
	movq	%r14, %rsi
	callq	*%r13
	cmpl	$1, 8(%rsp)
	je	.LBB19_16
	movq	16(%rsp), %r8
	movq	%r8, 24(%rbx)
	movq	$0, 32(%rbx)
	testq	%r8, %r8
	jne	.LBB19_20
.LBB19_23:
	movq	%r15, %rax
	addq	$48, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB19_22:
	.cfi_def_cfa_offset 96
	leaq	.L__unnamed_5(%rip), %rdx
	movq	%rcx, %rsi
	callq	*_ZN4core9panicking18panic_bounds_check17h00fd50079cb70ed9E@GOTPCREL(%rip)
	ud2
.LBB19_3:
	movups	16(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
.Ltmp6:
	leaq	.L__unnamed_6(%rip), %rdi
	leaq	.L__unnamed_7(%rip), %rcx
	leaq	.L__unnamed_8(%rip), %r8
	leaq	32(%rsp), %rbx
	movl	$43, %esi
	movq	%rbx, %rdx
	callq	*_ZN4core6result13unwrap_failed17hb53671404b9e33c2E@GOTPCREL(%rip)
.Ltmp7:
	jmp	.LBB19_4
.LBB19_24:
	callq	_ZN3std9panicking11begin_panic17h36ade0ca250eea5fE
	ud2
.LBB19_16:
	movups	16(%rsp), %xmm0
	movaps	%xmm0, 32(%rsp)
.Ltmp9:
	leaq	.L__unnamed_6(%rip), %rdi
	leaq	.L__unnamed_7(%rip), %rcx
	leaq	.L__unnamed_8(%rip), %r8
	leaq	32(%rsp), %rbx
	movl	$43, %esi
	movq	%rbx, %rdx
	callq	*_ZN4core6result13unwrap_failed17hb53671404b9e33c2E@GOTPCREL(%rip)
.Ltmp10:
.LBB19_4:
	ud2
.LBB19_18:
.Ltmp11:
	jmp	.LBB19_6
.LBB19_5:
.Ltmp8:
.LBB19_6:
	movq	%rax, %r14
	movq	%rbx, %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E
	movq	%r14, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end19:
	.size	_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE, .Lfunc_end19-_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE
	.cfi_endproc
	.section	".gcc_except_table._ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE","a",@progbits
	.p2align	2
GCC_except_table19:
.Lexception2:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end2-.Lcst_begin2
.Lcst_begin2:
	.uleb128 .Lfunc_begin2-.Lfunc_begin2
	.uleb128 .Ltmp6-.Lfunc_begin2
	.byte	0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin2
	.uleb128 .Ltmp7-.Ltmp6
	.uleb128 .Ltmp8-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp7-.Lfunc_begin2
	.uleb128 .Ltmp9-.Ltmp7
	.byte	0
	.byte	0
	.uleb128 .Ltmp9-.Lfunc_begin2
	.uleb128 .Ltmp10-.Ltmp9
	.uleb128 .Ltmp11-.Lfunc_begin2
	.byte	0
	.uleb128 .Ltmp10-.Lfunc_begin2
	.uleb128 .Lfunc_end19-.Ltmp10
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2

	.section	".text._ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE,@function
_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	cmpq	$0, (%rdi)
	je	.LBB20_1
	leaq	.L__unnamed_9(%rip), %rdx
	movq	%rdi, %rax
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.LBB20_1:
	.cfi_def_cfa_offset 16
	callq	*_ZN3std7process5abort17h28c80b83aca4de59E@GOTPCREL(%rip)
	ud2
.Lfunc_end20:
	.size	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE, .Lfunc_end20-_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE
	.cfi_endproc

	.section	".text._ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E,@function
_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	(%rdi), %rbx
	movq	8(%rdi), %r14
	movq	$0, (%rdi)
	testq	%rbx, %rbx
	je	.LBB21_3
	movl	$16, %edi
	movl	$8, %esi
	callq	*__rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB21_4
	movq	%rbx, (%rax)
	movq	%r14, 8(%rax)
	leaq	.L__unnamed_9(%rip), %rdx
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB21_3:
	.cfi_def_cfa_offset 32
	callq	*_ZN3std7process5abort17h28c80b83aca4de59E@GOTPCREL(%rip)
	ud2
.LBB21_4:
	movl	$16, %edi
	movl	$8, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.Lfunc_end21:
	.size	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E, .Lfunc_end21-_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E
	.cfi_endproc

	.section	.text._ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E,@function
_ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E:
.Lfunc_begin3:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception3
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	subq	$232, %rsp
	.cfi_def_cfa_offset 288
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	leaq	.L__unnamed_10(%rip), %rsi
	leaq	72(%rsp), %rbx
	movl	$9, %edx
	movq	%rbx, %rdi
	callq	*_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E@GOTPCREL(%rip)
.Ltmp12:
	movq	%rbx, %rdi
	callq	_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE
.Ltmp13:
	movq	%rax, %rbp
	testw	%bp, %bp
	je	.LBB22_26
	xorl	%ebx, %ebx
	movq	%rbp, 8(%rsp)
	jmp	.LBB22_4
	.p2align	4, 0x90
.LBB22_3:
	cmpw	%bp, %bx
	je	.LBB22_26
.LBB22_4:
.Ltmp15:
	leaq	72(%rsp), %rdi
	callq	_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE
.Ltmp16:
	movq	%rax, %rbp
	leaq	(,%rax,8), %r14
	movabsq	$34359738360, %rax
	andq	%rax, %r14
	je	.LBB22_7
	movl	$8, %esi
	movq	%r14, %rdi
	callq	*__rust_alloc@GOTPCREL(%rip)
	movq	%rax, %r12
	testq	%rax, %rax
	jne	.LBB22_8
	jmp	.LBB22_31
	.p2align	4, 0x90
.LBB22_7:
	movl	$8, %r12d
.LBB22_8:
	shrq	$3, %r14
	movq	%r12, 16(%rsp)
	movq	%r14, 24(%rsp)
	movq	$0, 32(%rsp)
	testl	%ebp, %ebp
	je	.LBB22_21
	movq	%rbx, 40(%rsp)
	xorl	%r14d, %r14d
	xorl	%r13d, %r13d
	leaq	72(%rsp), %rbx
	jmp	.LBB22_12
	.p2align	4, 0x90
.LBB22_10:
	movq	%r14, %rax
.LBB22_11:
	notl	%r15d
	andl	$1, %r15d
	addq	%r15, %r13
	movq	%r13, (%r12,%rax,8)
	leaq	1(%rax), %r14
	movq	%r14, 32(%rsp)
	addl	$-1, %ebp
	je	.LBB22_16
.LBB22_12:
.Ltmp18:
	movq	%rbx, %rdi
	callq	_ZN60_$LT$u64$u20$as$u20$easy_io..input_reader..InputReadable$GT$10from_input17haff4010bf8bfc70aE
.Ltmp19:
	movq	%rax, %r15
	cmpq	24(%rsp), %r14
	jne	.LBB22_10
.Ltmp20:
	leaq	16(%rsp), %rdi
	movq	%r14, %rsi
	callq	_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$7reserve21do_reserve_and_handle17h41107e7b1bdc4cc5E
.Ltmp21:
	movq	16(%rsp), %r12
	movq	32(%rsp), %rax
	jmp	.LBB22_11
	.p2align	4, 0x90
.LBB22_16:
	movq	16(%rsp), %r12
	testq	%r14, %r14
	je	.LBB22_22
	movl	$1, %r15d
	subq	%rax, %r15
	leaq	8(,%rax,8), %r14
	xorl	%r13d, %r13d
	xorl	%ebx, %ebx
	xorl	%ebp, %ebp
	.p2align	4, 0x90
.LBB22_18:
	movq	%rbp, %rax
	movq	%rbx, %rcx
	movq	(%r12,%r13), %rbx
	imulq	%r15, %rbx
	movq	%rbx, %rbp
	sarq	$63, %rbp
	addq	%rcx, %rbx
	adcq	%rax, %rbp
	movq	%rbx, 56(%rsp)
	movq	%rbp, 64(%rsp)
	leaq	56(%rsp), %rax
	movq	%rax, 48(%rsp)
	leaq	.L__unnamed_11(%rip), %rax
	movq	%rax, 168(%rsp)
	leaq	_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h1e9ba38442dbd92fE(%rip), %rax
	movq	%rax, 176(%rsp)
	leaq	.L__unnamed_12(%rip), %rcx
	movq	%rcx, 184(%rsp)
	movq	_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17hc9b8e4322a951e30E@GOTPCREL(%rip), %rcx
	movq	%rcx, 192(%rsp)
	leaq	.L__unnamed_13(%rip), %rcx
	movq	%rcx, 200(%rsp)
	movq	%rax, 208(%rsp)
	leaq	48(%rsp), %rax
	movq	%rax, 216(%rsp)
	leaq	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17he43a5ac815032f5dE(%rip), %rax
	movq	%rax, 224(%rsp)
	leaq	.L__unnamed_14(%rip), %rax
	movq	%rax, 120(%rsp)
	movq	$5, 128(%rsp)
	leaq	.L__unnamed_15(%rip), %rax
	movq	%rax, 136(%rsp)
	movq	$4, 144(%rsp)
	leaq	168(%rsp), %rax
	movq	%rax, 152(%rsp)
	movq	$4, 160(%rsp)
.Ltmp23:
	leaq	120(%rsp), %rdi
	callq	*_ZN3std2io5stdio7_eprint17h4d077c3ca706ec3fE@GOTPCREL(%rip)
.Ltmp24:
	addq	$2, %r15
	addq	$8, %r13
	cmpq	%r13, %r14
	jne	.LBB22_18
	movq	8(%rsp), %rbp
	movq	40(%rsp), %rbx
	addl	$1, %ebx
	movq	24(%rsp), %rsi
	testq	%rsi, %rsi
	jne	.LBB22_23
	jmp	.LBB22_3
	.p2align	4, 0x90
.LBB22_22:
	movq	40(%rsp), %rbx
.LBB22_21:
	movq	8(%rsp), %rbp
	addl	$1, %ebx
	movq	24(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB22_3
.LBB22_23:
	testq	%r12, %r12
	je	.LBB22_3
	shlq	$3, %rsi
	testq	%rsi, %rsi
	je	.LBB22_3
	movl	$8, %edx
	movq	%r12, %rdi
	callq	*__rust_dealloc@GOTPCREL(%rip)
	jmp	.LBB22_3
.LBB22_26:
	movl	112(%rsp), %edi
.Ltmp26:
	callq	*close@GOTPCREL(%rip)
.Ltmp27:
	movq	80(%rsp), %rsi
	testq	%rsi, %rsi
	je	.LBB22_30
	movq	72(%rsp), %rdi
	testq	%rdi, %rdi
	je	.LBB22_30
	movl	$1, %edx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB22_30:
	addq	$232, %rsp
	.cfi_def_cfa_offset 56
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB22_31:
	.cfi_def_cfa_offset 288
	movl	$8, %esi
	movq	%r14, %rdi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.LBB22_32:
.Ltmp28:
	movq	%rax, %rbx
	leaq	72(%rsp), %rdi
	callq	_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hb9e58c2a57f1d3d4E
	jmp	.LBB22_40
.LBB22_33:
.Ltmp14:
	jmp	.LBB22_35
.LBB22_34:
.Ltmp17:
.LBB22_35:
	movq	%rax, %rbx
	jmp	.LBB22_39
.LBB22_36:
.Ltmp25:
	jmp	.LBB22_38
.LBB22_37:
.Ltmp22:
.LBB22_38:
	movq	%rax, %rbx
	leaq	16(%rsp), %rdi
	callq	_ZN4core3ptr47drop_in_place$LT$alloc..vec..Vec$LT$u64$GT$$GT$17hdb675bb52832c8efE
.LBB22_39:
	leaq	72(%rsp), %rdi
	callq	_ZN4core3ptr76drop_in_place$LT$easy_io..input_reader..InputReader$LT$std..fs..File$GT$$GT$17h401190ddbf085572E
.LBB22_40:
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end22:
	.size	_ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E, .Lfunc_end22-_ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E
	.cfi_endproc
	.section	.gcc_except_table._ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E,"a",@progbits
	.p2align	2
GCC_except_table22:
.Lexception3:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end3-.Lcst_begin3
.Lcst_begin3:
	.uleb128 .Lfunc_begin3-.Lfunc_begin3
	.uleb128 .Ltmp12-.Lfunc_begin3
	.byte	0
	.byte	0
	.uleb128 .Ltmp12-.Lfunc_begin3
	.uleb128 .Ltmp13-.Ltmp12
	.uleb128 .Ltmp14-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp15-.Lfunc_begin3
	.uleb128 .Ltmp16-.Ltmp15
	.uleb128 .Ltmp17-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp18-.Lfunc_begin3
	.uleb128 .Ltmp21-.Ltmp18
	.uleb128 .Ltmp22-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp23-.Lfunc_begin3
	.uleb128 .Ltmp24-.Ltmp23
	.uleb128 .Ltmp25-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp26-.Lfunc_begin3
	.uleb128 .Ltmp27-.Ltmp26
	.uleb128 .Ltmp28-.Lfunc_begin3
	.byte	0
	.uleb128 .Ltmp27-.Lfunc_begin3
	.uleb128 .Lfunc_end22-.Ltmp27
	.byte	0
	.byte	0
.Lcst_end3:
	.p2align	2

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	pushq	%rax
	.cfi_def_cfa_offset 16
	movq	%rsi, %rcx
	movslq	%edi, %rdx
	leaq	_ZN12MAKEODD_RUST4main17h1bec8c0a8c701737E(%rip), %rax
	movq	%rax, (%rsp)
	leaq	.L__unnamed_1(%rip), %rsi
	movq	%rsp, %rdi
	callq	*_ZN3std2rt19lang_start_internal17hc4dd8cd3ec4518c2E@GOTPCREL(%rip)
	popq	%rcx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end23:
	.size	main, .Lfunc_end23-main
	.cfi_endproc

	.type	.L__unnamed_1,@object
	.section	.data.rel.ro..L__unnamed_1,"aw",@progbits
	.p2align	3
.L__unnamed_1:
	.quad	_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h9649afd2178b94d9E
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE
	.quad	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h64bf4b1d1a94825aE
	.size	.L__unnamed_1, 48

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	3
.L__unnamed_4:
	.quad	_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$8take_box17h7c9004cb8ede73a2E
	.quad	_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17hb244d7298eff661cE
	.size	.L__unnamed_4, 40

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.ascii	"called `Result::unwrap()` on an `Err` value"
	.size	.L__unnamed_6, 43

	.type	.L__unnamed_7,@object
	.section	.data.rel.ro..L__unnamed_7,"aw",@progbits
	.p2align	3
.L__unnamed_7:
	.quad	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17h462f5a6981b74a24E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h12528dc4af160724E
	.size	.L__unnamed_7, 32

	.type	.L__unnamed_3,@object
	.section	.rodata..L__unnamed_3,"a",@progbits
.L__unnamed_3:
	.ascii	"InputReader: Reached end of input!"
	.size	.L__unnamed_3, 34

	.type	.L__unnamed_16,@object
	.section	.rodata..L__unnamed_16,"a",@progbits
.L__unnamed_16:
	.ascii	"/home/sandstorm/.cargo/registry/src/github.com-1ecc6299db9ec823/easy_io-0.3.0/src/input_reader.rs"
	.size	.L__unnamed_16, 97

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	.L__unnamed_16
	.asciz	"a\000\000\000\000\000\000\000N\000\000\000\005\000\000"
	.size	.L__unnamed_2, 24

	.type	.L__unnamed_5,@object
	.section	.data.rel.ro..L__unnamed_5,"aw",@progbits
	.p2align	3
.L__unnamed_5:
	.quad	.L__unnamed_16
	.asciz	"a\000\000\000\000\000\000\000I\000\000\000\034\000\000"
	.size	.L__unnamed_5, 24

	.type	.L__unnamed_8,@object
	.section	.data.rel.ro..L__unnamed_8,"aw",@progbits
	.p2align	3
.L__unnamed_8:
	.quad	.L__unnamed_16
	.asciz	"a\000\000\000\000\000\000\000<\000\000\000=\000\000"
	.size	.L__unnamed_8, 24

	.type	.L__unnamed_9,@object
	.section	.data.rel.ro..L__unnamed_9,"aw",@progbits
	.p2align	3
.L__unnamed_9:
	.quad	_ZN4core3ptr28drop_in_place$LT$$RF$str$GT$17h8585d8fa57ca3745E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$7type_id17h309b23aa69096d0fE
	.size	.L__unnamed_9, 32

	.type	.L__unnamed_17,@object
	.section	.rodata..L__unnamed_17,"a",@progbits
.L__unnamed_17:
	.byte	91
	.size	.L__unnamed_17, 1

	.type	.L__unnamed_18,@object
	.section	.rodata..L__unnamed_18,"a",@progbits
.L__unnamed_18:
	.byte	58
	.size	.L__unnamed_18, 1

	.type	.L__unnamed_19,@object
	.section	.rodata..L__unnamed_19,"a",@progbits
.L__unnamed_19:
	.ascii	"] "
	.size	.L__unnamed_19, 2

	.type	.L__unnamed_20,@object
	.section	.rodata..L__unnamed_20,"a",@progbits
.L__unnamed_20:
	.ascii	" = "
	.size	.L__unnamed_20, 3

	.type	.L__unnamed_21,@object
	.section	.rodata..L__unnamed_21,"a",@progbits
.L__unnamed_21:
	.byte	10
	.size	.L__unnamed_21, 1

	.type	.L__unnamed_14,@object
	.section	.data.rel.ro..L__unnamed_14,"aw",@progbits
	.p2align	3
.L__unnamed_14:
	.quad	.L__unnamed_17
	.asciz	"\001\000\000\000\000\000\000"
	.quad	.L__unnamed_18
	.asciz	"\001\000\000\000\000\000\000"
	.quad	.L__unnamed_19
	.asciz	"\002\000\000\000\000\000\000"
	.quad	.L__unnamed_20
	.asciz	"\003\000\000\000\000\000\000"
	.quad	.L__unnamed_21
	.asciz	"\001\000\000\000\000\000\000"
	.size	.L__unnamed_14, 80

	.type	.L__unnamed_22,@object
	.section	.rodata..L__unnamed_22,"a",@progbits
.L__unnamed_22:
	.ascii	"src/main.rs"
	.size	.L__unnamed_22, 11

	.type	.L__unnamed_11,@object
	.section	.data.rel.ro..L__unnamed_11,"aw",@progbits
	.p2align	3
.L__unnamed_11:
	.quad	.L__unnamed_22
	.asciz	"\013\000\000\000\000\000\000"
	.size	.L__unnamed_11, 16

	.type	.L__unnamed_12,@object
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2
.L__unnamed_12:
	.asciz	"\016\000\000"
	.size	.L__unnamed_12, 4

	.type	.L__unnamed_23,@object
	.section	.rodata..L__unnamed_23,"a",@progbits
.L__unnamed_23:
	.ascii	"subarr_sum"
	.size	.L__unnamed_23, 10

	.type	.L__unnamed_13,@object
	.section	.data.rel.ro..L__unnamed_13,"aw",@progbits
	.p2align	3
.L__unnamed_13:
	.quad	.L__unnamed_23
	.asciz	"\n\000\000\000\000\000\000"
	.size	.L__unnamed_13, 16

	.type	.L__unnamed_15,@object
	.section	.rodata..L__unnamed_15,"a",@progbits
	.p2align	3
.L__unnamed_15:
	.asciz	"\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\001\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\003\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\002\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000 \000\000\000\004\000\000\000\003\000\000\000\000\000\000"
	.size	.L__unnamed_15, 224

	.type	.L__unnamed_10,@object
	.section	.rodata..L__unnamed_10,"a",@progbits
.L__unnamed_10:
	.ascii	"input.txt"
	.size	.L__unnamed_10, 9

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
