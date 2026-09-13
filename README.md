![GitHub Last Commit](https://img.shields.io/github/last-commit/brstrutt/wasmfenbein3d?logo=github)
![Build and Publish](https://github.com/brstrutt/wasmfenbein3d/actions/workflows/build-and-publish.yml/badge.svg?branch=main)

## Description

Wasmfenbein3d is a basic 3d renderer inspired by Wolfenstein3D and Doom. The goal is to see if I can get an interactive 3d environment with a good framerate without using hardware acceleration.

It is intended to be run in the browser as a WASM application. I have tried to build it to be platform agnostic, so no WASM specific code exists in the core library. An example website using this library is provided. This example is also the environemnt used during development to test changes to the library.

The latest example site can be seen at [https://brstrutt.github.io/wasmfenbein3d/](https://brstrutt.github.io/wasmfenbein3d/)

<img width="2521" height="1214" alt="image" src="https://github.com/user-attachments/assets/dd06de18-54cc-4fb1-b34f-c4cc70a33033" />

## How to develop

### Setup environment:
use VScode and open the devcontainer. It provides a fully functioning dev environment with no manual setup required.

### Development:

To run a local dev server:
```bash
cd example
trunk serve 
```
(Use `trunk serve --enable-cooldown` to stop it building twice every time you save changes)

To run the unit tests: `cargo test`

To format the codebase: `cargo fmt`

### Deployment
To deploy, push changes to `main`. Github Actions will automatically build the top commit of `main` and push the example site live.

To trigger a release build: Push a tag of the format `v1.2.3`.

To trigger a development release build: Push a tag like `v1.2.3-stuff`

