<p align="center">
    <img width="255" alt="Logo" src="logo.png">
</p>
<h1 align="center">Forester - A fast orchestration engine, implementing behavior trees.</h1>


## About

Forester is a framework that provides a toolkit to perform effective task orchestration.
The tasks can be performed synchronously or asynchronously, locally or remotely.
Forester takes care of the correct performance and distribution of the tasks.
The main concept of the framework is the flow based on behavior trees.
It can be effectively used in game, AI, and robotics areas, or anywhere a workflow engine can be applied.


# The console utility to work with [Forester](https://github.com/forester-bt/forester)

The details can be found in the [book](https://forester-bt.github.io/learn/)

The commands can be:

```shell
Commands:
  print-std-actions  Print the list of std actions from 'import std::actions'
  sim                Runs simulation. Expects a simulation profile
  run                Runs the tree from the file system. Accepts an optional run profile in yaml
  vis                Runs visualization. Output is in svg format.
  help               Print this message or the help of the given subcommand(s)

```

## Simulation (sim)

```shell
Options:
  -p, --profile <PATH>  a path to a sim profile
  -r, --root <ROOT>     a path to a root folder. The <PWD> folder by default
  -m, --main <MAIN>     a path to a main file. The 'main.tree' by default
  -t, --tree <TREE>     a root in a main file. If there is only one root it takes by default
  -h, --help            Print help
```

## Run (run)

Runs the tree from the file system using the Forester runner.
The optional run profile in yaml drives the blackboard (initial load and final dump),
the tracer, the http server and the remote actions.

```shell
Options:
  -p, --profile <PATH>  a path to a run profile in yaml. The default profile if empty
  -r, --root <ROOT>     a path to a root folder. The <PWD> folder by default
  -m, --main <MAIN>     a path to a main file. The 'main.tree' by default
  -t, --tree <TREE>     a root in a main file. The 'main' by default
  -h, --help            Print help
```

An example of the run profile:

```yaml
run_until:
  limit: 10          # or `run_until: no_limit` (the default)

bb:
  load: "bb_init.json"
  dump: "bb_final.json"

tracer:
  indent: 2
  time_format: "%H:%M:%S"
  to_file: "trace.log"

api:
  type: http
  host: "localhost"
  port: 8080

actions:
  - type: http
    name: fetch_data
    url: "http://localhost:10000/action"
```

## Visualization

```shell
Options:
  -o, --output <OUTPUT>  a file for svg. If none, the name from the main file will be taken.
  -r, --root <ROOT>      a path to a root folder. The <PWD> folder by default
  -m, --main <MAIN>      a path to a main file. The 'main.tree' by default
  -t, --tree <TREE>      a root in a main file. If there is only one root it takes by default
  -h, --help             Print help

```

The full list of commands can be obtained by the command:

```shell
forester -h
```