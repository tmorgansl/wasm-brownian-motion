# `wasm-brownian-motion`

This is a simple demo of brownian motion using rust and web assembly. You can see a demo of it running on [heroku](https://wasm-brownian-motion.herokuapp.com/).

## Introduction

The goal of this project was to have a working demo of a basic physics simulation running in the browser to try out web assembly. It is purely a stochastic simulation, meaning that all of the particle motion is described through random variables.

For a large number of particles it will be fairly CPU intensive. It will technically work on mobile devices, however performance will be poor.

Most of the css styling/presentation layer is also handled on the rust side using the [`web-sys` crate](https://crates.io/crates/web-sys).

This project was templated from the [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template) project, please check it out if you're interested in using webpack with web assembly.

## Prerequisites

You will need:-
* [rust stable](https://www.rust-lang.org/), tested on 1.32.0
* [node and npm](https://nodejs.org/en/), tested on v10.15.0
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

Optional:-
* [Docker](https://www.docker.com/)

## Installation

### Development environment

To run with a webpack development server, run the following:-

* `npm i` -- Pull in the necessary dependencies for the frontend to run
* `npm run start` -- Serves the project locally for development at `http://localhost:8080`

This fully supports hot reloading.

### Nodejs server

A simple nodejs express server has been included to serve the built files. To run this locally run the following commands:-

```
npm i
npm run build-dev # replace with build-prod to build for production
cd server
npm i
node index.js
```

By default it will run on port 8080, however this can be overridden using the `PORT` environment variable

### Docker

A dockerfile also provided which will run the whole environment:-

```
docker build -t wasm-brownian-motion .
docker run -p 8080:8080 wasm-brownian-motion:latest 
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details