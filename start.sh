#!/bin/bash

./rust-docker-web
/usr/bin/curl -v http://localhost:8000 > /tmp/result.txt