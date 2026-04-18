#!/bin/sh

docker run -d --restart unless-stopped -p 4000:4000 --name rofs --volume=/opt/rofs/static/:/static rofs:latest
