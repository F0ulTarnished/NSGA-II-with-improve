// use core::f64;
// use std::ops::DerefMut;
// use std::{collections::HashMap,  ops::Deref};
// use super::{operators::Operator, population::Species};
// use crate::utils::checkpoint::save_vec_to_file;
// use crate::utils::operators::*;

// pub fn nsga_ii_fn<F>( species:&mut Species,pf:F,mode:MODE,op:&Operator)->Vec<f64>
// where F:Fn(&Vec<f64>,usize)->Vec<f64>
// {//the species has initialized, so have other parameters
//     let max_gen=species.get_max_gen();
//     let pop_size=species.get_pop_size();
//     let var_num=species.get_var_num();

//     let mut new_parent:Vec<usize>;
//     let mut new_pop:Vec<f64>;
//     let mut son:Vec<f64>;
//     let mut dist_indiv_hash:HashMap<usize,f64>;
//     let mut fronts:Vec<Vec<usize>>;
//     let mut union:Vec<f64>;
//     let mut last_included_front:usize;
//     let mut order:Vec<Vec<usize>>=Vec::new();
//     let mut mei:Vec<f64>=Vec::new();
//     let mut extre_flag:bool=false;//this is flag of both {0}^n and {1}^n 
//     //****************************************preperation********************************
    
//     {
//         println!("preperation");
        
//         //fast non-dominated sort
//         fronts=fast_nondominated_sort(&species);

//         //crowding distance assignment,save into hash table (individual index,distance)
//         let front_num=fronts.len();
//         //println!("fronts:{}",front_num);
//         dist_indiv_hash=HashMap::new();
//         for i in 0..front_num{
//             dist_indiv_hash.extend(org_crowding_distance_assignment(&species, &mut fronts[i]));
//         }
//         last_included_front=front_num;
//         union=species.population.deref().clone();
        
//     }
//     //****************************************main loop********************************
    
//     let mut gen=0;
//     while gen<=max_gen{
//         println!("gen:{}",gen);
//     //----------------->>   Select/crossover/mutation

//         //fatherpool of half pop_size
//         let fatherpool=tournament(&dist_indiv_hash, &fronts, pop_size,last_included_front);
//         //generating offspring of pop_size
//         son=creating_child(species, &fatherpool,&union,&op);
        
//     //----------------->>   Union Parent and Son

//         species.append_population(&mut son);
//         species.update_obj(&pf);
//         assert_eq!(species.population.deref().len().wrapping_div(var_num),pop_size.wrapping_mul(2),"combined size wrong");
        
//     //----------------->>   Fast NonDominated Sort

//         //fast non-dominated sort
//         fronts=fast_nondominated_sort(&species);
//     //----------------->>   Crowding Distance Assignment

//         //crowding distance assignment,save into hash table (individual index,distance)
//         let front_num=fronts.len();
//         dist_indiv_hash=HashMap::new();
//         for i in 0..front_num{
//             dist_indiv_hash.extend(org_crowding_distance_assignment(&species, &mut fronts[i]));
//         }

//     //----------------->>   Elite Preserving(in reference of distance&rank)

//         new_parent=Vec::new();
//         let mut i=0;
//         while (new_parent.len()+fronts[i].len())<=pop_size{
//             let mut this_front=fronts[i].clone();
//             new_parent.append(&mut this_front);
//             i=i.wrapping_add(1);
//         }
//         last_included_front=i;
//         let remainder=pop_size.wrapping_sub(new_parent.len());
//         if remainder!=0 {
//             //sort front[i] by distance 
//             let len=fronts[i].len();
//             let mut tmp_front:Vec<(usize,f64)>=Vec::new();
//             for ii in 0..len{
//                 let index=fronts[i][ii];
//                 let ele=(index,*dist_indiv_hash.get(&index).unwrap());
//                 tmp_front.push(ele);
//             }
//             tmp_front.sort_by(|&(_,x),&(_,y)|x.partial_cmp(&y).unwrap());
//             let tmp_tmp:Vec<usize>=tmp_front.clone().into_iter().map(|(x,_)|x).collect();
            
//             //append 
//             let mut remaining:Vec<usize>=vec![0;remainder];
//             let last_index=fronts[i].len()-1;
//             remaining.clone_from_slice(&tmp_tmp[last_index-remainder..last_index]);//here should have arranged in desent distance order before choosing

//             new_parent.append(&mut remaining);
//         }
//         new_pop=Vec::new();
//         for i in 0..pop_size  {
//             let mut tmp=vec![0.0;var_num];
//             let k=new_parent[i];
//             tmp.clone_from_slice(&species.population.as_ref()[k.wrapping_mul(var_num)..k.wrapping_mul(var_num).wrapping_add(var_num)]);
//             new_pop.append(&mut tmp);
//         }
//         union=species.population.deref().clone();
//         species.update_population(&new_pop);
        

//         match &mode {
//             //**************** PLOT mode*****************//
//             MODE::Plot=>{
//                 let divison=max_gen.wrapping_div(5);
//                 if gen%divison==0{
//                 let _=species.plot_obj_figure(gen);
//                 }
//             },
//             MODE::Mei=>{
//                 if extre_flag==false{
//                     let check_result=species.check_extre_indiv();
//                     gen=0;
//                     if check_result{
//                         //set flag=true
//                         extre_flag=check_result;
//                         println!("extreme individual found");
//                         //save checkpoint
//                         let pf_name=format!("OneMinMax_post");
//                         let saving=save_vec_to_file(&new_pop, &gen, &pf_name,&"checkpoint");
//                         println!("save {:?}",saving);
//                         gen=0;
//                     }
//                 }
//                 if extre_flag&&(gen<=100||gen>=3000){
//                     species.update_obj(&pf);
//                     mei.push(species.get_mei());
//                 }
//             },
//             MODE::Pure=>{

//             }
//         }
//         gen+=1;

//     }
//     if mode==MODE::Mei{
//         gen-=1;
//         let pf_name=format!("OneMinMax_finished_c");
//         let saving=save_vec_to_file(&species.population, &gen, &pf_name,&"checkpoint");
//         println!("save {:?}",saving);

//     }
//     mei
// }

// // fn fast_nondominated_sort(species:&Species)->Vec<Vec<usize>>{
// //     let objective=species.objs.deref();
// //     let obj_num=species.get_obj_num();
// //     let pop_size=species.population.deref().len().wrapping_div(species.get_var_num());

// //     let mut set:Vec<Vec<usize>>=Vec::new();//dominating set for each individual
// //     for _ in 0..pop_size{
// //         let tmp:Vec<usize>=Vec::new();
// //         set.push(tmp);
// //     }
// //     let mut num:Vec<usize>=vec![0;pop_size];//dominating set's mem quantity
// //     let mut rank:Vec<usize>=vec![0;pop_size];//level of set
// //     let mut front:Vec<Vec<usize>>=Vec::new();
// //     front.push(Vec::new());

// //     //go through all, check dominating set and num
// //     for p in 0..pop_size{//as dominator
// //         for q in (p+1)..pop_size{//as dominatee
// //             let mut equal:bool =true;
// //             let mut dominating:bool=true;
// //             let mut dominated:bool=true;
// //             for i in 0..obj_num{//obj val judge
// //                 let p_index=p.wrapping_mul(obj_num).wrapping_add(i);
// //                 let q_index=q.wrapping_mul(obj_num).wrapping_add(i);
// //                 let p_obj=objective[p_index];
// //                 let q_obj=objective[q_index];
// //                 equal=equal&&(p_obj==q_obj);
// //                 dominating=dominating&&(p_obj<=q_obj);
// //                 dominated=dominated&&(p_obj>=q_obj);
// //             }
// //             //p do q
// //             if dominating&&(!equal){
// //                 set[p].push(q);
// //                 num[q]=num[q].wrapping_add(1);
// //             }
// //             //q do p
// //             else if dominated&&(!equal) {
// //                 set[q].push(p);
// //                 num[p]=num[p].wrapping_add(1);
// //             }
// //         }
// //         //check for front
// //         if num[p]==0{
// //             rank[p]=0;
// //             front[0].push(p);
// //         }
// //     }
// //     //update other front
// //     let mut i:usize=0;
// //     while front[i].len()!=0{
// //         let mut q_capital:Vec<usize>=Vec::new();
// //         for p in &front[i]{
// //             for q in &set[*p]{
// //                 num[*q]=num[*q].wrapping_sub(1);
// //                 if num[*q]==0{
// //                     rank[*q]=i+1;
// //                     if !q_capital.contains(&q){
// //                         q_capital.push(*q);
// //                     }
// //                 }
// //             }
// //         }
// //         i=i.wrapping_add(1);
// //         front.push(q_capital);
// //     }
// //     //println!("{:?}",front.len());
// //     front.pop();
// //     front//this is all fronts set,differ from "single front below"
// // }
// // fn crowding_distance_assignment(species:&Species,front:&mut Vec<usize>)->HashMap<usize,f64>{//return hash map of index and corresponding distance
// //     let front_num=front.len();
// //     let obj_num=species.get_obj_num();
// //     let objective=species.objs.deref();

// //     //let mut distance:Vec<f64>=vec![0.0;front_num];
    
// //     let mut dist_indiv_hash:HashMap<usize,f64>=HashMap::new();

// //     //initialize hashmap
// //     {
// //         let (min_value,max_value)=get_sort_findminmax(objective, front, 0, obj_num);
// //         let max_min=max_value-min_value;
// //         for j in 1..(front_num-1){
// //             let indi_index=front[j];
            
// //             if max_min!=0.0{
// //                 let obj_last=objective[front[j-1]*obj_num];
// //                 let obj_next=objective[front[j+1]*obj_num];
// //                 let distance=(obj_next-obj_last)/max_min;
                
// //                 dist_indiv_hash.insert(indi_index, distance);
// //             }
// //             else {
// //                 let distance=f64::INFINITY;
// //                 dist_indiv_hash.insert(indi_index, distance);
                
// //             }
// //         }
// //         dist_indiv_hash.insert(front[0],f64::INFINITY );
// //         dist_indiv_hash.insert(front[front_num-1],f64::INFINITY );
// //     }

// //     for i in 1..obj_num{
        
// //         //let mut objective_mut:Vec<f64>=objective.into_iter().enumerate().filter(|&(x,_)|x%i==0).map(|(_,&y)|y).collect();
// //         //objective_mut.sort_by(|a, b| a.partial_cmp(b).unwrap());
// //         let (min_value,max_value)=get_sort_findminmax(objective, front, i, obj_num);
// //         let max_min=max_value-min_value;
        
// //         for j in 1..(front_num-1){
// //             let indi_index=front[j];
// //             if max_min!=0.0{
// //                 let obj_last=objective[front[j-1]*obj_num+i];
// //                 let obj_next=objective[front[j+1]*obj_num+i];
// //                 let distance=dist_indiv_hash.get(&indi_index).unwrap()+(obj_next-obj_last)/max_min;
                
// //                 dist_indiv_hash.insert(indi_index, distance);
// //             }
// //             else {
// //                 let distance=f64::INFINITY;
// //                 dist_indiv_hash.insert(indi_index, distance);
// //             }
// //         }
// //         dist_indiv_hash.insert(front[0],f64::INFINITY );
// //         dist_indiv_hash.insert(front[front_num-1],f64::INFINITY );
// //     }
// //     dist_indiv_hash
// // }
// // fn get_sort_findminmax(objective:&Vec<f64>,front:&mut Vec<usize>,obj_index:usize,obj_num:usize)->(f64,f64){
// //     let mut selected:Vec<(usize,f64)>=Vec::new();
// //     let len=front.len();
// //     for k in 0..len{
// //         let i=front[k];
// //         let value=objective[i*obj_num+obj_index];
// //         selected.push((i,value));
// //     }
    
// //     selected.sort_by(|&(_,x),&(_,y)|x.partial_cmp(&y).unwrap());
// //     //println!("{:?}",selected);
// //     *front=selected.clone().into_iter().map(|(x,_)|x).collect();
    
// //     let min=selected[0].1;
// //     let max=selected.last().unwrap().1;
    
// //     (min,max)
    

// // }

// // // fn fatherpool_select(fronts:&Vec<Vec<usize>>,dist_indiv_hash:&HashMap<usize,f64>,pop_size:usize)->Vec<usize>{//reduncdant
// // //     let mut fatherpool:Vec<usize>=Vec::new();
// // //     let mut rng=rand::thread_rng();
    
// // //     for _i in 0..pop_size.wrapping_div(2){
// // //         let r1=rng.gen_range(0..fronts.len());
// // //         let p1=*fronts[r1].choose(&mut rng).unwrap();

// // //         let r2=rng.gen_range(0..fronts.len());
// // //         let p2=*fronts[r2].choose(&mut rng).unwrap();

// // //         if crowd_op(r1, r2, dist_indiv_hash.get(&p1).unwrap(), dist_indiv_hash.get(&p2).unwrap())==0
// // //         {
// // //             fatherpool.push(p1);
// // //         }
// // //         else {
// // //             fatherpool.push(p2);
// // //         }
// // //     }
// // //     fatherpool
// // // }
// // fn creating_child(species:& Species,fatherpool:&Vec<usize>,union:&Vec<f64>)->Vec<f64>{//fatherpool of half size of pop
// //     let crossover_prob=species.get_crossover_prob();
// //     let mut rng=rand::thread_rng();
// //     let pop_size=species.get_pop_size();
// //     let var_num=species.get_var_num();
// //     let population=union;
// //     let var_limit=species.var_limit.deref();
    
// //     //generating offspring
// //     let mut son:Vec<f64>=Vec::new();
// //     let mut i=0;
// //     while i<pop_size{
// //         let prob:f32=rng.gen_range(0.0..=1.0);
// //         if prob<crossover_prob{
// //             let index1=rng.gen_range(0..pop_size.wrapping_div(2));
// //             let row1=fatherpool[index1];
// //             let mut f1:Vec<f64>=vec![0.0;var_num];
// //             f1.copy_from_slice(&population[row1*var_num..(row1*var_num+var_num)]);

// //             let index2=rng.gen_range(0..pop_size.wrapping_div(2));
// //             let row2=fatherpool[index2];
// //             let mut f2:Vec<f64>=vec![0.0;var_num];
// //             f2.copy_from_slice(&population[row2*var_num..(row2*var_num+var_num)]);

// //             let mut child:Vec<f64>=crossover_sbx(&f1, &f2, var_num,var_limit);
// //             son.append(&mut child);
// //             i=i.wrapping_add(2);
// //         }
// //         else {
// //             let index1=rng.gen_range(0..pop_size.wrapping_div(2));
// //             let row1=fatherpool[index1];
// //             let mut f:Vec<f64>=vec![0.0;var_num];
// //             f.copy_from_slice(&population[row1*var_num..(row1*var_num+var_num)]);
            
// //             let var_limit=species.var_limit.deref();
// //             let mut child=mutation_poly(&f, var_num, var_limit);
// //             son.append(&mut child);
// //             i=i.wrapping_add(1);
// //         }
// //     }
// //     if i==pop_size.wrapping_add(1){
// //         son.resize(son.len()-var_num, 0.0);
// //     }
// //     assert_eq!(son.len(),species.population.deref().len(),"son num{},pop_size{}",son.len().wrapping_div(var_num),pop_size);
// //     son
    


// // }
// // fn crossover_sbx(f1:&Vec<f64>,f2:&Vec<f64>,var_num:usize,var_limit:&Vec<(f64,f64)>)->Vec<f64>{//generate two offspring in one Vec
// //     //let population=species.population.deref();
// //     //let f1=population
// //     //let var_num=species.get_var_num();
// //     let mut rng=rand::thread_rng();
// //     let ita=1.0;
// //     let mut gamma;
// //     let mut new_indi1:Vec<f64>=Vec::new();
// //     let mut new_indi2:Vec<f64>=Vec::new();
// //     for i in 0..var_num{
// //         let u:f64=rng.gen_range(0.0..1.0);
// //         if u<0.5{
// //             let exp=1_f64/(ita+1.0);
// //             gamma=(2.0*u).powf(exp);
// //         }
// //         else {
// //             let exp=1_f64/(ita+1.0);
// //             gamma=(1.0/(2.0-2.0*u)).powf(exp);
// //         }
// //         let mut new_var1=0.5*((1.0+gamma)*f1[i]+(1.0-gamma)*f2[i]);
// //         let mut new_var2=0.5*((1.0-gamma)*f1[i]+(1.0+gamma)*f2[i]);
// //         new_var1=new_var1.clamp(var_limit[i].0, var_limit[i].1);
// //         new_var2=new_var2.clamp(var_limit[i].0,var_limit[i].1);

// //         new_indi1.push(new_var1);
// //         new_indi2.push(new_var2);
// //     }
// //     new_indi1.append(&mut new_indi2);
// //     new_indi1
// // }
// // fn mutation_poly(f:&Vec<f64>,var_num:usize,var_limit:&Vec<(f64,f64)>)->Vec<f64>{
// //     let ita=1.0;
// //     let exp=1.0/(ita+1.0);
// //     let mut mutated:Vec<f64>=Vec::new();
// //     let mut rng=rand::thread_rng();
// //     for i in 0..var_num {
// //         let u:f64=rng.gen_range(0.0..=1.0);
// //         let sigma1=(f[i]-var_limit[i].0)*(var_limit[i].1-var_limit[i].0);
// //         let sigma2=(var_limit[i].1-f[i])*(var_limit[i].1-var_limit[i].0);
// //         let base;
// //         if u<0.5{
// //             base=(2.0*u+(1.0-2.0*u)*((1.0-sigma1).powf(ita+1.0))).powf(exp)-1.0;
// //         }
// //         else {
// //             base=1.0-(2.0*(1.0-u)+2.0*(u-0.5)*((1.0-sigma2).powf(ita+1.0))).powf(exp);
// //         }
// //         let mut mutated_var=f[i]+base*(var_limit[i].1-var_limit[i].0);
// //         mutated_var=mutated_var.clamp(var_limit[i].0, var_limit[i].1);
// //         mutated.push(mutated_var);
// //     }
// //     mutated
// // }
// // fn tournament(dist_indiv_hash:&HashMap<usize,f64>,fronts:&Vec<Vec<usize>>, pop_size:usize,mut last_included_front:usize)->Vec<usize>{//select best out of 'k' randomly
// //     let mut rng=rand::thread_rng();
// //     let front_num=fronts.len();
// //     assert!(front_num>=last_included_front,"last_included_front index out of bound");
// //     let mut fatherpool:Vec<usize>=Vec::new();
// //     if last_included_front==0{
// //         last_included_front=1;
// //     }
// //     for _i in 0..pop_size.wrapping_div(2){
// //         let r1=rng.gen_range(0..last_included_front);
// //         let front_ele_num1=fronts[r1].len();
// //         let tmp1=rng.gen_range(0..front_ele_num1);
// //         let p1=fronts[r1][tmp1];

// //         let r2=rng.gen_range(0..last_included_front);
// //         let front_ele_num2=fronts[r2].len();
// //         let tmp2=rng.gen_range(0..front_ele_num2);
// //         let p2=fronts[r2][tmp2];
        
// //         let index=crowd_op(r1, r2, dist_indiv_hash.get(&p1).unwrap(), dist_indiv_hash.get(&p2).unwrap());
// //         if index==0{
// //             fatherpool.push(p1);
// //         }
// //         else if index==1 {
// //             fatherpool.push(p2);
// //         }
// //         else {
// //             panic!("tourment error,fn crowd_op wrong output")
// //         }
// //     }
// //     fatherpool
// // }
// // fn crowd_op(r1:usize,r2:usize,d1:&f64,d2:&f64)->usize{
// //     if (r1<r2)||(r1==r2&&d1>d2){
// //         0
// //     }
// //     else if (r1>r2)||(r1==r2&&d1<d2) {
// //         1
// //     }
// //     else  {
// //         r1%2
// //     }
// // }
