#!/bin/bash

#
#
# -- Create base image for tea
#
#

docker build -t tea:base -f Dockerfile.base .

