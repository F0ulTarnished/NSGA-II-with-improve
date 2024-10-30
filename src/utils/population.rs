use std::{ ops::{Deref, DerefMut}, usize};
use rand::Rng;
use plotters::prelude::*;
use chrono::Local;
pub struct Species{
    pop_size:usize,
    max_gen:usize,
    crossover_prob:f32,
    var_num:usize,
    obj_num:usize,

    //2-D data organized in 1-d by row
    pub population:Box<Vec<f64>>,//one row reprsents individual'vars
    pub objs:Box<Vec<f64>>,//one row represents objs
    pub var_limit:Box<Vec<(f64,f64)>>//arranged in (min,max)

}

impl Species {
    pub fn new(pop_size:usize,max_gen:usize,crossover_prob:f32,var_num:usize,
        obj_num:usize,var_limit_inside:Vec<(f64, f64)>)->Self{
            //initialize population
            let total_var_num=pop_size.wrapping_mul(var_num );
            let mut population_vec:Vec<f64>=Vec::with_capacity(total_var_num as usize);
            let mut rng = rand::thread_rng(); 
            for _ in 0..pop_size {
                for i in 0..var_num{
                    let random_number: f64 = rng.gen_range(var_limit_inside[i].0..var_limit_inside[i].1);
                    population_vec.push(random_number); 
                }
                
            }
            let population=Box::new(population_vec);
            //pre-initialize obj
            let total_obj_num=obj_num.wrapping_mul(pop_size as usize);
            let objs_vec:Vec<f64>=Vec::with_capacity(total_obj_num);
            let objs=Box::new(objs_vec);

            //let mut var_limit_inside:Vec<(f64,f64)>=Vec::with_capacity(var_num);
            let var_limit=Box::new(var_limit_inside);
            Species{
                pop_size,
                max_gen,
                crossover_prob,
                var_num,
                objs,
                obj_num,
                population,
                var_limit
            }
    }
    pub fn get_pop_size(&self)->usize{
        self.pop_size
    }
    pub fn get_max_gen(&self)->usize{
        self.max_gen
    }
    pub fn get_crossover_prob(&self)->f32{
        self.crossover_prob
    }
    pub fn get_var_num(&self)->usize{
        self.var_num
    }
    pub fn get_obj_num(&self)->usize{
        self.obj_num
    }
    pub fn update_population(&mut self,new_pop:&Vec<f64>){
        //assert_eq!(self.pop_size,new_pop.len().wrapping_div(self.var_num),"new population size:{} differs from pop_size{}",new_pop.len().wrapping_div(self.var_num),self.pop_size);
        self.population=Box::new(new_pop.clone());
    }
    pub fn append_population(&mut self,append_pop:&mut Vec<f64>){
        self.population.deref_mut().append(append_pop);
    }
    pub fn update_obj<F>(&mut self,pf:F)
    where F:Fn(&Vec<f64>,usize)->Vec<f64>{
        let population=self.population.deref();
        let updated_obj=pf(population,self.var_num);
        self.objs=Box::new(updated_obj);
    }
    pub fn get_mei(&self)->f64{
        let mut obj_sum_0:Vec<f64>=self.objs.deref().iter().enumerate().filter(|&(i,_)|i%2==0).map(|(_,&x)|x).collect();
        obj_sum_0.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        let mut max_difference = 0.0;

        for window in obj_sum_0.windows(2) {
            let difference = window[0] - window[1];
            if difference > max_difference {
                max_difference = difference;
            }
        }
        max_difference
        
    }
    pub fn get_aei(&self)->f64{
        let mut obj_sum_0:Vec<f64>=self.objs.deref().iter().enumerate().filter(|&(i,_)|i%2==0).map(|(_,&x)|x).collect();
        obj_sum_0.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

        let mut sum_difference = 0.0;

        for window in obj_sum_0.windows(2) {
            let difference = window[0] - window[1];
            sum_difference+=difference;
        }
        sum_difference/(self.pop_size-1) as f64
    }
    pub fn check_extre_indiv(&self)->bool{//this is specifically for OneMinMax, when check both {0}^n and {1}^n indiv
        let obj_sum_0:Vec<f64>=self.objs.deref().iter().enumerate().filter(|&(i,_)|i%2==0).map(|(_,&x)|x).collect();
        
        let mut all_0=false;
        let mut all_1=false;
        for &ele in &obj_sum_0{
            all_0=all_0||ele as usize==self.var_num*10;
            all_1=all_1||ele as usize==0;
        }
        all_0&&all_1
    }
    pub fn plot_obj_figure(&self,gen:usize)-> Result<(), Box<dyn std::error::Error>>{
        assert_eq!(2,self.obj_num,"only support 2D figure");
        let x_obj:Vec<f64>;
        x_obj=self.objs.deref().into_iter().enumerate().filter(|(x,_)|x%2==0).map(|(_,&y)|y).collect();
        let y_obj:Vec<f64>;
        y_obj=self.objs.deref().into_iter().enumerate().filter(|(x,_)|x%2==1).map(|(_,&y)|y).collect();

        let current_time = Local::now();
        let formatted_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
        let file_name = format!("plot_{}_{}.png", formatted_time,gen);

        let root = BitMapBackend::new(&file_name, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        
        let caption=format!("Scatter Plot Gen:{}",gen);
        
        let mut chart = ChartBuilder::on(&root)
            .caption(caption, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(0f64..10000f64, 0f64..1000f64)?;

    
        chart
            .configure_mesh()
            .x_desc("f1 Axis")
            .y_desc("f2 Axis")
            .draw()?;

        chart.draw_series(
            x_obj.iter().zip(y_obj.iter()).map(|(&x, &y)| {
            Circle::new((x, y), 5, RED.filled())
            }),)?;

    
        //chart.draw_series(LineSeries::new(
        //    x_obj.iter().zip(y_obj.iter()).map(|(&x, &y)| (x, y)),
        //    &BLUE,
        //    ))?;
        root.present()?;
        Ok(())
    }
}