BIRB
====

![Example Render](/.github/cornell.jpg)

This is my toy path tracer written in Rust. It serves little purpose other than allowing me to learn about rust programming and path tracing at the same time. Path tracing is a method for rendering 3D scenes through realistic light simulation.

The current `master` branch contains a very simple path tracer that began its life as a rust port of <https://github.com/hunterloftis/pathtracer>.
The project is built to run in the browser using Wasm and a live version can be found [on aws](http://rust-pathtracer.s3-website.eu-west-2.amazonaws.com/).

The `pbr-rewrite` branch contains modifications I have started to experiment with while reading [The PBR Book](http://www.pbr-book.org/), although they are very incomplete and the branch mainly exists as a scratchpad while I read the material.

