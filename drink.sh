#!/bin/bash

# Check if a flavor argument was provided
if [ -z "$1" ]; then
  echo "Usage: $0 <flavor_name>"
  exit 1
fi

FLAVOR="$1"
IMAGE_NAME="tea:$FLAVOR"
CONTAINER_NAME="tea-${FLAVOR}-container"

# Check if the image exists
IMAGE_EXISTS=$(docker images -q "$IMAGE_NAME")

# If the image does not exist, show available flavors
if [ -z "$IMAGE_EXISTS" ]; then
  echo "Image for flavor '$FLAVOR' not found. But you can brew these teas:"
  ls ./flavors/
  exit 1
fi

# Check if a container with the same name already exists
CONTAINER_EXISTS=$(docker ps -a -q -f name="$CONTAINER_NAME")

if [ -z "$CONTAINER_EXISTS" ]; then
  # If the container does not exist, create and start it
  echo "Container for flavor '$FLAVOR' not found. Creating and starting container..."
  docker run -it --name "$CONTAINER_NAME" --hostname "$FLAVOR" "$IMAGE_NAME" zsh
  echo "Container '$CONTAINER_NAME' started."
else
  # If the container exists, start it and attach to it
  echo "Container for flavor '$FLAVOR' already exists. Starting and attaching..."
  docker start "$CONTAINER_NAME"
  docker attach "$CONTAINER_NAME"
fi

