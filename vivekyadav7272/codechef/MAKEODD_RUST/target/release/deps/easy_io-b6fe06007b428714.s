	.text
	.file	"easy_io.b59764c2-cgu.0"
	.section	".text._ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E","ax",@progbits
	.p2align	4, 0x90
	.type	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E,@function
_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E:
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
	jne	.LBB0_5
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
	je	.LBB0_4
	movq	(%rbx), %rdi
	movq	16(%rax), %rdx
	callq	*__rust_dealloc@GOTPCREL(%rip)
.LBB0_4:
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
.LBB0_5:
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.LBB0_6:
	.cfi_def_cfa_offset 32
.Ltmp2:
	movq	%rax, %r14
	movq	(%rbx), %rdi
	movq	8(%rbx), %rsi
	callq	_ZN5alloc5alloc8box_free17hb920f6390cb26aedE
	movq	8(%r15), %rdi
	callq	_ZN5alloc5alloc8box_free17h1c89cee29d8ad012E
	movq	%r14, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end0:
	.size	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E, .Lfunc_end0-_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E
	.cfi_endproc
	.section	".gcc_except_table._ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E","a",@progbits
	.p2align	2
GCC_except_table0:
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
	.uleb128 .Lfunc_end0-.Ltmp1
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2

	.section	.text._ZN5alloc5alloc8box_free17h1c89cee29d8ad012E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc8box_free17h1c89cee29d8ad012E,@function
_ZN5alloc5alloc8box_free17h1c89cee29d8ad012E:
	.cfi_startproc
	movl	$24, %esi
	movl	$8, %edx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.Lfunc_end1:
	.size	_ZN5alloc5alloc8box_free17h1c89cee29d8ad012E, .Lfunc_end1-_ZN5alloc5alloc8box_free17h1c89cee29d8ad012E
	.cfi_endproc

	.section	.text._ZN5alloc5alloc8box_free17hb920f6390cb26aedE,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN5alloc5alloc8box_free17hb920f6390cb26aedE,@function
_ZN5alloc5alloc8box_free17hb920f6390cb26aedE:
	.cfi_startproc
	movq	%rsi, %rax
	movq	8(%rsi), %rsi
	testq	%rsi, %rsi
	je	.LBB2_1
	movq	16(%rax), %rdx
	jmpq	*__rust_dealloc@GOTPCREL(%rip)
.LBB2_1:
	retq
.Lfunc_end2:
	.size	_ZN5alloc5alloc8box_free17hb920f6390cb26aedE, .Lfunc_end2-_ZN5alloc5alloc8box_free17hb920f6390cb26aedE
	.cfi_endproc

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI3_0:
	.quad	65536
	.quad	65536
	.section	".text._ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE","ax",@progbits
	.globl	_ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE
	.p2align	4, 0x90
	.type	_ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE,@function
_ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	callq	*_ZN3std2io5stdio5stdin17h7ddc52b357ca7934E@GOTPCREL(%rip)
	movq	%rax, %r14
	movl	$65536, %edi
	movl	$1, %esi
	callq	*__rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB3_1
	movq	%r14, (%rbx)
	movq	%rax, 8(%rbx)
	movaps	.LCPI3_0(%rip), %xmm0
	movups	%xmm0, 16(%rbx)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 32(%rbx)
	movq	%rbx, %rax
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB3_1:
	.cfi_def_cfa_offset 32
	movl	$65536, %edi
	movl	$1, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.Lfunc_end3:
	.size	_ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE, .Lfunc_end3-_ZN7easy_io12input_reader40InputReader$LT$std..io..stdio..Stdin$GT$3new17h05fc51e7de0a997aE
	.cfi_endproc

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI4_0:
	.quad	65536
	.quad	65536
	.section	".text._ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E","ax",@progbits
	.globl	_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E
	.p2align	4, 0x90
	.type	_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E,@function
_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E:
.Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception1
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	subq	$40, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r14
	movq	%rsi, %r15
	movq	%rdi, %rbx
	callq	*_ZN3std2fs11OpenOptions3new17heead8c8bfaf953f2E@GOTPCREL(%rip)
	movq	%rdx, 8(%rsp)
	movq	%rax, (%rsp)
	movq	%rsp, %rdi
	movl	$1, %esi
	callq	*_ZN3std2fs11OpenOptions4read17h867266528b1c33f2E@GOTPCREL(%rip)
	leaq	16(%rsp), %rdi
	movq	%rax, %rsi
	movq	%r15, %rdx
	movq	%r14, %rcx
	callq	*_ZN3std2fs11OpenOptions5_open17h84f65b66e75ab0a3E@GOTPCREL(%rip)
	cmpl	$1, 16(%rsp)
	je	.LBB4_1
	movl	20(%rsp), %ebp
	movl	$65536, %edi
	movl	$1, %esi
	callq	*__rust_alloc_zeroed@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB4_5
	movl	%ebp, 40(%rbx)
	movq	%rax, (%rbx)
	movaps	.LCPI4_0(%rip), %xmm0
	movups	%xmm0, 8(%rbx)
	xorps	%xmm0, %xmm0
	movups	%xmm0, 24(%rbx)
	movq	%rbx, %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB4_1:
	.cfi_def_cfa_offset 80
	movups	24(%rsp), %xmm0
	movaps	%xmm0, (%rsp)
.Ltmp3:
	leaq	.L__unnamed_1(%rip), %rdi
	leaq	.L__unnamed_2(%rip), %rcx
	leaq	.L__unnamed_3(%rip), %r8
	movq	%rsp, %rdx
	movl	$43, %esi
	callq	*_ZN4core6result13unwrap_failed17hb53671404b9e33c2E@GOTPCREL(%rip)
.Ltmp4:
	ud2
.LBB4_5:
	movl	$65536, %edi
	movl	$1, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.LBB4_3:
.Ltmp5:
	movq	%rax, %rbx
	movq	%rsp, %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end4:
	.size	_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E, .Lfunc_end4-_ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E
	.cfi_endproc
	.section	".gcc_except_table._ZN7easy_io12input_reader32InputReader$LT$std..fs..File$GT$9from_file17h9636447b30bf5a21E","a",@progbits
	.p2align	2
GCC_except_table4:
.Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end1-.Lcst_begin1
.Lcst_begin1:
	.uleb128 .Lfunc_begin1-.Lfunc_begin1
	.uleb128 .Ltmp3-.Lfunc_begin1
	.byte	0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin1
	.uleb128 .Ltmp4-.Ltmp3
	.uleb128 .Ltmp5-.Lfunc_begin1
	.byte	0
	.uleb128 .Ltmp4-.Lfunc_begin1
	.uleb128 .Lfunc_end4-.Ltmp4
	.byte	0
	.byte	0
.Lcst_end1:
	.p2align	2

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI5_0:
	.long	65536
	.long	0
	.long	0
	.long	0
	.section	".text._ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE","ax",@progbits
	.globl	_ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE
	.p2align	4, 0x90
	.type	_ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE,@function
_ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE:
	.cfi_startproc
	pushq	%r14
	.cfi_def_cfa_offset 16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	pushq	%rax
	.cfi_def_cfa_offset 32
	.cfi_offset %rbx, -24
	.cfi_offset %r14, -16
	movq	%rdi, %rbx
	callq	*_ZN3std2io5stdio6stdout17hb2e21e8f592e1303E@GOTPCREL(%rip)
	movq	%rax, %r14
	movl	$65536, %edi
	movl	$1, %esi
	callq	*__rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB5_1
	movq	%r14, (%rbx)
	movq	%rax, 8(%rbx)
	movaps	.LCPI5_0(%rip), %xmm0
	movups	%xmm0, 16(%rbx)
	movq	%rbx, %rax
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%r14
	.cfi_def_cfa_offset 8
	retq
.LBB5_1:
	.cfi_def_cfa_offset 32
	movl	$65536, %edi
	movl	$1, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.Lfunc_end5:
	.size	_ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE, .Lfunc_end5-_ZN7easy_io13output_writer42OutputWriter$LT$std..io..stdio..Stdout$GT$3new17h75b0e07acf63ed6eE
	.cfi_endproc

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4
.LCPI6_0:
	.long	65536
	.long	0
	.long	0
	.long	0
	.section	".text._ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE","ax",@progbits
	.globl	_ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE
	.p2align	4, 0x90
	.type	_ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE,@function
_ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE:
.Lfunc_begin2:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception2
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%rbx
	.cfi_def_cfa_offset 40
	subq	$40, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r14
	movq	%rsi, %r15
	movq	%rdi, %rbx
	callq	*_ZN3std2fs11OpenOptions3new17heead8c8bfaf953f2E@GOTPCREL(%rip)
	movq	%rdx, 8(%rsp)
	movq	%rax, (%rsp)
	movq	%rsp, %rdi
	movl	$1, %esi
	callq	*_ZN3std2fs11OpenOptions5write17h6fb0141c1a683582E@GOTPCREL(%rip)
	movq	%rax, %rdi
	movl	$1, %esi
	callq	*_ZN3std2fs11OpenOptions6create17had047f45bf9b6a83E@GOTPCREL(%rip)
	leaq	16(%rsp), %rdi
	movq	%rax, %rsi
	movq	%r15, %rdx
	movq	%r14, %rcx
	callq	*_ZN3std2fs11OpenOptions5_open17h84f65b66e75ab0a3E@GOTPCREL(%rip)
	cmpl	$1, 16(%rsp)
	je	.LBB6_1
	movl	20(%rsp), %ebp
	movl	$65536, %edi
	movl	$1, %esi
	callq	*__rust_alloc@GOTPCREL(%rip)
	testq	%rax, %rax
	je	.LBB6_5
	movl	%ebp, 24(%rbx)
	movq	%rax, (%rbx)
	movaps	.LCPI6_0(%rip), %xmm0
	movups	%xmm0, 8(%rbx)
	movq	%rbx, %rax
	addq	$40, %rsp
	.cfi_def_cfa_offset 40
	popq	%rbx
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	retq
.LBB6_1:
	.cfi_def_cfa_offset 80
	movups	24(%rsp), %xmm0
	movaps	%xmm0, (%rsp)
.Ltmp6:
	leaq	.L__unnamed_1(%rip), %rdi
	leaq	.L__unnamed_2(%rip), %rcx
	leaq	.L__unnamed_4(%rip), %r8
	movq	%rsp, %rdx
	movl	$43, %esi
	callq	*_ZN4core6result13unwrap_failed17hb53671404b9e33c2E@GOTPCREL(%rip)
.Ltmp7:
	ud2
.LBB6_5:
	movl	$65536, %edi
	movl	$1, %esi
	callq	*_ZN5alloc5alloc18handle_alloc_error17hca5d9002b7fd070cE@GOTPCREL(%rip)
	ud2
.LBB6_3:
.Ltmp8:
	movq	%rax, %rbx
	movq	%rsp, %rdi
	callq	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E
	movq	%rbx, %rdi
	callq	_Unwind_Resume@PLT
	ud2
.Lfunc_end6:
	.size	_ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE, .Lfunc_end6-_ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE
	.cfi_endproc
	.section	".gcc_except_table._ZN7easy_io13output_writer33OutputWriter$LT$std..fs..File$GT$9from_file17h0f9be5c5a193dbfaE","a",@progbits
	.p2align	2
GCC_except_table6:
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
	.uleb128 .Lfunc_end6-.Ltmp7
	.byte	0
	.byte	0
.Lcst_end2:
	.p2align	2

	.type	.L__unnamed_1,@object
	.section	.rodata..L__unnamed_1,"a",@progbits
.L__unnamed_1:
	.ascii	"called `Result::unwrap()` on an `Err` value"
	.size	.L__unnamed_1, 43

	.type	.L__unnamed_2,@object
	.section	.data.rel.ro..L__unnamed_2,"aw",@progbits
	.p2align	3
.L__unnamed_2:
	.quad	_ZN4core3ptr42drop_in_place$LT$std..io..error..Error$GT$17hdc54f5b80d4e6624E
	.asciz	"\020\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_ZN58_$LT$std..io..error..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h12528dc4af160724E
	.size	.L__unnamed_2, 32

	.type	.L__unnamed_5,@object
	.section	.rodata..L__unnamed_5,"a",@progbits
.L__unnamed_5:
	.ascii	"/home/sandstorm/.cargo/registry/src/github.com-1ecc6299db9ec823/easy_io-0.3.0/src/input_reader.rs"
	.size	.L__unnamed_5, 97

	.type	.L__unnamed_3,@object
	.section	.data.rel.ro..L__unnamed_3,"aw",@progbits
	.p2align	3
.L__unnamed_3:
	.quad	.L__unnamed_5
	.asciz	"a\000\000\000\000\000\000\000\034\000\000\000(\000\000"
	.size	.L__unnamed_3, 24

	.type	.L__unnamed_6,@object
	.section	.rodata..L__unnamed_6,"a",@progbits
.L__unnamed_6:
	.ascii	"/home/sandstorm/.cargo/registry/src/github.com-1ecc6299db9ec823/easy_io-0.3.0/src/output_writer.rs"
	.size	.L__unnamed_6, 98

	.type	.L__unnamed_4,@object
	.section	.data.rel.ro..L__unnamed_4,"aw",@progbits
	.p2align	3
.L__unnamed_4:
	.quad	.L__unnamed_6
	.asciz	"b\000\000\000\000\000\000\000\035\000\000\000\034\000\000"
	.size	.L__unnamed_4, 24

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"aGw",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.quad	rust_eh_personality
	.section	".note.GNU-stack","",@progbits
