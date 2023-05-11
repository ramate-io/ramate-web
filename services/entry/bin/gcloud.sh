#!/bin/bash
docker build --platform=linux/amd64 -t echo .
docker tag echo gcr.io/ramate/echo
docker push gcr.io/ramate/echo