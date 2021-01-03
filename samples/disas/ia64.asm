/////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (ia64.asm/disas.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

    .section .text
    .proc  TestCodes#
    .global TestCodes#
    .align 64

TestCodes:
{   .mii
        and     r21 = 7, r32
        mux1    r25 = r33, @brcst
        add     r16 = r32, r34
} { .mmb
        cmp.ge  p9 = 0, r34
        mov     r8 = r32
(p9)    br.ret.spnt b0
  ;;
}
// align on an 8-byte boundary
{   .mmi
        cmp.ne  p15 = 0, r21   //Low 3 bits zero?
        cmp.gt  p10 = 0x30, r34
        cmp.eq  p6, p7 = 0, r33
} { .mmb
        nop.m   0
        nop.m   0
(p15)   br.cond.dpnt Align_Loop
  ;;
} { .mmi
(p7)    mov     r27 = 0x88
(p6)    mov     r27 = 0x90
        tbit.nz p14,p13 = r32, 3 // is aligned on 8-bytes (to align on 16 before FP loop)?
} { .mbb
        nop.m   0
(p10)   br.cond.dpnt Aligned_Short  // blocks < 48 bytes
(p7)    br.cond.dpnt Aligned_Long;  // do 8-byte stores for non-zero fills, 16-byte f0 for zero-fills
  ;;
}

// zero-fills >= 48 bytes goes through an f0 16-byte store loop
Aligned_FP_Long:
{   .mmi
        add     r28 = 0x80, r27
        add     r29 = 0x100, r27
        add     r30 = 0x180, r27
} { .mmi
(p14)   add     r14 = 0x88, r32
(p14)   st8     [r32] = r0, 8
(p14)   add     r34 = -0x8, r34
  ;;
} { .mmi
(p13)   add     r14 = 0x80, r32
        cmp.ge  p7 = r34, r27
        add     r27 = 0x200, r27
} { .mmi
        add     r15 = 0x180, r32
        add     r17 = 0x200, r32
        nop.i   0
  ;;
} { .mmi
        add     r26 = 0x280, r32
        cmp.ge  p8 = r34, r28
        cmp.ge  p9 = r34, r29
} { .mmi
        stf.spill   [r32] = f0       // Line 0
(p7)    stf.spill   [r14] = f0,0x80  // Line +1 0x80
        add     r28 = 0x200, r28
  ;;
} { .mmi
(p8)    stf.spill   [r14] = f0   // Line +2 0x100
(p9)    stf.spill   [r15] = f0   // Line +3 0x180
        cmp.ge  p10 = r34, r28
} { .mmi
        cmp.ge  p11 = r34, r30
        cmp.ge  p12 = r34, r27
        nop.i   0
  ;;
} { .mmi
(p11)   stf.spill   [r17] = f0   // Line +4 0x200
(p12)   stf.spill   [r26] = f0, 0x80 // Line +5 0x280
        add    r31 = 0x10, r32
  ;;
}

    .align 32
Long_FP_loop:
{   .mmi
(p10)   stf.spill   [r26] = f0, 0x80 // Line +6
        stf.spill   [r31] = f0, 0x20
        cmp.le  p15,p12 = 0x40, r34
} { .mmb
        add     r32 = 0x20, r32
        add     r34 = -0x20, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   stf.spill   [r32] = f0, 0x20
(p15)   stf.spill   [r31] = f0, 0x20
        cmp.le  p15,p12 = 0x40, r34
} { .mmb
        add     r21 = -0x60, r34
        add     r34 = -0x20, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   stf.spill   [r32] = f0, 0x20
(p15)   stf.spill   [r31] = f0, 0x20
        cmp.le  p15,p12 = 0x40, r34
} { .mmb
        cmp.ge  p10 = r21, r28
        add     r34 = -0x20, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   stf.spill   [r32] = f0, 0x20
(p15)   stf.spill   [r31] = f0, 0x20
        cmp.le  p15,p12 = 0x40, r34
} { .mbb
        add     r34 = -0x20, r34
(p15)   br.cond.sptk.many   Long_FP_loop
        br.cond.dpnt.many   Aligned_Short
  ;;
}

    .align 32
// >= 48 bytes non-zero fills go through a 8-byte store based loop
Aligned_Long:
{   .mmi
        add     r28 = 0x80, r27
        add     r29 = 0x100, r27
        add     r30 = 0x180, r27
} { .mmi
        add     r14 = 0x80, r32
        cmp.ge  p7 = r34, r27
        nop.i   0
  ;;
} { .mmi
        add     r26 = 0x180, r32
        cmp.ge  p8 = r34, r28
        add     r31 = 8, r32
} { .mmi
        st8     [r32] = r25
(p7)    st8     [r14] = r25, 0x80
        cmp.ge  p9 = r34, r29
  ;;
} { .mmi
(p8)    st8     [r14] = r25
(p9)    st8     [r26] = r25, 0x80
        cmp.ge  p10 = r34, r30
  ;;
}
    .align 32
Long_loop:
{   .mmi
(p10)   st8     [r26] = r25, 0x80
        st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        add     r32 = 0x10, r32
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        nop.m   0
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        nop.m   0
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        nop.m   0
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        nop.m   0
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        add     r21 = -0x30, r34
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        cmp.ge  p10 = r21, r30
        add     r34 = -0x10, r34
(p12)   br.cond.dpnt Aligned_Short
  ;;
} { .mmi
(p15)   st8     [r32] = r25, 0x10
(p15)   st8     [r31] = r25, 0x10
        cmp.le  p15,p12 = 0x20, r34
} { .mmb
        add     r34 = -0x10, r34
        nop.m   0
(p15)   br.cond.sptk.many Long_loop
  ;;

}

//
// Do partial word stores
//
    .align 32
Aligned_Short:
{   .mmi
        and     r27 = 2, r34
        add     r31 = 8, r32
        tbit.nz p6 = r34, 0   //bit 0 on?
} { .mmb
        cmp.le  p11 = 0x10, r34
        cmp.eq  p10 = 0, r34
(p10)   br.ret.dpnt  b0
  ;;
} { .mmi
(p11)   st8     [r32] = r25, 0x10
(p11)   st8     [r31] = r25, 0x10
        cmp.le  p12 = 0x20, r34
} { .mmi
        add     r17 = -2, r16
        add     r18 = -4, r16
        tbit.nz p9 = r34, 3   //odd number of st8s?
  ;;
} { .mmi
(p12)   st8     [r32] = r25, 0x10
(p12)   st8     [r31] = r25, 0x10
        nop.i   0
} { .mmi
(p6)    add     r18 = -1, r18
(p6)    add     r16 = -1, r16
        cmp.ne  p7 = 0, r27
  ;;
} { .mmi
(p9)    st8     [r32] = r25
(p6)    st1     [r16] = r25
        tbit.nz p8 = r34, 2   //bit 2 on?
} { .mmi
(p7)    add     r18 = -2, r18
(p6)    add     r17 = -1, r17
        nop.i   0
  ;;
} { .mmb
(p8)    st4     [r18] = r25
(p7)    st2     [r17] = r25
        br.ret.sptk.many b0
  ;;
}

    .align 32
// Align the input pointer to an 8-byte boundary
Align_Loop:
{   .mmi
        st1     [r32] = r33,1
        add     r21 = 1, r21
        cmp.eq  p15 = 1, r34
} { .mmb
        cmp.ge  p11 = 0x30, r34
        add     r34 = -1, r34
(p15)   br.ret.dpnt  b0
  ;;
} { .mmb
        cmp.gt  p10 = 8, r21
        cmp.eq  p6, p7 = 0, r33
(p10)   br.cond.sptk Align_Loop
  ;;
} { .mmi
(p7)    mov     r27 = 0x88
(p6)    mov     r27 = 0x90
        tbit.nz p14,p13 = r32, 3 // is aligned on 8-bytes (to align on 16 before FP loop)?
} { .bbb
(p11)   br.cond.dpnt Aligned_Short  // blocks < 48 bytes
(p7)    br.cond.dpnt Aligned_Long;  // non-zero fills
        br.cond.dptk Aligned_FP_Long; // zero fills
  ;;
}
    .endp  TestCodes#

    .proc  Again#
Again:
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         TestCodes
    brl         Fore1
    brl         Fore2
    brl         Fore3
    brl         Fore4
    brl         Fore5
    brl         Fore6
    brl         Fore7
    .endp  Again#

    data4       0xffffff00
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    .align 64

    data4       0xffffff00
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff

{
    addl   r2=0xffffffffffffffff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xfffffffffffffff0, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xffffffffffffff00, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xfffffffffffff000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xfffffffffffff000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xffffffffffff0000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xfffffffffff00000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xffffffffffe00000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x00000000001fffff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x00000000000fffff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x000000000000ffff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000fff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x00000000000000ff, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x000000000000000f, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000001, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000002, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000004, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000008, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000010, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000020, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000040, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000080, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000100, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000200, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000400, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000000800, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000001000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000002000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000004000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000008000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000010000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000020000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000040000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000080000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0x0000000000100000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}
{
    addl   r2=0xffffffffffe00000, gp ;;
    ld8    r2=[r2]
    nop.i  0 ;;
}

    data4       0
    .align 64

    .align 64                                           ;
    .proc  Fore1#
Fore1:
    brl         Fore2
    .endp  Fore1#

    data4       0
    .align 64

    .proc  Fore2#
Fore2:
    brl         Fore3
    .endp  Fore2#

    data4       0
    .align 64

    .proc  Fore3#
Fore3:
    brl         Fore3
    movl        gp = 0xffffffffffffffff
    brl         Fore4
    movl        gp = 0x0000000000000000
    movl        gp = 0x0000000000000001
    movl        gp = 0x0000000000000002
    movl        gp = 0x0000000000000004
    movl        gp = 0x0000000000000008
    movl        gp = 0x0000000000000010
    movl        gp = 0x0000000000000020
    movl        gp = 0x0000000000000040
    movl        gp = 0x0000000000000080
    movl        gp = 0x0000000000000100
    movl        gp = 0x0000000000000200
    movl        gp = 0x0000000000000400
    movl        gp = 0x0000000000000800
    movl        gp = 0x0000000000001000
    movl        gp = 0x0000000000002000
    movl        gp = 0x0000000000004000
    brl         Fore4
    movl        gp = 0x0000000000000000
    brl         Fore4
    movl        gp = 0x0000000000000001
    brl         Fore4
    movl        gp = 0x0000000000000002
    brl         Fore4
    movl        gp = 0x0000000000000004
    brl         Fore4
    movl        gp = 0x0000000000000008
    brl         Fore4
    movl        gp = 0x0000000000000010
    brl         Fore4
    movl        gp = 0x0000000000000020
    brl         Fore4
    movl        gp = 0x0000000000000040
    brl         Fore4
    movl        gp = 0x0000000000000080
    brl         Fore4
    movl        gp = 0x0000000000000100
    brl         Fore4
    movl        gp = 0x0000000000000200
    brl         Fore4
    movl        gp = 0x0000000000000400
    brl         Fore4
    movl        gp = 0x0000000000000800
    brl         Fore4
    movl        gp = 0x0000000000001000
    brl         Fore4
    movl        gp = 0x0000000000002000
    brl         Fore4
    movl        gp = 0x0000000000004000
    brl         Fore4
    movl        gp = 0x0000000000008000
    brl         Fore4
    movl        gp = 0x0000000000010000
    brl         Fore4
    movl        gp = 0x0000000000020000
    brl         Fore4
    movl        gp = 0x0000000000040000
    brl         Fore4
    movl        gp = 0x0000000000080000
    brl         Fore4
    movl        gp = 0x0000000000100000
    brl         Fore4
    movl        gp = 0x0000000000200000
    brl         Fore4
    movl        gp = 0x0000000000400000
    brl         Fore4
    movl        gp = 0x0000000000800000
    brl         Fore4
    movl        gp = 0x0000000001000000
    brl         Fore4
    movl        gp = 0x0000000002000000
    brl         Fore4
    movl        gp = 0x0000000004000000
    brl         Fore4
    movl        gp = 0x0000000008000000
    brl         Fore4
    movl        gp = 0x0000000010000000
    brl         Fore4
    movl        gp = 0x0000000020000000
    brl         Fore4
    movl        gp = 0x0000000040000000
    brl         Fore4
    movl        gp = 0x0000000080000000
    brl         Fore4
    movl        gp = 0x0000000100000000
    brl         Fore4
    movl        gp = 0x0000000200000000
    brl         Fore4
    movl        gp = 0x0000000400000000
    brl         Fore4
    movl        gp = 0x0000000800000000
    brl         Fore4
    movl        gp = 0x0000001000000000
    brl         Fore4
    movl        gp = 0x0000002000000000
    brl         Fore4
    movl        gp = 0x0000004000000000
    brl         Fore4
    movl        gp = 0x0000008000000000
    brl         Fore4
    movl        gp = 0x0000010000000000
    brl         Fore4
    movl        gp = 0x0000020000000000
    brl         Fore4
    movl        gp = 0x0000040000000000
    brl         Fore4
    movl        gp = 0x0000080000000000
    brl         Fore4
    movl        gp = 0x0000100000000000
    brl         Fore4
    movl        gp = 0x0000200000000000
    brl         Fore4
    movl        gp = 0x0000400000000000
    brl         Fore4
    movl        gp = 0x0000800000000000
    brl         Fore4
    movl        gp = 0x0001000000000000
    brl         Fore4
    movl        gp = 0x0002000000000000
    brl         Fore4
    movl        gp = 0x0004000000000000
    brl         Fore4
    movl        gp = 0x0008000000000000
    brl         Fore4
    movl        gp = 0x0010000000000000
    brl         Fore4
    movl        gp = 0x0020000000000000
    brl         Fore4
    movl        gp = 0x0040000000000000
    brl         Fore4
    movl        gp = 0x0080000000000000
    brl         Fore4
    movl        gp = 0x0100000000000000
    brl         Fore4
    movl        gp = 0x0200000000000000
    brl         Fore4
    movl        gp = 0x0400000000000000
    brl         Fore4
    movl        gp = 0x0800000000000000
    brl         Fore4
    movl        gp = 0x1000000000000000
    brl         Fore4
    movl        gp = 0x2000000000000000
    brl         Fore4
    movl        gp = 0x4000000000000000
    brl         Fore4
    movl        gp = 0x8000000000000000
    brl         Fore4
    movl        gp = 0xffffffffffffffff
    brl         Fore4
    movl        gp = 0x0000000000000000
    brl         Fore4
    movl        gp = 0xfffffffffffffffe
    brl         Fore4
    movl        gp = 0xfffffffffffffffc
    brl         Fore4
    movl        gp = 0xfffffffffffffff8
    brl         Fore4
    movl        gp = 0xfffffffffffffff0
    brl         Fore4
    movl        gp = 0xffffffffffffffe0
    brl         Fore4
    movl        gp = 0xffffffffffffffc0
    brl         Fore4
    movl        gp = 0xffffffffffffff80
    brl         Fore4
    movl        gp = 0xffffffffffffff00
    brl         Fore4
    movl        gp = 0xfffffffffffffe00
    brl         Fore4
    movl        gp = 0xfffffffffffffc00
    brl         Fore4
    movl        gp = 0xfffffffffffff800
    brl         Fore4
    movl        gp = 0xfffffffffffff000
    brl         Fore4
    movl        gp = 0xffffffffffffe000
    brl         Fore4
    movl        gp = 0xffffffffffffc000
    brl         Fore4
    movl        gp = 0xffffffffffff8000
    brl         Fore4
    movl        gp = 0xffffffffffff0000
    brl         Fore4
    movl        gp = 0xfffffffffffe0000
    brl         Fore4
    movl        gp = 0xfffffffffffc0000
    brl         Fore4
    movl        gp = 0xfffffffffff80000
    brl         Fore4
    movl        gp = 0xfffffffffff00000
    brl         Fore4
    movl        gp = 0xffffffffffe00000
    brl         Fore4
    movl        gp = 0xffffffffffc00000
    brl         Fore4
    movl        gp = 0xffffffffff800000
    brl         Fore4
    movl        gp = 0xffffffffff000000
    brl         Fore4
    movl        gp = 0xfffffffffe000000
    brl         Fore4
    movl        gp = 0xfffffffffc000000
    brl         Fore4
    movl        gp = 0xfffffffff8000000
    brl         Fore4
    movl        gp = 0xfffffffff0000000
    brl         Fore4
    movl        gp = 0xffffffffe0000000
    brl         Fore4
    movl        gp = 0xffffffffc0000000
    brl         Fore4
    movl        gp = 0xffffffff80000000
    brl         Fore4
    movl        gp = 0xffffffff00000000
    brl         Fore4
    movl        gp = 0xfffffffe00000000
    brl         Fore4
    movl        gp = 0xfffffffc00000000
    brl         Fore4
    movl        gp = 0xfffffff800000000
    brl         Fore4
    movl        gp = 0xfffffff000000000
    brl         Fore4
    movl        gp = 0xffffffe000000000
    brl         Fore4
    movl        gp = 0xffffffc000000000
    brl         Fore4
    movl        gp = 0xffffff8000000000
    brl         Fore4
    movl        gp = 0xffffff0000000000
    brl         Fore4
    movl        gp = 0xfffffe0000000000
    brl         Fore4
    movl        gp = 0xfffffc0000000000
    brl         Fore4
    movl        gp = 0xfffff80000000000
    brl         Fore4
    movl        gp = 0xfffff00000000000
    brl         Fore4
    movl        gp = 0xffffe00000000000
    brl         Fore4
    movl        gp = 0xffffc00000000000
    brl         Fore4
    movl        gp = 0xffff800000000000
    brl         Fore4
    movl        gp = 0xffff000000000000
    brl         Fore4
    movl        gp = 0xfffe000000000000
    brl         Fore4
    movl        gp = 0xfffc000000000000
    brl         Fore4
    movl        gp = 0xfff8000000000000
    brl         Fore4
    movl        gp = 0xfff0000000000000
    brl         Fore4
    movl        gp = 0xffe0000000000000
    brl         Fore4
    movl        gp = 0xffc0000000000000
    brl         Fore4
    movl        gp = 0xff80000000000000
    brl         Fore4
    movl        gp = 0xff00000000000000
    brl         Fore4
    movl        gp = 0xfe00000000000000
    brl         Fore4
    movl        gp = 0xfc00000000000000
    brl         Fore4
    movl        gp = 0xf800000000000000
    brl         Fore4
    movl        gp = 0xf000000000000000
    brl         Fore4
    movl        gp = 0xe000000000000000
    brl         Fore4
    movl        gp = 0xc000000000000000
    brl         Fore4
    movl        gp = 0x8000000000000000
    brl         Fore4
    movl        gp = 0x0000000000000000
    brl         Fore4
    movl        gp = 0x0000000000000000
    brl         Fore4
    brl.sptk.many Fore4
    brl.sptk.many Fore4
    brl.sptk.many Fore4
    brl.sptk.many Fore4

    movl        gp = 0xf0f0f0f0f0f0f0f0
    brl.sptk.many Fore9
Fore9:
    brl.sptk.few Fore8
Fore8:
    brl.sptk.few Fore8
    brl.sptk.few Fore8
    brl.sptk.few Fore8
    data1       0x05,0x00,0x00,0x00,0x01,0x00,0xff,0xff,0xff,0xff,0x7f,0x00,0xf0,0xff,0xff,0xc8
    data1       0x05,0x00,0x00,0x00,0x01,0x00,0xff,0xff,0xff,0xff,0x7f,0x00,0xf0,0xff,0xff,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x00,0xff,0xff,0xff,0xff,0x7f,0x00,0xf0,0xff,0xff,0xc8
    data1       0x05,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x04,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x25,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x45,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x85,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x01,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x02,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x04,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x08,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x10,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x20,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x40,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x80,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x01,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x02,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x04,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x08,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x10,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x20,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x40,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x80,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x02,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x04,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x08,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x10,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x20,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x40,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x80,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x05,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x09,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x11,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x21,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x41,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x81,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x02,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x04,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x08,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x10,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x20,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data1       0x05,0x00,0x00,0x00,0x01,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0xc0
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    movl        gp = 0xf0f0f0f0f0f0f0f0

{   .mii
        nop.m 0
        nop.i 0
        nop.i 0
}
{   .mmi
        nop.m 0
        nop.m 0
        nop.i 0
}
{ .mmb
        nop.m 0
        nop.m 0
        nop.b 0
}
{ .mmf
        nop.m 0
        nop.m 0
        nop.f 0
}
{ .mbb
        nop.m 0
        nop.b 0
        nop.b 0
}
    movl        gp = 0x0000000000000000
{ .mlx
    flushrs
    movl        gp = 0x0000000000000000
}
    movl        gp = 0x0000000000000000
    .endp  Fore3#

    data4       0
    .align 64

    .proc  Fore4#
Fore4:
    movl        gp = 0x0000000000000000
    brl         Fore5
    .endp  Fore4#

    data4       0
    .align 64

    data4       0
    .align 64

    .proc  Fore5#
Fore5:
    movl        gp = 0xffffffffffffffff
    brl         Fore6
    .endp  Fore5#

    data4       0
    .align 64

    .proc  Fore6#
Fore6:
    movl        gp = 0x0000000000000000
    brl         Fore7
    .endp  Fore6#

    data4       0
    .align 64

    .proc  Fore7#
Fore7:
    movl        gp = 0xffffffffffffffff
    brl         Fore6
    brl.call.dptk.many b0=Fore6
    br.call.dptk.many b0=Fore6
    br.ret.dpnt.many b0
    br.ret.dptk.many b0
    br.ret.spnt.many b0
    br.ret.sptk.many b0
    .endp  Fore7#

    data4       0
    .align 64

    .proc  Call8#
Call8:
{
        alloc  r41=ar.pfs, 10, 0, 8, 0
        adds   r49=0, r39
        mov    r40=rp
}
{
        adds   r48=0, r38
        adds   r47=0, r37
        adds   r46=0, r36 ;;
}
{
        adds   r45=0, r35
        adds   r44=0, r34
        adds   r43=0, r33
}
{
        movl   gp = 0xffffffffffffffff ;;
}
{
        adds   r42=0, r32
        brl.call.sptk.few rp=Call8Real  ;;
}
Call8Real:
{
        movl   gp = 0xffffffffffffffff ;;
}
{
        nop.m  0
        mov    rp=r40, +0 ;;
        mov.i  ar.pfs=r41
}
{
        nop.m  0
        nop.m  0
        br.ret.sptk.many rp ;;
}
    .endp  Call8#

    data4       0
    .align 64

    .proc  Pass8#
Pass8:
{
        alloc  r41=ar.pfs, 11, 0, 8, 0
        mov    r40=rp
}
{
        adds   r50=0, r39
        adds   r49=0, r38
        adds   r48=0, r37 ;;
}
{
        adds   r47=0, r36
        adds   r46=0, r35
        adds   r45=0, r34
}
{
        adds   r44=0, r33
        adds   r43=0, r32
        adds   r42=0, gp  ;;
}
{
        movl   gp = 0xffffffffffffffff ;;
}
{
        brl.call.sptk.few rp=Pass8Real  ;;
}
Pass8Real:
{
        adds   gp=0, r42
        mov    rp=r40, +0 ;;
        mov.i  ar.pfs=r41
}
{
        nop.m  0
        nop.m  0
        br.ret.sptk.many rp ;;
}
    .endp  Pass8#

    data4       0
    .align 64

    .proc  Last#
Last:
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    .skip       0x1000
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    data4       0xffffffff
    .endp  Last#

// End
