#!/bin/bash
set -eo pipefail
IFS=$'\n\t'

# This script is called from the GitHub action defined in build-docker-image.yml.
# It expects the working directory to be the repo root and 
# uses the following environment variables:
# DOCKER_REPO - if not present, the image will be built, but not pushed.
# APP_NAME - *required*, name as in package.json.
# APP_DIR - if different from APP_NAME without any @scope.
# COMMIT_HASH - defaults to 'unknown' Image will have this value as an env var.

# Set defaults.

COMMIT_HASH=${COMMIT_HASH:-unknown}

if [ -z "$APP_NAME" ]; then
  echo "WARNING: APP_NAME is required as an environment variable."
  exit 1
fi

# Set variable values.

APP_FULL_NAME="$APP_NAME"
APP_NAME=$(echo $APP_NAME | sed s/".*\/"//) # remove scope like "@my-org/", if any.

if [ -z "$APP_DIR" ]; then
  APP_DIR="$APP_NAME"
fi

if [ -z "$DOCKER_REPO" ]; then
  echo "DOCKER_REPO environment variable not set. Images will not be pulled or pushed."
  APP_IMAGE="$APP_NAME"
else
  APP_IMAGE="$DOCKER_REPO/$APP_NAME"
fi

BUILD_IMAGE=$APP_IMAGE-build
export TEMP_DEPS_DIR="./_tmp/deps"

# Copy ./packages/*/package.json files to TEMP_DEPS_DIR,
# while maintaining directory structure.
# We do this so we can take advantage of Docker caching.
rm -rf ${TEMP_DEPS_DIR}
find "./packages" -name "package.json" -maxdepth 2 | \
  sed "s|/package.json||g" | \
  xargs -I % bash -c 'mkdir -p ${TEMP_DEPS_DIR}/% && cp %/package.json ${TEMP_DEPS_DIR}/%/'

# Pull the latest build stage image, if it exists.
if [ -n "$DOCKER_REPO" ]; then
  docker pull ${BUILD_IMAGE}:latest || \
    echo "No existing image found for ${BUILD_IMAGE}:latest"
fi

# Build the build stage image.
docker build \
  --target build \
  --cache-from ${BUILD_IMAGE}:latest \
  --tag ${BUILD_IMAGE}:latest \
  --build-arg TEMP_DEPS_DIR=${TEMP_DEPS_DIR} \
  --build-arg APP_NAME=${APP_FULL_NAME} \
  --build-arg APP_DIR=${APP_DIR} \
  --build-arg COMMIT_HASH=${COMMIT_HASH} \
  --file ./docker/Dockerfile \
  .

# Push the build stage image to the working repo.
if [ -n "$DOCKER_REPO" ]; then
  docker push ${BUILD_IMAGE}:latest
fi

# Pull the latest app image, if it exists.
if [ -n "$DOCKER_REPO" ]; then
  docker pull ${APP_IMAGE}:latest || \
    echo "No existing image found for ${APP_IMAGE}:latest"
fi

# Build the app image.
docker build \
  --cache-from ${APP_IMAGE}:latest \
  --cache-from ${BUILD_IMAGE}:latest \
  --tag ${APP_IMAGE}:latest \
  --tag ${APP_IMAGE}:${COMMIT_HASH} \
  --build-arg TEMP_DEPS_DIR=${TEMP_DEPS_DIR} \
  --build-arg APP_NAME=${APP_FULL_NAME} \
  --build-arg APP_DIR=${APP_DIR} \
  --build-arg COMMIT_HASH=${COMMIT_HASH} \
  --file ./docker/Dockerfile \
  .

# Push the app image to the working repo.
if [ -n "$DOCKER_REPO" ]; then
  docker push ${APP_IMAGE}:latest
fi
