# REST. Reflect. Enjoy a simple HTTP Echo server. [![Status](https://github.com/yannh/restreflect/actions/workflows/deploy.yml/badge.svg)](https://github.com/yannh/restreflect/actions/workflows/deploy.yml)

RESTReflect is a reimplementation/clone of the excellent
[HTTPBin](https://httpbin.org) by [Kenneth Reitz](https://kennethreitz.org/), but written
in [Rust](https://www.rust-lang.org/) so it can easily be deployed on Fastly's
serverless platform, [Compute@Edge](https://www.fastly.com/products/edge-compute).

This makes the API very fast (it executes closer to the user), highly scalable and with
very little operational overhead.

The two reasons behind this project are:
 * Needing to practice some rust, and this seemed like a good and easy project
 * A project required sending quite a bit of traffic to a public endpoint, 
and Fastly's platform is really quite a bit easier to operate than any container
orchestration system.

## Demo

RESTReflect is deployed to [restreflect.edgecompute.app](https://restreflect.edgecompute.app/)

## Credits

 - @kennethreitz for the original [HTTPBin](https://httpbin.org) app ❤️
