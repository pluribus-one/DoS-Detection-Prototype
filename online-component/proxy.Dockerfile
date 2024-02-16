########################################
FROM rust:1.76 AS builder

ARG TARGETARCH
ARG CARGO_FEATURES
ENV CARGO_FEATURES ${CARGO_FEATURES}

RUN echo "x86_64" > /arch;

ENV CFLAGS=-Ofast

ENV RUSTFLAGS "-C link-arg=-s"

WORKDIR /tmp

COPY ./online-component/proxy/ /tmp/

RUN echo "Building proxy from source" && \
  cargo build --release --target $(cat /arch)-unknown-linux-gnu ${CARGO_FEATURES} && \
  cp /tmp/target/$(cat /arch)-unknown-linux-gnu/release/proxy /tmp/target/release/proxy


########################################
FROM gcr.io/distroless/cc-debian12 AS runner

COPY --from=builder /tmp/target/release/proxy /proxy
# COPY salvo-proxy/certs /proxy/certs/

EXPOSE 8080 8081

CMD [ "./proxy" ]
