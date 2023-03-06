ENDPOINT ?= polygon.streamingfast.io:443
ROOT_DIR := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen --exclude-paths sf/substreams,google


.PHONY: stream
stream: build
	substreams run  -e  polygon.streamingfast.io:443 substreams.yaml   db_out  -t +10
