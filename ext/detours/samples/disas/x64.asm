;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;
;;  Detours Test Program (x64.asm/disas.exe)
;;
;;  Microsoft Research Detours Package
;;
;;  Copyright (c) Microsoft Corporation.  All rights reserved.
;;

.xlist
.list
.code

PUBLIC  TestCodes

_TEXT   SEGMENT

TestCodes   PROC

begin:
faraway:
        int 3
        nop
        int 3
        db      066h,090h ; // 2-byte NOP.
        int 3
        db      00fh, 01fh, 000h ; // 3-byte NOP.
        int 3
        db      00fh, 01fh, 040h, 000h ; // 4-byte NOP.
        int 3
        db      00fh, 01fh, 044h, 000h, 000h ; // 5-byte NOP.
        int 3
        db      066h, 00fh, 01fh, 044h, 000h, 000h ; // 6-byte NOP.
        int 3
        db      00fh, 01fh, 080h, 000h, 000h, 000h, 000h ; // 7-byte NOP.
        int 3
        db      00fh, 01fh, 084h, 000h, 000h, 000h, 000h, 000h ; // 8-byte NOP.
        int 3
        db      066h, 00fh, 01fh, 084h, 000h, 000h, 000h, 000h, 000h ; // 9-byte NOP.
        int 3
        mov     rax, cr8
        int 3
        mov     rcx, cr8
        int 3
        mov     rdx, cr8
        int 3
        mov     rbx, cr8
        int 3
        mov     rsp, cr8
        int 3
        mov     rbp, cr8
        int 3
        mov     rsi, cr8
        int 3
        mov     rdi, cr8
        int 3
        mov     r8, cr8
        int 3
        mov     r9, cr8
        int 3
        mov     r10, cr8
        int 3
        mov     r11, cr8
        int 3
        mov     r12, cr8
        int 3
        mov     r13, cr8
        int 3
        mov     r14, cr8
        int 3
        mov     r15, cr8
        int 3
        mov     cr8, rax
        int 3
        mov     cr8, rcx
        int 3
        mov     cr8, rdx
        int 3
        mov     cr8, rbx
        int 3
        mov     cr8, rsp
        int 3
        mov     cr8, rbp
        int 3
        mov     cr8, rsi
        int 3
        mov     cr8, rdi
        int 3
        mov     cr8, r8
        int 3
        mov     cr8, r9
        int 3
        mov     cr8, r10
        int 3
        mov     cr8, r11
        int 3
        mov     cr8, r12
        int 3
        mov     cr8, r13
        int 3
        mov     cr8, r14
        int 3
        mov     cr8, r15
        int 3
        xor     rax, rax
        int 3
        xor     rcx, rcx
        int 3
        xor     rdx, rdx
        int 3
        xor     rbx, rbx
        int 3
        xor     rsp, rsp
        int 3
        xor     rbp, rbp
        int 3
        xor     rsi, rsi
        int 3
        xor     rdi, rdi
        int 3
        xor     r8, r8
        int 3
        xor     r9, r9
        int 3
        xor     r10, r10
        int 3
        xor     r11, r11
        int 3
        xor     r12, r12
        int 3
        xor     r13, r13
        int 3
        xor     r14, r14
        int 3
        xor     r15, r15
        int 3
        jmp     rax
        int 3
        jmp     rbx
        int 3
        jmp     rcx
        int 3
        jmp     rdx
        int 3
        push    rax
        int 3
        push    rbx
        int 3
        push    rcx
        int 3
        push    rdx
        int 3
        push    0
        int 3
        pop     rax
        int 3
        pop     rbx
        int 3
        pop     rcx
        int 3
        pop     rdx
        int 3
        mov     rax,[value]
        int 3
        sub     rsp,0418h
        int 3
        mov     [rsp+0410h],rbx
        int 3
        mov     [rsp+0408h],rsi
        int 3
        mov     [rsp+0400h],rdi
        int 3
        mov     [rsp+03f8h],r12
        int 3
        mov     [rsp+03f0h],r13
        int 3
        mov     [rsp+03e8h],r14
        int 3
        mov     [rsp+03e0h],r15
        int 3
        add     [rax],al                                ; 0000
        int 3
        add     [rcx],al                                ; 0001
        int 3
        add     [rbx],al                                ; 0003
        int 3
        add     [rax+rax],al                            ; 000400
        int 3
        add     [rdi],al                                ; 0007
        int 3
        add     [rax],cl                                ; 0008
        int 3
        add     [rdi],cl                                ; 000f
        int 3
        add     [rax],dl                                ; 0010
        int 3
        add     [rdi],bl                                ; 001f
        int 3
        add     [rax],ah                                ; 0020
        int 3
        add     [rdi],bh                                ; 003f
        int 3
        add     [rax+03bh],cl                           ; 00483b
        int 3
        add     [rdi],bh                                ; 007f00
        int 3
        add     [rax+040000000h],al                     ; 008000000040
        int 3
        add     bh,bh                                   ; 00ff
        int 3
        add     [rax],eax                               ; 0100
        int 3
        add     al,[rax]                                ; 0200
        int 3
        add     eax,06603ebc3h                          ; 05c3eb0366
        int 3
        syscall                                         ; 0f05
        int 3
        prefetchw      byte ptr [rcx]                   ; 0f0d09
        int 3
        prefetchnta    byte ptr [rcx]                   ; 0f1801
        int 3
        prefetchnta    byte ptr [rax+rdx]               ; 0f180410
        int 3
        jb             again                            ; 0f8247070000
        int 3
        jnb            again                            ; 0f8306050000
        int 3
        je             again                            ; 0f8432010000
        int 3
        jne            again                            ; 0f8508010000
        int 3
        jnbe           again                            ; 0f878a000000
        int 3
        ldmxcsr        dword ptr [rcx+034h]             ; 0fae5134
        int 3
        stmxcsr        dword ptr [rcx+034h]             ; 0fae5934
        int 3
        and            ecx,[rdx+rbx*4]                  ; 230c9a
        int 3
        xor            eax,eax                          ; 33c0
        int 3
        xor            ecx,ecx                          ; 33c9
        int 3
        xor            edx,ecx                          ; 33d1
        int 3
        xor            edx,edx                          ; 33d2
        int 3
        add            r10d,010001h                     ; 4181c201000100
        int 3
        and            r11d,0ffffh                      ; 4181e3ffff0000
        int 3
        mov            eax,r8d                          ; 418bc0
        int 3
        mov            byte ptr [r11],00h               ; 41c60300
        int 3
        call           qword ptr [r9+030h]              ; 41ff5130
        int 3
        call           qword ptr [r9+r8*8]              ; 43ff14c1
        int 3
        mov            [rcx+034h],r8d                   ; 44894134
        int 3
        mov            [rsp+030h],r9d                   ; 44894c2430
        int 3
        mov            r8d,[rcx]                        ; 448b01
        int 3
        mov            r9d,[rcx]                        ; 448b09
        int 3
        mov            r8d,[rax+058h]                   ; 448b4058
        int 3
        mov            r8d,[rsp+02ch]                   ; 448b44242c
        int 3
        mov            r8d,eax                          ; 448bc0
        int 3
        mov            r8d,edx                          ; 448bc2
        int 3
        xor            r8b,r8b                          ; 4532c0
        int 3
        mov            r9d,r8d                          ; 458bc8
        int 3
        lea            r11d,[r9+rax]                    ; 458d1c01
        int 3
        add            rdx,rcx                          ; 4803d1
        int 3
        or             rsi,rdx                          ; 480bf2
        int 3
        movnti         [rcx],rax                        ; 480fc301
        int 3
        and            rax,0fe000000h                   ; 4825000000fe
        int 3
        sub            rax,rcx                          ; 482bc1
        int 3
        sub            rdx,rcx                          ; 482bd1
        int 3
        cmp            rdi,rbp                          ; 483bfd
        int 3
        push           rbp                              ; 4855
        int 3
        add            rcx,03d0h                        ; 4881c1d0030000
        int 3
        add            rsp,0c8h                         ; 4881c4c8000000
        int 3
        and            rdx,0fe000000h                   ; 4881e2000000fe
        int 3
        sub            rsp,0c8h                         ; 4881ecc8000000
        int 3
        sub            rsp,03d0h                        ; 4881ecd0030000
        int 3
        add            rax,040h                         ; 4883c040
        int 3
        add            rcx,08h                          ; 4883c108
        int 3
        add            rcx,040h                         ; 4883c140
        int 3
        add            rsp,08h                          ; 4883c408
        int 3
        add            rsi,09h                          ; 4883c609
        int 3
        add            rdi,01h                          ; 4883c701
        int 3
        and            rcx,0f8h                         ; 4883e1f8
        int 3
        sub            rax,040h                         ; 4883e840
        int 3
        sub            rdx,08h                          ; 4883ea08
        int 3
        sub            rdx,040h                         ; 4883ea40
        int 3
        sub            rsp,08h                          ; 4883ec08
        int 3
        sub            rsi,08h                          ; 4883ee08
        int 3
        sub            rdi,01h                          ; 4883ef01
        int 3
        test           rax,rax                          ; 4885c0
        int 3
        test           rdx,rdx                          ; 4885d2
        int 3
        mov            [rsp],rax                        ; 48890424
        int 3
        mov            [rsp],rbp                        ; 48892c24
        int 3
        mov            [rsp],rsi                        ; 48893424
        int 3
        mov            [rsp],rdi                        ; 48893c24
        int 3
        mov            [rcx+08h],rax                    ; 48894108
        int 3
        mov            [rcx+078h],rax                   ; 48894178
        int 3
        mov            [rcx-08h],rax                    ; 488941f8
        int 3
        mov            [rsp+018h],rax                   ; 4889442418
        int 3
        mov            [rcx+010h],rdx                   ; 48895110
        int 3
        mov            [rsp+08h],rbx                    ; 48895c2408
        int 3
        mov            [rsp+018h],rsi                   ; 4889742418
        int 3
        mov            [rsp+08h],rdi                    ; 48897c2408
        int 3
        mov            [rsp+010h],rdi                   ; 48897c2410
        int 3
        mov            [rcx+098h],rax                   ; 48898198000000
        int 3
        mov            [rcx+080h],rcx                   ; 48898980000000
        int 3
        mov            [rcx+088h],rdx                   ; 48899188000000
        int 3
        mov            [rcx+090h],rbx                   ; 48899990000000
        int 3
        mov            [rcx+0a0h],rbp                   ; 4889a9a0000000
        int 3
        mov            [rcx+0a8h],rsi                   ; 4889b1a8000000
        int 3
        mov            [rcx+0b0h],rdi                   ; 4889b9b0000000
        int 3
        mov            rax,[rcx]                        ; 488b01
        int 3
        mov            rax,[rcx+rdx]                    ; 488b0411
        int 3
        mov            rax,[value]                      ; 488b05318c0100
        int 3
        mov            rcx,[rsp]                        ; 488b0c24
        int 3
        mov            rsi,[rsp]                        ; 488b3424
        int 3
        mov            rdi,[rsp]                        ; 488b3c24
        int 3
        mov            rax,[rax+018h]                   ; 488b4018
        int 3
        mov            rax,[rcx+078h]                   ; 488b4178
        int 3
        mov            rax,[rdx+020h]                   ; 488b4220
        int 3
        mov            rax,[rsp+08h]                    ; 488b442408
        int 3
        mov            rcx,[rcx+08h]                    ; 488b4908
        int 3
        mov            rcx,[rsp+020h]                   ; 488b4c2420
        int 3
        mov            rdx,[rsp+08h]                    ; 488b542408
        int 3
        mov            rdi,[rsp+08h]                    ; 488b7c2408
        int 3
        mov            rax,[rcx+098h]                   ; 488b8198000000
        int 3
        mov            rax,[rcx+0f8h]                   ; 488b81f8000000
        int 3
        cmp            ebx,0                            ;
        int 3
        cmp            rbx,0                            ;
        int 3
        cmp            byte ptr [value],77h             ; 803d........77
        int 3
        cmp            dword ptr [value],77h            ; 833d........77
        int 3
        cmp            qword ptr [value],77h            ; 48833d........77
        int 3
        cmp            dword ptr [value],77777777h      ; 813d........77777777
        int 3
        cmp            qword ptr [value],77777777h      ; 48813d........77777777
        int 3
nearby:
        jo      nearby                                  ; 70xx
        int 3
        jno     nearby                                  ; 71xx
        int 3
        jb      nearby                                  ; 72xx
        int 3
        jae     nearby                                  ; 73xx
        int 3
        je      nearby                                  ; 74xx
        int 3
        jne     nearby                                  ; 75xx
        int 3
        jbe     nearby                                  ; 76xx
        int 3
        ja      nearby                                  ; 77xx
        int 3
        js      nearby                                  ; 78xx
        int 3
        jns     nearby                                  ; 79xx
        int 3
        jp      nearby                                  ; 7axx
        int 3
        jnp     nearby                                  ; 7bxx
        int 3
        jl      nearby                                  ; 7cxx
        int 3
        jge     nearby                                  ; 7dxx
        int 3
        jle     nearby                                  ; 7exx
        int 3
        jg      nearby                                  ; 7fxx
        int 3
        jmp     nearby                                  ; ebxx

        int 3
        jo      faraway                                 ; 0f80xxxxxxxx
        int 3
        jno     faraway                                 ; 0f81xxxxxxxx
        int 3
        jb      faraway                                 ; 0f82xxxxxxxx
        int 3
        jae     faraway                                 ; 0f83xxxxxxxx
        int 3
        je      faraway                                 ; 0f84xxxxxxxx
        int 3
        jne     faraway                                 ; 0f85xxxxxxxx
        int 3
        jbe     faraway                                 ; 0f86xxxxxxxx
        int 3
        ja      faraway                                 ; 0f87xxxxxxxx
        int 3
        js      faraway                                 ; 0f88xxxxxxxx
        int 3
        jns     faraway                                 ; 0f89xxxxxxxx
        int 3
        jp      faraway                                 ; 0f8axxxxxxxx
        int 3
        jnp     faraway                                 ; 0f8bxxxxxxxx
        int 3
        jl      faraway                                 ; 0f8cxxxxxxxx
        int 3
        jge     faraway                                 ; 0f8dxxxxxxxx
        int 3
        jle     faraway                                 ; 0f8exxxxxxxx
        int 3
        jg      faraway                                 ; 0f8fxxxxxxxx
        int 3
        jmp     faraway                                 ; e9xxxxxxxx

        int 3
        lea     rax,[rsp]                               ; 488d0424        
        int 3
        mov rcx,0BADC0DEBA5Eh                           ;  48b95ebadec0ad0b0000 
        int 3
        cmp     rax,rcx                                 ;  483bc1

        int 3
        sub rsp, 28h
        int 3
        add rsp,28h
        int 3
        ret
        int 3


;; The list is terminated by two "int 3" in a row.
again:
        int 3
        int 3
TestCodes   ENDP

value   QWORD   0

_TEXT   ENDS
END
