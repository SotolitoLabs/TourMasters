#!/bin/bash

if [[ $1 == "" ]]; then
    echo "Usage: prepare_places.sh <json_file>"
    exit 1
fi

JSON_FILE=$1

jq -c '.[]' $JSON_FILE | tail -2  | head -1 | sed -f prepare_places.sed
