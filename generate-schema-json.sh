#!/bin/sh
graphql-client introspect-schema http://localhost:8081/graphql --output ./client/graphql/schema.json