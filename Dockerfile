FROM rust:1.32.0-stretch
RUN mkdir -p /app/
ADD . /app/
WORKDIR /app/
RUN curl -sL https://deb.nodesource.com/setup_10.x | bash - && \
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh -s -- -f && \
    apt-get update && \
    apt-get install nodejs -y && \
    npm i && \
    npm run build-prod

FROM node:10.15.0-alpine
RUN mkdir -p /app/dist/ && mkdir /app/server
WORKDIR /app/server/
COPY --from=0 /app/dist/ /app/dist/
COPY server/ /app/server/
RUN npm i

CMD ["node", "index.js"]