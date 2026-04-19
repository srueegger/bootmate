#!/usr/bin/env bash
set -euo pipefail

distrobox enter bootmate-devsystem -- bash -lc "sudo dnf -y upgrade --refresh"
