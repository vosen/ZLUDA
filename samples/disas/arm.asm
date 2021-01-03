;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;
;;  Detours Test Program (rlo.asm/disas.exe)
;;
;;  Microsoft Research Detours Package
;;
;;  Copyright (c) Microsoft Corporation.  All rights reserved.
;;

        MACRO
        BREAK
        DCW 0xdefe
        MEND

        AREA    |.text|,ALIGN=2,CODE,READONLY

        AREA    |.text|,CODE,READONLY

        ALIGN 0x1000

        EXPORT  |TestCodes|
|TestCodes|

;        dcw 0xf8df,0xe00e                               ; 94  = -16 = -12 ; 94 ; 98 + e = a6
;        BREAK                                           ; 98  = -14 = -10 ; 98 ; 9c
;        dcw 0xf8df,0xe00a                               ; 9a  = -12 = -8  ; 98 ; 9c + a = a6
;        BREAK                                           ; 9e =  -8 = -4   ; 9c ; a0
;        dcw 0xf8df,0xe002                               ; a0 =  -6 = -2   ; a0 ; a4 + 2 = a6
;        BREAK                                           ; a4 =  -2        ; a4 ; a8
;        movs r2, r0                                     ; a6 <===
;        movs r3, r0                                     ;
;        BREAK
;        BREAK
;
;        ldr     lr,=0xa98765
;        ldr     pc,=0xa98765
;        ldr     pc,=0xa98765
;        ldr     pc,=0xa98765
;        BREAK
;       BREAK

        BREAK
        ldr     lr, =0xa98765
        BREAK
        blx     lr

        BREAK
        pop     pc
        BREAK
        pop     {r11,pc}
        BREAK
        pop     {r10,r11,pc}
        BREAK
        pop     {r9,r10,r11,pc}
        BREAK
        pop     {r1,r2,r3,r4,r5,r6,r7,r8,r9,r10,r11,pc}

        BREAK
        ldr.w     r0,=0xa98765
        BREAK
        nop
        ldr.w     r0,=0xa98765
        BREAK
        nop
        nop
        ldr.w     r0,=0xa98765

        BREAK
        ldr     r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765
        BREAK
        ldr     r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765
        BREAK
        ldr     r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765
        BREAK
        ldr.w   r0,=0xa98765

        BREAK
        ldr     r0,=0xa98765
        BREAK
        nop
        ldr     r0,=0xa98765
        BREAK
        nop
        nop
        ldr     r0,=0xa98765

        BREAK
        nop
        ldr     r0,=0xa
        BREAK
        ldr     r0,=0xa9
        BREAK
        ldr     r0,=0xa98
        BREAK
        ldr     r0,=0xa987
        BREAK
        ldr     r0,=0xa9876
        BREAK
        ldr     r0,=0xa98765
        BREAK
        ldr     r0,=0xa987654
        BREAK
        ldr     r0,=0xa9876543

        ;; Simple instructions.
        BREAK
        adds r0,r0, #5                                  ; 1d40
        BREAK
        movs r2, #0                                     ; 2200
        BREAK
        movs r3, #0                                     ; 2300
        BREAK
        bx lr                                           ; 4770 [FFFFFFFF]

        ;; Known 16-bit instructions
        BREAK
        mov r11, sp                                     ; 46eb
        BREAK
        movs r2, r0                                     ; 0002
        BREAK
        push r0, r1                                     ; b403
        BREAK
        str r3,[r7,#0x28]                               ; 62bb
        BREAK
        bx r5                                           ; 4728 [FFFFFFFF]
        BREAK
        blx r5                                          ; 47a8
        BREAK
        DCW 0x4878 ;  ldr r0, [PC + 0x1E0]              ; 4878
        BREAK
        str r3,[r7,#0x1C]                               ; 61fb
        BREAK
        ldr r3,[r7,#0x38]                               ; 6bbb
        BREAK
        add r3,sp,#0xCC                                 ; ab33
        BREAK
        cbz r2,+0x56                                    ; b34a [00xx1510]
        BREAK
        cbnz r2,+0x56                                   ; bb4a [00xx1514]
        BREAK
        push {r0,r2,r4,r6,lr}                           ; b555
        BREAK
        nop                                             ; bf00

        ;; Placeholder for IT instruction
        BREAK
        bne +0x6E                                       ; d135 [00xx1538] -??? d137
        BREAK
        svc #0x24                                       ; df24
        BREAK
        b +0x7FE                                        ; e3fd [00xx1cd0] -??? e3ff

        ;; 32 bit test codes
        BREAK
        adds r0,r7,#8                                   ; f1170008
        BREAK
        str r3,[r5,#0x677]                              ; f8c53677
        BREAK
        ldrsh r10,[r5,#0x5A5]                           ; f9b5a5a5
        BREAK
        DCW 0xf89f,0x55a5 ;ldrb r5, [+0x5A5]            ; f89f55a5
        BREAK
        bls.w +0x86;    0xf240; 0x8043; //              ; f2408041 [00xx157A]
        BREAK
        bl +0xFE;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFE;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFE;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFFE;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFFFE;    0xf7ff; 0xff80; //
        BREAK
        bl +0xF0;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFF0;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFF0;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFF0;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFFF0;    0xf7ff; 0xff80; //
        BREAK
        bl +0xF00;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFF00;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFF00;    0xf7ff; 0xff80; //
        BREAK
        bl +0xFFFF00;    0xf7ff; 0xff80; //
        BREAK
        DCW 0xf7ff,0xff80
        ;bl +0xFFFFFF00;    0xf7ff; 0xff80; //
        BREAK
        DCW 0xf7ff,0xbe02
        ;        b.w ;    0xf7ff; 0xbe02; //  (10053528)
        BREAK
        push {r7,r11,lr};    0xe92d; 0x4880; //

        ;; 32 bit expected results
        BREAK
        adds r0,r7,#8                                   ; 0xf1170008
        BREAK
        str r3,[r5,#0x677]                              ; 0xf8c53677
        BREAK
        ldrsh r10,[r5,#0x5A5]                           ; 0xf9b5a5a5

        BREAK
        DCW 0xf6af,0xfbd2
        ; bl (0008ef3c);    ResultCode(4, 0xf6af, 0xfbd2, Target(ADDRESS(&g_pTestCodes32[i*2], 0xFFFFFF00))); // 0xf7ff, 0xff80: -> 0xf6affbd2
        BREAK
        bl (00090300);    ResultCode(4, 0xf6af, 0xba54, Target(ADDRESS(&g_pTestCodes32[i*2], 0xFFFFFC04))); // 0xf7ff, 0xff80: -> f6afba54 bl (00090300)
        BREAK
        push {r7,r11,lr};    ResultCode(4, 0xe92d, 0x4880); // 0xe92d, 0x4880: //

        BREAK
        BREAK

|TestCodes_end|

        END
