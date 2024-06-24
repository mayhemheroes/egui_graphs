# ARGS
ARG project=egui_graphs

# Setup Fuzz-Rust Builder
FROM ghcr.io/nottirb/fuzz-rust:latest as builder

ADD . /${project}
WORKDIR /${project}

RUN cd ./fuzz && cargo +nightly hfuzz build

# Setup Fuzz-Env
FROM ubuntu:20.04
#ghcr.io/nottirb/fuzz-env:latest
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/graph_all /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/graph_edge /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/graph_node /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/nodes /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/run_simulation /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/simulate_all /
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/simulation /
