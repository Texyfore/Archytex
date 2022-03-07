FROM rust:alpine AS viewport

RUN apk update
RUN apk add --no-cache curl musl-dev perl openssl-dev alpine-sdk
#RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install wasm-pack

ADD editor /archytex/editor

WORKDIR /archytex/editor/packages/web-runner
RUN apk add --no-cache binaryen
RUN wasm-pack build --release --out-dir /archytex/pkg

FROM node:16-alpine AS frontend

COPY --from=viewport /archytex/pkg /archytex/pkg
ADD frontend /archytex/frontend


WORKDIR /archytex/frontend
RUN yarn
ENV REACT_APP_STAGE=production
RUN yarn build

FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

COPY --from=frontend /archytex/frontend/build /usr/share/nginx/html