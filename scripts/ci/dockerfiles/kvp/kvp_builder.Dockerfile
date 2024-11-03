# This is the build stage for kvp. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /kvp
COPY . /kvp

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the kvp binary."
FROM docker.io/parity/base-bin:latest

LABEL description="Multistage Docker image for kvp: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="chevdor@gmail.com, devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.description="kvp: a platform for web3" \
	io.parity.image.source="https://github.com/paritytech/kvp/blob/${VCS_REF}/scripts/ci/dockerfiles/kvp/kvp_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/paritytech/kvp/"

COPY --from=builder /kvp/target/release/kvp /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /kvp kvp && \
	mkdir -p /data /kvp/.local/share && \
	chown -R kvp:kvp /data && \
	ln -s /data /kvp/.local/share/kvp && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/kvp --version

USER kvp

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/kvp"]
