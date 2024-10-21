#!/bin/bash

PORT=8010
# TODO for amdgpu --gpus all \
podman run \
    -it \
    -v postgresml_data:/var/lib/postgresql \
    -p 5433:5432 \
    -p $PORT:8000 \
    ghcr.io/postgresml/postgresml:2.9.3 \
    sudo -u postgresml psql -d postgresml
