#!/bin/sh

docker build -t http_benchmarking ./http_benchmarking \
    && docker run --network host http_benchmarking