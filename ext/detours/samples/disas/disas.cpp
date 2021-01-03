/////////////////////////////////////////////////////////////////////////////
//
//  Module: disas.cpp (disas.exe - Detours Test Program)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//

#define DETOURS_INTERNAL
#include <detours.h>
#include <stdio.h>
#include <stdlib.h>

///////////////////////////////////////////////////////////////////////// ARM.
//
#ifdef DETOURS_ARM

extern "C" BYTE TestCodes[];

void DumpMemoryFragment(PBYTE pbData, ULONG cbData, ULONG cbSpace)
{
    ULONG n = 0;
    if (cbData >= 4) {
        printf("%04x%04x ", ((PUSHORT)pbData)[0], ((PUSHORT)pbData)[1]);
        n += 4;
    }
    else if (cbData >= 2) {
        printf("%04x ", *((PUSHORT)pbData));
        n += 2;
    }

    for (; n < cbSpace; n++) {
        if (n < cbData) {
            printf("%02x", pbData[n]);
        }
        else {
            printf("  ");
        }
    }
    if (n < cbData) {
        printf(".");
    }
    else {
        printf(" ");
    }
}

inline ULONG fetch_thumb_opcode(PBYTE pbCode)
{
    ULONG Opcode = *(UINT16 *)&pbCode[0];
    if (Opcode >= 0xe800) {
        Opcode = (Opcode << 16) | *(UINT16 *)&pbCode[2];
    }
    return Opcode;
}

BOOL IsTerminate(PBYTE pbSrc)
{
    ULONG opcode = fetch_thumb_opcode(pbSrc);

    if ((opcode & 0xff87) == 0x4700) {
        // bx r
        return TRUE;
    }

#if 0
    if ((opcode & 0xfbf08f00) == 0xf2400c00) {          // movw r12,#xxxx
        return TRUE;
    }

    if ((opcode == 0xf8dcf000) {                 // ldr  pc,[r12]
                ULONG Immediate = ((opcode2 << 12) & 0xf7000000) |
                                  ((opcode2 <<  1) & 0x08000000) |
                                  ((opcode2 << 16) & 0x00ff0000) |
                                  ((opcode  >>  4) & 0x0000f700) |
                                  ((opcode  >> 15) & 0x00000800) |
                                  ((opcode  >>  0) & 0x000000ff);
                PBYTE pbTarget = *(PBYTE *)Immediate;
                if (detour_is_imported(pbCode, pbTarget)) {
                    PBYTE pbNew = *(PBYTE *)pbTarget;
                    DETOUR_TRACE(("%p->%p: skipped over import table.\n", pbCode, pbNew));
                    return pbNew;
                }
            }
        }
    }
#endif

    return FALSE;
}

#endif // DETOURS_ARM

///////////////////////////////////////////////////////////////// X86 and X64.
//
#if defined(DETOURS_X86) || defined(DETOURS_X64)

extern "C" BYTE TestCodes[];

void DumpMemoryFragment(PBYTE pbData, ULONG cbData, ULONG cbSpace)
{
    ULONG n = 0;
    for (; n < cbSpace; n++) {
        if (n < cbData) {
            printf("%02x", pbData[n]);
        }
        else {
            printf("  ");
        }
    }
    if (n < cbData) {
        printf(".");
    }
    else {
        printf(" ");
    }
}

BOOL IsTerminate(PBYTE pbSrc)
{
    if ((0xC3 == pbSrc[0] && 0x00 == pbSrc[1]) ||       // bx lr
        0xCB == pbSrc[0] ||                             // RETF
        0xC2 == pbSrc[0] ||                             // RET dw
        0xCA == pbSrc[0] ||                             // RETF dw
        0xEB == pbSrc[0] ||                             // JMP ob
        0xE9 == pbSrc[0] ||                             // JMP ol
        0xEA == pbSrc[0]) {                             // JMP ol

        return TRUE;
    }
    if (0xff == pbSrc[0] && 0x25 == pbSrc[1])           // JMP [addr]
        return TRUE;
    return FALSE;
}

#endif // DETOURS_X86 || DETOURS_X64

/////////////////////////////////////////////////////////// X86, X64, and ARM.
//
#if defined(DETOURS_X86) || defined(DETOURS_X64) || defined(DETOURS_ARM)
struct BasicBlockLink
{
  public:
    BasicBlockLink *    m_pNext;
    PBYTE               m_pbEntry;
    PCHAR               m_pszName;

  public:
    BasicBlockLink(PBYTE pbEntry, PCHAR pszName = NULL)
    {
        m_pNext = NULL;
        m_pbEntry = pbEntry;
        m_pszName = pszName;

        *s_ppTail = this;
        s_ppTail = &m_pNext;
    }

    BasicBlockLink * Next()
    {
        return m_pNext;
    }

    static BasicBlockLink * GetListHead()
    {
        return s_pHead;
    }

  protected:
    static BasicBlockLink *     s_pHead;
    static BasicBlockLink **    s_ppTail;
};

BasicBlockLink *    BasicBlockLink::s_pHead = NULL;
BasicBlockLink **   BasicBlockLink::s_ppTail = &BasicBlockLink::s_pHead;

static PBYTE s_pbBegin = NULL;
static PBYTE s_pbLimit = NULL;

int TestDetourCopyInstruction(PBYTE pbSrcInstruction, PCHAR pszFunction)
{
    PBYTE pbSrc = pbSrcInstruction;
    ULONG nIns = 0;

    if (pszFunction) {
        printf("%s:\n", pszFunction);
    }
    for (; nIns < 4096; nIns++) {
        BYTE rbDst[128];
        PVOID pbDstPool = (PVOID)(rbDst + sizeof(rbDst));
        LONG lExtra = 0;
        PVOID pbTarget = NULL;
        ULONG cbStep = (ULONG)((PBYTE)DetourCopyInstruction(rbDst, &pbDstPool, pbSrc,
                                                            &pbTarget, &lExtra) - pbSrc);

        printf("    %p:", pbSrc);
        DumpMemoryFragment(rbDst, cbStep, 10);
        printf(" ");
        DumpMemoryFragment(rbDst, cbStep, 10);
        if (pbTarget) {
            if (pbTarget == DETOUR_INSTRUCTION_TARGET_DYNAMIC) {
                printf("  Dynamic\n");
            }
            else {
                printf(" %p%c\n", pbTarget,
                       (pbTarget >= s_pbBegin && pbTarget < s_pbLimit) ? ' ' : '!');
            }
        }
        else {
            printf("\n");
        }

        if (pbTarget && pbTarget != DETOUR_INSTRUCTION_TARGET_DYNAMIC) {
            if (pbTarget > pbSrc &&
                pbTarget >= s_pbBegin &&
                pbTarget < s_pbLimit
               ) {
                (void) new BasicBlockLink((PBYTE)pbTarget, NULL);
            }
        }

        if (IsTerminate(pbSrc)) {
            break;
        }

        pbSrc += cbStep;
    }
    return nIns;
}

BOOL CALLBACK ExportCallback(_In_opt_ PVOID pContext,
                             _In_ ULONG nOrdinal,
                             _In_opt_ LPCSTR pszName,
                             _In_opt_ PVOID pCode)
{
    (void)pContext;
    (void)nOrdinal;
    (void)pCode;

    (VOID) new BasicBlockLink((PBYTE)pCode, pszName ? pszName : "[NO NAME]");
    return TRUE;
}
#endif // DETOURS_X86 || DETOURS_X64

//////////////////////////////////////////////////////////////////////// IA64.
//
#ifdef DETOURS_IA64
#pragma warning(disable: 4201)  // ignore warning about unnamed sturcture in union.

void DumpHi(PBYTE pbData, ULONG cbData, ULONG cbSpace)
{
    ULONG n = 0;
    for (; n < cbSpace; n++) {
        if (n < cbData) {
            printf("%02x", pbData[(cbData - 1) - n]);
        }
        else {
            printf("  ");
        }
    }
    printf("\n");
}

struct DETOUR_IA64_BUNDLE_DISASSEMBLE : public DETOUR_IA64_BUNDLE
{
  public:
    void SetBrx(UINT64 raw)
    {
        SetBrl();
        SetBrlImm(raw);
    }

    void Dis()
    {
        const char szUnitNames[17] = "?aimbflx?AIMBFLX";

        printf("%p: ", data);
        BYTE nTemplate = GetTemplate();
        BYTE nInst0 = GetInst0();
        BYTE nInst1 = GetInst1();
        BYTE nInst2 = GetInst2();
        BYTE nUnit0 = GetUnit0();
        BYTE nUnit1 = GetUnit1();
        BYTE nUnit2 = GetUnit2();
        if (nUnit1 == L_UNIT) { // MLX instruction
            UINT64 d2 = (
                         //          0x0000000000fffff0
                         ((wide[1] & 0x00fffff000000000) >> 32) |
                         //          0x000000ffff000000
                         ((wide[0] & 0xffff000000000000) >> 24) |
                         //          0x7fffff0000000000
                         ((wide[1] & 0x00000000007fffff) << 40) |
                         //          0x8000000000000000
                         ((wide[1] & 0x0800000000000000) <<  4)
                        );
            printf("%02x %c%01x %010I64lx %c%01x %016I64lx",
                   nTemplate,
                   szUnitNames[nUnit0], nInst0, GetData0(),
                   szUnitNames[nUnit2], nInst2, d2);
        }
        else {
            printf("%02x %c%01x %010I64lx %c%01x %010I64lx %c%01x %010I64lx",
                   nTemplate,
                   szUnitNames[nUnit0], nInst0, GetData0(),
                   szUnitNames[nUnit1], nInst1, GetData1(),
                   szUnitNames[nUnit2], nInst2, GetData2());
        }

        if (IsBrl()) {
            printf(" brl  %p", GetBrlTarget());
        }
        else if (IsMovlGp()) {
            printf(" movl gp=%p", GetMovlGp());
        }
        if ((wide[0] & 0xfffffc000603ffff) == 0x002024000200100b &&
            wide[1] == 0x0004000000203008) {

            ULONG64 offset =
                ((wide[0] & 0x0000000001fc0000) >> 18) |  // imm7b
                ((wide[0] & 0x000001ff00000000) >> 25) |  // imm9d
                ((wide[0] & 0x00000000f8000000) >> 11);   // imm5c
            if (wide[0] & 0x0000020000000000) {
                offset |= 0xffffffffffe00000;
            }
            printf(" imm=%016I64lx", offset);
        }
        printf("\n");
    }
};

//////////////////////////////////////////////////////////////////////////////
//
BOOL CALLBACK ExportCallbackIA64(_In_opt_ PVOID pContext,
                                 _In_ ULONG nOrdinal,
                                 _In_opt_ LPCSTR pszName,
                                 _In_opt_ PVOID pCode)
{
    (void)pContext;
    (void)nOrdinal;

    DETOUR_IA64_BUNDLE_DISASSEMBLE *pb = *(DETOUR_IA64_BUNDLE_DISASSEMBLE **)pCode;
    DETOUR_IA64_BUNDLE temp;

    if (!pb[0].Copy(&temp)) {
        printf("%s:\n  ", pszName ? pszName : "[NO NAME]");
        pb[0].Dis();
    }
    return TRUE;
}

#if 0
void TestBoth()
{
    LPVOID pvBase = VirtualAlloc((PBYTE)0x800000000, 0x10000,
                                 MEM_RESERVE | MEM_COMMIT,
                                 PAGE_EXECUTE_READWRITE);

    DETOUR_IA64_BUNDLE *pbBase = (DETOUR_IA64_BUNDLE *)pvBase;
    DETOUR_IA64_BUNDLE *pb = pbBase;

    printf("TestBoth:\n");
    for (UINT64 i = 0x10; i < 0x8000000000000000; i <<= 1) {
        pb->SetMovlGp(i);
        if (pb->GetMovlGp() != i) {
            printf("Error in MovlGp!\n");
            return;
        }
        pb++;

        pb->SetBrl(i);
        if (pb->GetBrlEip() != i) {
            printf("Error in Brl!\n");
            return;
        }
        pb++;
    }

    for (UINT64 i = (UINT64)(INT64)-0x10; i > 0; i <<= 1) {
        pb->SetMovlGp(i);
        if (pb->GetMovlGp() != i) {
            printf("Error in MovlGp!\n");
            return;
        }
        pb++;

        pb->SetBrl(i);
        if (pb->GetBrlEip() != i) {
            printf("Error in Brl!\n");
            return;
        }
        pb++;
    }

    printf("u %p %p\n", pbBase, pb);
}
#endif
#endif // DETOURS_IA64

int WINAPI WinMain(HINSTANCE hinst, HINSTANCE hprev, LPSTR lpszCmdLine, int nCmdShow)
{
    (void)hprev;
    (void)hinst;
    (void)lpszCmdLine;
    (void)nCmdShow;

    // Bug report, but it works here.
    // 07ff8`4b783054 49ba 70b3d93a d40fb998 mov r10,98B90FD43AD9B370h
    //
    {
        static const UCHAR mov_r10_imm64[] = {0x49, 0xba, 1, 2, 3, 4, 5, 6, 7, 8 };

        PVOID const after = DetourCopyInstructionX64(0, 0, const_cast<PUCHAR>(mov_r10_imm64), 0, 0);

        if (after != &mov_r10_imm64 + 1)
        {
            printf("mov_r10_imm64 failed, expected:%p vs. got:%p\n", &mov_r10_imm64 + 1, after);
            if (IsDebuggerPresent())
            {
                __debugbreak();
                DetourCopyInstructionX64(0, 0, const_cast<PUCHAR>(mov_r10_imm64), 0, 0);
            }
            return 1;
        }
    }

#ifdef DETOURS_IA64
    // First we check the pre-canned TestCodes from disasm.asm
    //
    PBYTE pbTest = *(PBYTE*)WinMain;
    for (;; pbTest += 16) {
        DETOUR_IA64_BUNDLE_DISASSEMBLE *pb = (DETOUR_IA64_BUNDLE_DISASSEMBLE *)pbTest;

        pb->Dis();
        if (pbTest[0] == 0xff) {
            break;
        }
        DumpHi(pbTest, 16, 16);
    }

#if 0
    printf("\n\n");

    DETOUR_IA64_BUNDLE_DISASSEMBLE *pb = (DETOUR_IA64_BUNDLE_DISASSEMBLE *)pbTest;
    DETOUR_IA64_BUNDLE_DISASSEMBLE *pbBeg = pb;
    DWORD dwOld;
    VirtualProtect(pb, 0x2000, PAGE_EXECUTE_READWRITE, &dwOld);
    printf("%p: (%d)\n", pb, sizeof(pb));
    pb++;
    printf("%p: (%d)\n", pb, sizeof(pb));
    pb++; pb->SetBrx(0);
    pb++; pb->SetBrx(0);
    pb++; pb->SetBrx(0);
    pb++; pb->SetBrx(0xffffffffffffffff);
    pb++; pb->SetBrx(0x0fffffffffffffff);
    pb++; pb->SetBrx(0x00ffffffffffffff);
    pb++; pb->SetBrx(0x000fffffffffffff);
    pb++; pb->SetBrx(0x0000ffffffffffff);
    pb++; pb->SetBrx(0x00000fffffffffff);
    pb++; pb->SetBrx(0x000000ffffffffff);
    pb++; pb->SetBrx(0x0000000fffffffff);
    pb++; pb->SetBrx(0x00000000ffffffff);
    pb++; pb->SetBrx(0x000000000fffffff);
    pb++; pb->SetBrx(0x0000000000ffffff);
    pb++; pb->SetBrx(0x00000000000fffff);
    pb++; pb->SetBrx(0x000000000000ffff);
    pb++; pb->SetBrx(0x0000000000000fff);
    pb++; pb->SetBrx(0x00000000000000ff);
    pb++; pb->SetBrx(0x000000000000000f);
    pb++; pb->SetBrx(0x0000000000000000);
    pb++; pb->SetBrx(0xffffffffffffffff);
    pb++; pb->SetBrx(0xffffffffffffffff);
    pb->SetInst0(0xff);
    pb->SetData0(0xffffffffffffffff);
    printf("%p:\n", pb);
    DETOUR_IA64_BUNDLE_DISASSEMBLE *pbEnd = pb;
    for (pb = pbBeg; pb < pbEnd; pb++) {
        printf("  %p: ", pb);
        DumpHi((BYTE*)pb, 16, 16);
    }
#endif

#if 1
    {
        // Then we check all of the code we can find in user32.dll
        //
        printf("\n");
        HINSTANCE hInst = LoadLibraryA("user32.dll");
        printf("Loaded: user32.dll: %p\n", hInst);

        PBYTE pbEntry = (PBYTE)DetourGetEntryPoint(hInst);
        printf("Entry: %p\n", pbEntry);
        ExportCallbackIA64(NULL, 0, "[Entry]", pbEntry);
        DetourEnumerateExports(hInst, NULL, ExportCallbackIA64);
    }

    {
        // Then we check all of the code we can find in opengl32.dll
        //
        printf("\n");
        HINSTANCE hInst = LoadLibraryA("opengl32.dll");
        printf("Loaded: opengl32.dll: %p\n", hInst);

        PBYTE pbEntry = (PBYTE)DetourGetEntryPoint(hInst);
        printf("Entry: %p\n", pbEntry);
        ExportCallbackIA64(NULL, 0, "[Entry]", pbEntry);
        DetourEnumerateExports(hInst, NULL, ExportCallbackIA64);
    }

    printf("\n");
    for (HINSTANCE hInst = NULL; (hInst = DetourEnumerateModules(hInst)) != NULL;) {
        CHAR szModuleName[512];
        GetModuleFileNameA(hInst, szModuleName,
                           sizeof(szModuleName)/sizeof(szModuleName[0]));
        printf("%p : %s\n", hInst, szModuleName);
        DetourEnumerateExports(hInst, NULL, ExportCallbackIA64);
    }

    printf("\n");
#endif
#if 0
    TestBoth();
#endif
#endif // DETOURS_IA64

#if defined(DETOURS_X64) || defined(DETOURS_X86)
    // First we check the pre-canned TestCodes from disasm.asm
    //
    PBYTE pbBegin = (PBYTE)DetourCodeFromPointer(TestCodes, NULL);
    printf("%p:\n", pbBegin);
    for (PBYTE pbTest = pbBegin;;) {
        if (pbTest[0] != 0xcc) {    // int 3
            printf("%08lx  ", (ULONG)(pbTest - pbBegin));
            DumpMemoryFragment(pbTest, 8, 8);
            printf("\n");
            printf("failed on last.\n");
            return 1;
        }
        pbTest++;

        if (pbTest[0] == 0x70 || pbTest[0] == 0x71) {
            printf("[%p]:\n", pbTest);
        }
        BYTE rbDst[128];
        PVOID pbDstPool = (PVOID)(rbDst + sizeof(rbDst));
        LONG lExtra = 0;
        PVOID pbTarget = NULL;
        PBYTE pbNext = (PBYTE)DetourCopyInstruction(rbDst, &pbDstPool, pbTest,
                                                    &pbTarget, &lExtra);

        LONG cbTest = (LONG)(pbNext - pbTest);

        printf("%08lx  ", (ULONG)(pbTest - pbBegin));
        DumpMemoryFragment(pbTest, cbTest, 12);
        printf("[%16p] ", pbTarget);
        DumpMemoryFragment(rbDst, cbTest + lExtra, 11);
        printf("\n");

        if (pbTest[cbTest] != 0xcc) {
            printf("failed!\n");
            return 1;
        }

        pbTest += cbTest;

        if (pbTest[0] == 0xcc && pbTest[1] == 0xcc) {
            break;
        }
    }

#if 0
    // Then we check all of the code we can find in user32.dll
    //
    HINSTANCE hInst = LoadLibraryA("user32.dll");
    printf("Loaded: user32.dll: %p\n", hInst);

    s_pbBegin = (PBYTE)hInst;
    s_pbLimit = s_pbBegin + DetourGetModuleSize(hInst);

    PBYTE pbEntry = DetourGetEntryPoint(hInst);
    (VOID) new BasicBlockLink(pbEntry, "user32.dll");

    DetourEnumerateExports(hInst, NULL, ExportCallback);

    ULONG nIns = 0;
    for (BasicBlockLink *pLink = BasicBlockLink::GetListHead();
         pLink; pLink = pLink->Next()) {

        nIns += TestDetourCopyInstruction(pLink->m_pbEntry, pLink->m_pszName);
        if (nIns > 100000) {
            break;
        }
    }
    printf("Disassembled %d instructions.\n", nIns);
#endif
#endif // DETOURS_X86 || DETOURS_X64

#ifdef DETOURS_ARM
    // Create an output buffer and fill it with debugbreaks.
    //
    PBYTE pbBuffer
        = (PBYTE)VirtualAlloc(NULL, 0x400, MEM_COMMIT|MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    for (PBYTE pbOut = pbBuffer; pbOut < pbBuffer + 0x400;) {
        *pbOut++ = 0xfe;
        *pbOut++ = 0xde;
    }
    PBYTE pbDst = pbBuffer;
    PVOID pvDstPool = (PVOID)(pbBuffer + 0x400);

    // First we check the pre-canned TestCodes from disasm.asm
    //
    PBYTE pbBegin = (PBYTE)DetourCodeFromPointer(TestCodes, NULL);
    printf("%p: (TestCodes %p) => %p\n", pbBegin, TestCodes, pbBuffer);
    for (PBYTE pbSrc = pbBegin;;) {
        if (pbSrc[0] != 0xfe && pbSrc[1] != 0xde) {    // BREAK
            printf("%08x  ", pbSrc - pbBegin);
            DumpMemoryFragment(pbSrc, 8, 8);
            printf("\n");
            printf("failed on last.\n");
            return 1;
        }
        pbSrc += 2;
        *pbDst++ = 0xfe;
        *pbDst++ = 0xde;

        if ((pbSrc[0] == 0x00 && pbSrc[1] == 0xbf) &&  // NOP
            (pbSrc[2] != 0xfe && pbSrc[3] != 0xde)) {    // BREAK
            // Skip over a single NOP so we can test alignment.
            pbSrc += 2;
        }

        if ((pbSrc[0] == 0x00 && pbSrc[1] == 0xbf) &&  // NOP
            (pbSrc[2] != 0xfe && pbSrc[3] != 0xde)) {    // BREAK
            // If there is a second NOP, then we insert alignment.
            pbSrc += 2;
            *pbDst++ = 0x00;
            *pbDst++ = 0xbf;
        }


        LONG lExtra = 0;
        PVOID pbTarget = NULL;
        PBYTE pbNext = (PBYTE)DetourCopyInstruction(pbDst, &pvDstPool, pbSrc, &pbTarget, &lExtra);

        LONG cbTest = (LONG)(pbNext - pbSrc);

        printf("%08x  ", pbSrc - pbBegin);
        DumpMemoryFragment(pbSrc, cbTest, 4);
        printf("[%8p] ", pbTarget);
        DumpMemoryFragment(pbDst, cbTest + lExtra, 16);
        printf("\n");

        if (pbSrc[cbTest] != 0xfe || pbSrc[cbTest+1] != 0xde) {
            printf("%p: failed! (pbSrc[n]=%02x, pbSrc[n+1]=%02x\n",
                   pbSrc,
                   pbSrc[cbTest], pbSrc[cbTest+1]);
            __debugbreak();
            pbNext = (PBYTE)DetourCopyInstruction(pbDst, &pvDstPool, pbSrc, &pbTarget, &lExtra);
            cbTest = (LONG)(pbNext - pbSrc);
            return 1;
        }

        pbDst += cbTest + lExtra;
        pbSrc += cbTest;

        if (pbSrc[0] == 0xfe && pbSrc[1] == 0xde &&
            pbSrc[2] == 0xfe && pbSrc[3] == 0xde) {
            break;
        }
    }

#if 0
    // Then we check all of the code we can find in user32.dll
    //
    HINSTANCE hInst = LoadLibraryA("user32.dll");
    printf("Loaded: user32.dll: %p\n", hInst);

    s_pbBegin = (PBYTE)hInst;
    s_pbLimit = s_pbBegin + DetourGetModuleSize(hInst);

    PBYTE pbEntry = DetourGetEntryPoint(hInst);
    (VOID) new BasicBlockLink(pbEntry, "user32.dll");

    DetourEnumerateExports(hInst, NULL, ExportCallback);

    ULONG nIns = 0;
    for (BasicBlockLink *pLink = BasicBlockLink::GetListHead();
         pLink; pLink = pLink->Next()) {

        nIns += TestDetourCopyInstruction(pLink->m_pbEntry, pLink->m_pszName);
        if (nIns > 100000) {
            break;
        }
    }
    printf("Disassembled %d instructions.\n", nIns);
#endif
#endif // DETOURS_ARM

    return 0;
}
//
///////////////////////////////////////////////////////////////// End of File.
