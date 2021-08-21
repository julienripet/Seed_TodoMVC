# Todo List, implemented in [Seed](https://seed-rs.org/) 
As per the specifications of the classic [todomvc exercice of tasteJS](https://github.com/tastejs/todomvc/blob/master/app-spec.md#functionality)

Following the tutorial of [Seed](https://seed-rs.org/0.8.0/app_2_todomvc?utm_campaign=platform&utm_medium=website&utm_source=thenewstack)

## Quick start

This repo contains an unoptimised [dockerfile](https://docs.docker.com/get-docker/), allowing you to get started quickly even on Windows. To start it, enter the following command in a terminal opened at the root of this repo :

```
docker build -t seed/todo_mvc .
```
this command will create an image of the docker container for you. Once it is done, enter the next one :
```
docker run -dp 8000:8000 --name seed_todo_mvc seed/todo_mvc
```
the second command will create the docker container and start 2 processes:

 * `cargo make serve` : starts a web server that will serve your compiled code to [localhost:8000](http://localhost:8000)
 * `cargo make watch` : starts a program that will watch your code for changes, and compile it to wasm for your web server
