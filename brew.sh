#!/bin/bash

#
#
# -- Brew (Image) creator handler
#
#

# Print a logo
cat ./misc/logo_redstraw_nocap

# Function to list available flavors with their descriptions
list_flavors() {
  echo "Available flavors:"
  for dir in ./flavors/*/; do
    if [ -f "${dir}description.txt" ]; then
      echo ""
      echo "🍵 Flavor: $(basename "$dir")"
      cat "${dir}description.txt"
    else
      echo ""
      echo "🍵 Flavor: $(basename "$dir") (no description.txt found)"
    fi
  done
}

# Check if a flavor argument was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <flavor_name>"
  echo ""
  list_flavors
  exit 1
fi

FLAVOR="$1"
FLAVOR_DIR="./flavors/$FLAVOR"

# Check if the directory for the flavor exists
if [ ! -d "$FLAVOR_DIR" ]; then
  echo "Flavor '$FLAVOR' not found."
  echo ""
  list_flavors
  exit 1
fi

# Build the Docker image for the specified flavor
echo "Brewing flavor: '$FLAVOR'..."
docker build -t "tea:$FLAVOR" -f "$FLAVOR_DIR/Dockerfile" .

