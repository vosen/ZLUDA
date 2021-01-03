/////////////////////////////////////////////////////////////////////////////
//
//  Detours Test Program (x86.asm of disas.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

static int value = 0;

extern "C" void __declspec(naked) TestCodes()
{
    __asm {
// Each instruction is proceeded by an "int 3".
      faraway:
        int 3;
        nop;        // 1-byte NOP.
        int 3;
        _emit 0x66; // 2-byte NOP.
        _emit 0x90;
        int 3;
        _emit 0x0f; // 3-byte NOP.
        _emit 0x1f;
        _emit 0x00;
        int 3;
        _emit 0x0f; // 4-byte NOP.
        _emit 0x1f;
        _emit 0x40;
        _emit 0x00;
        int 3;
        _emit 0x0f; // 5-byte NOP.
        _emit 0x1f;
        _emit 0x44;
        _emit 0x00;
        _emit 0x00;
        int 3;
        _emit 0x66; // 6-byte NOP.
        _emit 0x0f;
        _emit 0x1f;
        _emit 0x44;
        _emit 0x00;
        _emit 0x00;
        int 3;
        _emit 0x0f; // 7-byte NOP.
        _emit 0x1f;
        _emit 0x80;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        int 3;
        _emit 0x0f; // 8-byte NOP.
        _emit 0x1f;
        _emit 0x84;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        int 3;
        _emit 0x66; // 9-byte NOP.
        _emit 0x0f;
        _emit 0x1f;
        _emit 0x84;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        _emit 0x00;
        int 3;
        mov     ecx, eax;
        int 3;
        mov     ebx, 0ffff000eh;
        int 3;
        call    ebx;
        int 3;
        call    dword ptr [eax];
        int 3;
        call    dword ptr [ebx];
        int 3;
        call    dword ptr [ecx];
        int 3;
        call    dword ptr [edx];
        int 3;
        jmp     dword ptr [eax];
        int 3;
        jmp     dword ptr [ebx];
        int 3;
        jmp     dword ptr [ecx];
        int 3;
        jmp     dword ptr [edx];
        int 3;
        call    ecx;
        int 3;
        call    eax;
        int 3;
        mov     ebx, 0ffff000eh;
        int 3;
        push    eax;
        int 3;
        call    ebx;
        int 3;
        cmp     ebx, 0;
        int 3;
        cmp     byte ptr [value], 77h;
        int 3;
        cmp     dword ptr [value], 77h;
        int 3;
        cmp     dword ptr [value], 77777777h;
      nearby:
        int 3
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
        jo      faraway                                 ; 0f80xx
        int 3
        jno     faraway                                 ; 0f81xx
        int 3
        jb      faraway                                 ; 0f82xx
        int 3
        jae     faraway                                 ; 0f83xx
        int 3
        je      faraway                                 ; 0f84xx
        int 3
        jne     faraway                                 ; 0f85xx
        int 3
        jbe     faraway                                 ; 0f86xx
        int 3
        ja      faraway                                 ; 0f87xx
        int 3
        js      faraway                                 ; 0f88xx
        int 3
        jns     faraway                                 ; 0f89xx
        int 3
        jp      faraway                                 ; 0f8axx
        int 3
        jnp     faraway                                 ; 0f8bxx
        int 3
        jl      faraway                                 ; 0f8cxx
        int 3
        jge     faraway                                 ; 0f8dxx
        int 3
        jle     faraway                                 ; 0f8exx
        int 3
        jg      faraway                                 ; 0f8fxx

// The list is terminated by two "int 3" in a row.
        int 3;
        int 3;
        ret;
    }
}

