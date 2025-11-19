	.text
	.amdgcn_target "amdgcn-amd-amdhsa--gfx1011"
	.amdhsa_code_object_version 5
	.globl	mma_m16n8k32_s32_s8_s8_s32
	.p2align	8
	.type	mma_m16n8k32_s32_s8_s8_s32,@function
mma_m16n8k32_s32_s8_s8_s32:
	v_lshlrev_b16 v1, 4, v0
	v_mov_b32_e32 v2, 6
	v_mov_b32_e32 v3, 7
	v_mbcnt_lo_u32_b32 v18, -1, 0
	s_load_dwordx2 s[0:1], s[4:5], 0x0
	v_or_b32_e32 v4, 1, v1
	v_or_b32_sdwa v9, v1, v2 dst_sel:WORD_1 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_mov_b32_e32 v2, 10
	v_or_b32_e32 v6, 3, v1
	v_or_b32_sdwa v10, v1, v3 dst_sel:BYTE_3 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_mov_b32_e32 v3, 11
	v_or_b32_e32 v5, 2, v1
	v_or_b32_sdwa v13, v1, v2 dst_sel:WORD_1 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_mov_b32_e32 v2, 14
	v_or_b32_e32 v7, 4, v1
	v_or_b32_e32 v8, 5, v1
	v_or_b32_sdwa v14, v1, v3 dst_sel:BYTE_3 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_mov_b32_e32 v3, 15
	v_or_b32_sdwa v17, v1, v2 dst_sel:WORD_1 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_and_b32_e32 v2, 0xffff, v4
	v_and_b32_e32 v4, 0xffff, v6
	v_lshlrev_b32_e32 v6, 3, v18
	v_or_b32_sdwa v19, v1, v3 dst_sel:BYTE_3 dst_unused:UNUSED_PAD src0_sel:DWORD src1_sel:DWORD
	v_lshlrev_b32_e32 v20, 16, v5
	v_and_b32_e32 v7, 0xffff, v7
	v_and_b32_e32 v8, 0xffff, v8
	v_and_b32_e32 v3, 0xffff, v5
	v_and_b32_e32 v5, 24, v6
	v_or_b32_e32 v11, 8, v1
	v_or_b32_e32 v12, 9, v1
	v_or_b32_e32 v15, 12, v1
	v_or_b32_e32 v16, 13, v1
	v_and_b32_e32 v1, 0xffff, v1
	v_lshlrev_b32_e32 v6, 24, v4
	v_lshl_or_b32 v20, v2, 8, v20
	v_lshl_or_b32 v7, v8, 8, v7
	v_add_nc_u32_e32 v8, 1, v18
	v_and_or_b32 v21, v18, 3, v5
	v_and_b32_e32 v11, 0xffff, v11
	v_and_b32_e32 v12, 0xffff, v12
	v_or3_b32 v6, v20, v6, v1
	v_and_or_b32 v8, v8, 3, v5
	v_lshlrev_b32_e32 v20, 2, v21
	v_or3_b32 v7, v7, v9, v10
	v_lshl_or_b32 v9, v12, 8, v11
	v_and_b32_e32 v10, 0xffff, v15
	v_lshlrev_b32_e32 v8, 2, v8
	v_add_nc_u32_e32 v12, -1, v18
	v_and_b32_e32 v15, 0xffff, v16
	ds_bpermute_b32 v16, v20, v6 offset:16
	ds_bpermute_b32 v11, v20, v6
	ds_bpermute_b32 v18, v8, v6
	v_xor_b32_e32 v20, 8, v20
	v_and_or_b32 v5, v12, 3, v5
	ds_bpermute_b32 v8, v8, v6 offset:16
	v_lshl_or_b32 v10, v15, 8, v10
	v_or3_b32 v9, v9, v13, v14
	ds_bpermute_b32 v13, v20, v6 offset:16
	v_lshlrev_b32_e32 v5, 2, v5
	ds_bpermute_b32 v12, v20, v6
	v_or3_b32 v10, v10, v17, v19
	v_mov_b32_dpp v14, v6 quad_perm:[1,2,3,0] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_mov_b32_dpp v15, v7 quad_perm:[1,2,3,0] row_mask:0xf bank_mask:0xf bound_ctrl:1
	ds_bpermute_b32 v17, v5, v6
	ds_bpermute_b32 v5, v5, v6 offset:16
	v_mov_b32_dpp v19, v6 quad_perm:[2,3,0,1] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_mov_b32_dpp v20, v7 quad_perm:[2,3,0,1] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_mov_b32_dpp v21, v6 quad_perm:[3,0,1,2] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_mov_b32_dpp v22, v7 quad_perm:[3,0,1,2] row_mask:0xf bank_mask:0xf bound_ctrl:1
	s_waitcnt lgkmcnt(0)
	v_dot4c_i32_i8 v2, v6, v16
	v_dot4c_i32_i8 v4, v7, v16
	v_dot4c_i32_i8 v1, v6, v11
	v_dot4c_i32_i8 v3, v7, v11
	v_mov_b32_dpp v6, v9 quad_perm:[1,2,3,0] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v14, v8
	v_dot4c_i32_i8 v4, v15, v8
	v_dot4c_i32_i8 v1, v14, v18
	v_dot4c_i32_i8 v3, v15, v18
	v_mov_b32_dpp v7, v10 quad_perm:[1,2,3,0] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v19, v13
	v_dot4c_i32_i8 v4, v20, v13
	v_dot4c_i32_i8 v1, v19, v12
	v_dot4c_i32_i8 v3, v20, v12
	v_mov_b32_dpp v14, v9 quad_perm:[2,3,0,1] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v21, v5
	v_dot4c_i32_i8 v4, v22, v5
	v_dot4c_i32_i8 v1, v21, v17
	v_dot4c_i32_i8 v3, v22, v17
	v_mov_b32_dpp v15, v10 quad_perm:[2,3,0,1] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v9, v16
	v_dot4c_i32_i8 v4, v10, v16
	v_dot4c_i32_i8 v1, v9, v11
	v_dot4c_i32_i8 v3, v10, v11
	v_mov_b32_dpp v9, v9 quad_perm:[3,0,1,2] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v6, v8
	v_dot4c_i32_i8 v4, v7, v8
	v_dot4c_i32_i8 v1, v6, v18
	v_dot4c_i32_i8 v3, v7, v18
	v_mov_b32_dpp v6, v10 quad_perm:[3,0,1,2] row_mask:0xf bank_mask:0xf bound_ctrl:1
	v_dot4c_i32_i8 v2, v14, v13
	v_dot4c_i32_i8 v4, v15, v13
	v_lshlrev_b32_e32 v0, 4, v0
	v_dot4c_i32_i8 v1, v14, v12
	v_dot4c_i32_i8 v3, v15, v12
	v_dot4c_i32_i8 v2, v9, v5
	v_dot4c_i32_i8 v4, v6, v5
	v_add_co_u32 v5, s0, s0, v0
	v_dot4c_i32_i8 v1, v9, v17
	v_dot4c_i32_i8 v3, v6, v17
	v_add_co_ci_u32_e64 v6, s0, s1, 0, s0
	flat_store_dwordx4 v[5:6], v[1:4]
	s_endpgm
	.section	.rodata,"a",@progbits
	.p2align	6, 0x0
	.amdhsa_kernel mma_m16n8k32_s32_s8_s8_s32
		.amdhsa_group_segment_fixed_size 0
		.amdhsa_private_segment_fixed_size 0
		.amdhsa_kernarg_size 8
		.amdhsa_user_sgpr_count 6
		.amdhsa_user_sgpr_private_segment_buffer 1
		.amdhsa_user_sgpr_dispatch_ptr 0
		.amdhsa_user_sgpr_queue_ptr 0
		.amdhsa_user_sgpr_kernarg_segment_ptr 1
		.amdhsa_user_sgpr_dispatch_id 0
		.amdhsa_user_sgpr_flat_scratch_init 0
		.amdhsa_user_sgpr_private_segment_size 0
		.amdhsa_wavefront_size32 1
		.amdhsa_uses_dynamic_stack 0
		.amdhsa_system_sgpr_private_segment_wavefront_offset 0
		.amdhsa_system_sgpr_workgroup_id_x 1
		.amdhsa_system_sgpr_workgroup_id_y 0
		.amdhsa_system_sgpr_workgroup_id_z 0
		.amdhsa_system_sgpr_workgroup_info 0
		.amdhsa_system_vgpr_workitem_id 0
		.amdhsa_next_free_vgpr 23
		.amdhsa_next_free_sgpr 6
		.amdhsa_reserve_vcc 0
		.amdhsa_reserve_flat_scratch 0
		.amdhsa_reserve_xnack_mask 1
		.amdhsa_float_round_mode_32 0
		.amdhsa_float_round_mode_16_64 0
		.amdhsa_float_denorm_mode_32 0
		.amdhsa_float_denorm_mode_16_64 0
		.amdhsa_dx10_clamp 1
		.amdhsa_ieee_mode 0
		.amdhsa_fp16_overflow 0
		.amdhsa_workgroup_processor_mode 0
		.amdhsa_memory_ordered 1
		.amdhsa_forward_progress 0
		.amdhsa_shared_vgpr_count 0
		.amdhsa_exception_fp_ieee_invalid_op 0
		.amdhsa_exception_fp_denorm_src 0
		.amdhsa_exception_fp_ieee_div_zero 0
		.amdhsa_exception_fp_ieee_overflow 0
		.amdhsa_exception_fp_ieee_underflow 0
		.amdhsa_exception_fp_ieee_inexact 0
		.amdhsa_exception_int_div_zero 0
	.end_amdhsa_kernel
	.text
.Lfunc_end0:
	.size	mma_m16n8k32_s32_s8_s8_s32, .Lfunc_end0-mma_m16n8k32_s32_s8_s8_s32

	.p2alignl 6, 3214868480
	.fill 48, 4, 3214868480
	.ident	"AMD clang version 19.0.0git (https://github.com/RadeonOpenCompute/llvm-project roc-6.4.0 25133 c7fe45cf4b819c5991fe208aaa96edf142730f1d)"
	.section	".note.GNU-stack","",@progbits
	.amdgpu_metadata
---
amdhsa.kernels:
  - .args:
      - .name:           !str '149'
        .offset:         0
        .size:           8
        .value_kind:     by_value
    .group_segment_fixed_size: 0
    .kernarg_segment_align: 8
    .kernarg_segment_size: 8
    .language:       OpenCL C
    .language_version:
      - 2
      - 0
    .max_flat_workgroup_size: 1024
    .name:           mma_m16n8k32_s32_s8_s8_s32
    .private_segment_fixed_size: 0
    .sgpr_count:     6
    .sgpr_spill_count: 0
    .symbol:         mma_m16n8k32_s32_s8_s8_s32.kd
    .uniform_work_group_size: 1
    .uses_dynamic_stack: false
    .vgpr_count:     23
    .vgpr_spill_count: 0
    .wavefront_size: 32
    .workgroup_processor_mode: 0
amdhsa.target:   amdgcn-amd-amdhsa--gfx1011
amdhsa.version:
  - 1
  - 2
...

	.end_amdgpu_metadata
