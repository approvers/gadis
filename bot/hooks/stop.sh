#!/bin/sh

pkill gadis_bot || echo "[Warning] Killing gadis_bot failed; ignoring exit code"
mkdir /home/ec2-user/bot/executable_backups

PREFIX="$(date '+%Y%m%d-%H%M%S')-$RANDOM" mv /home/ec2-user/bot/gadis_bot /home/ec2/bot/executable_backups/$PREFIX-gadis_bot
