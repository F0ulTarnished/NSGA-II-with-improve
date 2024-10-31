# NSGA-II-with-improve
Using Current crowding distance to improve NSGA-II in referrence of "https://dl.acm.org/doi/abs/10.1145/3512290.3528847"  
Note: this is project of Intelligent Computing by Prof Wang Feng in WHU

## Environment
Windows 11  
MSVC（VC Toolchain） 14.41.34120  
rustc 1.82  
Dependency see file "cargo.toml"

## Usage
If no rust,see "https://www.rust-lang.org/zh-CN/tools/install" to install.Better run in IDE for convenience of changing parameters.
Presently,only in debug version can you modify your parameter.Below is for costomized usage.
### 0.Git code
### 1.Model Parameters Setting
`var_limit_inside`:set the lower and upper bound of each para, in form of (min,max)  
`species()`:set pop_size,max_gen,crossover_prob,var_num,obj_num,var_limit_inside  in order  
`load_ck`:load a population from a checkpoint or not  
`op`:use to set the operator *currently only support changing mutation op  
`model`:choose the model of NSGA-II, `Origin` is no-modified ,`CurDist` is modified by the thesis, `GenDist` is useless modified  
`mode`:choose the mode to run, `Pure` will only change the population,`Plot` will plot the pareto front every max_gen/5 generations,`Mei` will do thing as the thesis  
`pf`:choose the MOP
### Secondary Parameter
Save a record(specifically for population and mei):  
`fn save_vec_to_file(vec: &Vec<f64>, cur_gen:&usize,pf:&str,directory:&str) -> io::Result<()>`receives 3 parameters in order of "data,generation of data,directory to save".  
`fn read_vec_from_file(filename: &str,directory:&str) -> io::Result<Vec<f64>>`receives 3 parameters in order of "filename,directory to locate".the directoy need to be in the root.Also filename includes affix.
### 2.Run
Recommand run in IDE.  
Build proj:  
In terminal of the root dir,input
```
cargo build
```
Run:
```
cargo run
```
will generate the result. 
## Designed Pattern
###To get the result of the thesis, choose parameters below  
```
let var_limit_inside=[(0.0,1.0);601].to_vec();  
let mut species:Species=Species::new(76, 3100, 0.0,  601, 2, var_limit_inside);  
let pf=one_minmax;  
let op=utils::operators::Operator{mutation:utils::operators::Mutation::OneBit};  

let mode=MODE::Mei;
```
1. Original NSGA-II's performance after two extreme points found:
   `let model=MODEL::Origin;  `
2. NSGA-II with current crowding distance's:
   `let model=MODEL::CurDist;  `
The result of mei will be saved in folder "mei", distinguished by "o"/"c" within the name. And a figure of pareto front will be shown in root dir.

### To get result of my shalow work
Both Original and Currental model have defect in searching, which need 3000gen or so to find extreme points. This is because the iniatial population lacks diversity in spite of random initation, with regard of OneMinMax problem. Hence I add scattered extreme points to amend for lack of diversity.  
To enable it ,remove "//" ahead of `species.add_margin_indiv();`.Then you will find only need 1500 gen to find the extreme points, plus that the mei result remains unchanged.

##Note
1. if the pop size is enormous,build your release version to run  
2. In Mei model of OneMinMax problem, the extreme points will be found aound 3000+-500 generation. Occassionally,4000 gen  
   
