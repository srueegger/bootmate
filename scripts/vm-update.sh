#!/usr/bin/env bash
set -euo pipefail

multipass exec bootmate-dev -- bash -c "sudo apt-get update && sudo DEBIAN_FRONTEND=noninteractive apt-get -y full-upgrade"
