	.file	"printf.ll"
	.text
	.globl	say                             // -- Begin function say
	.p2align	1
	.type	say,@function
say:                                    // @say
	// uniform registers: 10 x 16-bit (threadgroups_per_grid at u0)
	// general purpose registers: 8 x 32-bit
	// global data buffer at u3_u4
	// threadgroup memory: 0 bytes
	// stack size: 0 bytes
// %bb.0:
	bitop 0101, r2, u3, 0
	bitop 0101, r3, u4, 0
	iadd r4_r5, r2_r3, 8
	mov_imm r6, 24
	atomic add, 0, r1, r4_r5, 0, signed, r6, 0
	wait 0
	iadd r1, r1, 8
	iadd r4_r5, r4_r5, r1
	get_sr r1, sr80 (thread_position_in_grid.x)
	mov_imm r7, 4
	device_store 0, i32, xy, r6_r7, r4_r5, 0, signed
	iadd r1, r1, 41
	device_store 0, i32, xy, r2_r3, r4_r5, 2, signed
	device_store 0, i32, x, r1, r4_r5, 4, signed
	stop
	trap
	trap
	trap
	trap
	trap
	trap
	trap
	trap
.Lfunc_end0:
	.size	say, .Lfunc_end0-say
                                        // -- End function
	.section	".note.GNU-stack","",@progbits
	.section	.agx.globals,"aw",@progbits
	.p2align	3, 0x0
	.type	.Lfmt,@object
	.size	.Lfmt, 4
.Lfmt:
	.asciz	"%d\n"
	.zero	4
	.type	__zluda_agx_printf_buffer,@object
	.size	__zluda_agx_printf_buffer, 256
__zluda_agx_printf_buffer:
	.zero	256
