//////////////////////////////////////////////////////////////////////////////
//
//  Unit Tests for Detours Module API (test_module_api.cpp of unittests.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include "catch.hpp"
#include "windows.h"
#include "detours.h"
#include "corruptor.h"

// Expose the image base of the current module for test assertions.
//
extern "C" IMAGE_DOS_HEADER __ImageBase;

// Expose default module entry point for test assertions.
//
extern "C" int mainCRTStartup();

// Dummy function pointer used for tests.
//
void NoopFunction() { }

TEST_CASE("DetourGetContainingModule", "[module]")
{
    SECTION("Passing nullptr, results in nullptr")
    {
        auto mod = DetourGetContainingModule(nullptr);
        REQUIRE( GetLastError() == ERROR_BAD_EXE_FORMAT );
        REQUIRE( mod == nullptr );
    }

    SECTION("Passing GetCommandLineW, results in kernel32 HMODULE")
    {
        auto mod = DetourGetContainingModule(GetCommandLineW);
        REQUIRE( GetLastError() == NO_ERROR );
        REQUIRE( mod == LoadLibrary("kernel32.dll") );
    }

    SECTION("Passing own function, results in own HMODULE")
    {
        auto mod = DetourGetContainingModule(NoopFunction);
        REQUIRE( GetLastError() == NO_ERROR );
        REQUIRE( mod == reinterpret_cast<HMODULE>(&__ImageBase) );
    }
}

TEST_CASE("DetourGetEntyPoint", "[module]")
{
    SECTION("Passing nullptr, results in CRT entrypoint")
    {
        auto entry = DetourGetEntryPoint(nullptr);
        REQUIRE( GetLastError() == NO_ERROR );
        REQUIRE( entry == mainCRTStartup );
    }

    SECTION("Passing nullptr, equals executing image")
    {
        REQUIRE( DetourGetEntryPoint(nullptr) ==
                 DetourGetEntryPoint(reinterpret_cast<HMODULE>(&__ImageBase)) );
    }

    SECTION("Passing ImageBase, results in CRT main")
    {
        auto entry = DetourGetEntryPoint(reinterpret_cast<HMODULE>(&__ImageBase));
        REQUIRE( GetLastError() == NO_ERROR );
        REQUIRE( entry == mainCRTStartup );
    }

    SECTION("Corrupt image DOS header magic, results in bad exe format error")
    {
        ImageCorruptor corruptor(&__ImageBase);
        corruptor.ModifyDosMagic(0xDEAD);

        auto entry = DetourGetEntryPoint(reinterpret_cast<HMODULE>(&__ImageBase));
        REQUIRE( GetLastError() == ERROR_BAD_EXE_FORMAT );
        REQUIRE( entry == nullptr );
    }

    SECTION("Corrupt image NT header signature, results in invalid signature error")
    {
        ImageCorruptor corruptor(&__ImageBase);
        corruptor.ModifyNtSignature(0xDEADBEEF);

        auto entry = DetourGetEntryPoint(reinterpret_cast<HMODULE>(&__ImageBase));
        REQUIRE( GetLastError() == ERROR_INVALID_EXE_SIGNATURE );
        REQUIRE( entry == nullptr );
    }
}

TEST_CASE("DetourGetModuleSize", "[module]")
{
    SECTION("Passing nullptr, results in current module size")
    {
        auto size = DetourGetModuleSize(nullptr);
        REQUIRE( GetLastError() == NO_ERROR );
        REQUIRE( size >= 0 );
    }

    SECTION("Passing stack, results in error")
    {
        int value;
        auto size = DetourGetModuleSize(reinterpret_cast<HMODULE>(&value));
        REQUIRE( GetLastError() == ERROR_BAD_EXE_FORMAT);
        REQUIRE( size == 0 );
    }

    SECTION("Passing nullptr, equals executing image")
    {
        REQUIRE( DetourGetModuleSize(nullptr) ==
                 DetourGetModuleSize(reinterpret_cast<HMODULE>(&__ImageBase)) );
    }

    SECTION("Corrupt image DOS header magic, results in bad exe format error")
    {
        ImageCorruptor corruptor(&__ImageBase);
        corruptor.ModifyDosMagic(0xDEAD);

        auto size = DetourGetModuleSize(reinterpret_cast<HMODULE>(&__ImageBase));
        REQUIRE( GetLastError() == ERROR_BAD_EXE_FORMAT );
        REQUIRE( size == 0 );
    }

    SECTION("Corrupt image NT header signature, results in invalid signature error")
    {
        ImageCorruptor corruptor(&__ImageBase);
        corruptor.ModifyNtSignature(0xDEADBEEF);

        auto size = DetourGetModuleSize(reinterpret_cast<HMODULE>(&__ImageBase));
        REQUIRE( GetLastError() == ERROR_INVALID_EXE_SIGNATURE );
        REQUIRE( size == 0 );
    }
}
