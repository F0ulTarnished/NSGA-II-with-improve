# NSGA-II-with-improve
Using Current crowding distance to improve NSGA-II

## Environment
Windows 11
rust 1.82
Dependency see file "cargo.toml"

## Usage
Presently,only in debug version can you modify your parameter
### 0.Git code
### 1.Parameters setting
`var_limit_inside`:set the lower and upper bound of each para, in form of (min,max)
`species()`:set pop_size,max_gen,crossover_prob,var_num,obj_num,var_limit_inside  in order
`load_ck`:load a population from a checkpoint or not
`op`:use to set the operator *currently only support changing mutation op
`model`:choose the model of NSGA-II, `Origin` is no-modified ,`CurDist` is modified by the thesis, `GenDist` is useless modified
`mode`:choose the mode to run, `Pure` will only change the population,`Plot` will plot the pareto front every max_gen/5 generations,`Mei` will do thing as the thesis
`pf`:choose the MOP
### 2.Run
Recommand run in IDE.
In terminal of the root dir,input
```
cargo build
```
will generate the result
