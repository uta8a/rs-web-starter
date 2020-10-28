#!/bin/sh
set -ue
timeout 30 sh -c "until nc -vz db 5432; do sleep 1; done" && /usr/local/bin/backend