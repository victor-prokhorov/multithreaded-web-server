FROM alpine:3.14

RUN apk update \
    && apk upgrade \
    && apk add --no-cache \
        luajit \
        perl \
        build-base \
        openssl-dev \
        linux-headers \
        git

RUN git clone --branch 4.2.0 https://github.com/wg/wrk.git 
WORKDIR /wrk
RUN make

CMD ["./wrk", "-t12", "-c400", "-d30s", "--latency", "http://localhost:3000/api/orders"]
