#!/usr/bin/env bash

set -e

if [ "$(uname -m)" != "aarch64" ]; then
  echo "This script is only for aarch64 architecture."
  exit 1
fi

bash scripts/docker/build.sh

docker push igncp/market-sim:latest

echo "Deployment complete."
