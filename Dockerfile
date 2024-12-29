FROM rust:1.83 as builder
COPY . /text_analyzer
WORKDIR /text_analyzer
RUN cargo build --release

FROM rust:1.83
COPY --from=builder /text_analyzer/target/release/text_analyzer /
# CMD ["/text_analyzer"]