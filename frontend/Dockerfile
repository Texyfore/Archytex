FROM rust:alpine AS viewport

RUN apk update
RUN apk add --no-cache curl musl-dev
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

ADD editor /archytex/editor

WORKDIR /archytex/editor/packages/web-runner
RUN wasm-pack build --debug --out-dir /archytex/pkg

FROM node:16-alpine AS frontend

COPY --from=viewport /archytex/pkg /archytex/pkg
ADD frontend /archytex/frontend


WORKDIR /archytex/frontend
ENV STAGE=production
RUN yarn --network-timeout 1000000
RUN yarn build

FROM nginx:alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

COPY --from=frontend /archytex/frontend/build /usr/share/nginx/html