#!/usr/bin/env bash

# Sample call:
# $0 /path/to/folder_with_binary
# This script replace the former dedicated Dockerfile
# and shows how to use the generic binary_injected.dockerfile

PROJECT_ROOT=`git rev-parse --show-toplevel`

export BINARY=kvp,kvp-execute-worker,kvp-prepare-worker
export BIN_FOLDER=$1

$PROJECT_ROOT/scripts/ci/dockerfiles/build-injected.sh
