#!/usr/bin/env bash

set -e

cargo watch \
  -i 'docs/*' \
  -x 'run -- start -f'
