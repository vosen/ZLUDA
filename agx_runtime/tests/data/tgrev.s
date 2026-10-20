	.file	"tgrev.ll"
	.text
	.globl	tgrev                           // -- Begin function tgrev
	.p2align	1
	.type	tgrev,@function
tgrev:                                  // @tgrev
	// uniform registers: 18 x 16-bit (threadgroups_per_grid at u6)
	// general purpose registers: 5 x 32-bit
	// threadgroup memory: 256 bytes
	// stack size: 0 bytes
// %bb.0:
	get_sr r1, sr80 (thread_position_in_grid.x)
	device_load 0, i32, x, r2, u0_u1, r1, signed
	get_sr r3, sr51 (thread_index_in_threadgroup)
	bfi r4, 0, r3, 2
	isub r3, 63, r3
	bfi r3, 0, r3, 2
	wait 0
	threadgroup_store i32, x, r2, r4l, 0
	threadgroup_barrier
	threadgroup_load i32, x, r2, r3l, 0
	device_store 0, i32, x, r2, u2_u3, r1, signed
	mov_imm r1, 1
	atomic add, 0, r1, u4_u5, 0, signed, r1, 0
	wait 0
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
	.size	tgrev, .Lfunc_end0-tgrev
                                        // -- End function
	.section	".note.GNU-stack","",@progbits
