#!/usr/bin/env bash

docker build \
  --load \
  --progress=plain \
  -t igncp/market-sim:latest \
  .
