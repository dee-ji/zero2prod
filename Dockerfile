# We use the latest Rust stable release as base image
FROM rust:1.72.1

# Let's switch out working directoy to `app` (equipvalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update && apt install lld clang -y
# Copy all files from our working environment to our Docker image
COPY . .
# Setting sqlx to offline to avoid build failures from lack of active connection
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the relase profile to make it faaaast
RUN cargo build --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]