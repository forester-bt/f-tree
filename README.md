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
  sim   Runs simulation. Expects a simulation profile
  vis   Runs visualization. Output is in svg format.
  help  Print this message or the help of the given subcommand(s)

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