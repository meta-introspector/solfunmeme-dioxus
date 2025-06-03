# # syntax = docker/dockerfile:1

# # Adjust NODE_VERSION as desired
# ARG NODE_VERSION=20.18.0
# FROM node:${NODE_VERSION}-slim as base

# LABEL fly_launch_runtime="NodeJS"

# # NodeJS app lives here
# WORKDIR /app

# # Set production environment
# ENV NODE_ENV=production


# # Throw-away build stage to reduce size of final image
# FROM base as build

# # Install packages needed to build node modules
# RUN apt-get update -qq && \
#     apt-get install -y python-is-python3 pkg-config build-essential 

# # Install node modules
# COPY --link package.json package-lock.json .
# RUN npm install --production=false

# # Copy application code
# COPY --link . .

# # Remove development dependencies
# RUN npm prune --production


# # Final stage for app image
# FROM base

# # Copy built application
# COPY --from=build /app /app

# # Start the server by default, this can be overwritten at runtime
# CMD [ "npm", "run", "start" ]
FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --platform web

#FROM chef AS runtime
#COPY --from=builder /app/target/dx/solfunmeme-dioxus/debug/web/ /usr/local/app
#COPY --from=builder /app/target/wasm32-unknown-unknown/release/solfunmeme-dioxus.wasm/ /usr/local/app/
#COPY --from=builder /app/target/dx/solfunmeme-dioxus/web/ /usr/local/app


FROM caddy:2.7.5-alpine as runtime

COPY --from=builder /app/target/dx/solfunmeme-dioxus/release/web/  /usr/share/caddy
COPY Caddyfile /etc/caddy/Caddyfile

RUN caddy validate --config /etc/caddy/Caddyfile

#COPY --from=builder /app/target/dx/solfunmeme-dioxus/web/ /usr/local/app