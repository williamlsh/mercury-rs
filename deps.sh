#!/usr/bin/env bash

set -ex

sudo apt install -y \
    abi-compliance-checker \
    abi-dumper \
    build-essential \
    debhelper \
    fakeroot \
    gcc \
    git \
    libnl-3-200 libnl-3-dev libnl-route-3-200 libnl-route-3-dev \
    libnuma-dev \
    libudev-dev \
    uuid-dev \
    make \
    pandoc \
    pkg-config \
    python \
    rpm \
    sparse \
    valgrind \
    cmake-curses-gui

mkdir deps
pushd deps
git clone https://github.com/mercury-hpc/mercury.git
git clone --depth 1 -b stable-v42 https://github.com/linux-rdma/rdma-core.git
git clone --depth 1 -b v1.15.x https://github.com/ofiwg/libfabric.git
popd

pushd deps/rdma-core
mkdir build && cd build
cmake -D'CMAKE_BUILD_TYPE=RELEASE' -D'CMAKE_INSTALL_PREFIX=/usr/local' ..
make
sudo make install || :
sudo ldconfig
popd

pushd deps/libfabric
./autogen.sh
./configure --prefix=/usr/local \
    --enable-efa=../rdma-core/build \
    --enable-mrail \
    --enable-psm3=../rdma-core/build \
    --enable-rxd \
    --enable-rxm \
    --enable-shm \
    --enable-tcp \
    --enable-udp \
    --enable-usnic \
    --enable-verbs=../rdma-core/build \
    CC=gcc
make -j $(nproc)
sudo make install
sudo ldconfig
fi_info -l
popd

# Optional
pushd deps/mercury
git submodule update --init
mkdir build && cd build
cmake "-DBUILD_SHARED_LIBS=ON" "-DMERCURY_USE_BOOST_PP=ON" "-DNA_USE_OFI=ON" "-DCMAKE_INSTALL_PREFIX=/usr/local"  "-DCMAKE_BUILD_TYPE=Release" ..
make
sudo make install
sudo ldconfig
popd
