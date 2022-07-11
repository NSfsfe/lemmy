#!/bin/sh

# SPDX-FileCopyrightText: 2019-2022 2019 Felix Ableitner, <me@nutomic.com> et al.
#
# SPDX-License-Identifier: AGPL-3.0-only

set -e

# This script uses a Dockerfile that takes advantage of docker volume mounts,
# And runs on an ubuntu image. A little faster for development than the other
# script

mkdir -p volumes/pictrs
sudo chown -R 991:991 volumes/pictrs
sudo docker-compose down
sudo docker build ../../ --file ../dev/volume_mount.dockerfile -t lemmy-dev:latest
sudo docker-compose pull --ignore-pull-failures || true
sudo docker-compose up
