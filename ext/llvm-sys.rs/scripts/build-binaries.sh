#!/bin/sh
set -e

if [ $? -lt 1 ]
then
    echo "Usage: $0 <version>" >&2
    echo "Example: $0 3.9.1" >&2
    exit 1
fi

VERSION=$1

# Dependencies (for Ubuntu):
#  * wget
#  * xz-utils
#  * ninja-build
#  * cmake
#  * build-essential
#  * python

wget http://releases.llvm.org/$VERSION/llvm-$VERSION.src.tar.xz
tar xJf llvm-$VERSION.src.tar.xz
mkdir build llvm-$VERSION
cd build
cmake -G Ninja ../llvm-$VERSION.src -DLLVM_TARGETS_TO_BUILD=X86 -DCMAKE_BUILD_TYPE=MinSizeRel -DLLVM_ENABLE_ASSERTIONS=ON -DCMAKE_INSTALL_PREFIX=/usr/local/llvm-$VERSION -DCMAKE_INSTALL_UTILS
cmake --build . --target install
cd ..
tar cJf llvm-$VERSION.linux.tar.xz /usr/local/llvm-$VERSION

# Additional flags for MSVC
#  (CXX) /GL /Gy /Gw
#  (link) /LTCG /OPT:REF,ICF
