#!/bin/bash

aws ecr get-login-password --region us-east-2 | docker login --username AWS --password-stdin 684317180556.dkr.ecr.us-east-2.amazonaws.com
docker buildx build --platform=linux/amd64 -t updater-service:latest .
docker tag updater-service:latest 684317180556.dkr.ecr.us-east-2.amazonaws.com/updater-service:latest
docker push 684317180556.dkr.ecr.us-east-2.amazonaws.com/updater-service:latest
