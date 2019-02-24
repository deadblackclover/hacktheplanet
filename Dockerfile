FROM rust:1.32.0-slim

RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates openssl libssl-dev pkg-config

WORKDIR /usr/src/hacktheplanet
COPY . .

RUN cargo install

CMD ["hacktheplanet"]
