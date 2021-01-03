//////////////////////////////////////////////////////
//
//  Unit Test Image Corruptor (corruptor.h of unittests.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#pragma once

class ImageCorruptor final
{
public:

    ImageCorruptor(PIMAGE_DOS_HEADER Header);

    ~ImageCorruptor();

    void ModifyDosMagic(WORD Value);

    void ModifyNtSignature(ULONG Value);

private:
    // Pointer to the target image header to corrupt.
    //
    PIMAGE_DOS_HEADER m_TargetDosHeader;

    // Cached copy of the DOS header, to restore state with.
    //
    IMAGE_DOS_HEADER m_OriginalDosHeader;

    // The original protection of the DOS header.
    //
    DWORD m_OriginalDosProtection;

    // Pointer to the target NT image header to corrupt.
    //
    PIMAGE_NT_HEADERS m_TargetNtHeaders;

    // Cached copy of the NT headers, to restore state with.
    //
    IMAGE_NT_HEADERS m_OriginalNtHeaders;

    // The original protection of the NT headers.
    //
    DWORD m_OriginalNtProtection;
};
