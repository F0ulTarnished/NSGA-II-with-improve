use core::f64;
use std::{collections::HashMap,  ops::Deref};
use crate::utils::checkpoint::save_vec_to_file;
use super::population::Species;
use crate::utils::operators::*;


pub fn nsga_ii_c_fn<F>( species:&mut Species,pf:F,mode:&MODE,op:&Operator,model:&MODEL)->Vec<f64>
where F:Fn(&Vec<f64>,usize)->Vec<f64>
{//the species has initialized, so have other parameters
    let max_gen=species.get_max_gen();
    let pop_size=species.get_pop_size();
    let var_num=species.get_var_num();
    

    let mut new_parent:Vec<usize>;
    let mut new_pop:Vec<f64>;
    let mut son:Vec<f64>;
    let mut dist_indiv_hash:HashMap<usize,f64>;
    let mut fronts:Vec<Vec<usize>>;
    let mut union:Vec<f64>;
    let mut last_included_front:usize;
    let mut order:Vec<Vec<usize>>=Vec::new();
    let mut mei:Vec<f64>=Vec::new();
    let mut extre_flag:bool=false;//this is flag of both {0}^n and {1}^n 
    
    //****************************************preperation********************************
    
    {
        println!("preperation");
        //fast non-dominated sort
        fronts=fast_nondominated_sort(&species);

        //crowding distance assignment,save into hash table (individual index,distance)
        let front_num=fronts.len();
        //println!("fronts:{}",front_num);
        dist_indiv_hash=HashMap::new();
        for i in 0..front_num{
            dist_indiv_hash.extend(crowding_distance_assignment(&species, &mut fronts[i],&mut order,false));
        }
        last_included_front=front_num;
        union=species.population.deref().clone();
        
    }
    //****************************************main loop********************************
    let mut pre_gen=0;
    let mut gen=0;
    while gen<=max_gen{
        println!("gen:{}",gen);
    //----------------->>   Select/crossover/mutation

        //fatherpool of half pop_size
        let fatherpool=tournament(&dist_indiv_hash, &fronts, pop_size,last_included_front);
        //generating offspring of pop_size
        son=creating_child(species, &fatherpool,&union,&op);
        
    //----------------->>   Union Parent and Son

        species.append_population(&mut son);
        species.update_obj(&pf);
        assert_eq!(species.population.deref().len().wrapping_div(var_num),pop_size.wrapping_mul(2),"combined size wrong");
        
    //----------------->>   Fast NonDominated Sort

        //fast non-dominated sort
        fronts=fast_nondominated_sort(&species);


    //----------------->>   Elite Preserving(in reference of distance&rank)

        new_parent=Vec::new();
        let mut i=0;
        while (new_parent.len()+fronts[i].len())<=pop_size{
            let mut this_front=fronts[i].clone();
            new_parent.append(&mut this_front);
            i=i.wrapping_add(1);
        }
        last_included_front=i;
        let remainder=pop_size.wrapping_sub(new_parent.len());
    //----------------->>   Crowding Distance Assignment(embeded in elite preserving)

        //crowding distance assignment,save into hash table (individual index,distance)
        let front_num=fronts.len();
        
        dist_indiv_hash=HashMap::new();
        order=Vec::new();
        for ii in 0..front_num{
            let is_order=ii==last_included_front;
            dist_indiv_hash.extend(crowding_distance_assignment(&species, &mut fronts[ii],&mut order,is_order));
        }
        
        if remainder!=0 {
            //sort front[i] by distance ,use this as the binary heap,aka priority queue
            let len=fronts[i].len();
            let mut tmp_front:Vec<(usize,f64)>=Vec::new();
            
            for ii in 0..len{
                let index=fronts[i][ii];
                let ele=(index,*dist_indiv_hash.get(&index).unwrap());
                tmp_front.push(ele);
                
            }
            
            tmp_front.sort_by(|&(_,x),&(_,y)|y.partial_cmp(&x).unwrap());
           
            //individual order of different obj is recorded in "order" in increase order
            
            //delete

            let delete_num=fronts[i].len()-pop_size.wrapping_sub(new_parent.len());
            match model{
                MODEL::CurDist=>{
                    for _i in 0..delete_num{
                        current_crowding_distance(species, &mut order, &mut tmp_front, &mut dist_indiv_hash);
                    }
                },

                MODEL::Origin=>{

                },
                MODEL::GenDist=>{
                    for _i in 0..delete_num{
                        genuine_crowding_distance(species, &mut order, &mut tmp_front, &mut dist_indiv_hash);
                    }
                },
            }
            
            
            
              
            //append 
            let mut tmp_tmp:Vec<usize>=tmp_front.clone().into_iter().map(|(x,_)|x).collect();
            //let mut remaining:Vec<usize>=vec![0;remainder];
            //let last_index=fronts[i].len()-1;
            //remaining.clone_from_slice(&tmp_tmp[last_index-remainder..last_index]);//here should have arranged in desent distance order before choosing

            new_parent.append(&mut tmp_tmp);
        }
        new_pop=Vec::new();
        for i in 0..pop_size  {
            let mut tmp=vec![0.0;var_num];
            let k=new_parent[i];
            tmp.clone_from_slice(&species.population.as_ref()[k.wrapping_mul(var_num)..k.wrapping_mul(var_num).wrapping_add(var_num)]);
            new_pop.append(&mut tmp);
        }
        union=species.population.deref().clone();

        
        
        species.update_population(&new_pop);


        match &mode {
            //**************** PLOT mode*****************//
            MODE::Plot=>{
                let divison=max_gen.wrapping_div(5);
                if gen%divison==0{
                let _=species.plot_obj_figure(gen);
                }
            },
            //***************** MEI mode************** //
            MODE::Mei=>{
                if extre_flag==false{
                    let check_result=species.check_extre_indiv();
                    pre_gen+=1;
                    print!("pre_gen:{}",pre_gen);

                    gen=0;
                    if check_result{
                        //set flag=true
                        extre_flag=check_result;
                        println!("extreme individual found");
                        //save checkpoint
                        let pf_name:String;
                        match &model{
                            MODEL::Origin=>{
                                pf_name=format!("OneMinMax_o");
                            },
                            MODEL::CurDist=>{
                                pf_name=format!("OneMinMax_c");
                            }
                            MODEL::GenDist=>{
                                pf_name=format!("OneMinMax_g");
                            }
                        }
                        let saving=save_vec_to_file(&new_pop, &gen, &pf_name,&"checkpoint");
                        println!("save {:?}",saving);
                        gen=0;
                    }
                }
                if extre_flag&&(gen<=100||gen>=3000){
                    species.update_obj(&pf);
                    mei.push(species.get_mei());
                }
            },
            MODE::Pure=>{

            }
            
        }
        gen+=1;

    }
    if *mode==MODE::Mei{
        gen-=1;
        let pf_name:String;
        match &model{
            MODEL::Origin=>{
                pf_name=format!("OneMinMax_o");
            },
            MODEL::CurDist=>{
                pf_name=format!("OneMinMax_c");
            }
            MODEL::GenDist=>{
                pf_name=format!("OneMinMax_g");
            }
        }
        let saving=save_vec_to_file(&species.population, &gen, &pf_name,&"checkpoint");
        println!("save {:?}",saving);

    }
    mei
    
}
