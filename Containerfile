FROM debian:buster-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG IMAGE_NAME

LABEL org.valibre.image.authors="daniel@olanod.com" \
	io.parity.image.vendor="Valibre Network" \
	io.parity.image.title="${IMAGE_NAME}" \
	io.parity.image.description="vln: Decentralized local interactions" \
	io.parity.image.source="https://github.com/valibre-org/vln-node/blob/${VCS_REF}/Containerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/valibre-org/vln-node/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		libssl1.1 \
		ca-certificates \
		curl && \
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user and link ~/.local/share/polkadot to /data
	useradd -m -u 1000 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot

# add polkadot binary to docker image
COPY ./target/release/vln_parachain /usr/local/bin/vln

USER vln

# check if executable works in this container
RUN /usr/local/bin/vln --version

EXPOSE 30333 9933 9944
VOLUME ["/vln"]

ENTRYPOINT ["/usr/local/bin/vln"]

