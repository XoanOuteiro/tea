#!/usr/bin/env bash

#
#
# -- Create base image for tea
#
#

# Print logo
cat ./misc/logo_foam

docker build -t tea:base -f Dockerfile.base .

# Print thank you message
cat ./misc/thanks.txt

