#!/bin/sh

docker kill rofs
docker rm rofs

docker build -t rofs .
