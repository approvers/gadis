#!/bin/sh

pkill gadis_provider || echo "[Warning] Killing gadis_bot failed; ignoring exit code"
mkdir /home/ec2-user/bot/executable_backups

PREFIX="$(date '+%Y%m%d-%H%M%S')-$RANDOM" mv /home/ec2-user/bot/gadis_provider /home/ec2/bot/executable_backups/$PREFIX-gadis_provider
