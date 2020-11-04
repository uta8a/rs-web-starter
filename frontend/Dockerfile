FROM rust:1.47 AS builder

WORKDIR /main
RUN cargo install cargo-web
RUN cargo install wasm-bindgen-cli
RUN cargo install wasm-pack

COPY src src
COPY static static
COPY tests tests
COPY bootstrap.js bootstrap.js
COPY Cargo.toml Cargo.toml
COPY package.json package.json
COPY webpack.config.js webpack.config.js
COPY yarn.lock yarn.lock

RUN curl https://deb.nodesource.com/setup_12.x | bash
RUN curl https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -
RUN echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list
RUN apt-get update && apt-get install -y nodejs yarn

RUN yarn install
RUN yarn build

FROM nginx:1.19
COPY ./nginx.conf /etc/nginx/conf.d/server.conf
RUN rm /etc/nginx/conf.d/default.conf
COPY --from=builder /main/dist /var/www/app

# sudo docker build -t front .
# sudo docker run -it --rm -p "8001:80" front