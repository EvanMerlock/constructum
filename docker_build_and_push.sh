#!/bin/sh

docker build -f Dockerfile.client . -t git.k8s.internal.merlock.dev/evanmerlock/constructum-client:latest
docker push git.k8s.internal.merlock.dev/evanmerlock/constructum-client:latest