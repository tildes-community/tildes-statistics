FROM rust:1.64 as builder

# Create a new empty project.
RUN USER=root cargo new --bin tildes-statistics
WORKDIR /tildes-statistics
RUN mv src source

# Copy the configuration files and build in release, caching the dependencies.
COPY Cargo.lock Cargo.toml askama.toml .
RUN cargo build --release

# Then copy our code. This way when only the source code changes, the
# dependencies don't have to be entirely rebuilt.
COPY source source
COPY node_modules/modern-normalize node_modules/modern-normalize

# Remove the cached tildes-statistics dependencies.
RUN rm target/release/deps/tildes_statistics*

# Build the executable with actual source code.
RUN cargo install --path .

# Copy the executable to a smaller final image.
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/tildes-statistics /usr/local/bin
CMD ["tildes-statistics"]
