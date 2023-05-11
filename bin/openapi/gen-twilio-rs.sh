#!/bin/bash
openapi-generator generate -g rust \
  -i https://raw.githubusercontent.com/twilio/twilio-oai/main/spec/json/twilio_api_v2010.json \
  -o packages/twilio-rs \
  --additional-properties=useSingleRequestParameter=true