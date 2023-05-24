#!/bin/bash
PROTO_PATH=$1
MODULE_NAME=$2
mkdir -p ./node_modules/$MODULE_NAME
protoc -I=$PROTO_PATH *.proto \
    --js_out=import_style=commonjs,binary:./node_modules/$MODULE_NAME \
    --grpc-web_out=import_style=typescript,mode=grpcweb:./node_modules/$MODULE_NAME
echo "{ \"type\" : \"commonjs\", \"name\" : \"$MODULE_NAME\" }" > ./node_modules/$MODULE_NAME/package.json