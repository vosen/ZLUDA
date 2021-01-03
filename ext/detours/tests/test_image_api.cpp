//////////////////////////////////////////////////////////////////////////////
//
//  Unit Tests for Detours Image API (test_image_api.cpp of unittests.exe)
//
//  Microsoft Research Detours Package
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
#include "catch.hpp"
#include "windows.h"
#include "detours.h"

TEST_CASE("DetourBinaryOpen", "[image]")
{
    SECTION("Passing INVALID_HANDLE, results in error")
    {
        auto binary = DetourBinaryOpen(INVALID_HANDLE_VALUE);
        REQUIRE( GetLastError() == ERROR_INVALID_HANDLE );
        REQUIRE( binary == nullptr );
    }
}
