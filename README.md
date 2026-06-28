![GitHub Last Commit](https://img.shields.io/github/last-commit/brstrutt/wasmfenbein3d?logo=github)
![Build and Publish](https://github.com/brstrutt/wasmfenbein3d/actions/workflows/build-and-publish.yml/badge.svg?branch=main)

## Description

This repo is intended to contain a basic implementation of the Wolfenstein 3D renderer, running in WASM.
It's a proof of concept/mess around type repo to see if I can get something like this working.

Current live site can be seen at [https://brstrutt.github.io/wasmfenbein3d/](https://brstrutt.github.io/wasmfenbein3d/)


<img width="2520" height="1245" alt="image" src="https://github.com/user-attachments/assets/e98b1dc8-07a6-4a60-bf9c-be35202f54b3" />


## How to develop

To setup the dev environment: use the devcontainer

To build: `trunk build`

To run a local dev server: `trunk serve`

To run the unit tests: `cargo test`

To push live: Push changes to `main`. Github Actions will automatically build top of `main` and push it live.

To create a release: Push a tag of the format `v1.2.3`. Tags like `v1.2.3-stuff` will create a pre-release. Releases mostly exist just to mark milestones in the project.

To format the codebase: `cargo fmt`
