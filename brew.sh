#!/bin/bash

# Check if a flavor argument was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <flavor_name>"
  exit 1
fi

FLAVOR="$1"
FLAVOR_DIR="./flavors/$FLAVOR"

# Check if the directory for the flavor exists
if [ ! -d "$FLAVOR_DIR" ]; then
  echo "Flavor '$FLAVOR' not found. Available flavors:"
  ls ./flavors/
  exit 1
fi

# Build the Docker image for the specified flavor
echo "Building Docker image for flavor '$FLAVOR'..."
docker build -t "tea:$FLAVOR" -f "$FLAVOR_DIR/Dockerfile" .

