#!/bin/bash
set -ex
TEST_EXECUTABLES_DIR=$1
SUFFIX=$2

ls ${TEST_EXECUTABLES_DIR}/* | sort -u | while read -r executable; do
    output=$("$executable" --list 2>/dev/null)
    exit_code=$?
    if [ $exit_code -eq 0 ] && echo "$output" | grep -q "_${SUFFIX}: test$"; then
        mv "$executable" "${TEST_EXECUTABLES_DIR}/${SUFFIX}/"
    fi
done
