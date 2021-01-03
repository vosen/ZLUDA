//////////////////////////////////////////////////////////////////////////////
//
//  Unit Test Image Corruptor (corruptor.cpp of unittests.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include "windows.h"
#include "corruptor.h"

ImageCorruptor::ImageCorruptor(PIMAGE_DOS_HEADER Header)
{
    m_TargetDosHeader = Header;
    m_OriginalDosHeader = *Header;
    m_OriginalDosProtection = 0;
    m_TargetNtHeaders = (PIMAGE_NT_HEADERS)((PBYTE)Header + Header->e_lfanew);
    m_OriginalNtHeaders = *m_TargetNtHeaders;
    m_OriginalNtProtection = 0;

    VirtualProtect(
            m_TargetDosHeader,
            sizeof(*m_TargetDosHeader),
            PAGE_READWRITE,
            &m_OriginalDosProtection);

    VirtualProtect(
            m_TargetNtHeaders,
            sizeof(*m_TargetNtHeaders),
            PAGE_READWRITE,
            &m_OriginalNtProtection);
}

ImageCorruptor::~ImageCorruptor()
{
    // Restore original header contents.
    //
    *m_TargetDosHeader = m_OriginalDosHeader;
    *m_TargetNtHeaders = m_OriginalNtHeaders;

    // Restore original protection of DOS header.
    //
    DWORD OldProtection {};
    VirtualProtect(
            m_TargetDosHeader,
            sizeof(*m_TargetDosHeader),
            m_OriginalDosProtection,
            &OldProtection);

    // Restore original protection of NT headers.
    //
    VirtualProtect(
            m_TargetNtHeaders,
            sizeof(*m_TargetNtHeaders),
            m_OriginalNtProtection,
            &OldProtection);
}

void ImageCorruptor::ModifyDosMagic(WORD Value)
{
    m_TargetDosHeader->e_magic = Value;
}

void ImageCorruptor::ModifyNtSignature(ULONG Value)
{
    m_TargetNtHeaders->Signature = Value;
}
