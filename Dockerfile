FROM node:latest as node_base

WORKDIR /usr/src/app

COPY . .

RUN npm install -g pnpm && pnpm install 


FROM node_base as node_builder

RUN pnpm css:build
RUN rm -rf node_modules

FROM rustlang/rust:nightly as rust_builder

WORKDIR /usr/src/app

COPY --from=node_builder /usr/src/app .

RUN cargo build --release


FROM debian:latest 

RUN useradd -ms /bin/bash app_user
USER app_user

COPY --from=rust_builder /usr/src/app/target/release/seeable-server /usr/local/bin/seeable-server
COPY --from=rust_builder /usr/src/app/static /static

EXPOSE 3000

ENV RUST_LOG=trace

CMD ["seeable-server"]
