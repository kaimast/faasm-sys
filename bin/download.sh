#!/bin/bash

set -e

THIS_DIR=$(dirname $(readlink -f $0))

TARGET_DIR=${THIS_DIR}/../vendor/faasm-libs
FAASM_VERSION=0.0.9

SYSROOT_TAR=faasm-sysroot-${FAASM_VERSION}.tar.gz

LIB_URL=https://github.com/lsds/Faasm/releases/download/v${FAASM_VERSION}

function download_tar {
  echo ${LIB_URL}/$1
  curl -L ${LIB_URL}/$1 --output $1
  tar -xf $1
  rm $1
}

mkdir -p ${TARGET_DIR}
pushd ${TARGET_DIR} >> /dev/null

download_tar ${SYSROOT_TAR}

popd >> /dev/null