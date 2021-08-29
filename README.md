# Todo List, implemented in [Seed](https://seed-rs.org/) 
As per the specifications of the classic [todomvc exercice of tasteJS](https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality)

Following the tutorial of [Seed](https://seed-rs.org/0.8.0/app_2_todomvc?utm_campaign=platform&utm_medium=website&utm_source=thenewstack)

## What is Seed?

The full explanation can be found on their [site](https://seed-rs.org/), but from a webdev point of view, Seed is kinda like React, except you use Rust to create your components. The Rust code is compiled into [webAssembly](https://webassembly.org/), which will create the html, css, and handle the logic of your app, just like Javascript would in your typical ReactJS SPA. Web assembly (WASM) is much faster than Javascript, and should therefore produce a much smoother experience, even on the weakest devices. WASM files are also lighter than their equivalent in javascript, meaning faster downloads and thus, faster page rendering.

## Quick start

This repo contains an unoptimised [dockerfile](https://docs.docker.com/get-docker/), allowing you to get started quickly even on Windows. To start it, enter the following command in a terminal opened at the root of this repo :

```
docker build -t seed/todo_mvc .
```
this command will create an image of the docker container for you. Once it is done, enter the next one :
```
docker run -dp 8000:8000 --name seed_todo_mvc seed/todo_mvc
```
the second command will create the docker container and start the following process:

 * `cargo make serve` : starts a web server that will serve your compiled code to [localhost:8000](http://localhost:8000)

Once the container is started and running, type the following command in your terminal:
```
docker exec seed_todo_mvc cargo make watch
```
this will start a process in the container to watch the program's code and rebuild the WASM after every change. Do not close that terminal, or the process will stop.
