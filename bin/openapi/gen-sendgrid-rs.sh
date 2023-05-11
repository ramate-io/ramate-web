#!/bin/bash
openapi-generator generate -g rust \
  -i https://raw.githubusercontent.com/sendgrid/sendgrid-oai/main/oai_v3_stoplight.json \
  -o packages/sendgrid-rs \
  --additional-properties=useSingleRequestParameter=true