#!/usr/bin/env sh

set -xe

cargo build --release && systemctl restart pluto.uz.service