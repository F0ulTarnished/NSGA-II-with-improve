use core::f64;
use rand::Rng;
use std::{collections::HashMap, ops::Deref};

use super::population::Species;
#[derive(PartialEq)]
pub enum MODE {
    Pure,//only change specise'pop
    Plot,//will generate figure of pareto(only support 2 objs)
    Mei,//will return mei of gen0~100&3000~3100 after extreme points found,only support OneMinMax
}
#[derive(PartialEq)]
pub enum MODEL {
    Origin,
    CurDist,
    GenDist,//Meaningless
}
#[derive(PartialEq)]
pub enum Mutation{
    OneBit,
    Poly,
}
//this struct is for operator chosen in the alg. Though, present version only implements mutataion choice,
#[derive(PartialEq)]
pub struct Operator{
    pub mutation:Mutation
    //crossover todo
    //selection todo
}

pub fn genuine_crowding_distance(
    species: &Species,
    order: &mut Vec<Vec<usize>>,
    tmp_front: &mut Vec<(usize, f64)>,
    dist_indiv_hash: &mut HashMap<usize, f64>,
) {
    let obj = species.objs.deref();
    //pop min distance individual
    let min_index_in_pop = tmp_front.pop().unwrap().0;
    //println!("{:?}",tmp_front);
    //update neighbor
    let obj_num = species.get_obj_num();
    for j in 0..obj_num {
        let min_place_in_objorder = order[j].iter().position(|&x| x == min_index_in_pop);

        assert!(
            {
                match min_place_in_objorder {
                    Some(_) => true,
                    _ => false,
                }
            },
            "binary search fail in order,{:?},order{:?},min_index{:?}",
            min_place_in_objorder,
            order,
            min_index_in_pop
        );
        //if change min/max obj,then all individual in the front need update
        if min_place_in_objorder.unwrap()==0||min_place_in_objorder.unwrap()==(order[j].len()-1){

            println!("{:?}",min_place_in_objorder.unwrap());
            let (old_min_value, old_max_value) = (obj[order[j][0]*obj_num+j],obj[order[j].last().unwrap()*obj_num+j]);
            let old_max_min = old_max_value - old_min_value;
            println!("{:?}",(old_min_value, old_max_value));
            order[j].remove(min_place_in_objorder.unwrap());
            let (min_value, max_value) = (obj[order[j][0]*obj_num+j],obj[order[j].last().unwrap()*obj_num+j]);
            
            let max_min = max_value - min_value;
            println!("{:?}",(min_value, max_value));
            
            let order_num=order[j].len();
            for k in 1..(order_num - 1) {
                let indi_index = order[j][k];

                if max_min != 0.0 {
                    let obj_last = obj[order[j][k-1]*obj_num+j];
                    let obj_next = obj[order[j][k+1]*obj_num+j];
                    let distance = dist_indiv_hash.get(&indi_index).unwrap()-(obj_next - obj_last) / old_max_min+
                                        (obj_next - obj_last) / max_min;

                    dist_indiv_hash.insert(indi_index, distance);
                } else {
                    let distance = f64::INFINITY;
                    dist_indiv_hash.insert(indi_index, distance);
                }
            }
            dist_indiv_hash.insert(order[j][0], f64::INFINITY);
            dist_indiv_hash.insert(*order[j].last().unwrap(), f64::INFINITY);
        }
        else{
            let cur = order[j][min_place_in_objorder.unwrap()];
            //update neighbor in hash table
            let max_min = obj[order[j].last().unwrap() * obj_num + j] - obj[order[j][0] * obj_num + j];

            //println!("{:?}",min_place_in_objorder.unwrap());
            let last = order[j][min_place_in_objorder.unwrap() - 1];
            let next = order[j][min_place_in_objorder.unwrap() + 1];

            let last_dist = dist_indiv_hash.get(&last).unwrap()
                + (-obj[cur * obj_num + j] + obj[next * obj_num + j]) / max_min;
            let next_dist = dist_indiv_hash.get(&next).unwrap()
                + (obj[cur * obj_num + j] - obj[last * obj_num + j]) / max_min;
            //println!("{:?},{:?},{:?}",dist_indiv_hash.get(&last).unwrap(),-obj[cur*obj_num+j],obj[next*obj_num+j]);
            //println!("{:?},{:?},{:?}",dist_indiv_hash.get(&next).unwrap(),obj[last*obj_num+j],obj[cur*obj_num+j]);

            dist_indiv_hash.insert(last, last_dist);
            dist_indiv_hash.insert(next, next_dist);
            //neighbor is indexed by "order",so delete op is in "order"
            order[j].remove(min_place_in_objorder.unwrap());
            //update in priority queue
            let last_index = tmp_front.iter().position(|&(x, _)| x == last).unwrap();
            tmp_front[last_index] = (last, last_dist);
            let next_index = tmp_front.iter().position(|&(x, _)| x == next).unwrap();
            tmp_front[next_index] = (next, next_dist);
            //println!("{:?},{:?}",last_dist,next_dist);
        }
        
    }
    tmp_front.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
}
pub fn current_crowding_distance(
    species: &Species,
    order: &mut Vec<Vec<usize>>,
    tmp_front: &mut Vec<(usize, f64)>,
    dist_indiv_hash: &mut HashMap<usize, f64>,
) {
    let obj = species.objs.deref();
    //pop min distance individual
    let min_index_in_pop = tmp_front.pop().unwrap().0;
    //println!("{:?}",tmp_front);
    //update neighbor
    let obj_num = species.get_obj_num();
    for j in 0..obj_num {
        let min_place_in_objorder = order[j].iter().position(|&x| x == min_index_in_pop);
        assert!(
            {
                match min_place_in_objorder {
                    Some(_) => true,
                    _ => false,
                }
            },
            "binary search fail in order,{:?},order{:?},min_index{:?}",
            min_place_in_objorder,
            order,
            min_index_in_pop
        );
        let cur = order[j][min_place_in_objorder.unwrap()];
        //update neighbor in hash table
        let max_min = obj[order[j].last().unwrap() * obj_num + j] - obj[order[j][0] * obj_num + j];

        //println!("{:?}",min_place_in_objorder.unwrap());
        let last = order[j][min_place_in_objorder.unwrap() - 1];
        let next = order[j][min_place_in_objorder.unwrap() + 1];

        let last_dist = dist_indiv_hash.get(&last).unwrap()
            + (-obj[cur * obj_num + j] + obj[next * obj_num + j]) / max_min;
        let next_dist = dist_indiv_hash.get(&next).unwrap()
            + (obj[cur * obj_num + j] - obj[last * obj_num + j]) / max_min;
        //println!("{:?},{:?},{:?}",dist_indiv_hash.get(&last).unwrap(),-obj[cur*obj_num+j],obj[next*obj_num+j]);
        //println!("{:?},{:?},{:?}",dist_indiv_hash.get(&next).unwrap(),obj[last*obj_num+j],obj[cur*obj_num+j]);

        dist_indiv_hash.insert(last, last_dist);
        dist_indiv_hash.insert(next, next_dist);
        //neighbor is indexed by "order",so delete op is in "order"
        order[j].remove(min_place_in_objorder.unwrap());
        //update in priority queue
        let last_index = tmp_front.iter().position(|&(x, _)| x == last).unwrap();
        tmp_front[last_index] = (last, last_dist);
        let next_index = tmp_front.iter().position(|&(x, _)| x == next).unwrap();
        tmp_front[next_index] = (next, next_dist);
        //println!("{:?},{:?}",last_dist,next_dist);
    }
    tmp_front.sort_by(|&(_, x), &(_, y)| y.partial_cmp(&x).unwrap());
}
pub fn fast_nondominated_sort(species: &Species) -> Vec<Vec<usize>> {
    let objective = species.objs.deref();
    let obj_num = species.get_obj_num();
    let pop_size = species
        .population
        .deref()
        .len()
        .wrapping_div(species.get_var_num());

    let mut set: Vec<Vec<usize>> = Vec::new(); //dominating set for each individual
    for _ in 0..pop_size {
        let tmp: Vec<usize> = Vec::new();
        set.push(tmp);
    }
    let mut num: Vec<usize> = vec![0; pop_size]; //dominating set's mem quantity
    let mut rank: Vec<usize> = vec![0; pop_size]; //level of set
    let mut front: Vec<Vec<usize>> = Vec::new();
    front.push(Vec::new());

    //go through all, check dominating set and num
    for p in 0..pop_size {
        //as dominator
        for q in (p + 1)..pop_size {
            //as dominatee
            let mut equal: bool = true;
            let mut dominating: bool = true;
            let mut dominated: bool = true;
            for i in 0..obj_num {
                //obj val judge
                let p_index = p.wrapping_mul(obj_num).wrapping_add(i);
                let q_index = q.wrapping_mul(obj_num).wrapping_add(i);
                let p_obj = objective[p_index];
                let q_obj = objective[q_index];
                equal = equal && (p_obj == q_obj);
                dominating = dominating && (p_obj <= q_obj);
                dominated = dominated && (p_obj >= q_obj);
            }
            //p do q
            if dominating && (!equal) {
                set[p].push(q);
                num[q] = num[q].wrapping_add(1);
            }
            //q do p
            else if dominated && (!equal) {
                set[q].push(p);
                num[p] = num[p].wrapping_add(1);
            }
        }
        //check for front
        if num[p] == 0 {
            rank[p] = 0;
            front[0].push(p);
        }
    }
    //update other front
    let mut i: usize = 0;
    while front[i].len() != 0 {
        let mut q_capital: Vec<usize> = Vec::new();
        for p in &front[i] {
            for q in &set[*p] {
                num[*q] = num[*q].wrapping_sub(1);
                if num[*q] == 0 {
                    rank[*q] = i + 1;
                    if !q_capital.contains(&q) {
                        q_capital.push(*q);
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        front.push(q_capital);
    }
    //println!("{:?}",front.len());
    front.pop();
    front //this is all fronts set,differ from "single front below"
}
pub fn crowding_distance_assignment(
    species: &Species,
    front: &mut Vec<usize>,
    order: &mut Vec<Vec<usize>>,
    is_order: bool,
) -> HashMap<usize, f64> {
    //return hash map of index and corresponding distance
    let front_num = front.len();
    let obj_num = species.get_obj_num();
    let objective = species.objs.deref();

    //let mut distance:Vec<f64>=vec![0.0;front_num];

    let mut dist_indiv_hash: HashMap<usize, f64> = HashMap::new();

    //initialize hashmap
    {
        let (min_value, max_value) = get_sort_findminmax(objective, front, 0, obj_num);
        let max_min = max_value - min_value;
        for j in 1..(front_num - 1) {
            let indi_index = front[j];

            if max_min != 0.0 {
                let obj_last = objective[front[j - 1] * obj_num];
                let obj_next = objective[front[j + 1] * obj_num];
                let distance = (obj_next - obj_last) / max_min;

                dist_indiv_hash.insert(indi_index, distance);
            } else {
                let distance = f64::INFINITY;
                dist_indiv_hash.insert(indi_index, distance);
            }
        }
        if is_order {
            order.push(front.clone());
        }

        dist_indiv_hash.insert(front[0], f64::INFINITY);
        dist_indiv_hash.insert(front[front_num - 1], f64::INFINITY);
    }

    for i in 1..obj_num {
        //let mut objective_mut:Vec<f64>=objective.into_iter().enumerate().filter(|&(x,_)|x%i==0).map(|(_,&y)|y).collect();
        //objective_mut.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (min_value, max_value) = get_sort_findminmax(objective, front, i, obj_num);
        let max_min = max_value - min_value;

        for j in 1..(front_num - 1) {
            let indi_index = front[j];
            if max_min != 0.0 {
                let obj_last = objective[front[j - 1] * obj_num + i];
                let obj_next = objective[front[j + 1] * obj_num + i];
                let distance =
                    dist_indiv_hash.get(&indi_index).unwrap() + (obj_next - obj_last) / max_min;

                dist_indiv_hash.insert(indi_index, distance);
            } else {
                let distance = f64::INFINITY;
                dist_indiv_hash.insert(indi_index, distance);
            }
        }
        if is_order {
            order.push(front.clone());
        }
        dist_indiv_hash.insert(front[0], f64::INFINITY);
        dist_indiv_hash.insert(front[front_num - 1], f64::INFINITY);
    }
    dist_indiv_hash
}
// pub fn org_crowding_distance_assignment(species:&Species,front:&mut Vec<usize>)->HashMap<usize,f64>{//return hash map of index and corresponding distance
//     let front_num=front.len();
//     let obj_num=species.get_obj_num();
//     let objective=species.objs.deref();

//     //let mut distance:Vec<f64>=vec![0.0;front_num];
    
//     let mut dist_indiv_hash:HashMap<usize,f64>=HashMap::new();

//     //initialize hashmap
//     {
//         let (min_value,max_value)=get_sort_findminmax(objective, front, 0, obj_num);
//         let max_min=max_value-min_value;
//         for j in 1..(front_num-1){
//             let indi_index=front[j];
            
//             if max_min!=0.0{
//                 let obj_last=objective[front[j-1]*obj_num];
//                 let obj_next=objective[front[j+1]*obj_num];
//                 let distance=(obj_next-obj_last)/max_min;
                
//                 dist_indiv_hash.insert(indi_index, distance);
//             }
//             else {
//                 let distance=f64::INFINITY;
//                 dist_indiv_hash.insert(indi_index, distance);
                
//             }
//         }
//         dist_indiv_hash.insert(front[0],f64::INFINITY );
//         dist_indiv_hash.insert(front[front_num-1],f64::INFINITY );
//     }

//     for i in 1..obj_num{
        
//         //let mut objective_mut:Vec<f64>=objective.into_iter().enumerate().filter(|&(x,_)|x%i==0).map(|(_,&y)|y).collect();
//         //objective_mut.sort_by(|a, b| a.partial_cmp(b).unwrap());
//         let (min_value,max_value)=get_sort_findminmax(objective, front, i, obj_num);
//         let max_min=max_value-min_value;
        
//         for j in 1..(front_num-1){
//             let indi_index=front[j];
//             if max_min!=0.0{
//                 let obj_last=objective[front[j-1]*obj_num+i];
//                 let obj_next=objective[front[j+1]*obj_num+i];
//                 let distance=dist_indiv_hash.get(&indi_index).unwrap()+(obj_next-obj_last)/max_min;
                
//                 dist_indiv_hash.insert(indi_index, distance);
//             }
//             else {
//                 let distance=f64::INFINITY;
//                 dist_indiv_hash.insert(indi_index, distance);
//             }
//         }
//         dist_indiv_hash.insert(front[0],f64::INFINITY );
//         dist_indiv_hash.insert(front[front_num-1],f64::INFINITY );
//     }
//     dist_indiv_hash
// }
fn get_sort_findminmax(
    objective: &Vec<f64>,
    front: &mut Vec<usize>,
    obj_index: usize,
    obj_num: usize,
) -> (f64, f64) {
    let mut selected: Vec<(usize, f64)> = Vec::new();
    let len = front.len();
    for k in 0..len {
        let i = front[k];
        let value = objective[i * obj_num + obj_index];
        selected.push((i, value));
    }

    selected.sort_by(|&(_, x), &(_, y)| x.partial_cmp(&y).unwrap());
    //println!("{:?}",selected);
    *front = selected.clone().into_iter().map(|(x, _)| x).collect();

    let min = selected[0].1;
    let max = selected.last().unwrap().1;

    (min, max)
}

// fn fatherpool_select(fronts:&Vec<Vec<usize>>,dist_indiv_hash:&HashMap<usize,f64>,pop_size:usize)->Vec<usize>{//reduncdant
//     let mut fatherpool:Vec<usize>=Vec::new();
//     let mut rng=rand::thread_rng();

//     for _i in 0..pop_size.wrapping_div(2){
//         let r1=rng.gen_range(0..fronts.len());
//         let p1=*fronts[r1].choose(&mut rng).unwrap();

//         let r2=rng.gen_range(0..fronts.len());
//         let p2=*fronts[r2].choose(&mut rng).unwrap();

//         if crowd_op(r1, r2, dist_indiv_hash.get(&p1).unwrap(), dist_indiv_hash.get(&p2).unwrap())==0
//         {
//             fatherpool.push(p1);
//         }
//         else {
//             fatherpool.push(p2);
//         }
//     }
//     fatherpool
// }
pub fn creating_child(species: &Species, fatherpool: &Vec<usize>, union: &Vec<f64>,op:&Operator) -> Vec<f64> {
    //fatherpool of half size of pop
    let crossover_prob = species.get_crossover_prob();
    let mut rng = rand::thread_rng();
    let pop_size = species.get_pop_size();
    let var_num = species.get_var_num();
    let population = union;
    let var_limit = species.var_limit.deref();

    //generating offspring
    let mut son: Vec<f64> = Vec::new();
    let mut i = 0;
    while i < pop_size {
        let prob: f32 = rng.gen_range(0.0..=1.0);
        //crossover
        if prob < crossover_prob {
            let index1 = rng.gen_range(0..pop_size.wrapping_div(2));
            let row1 = fatherpool[index1];
            let mut f1: Vec<f64> = vec![0.0; var_num];
            f1.copy_from_slice(&population[row1 * var_num..(row1 * var_num + var_num)]);

            let index2 = rng.gen_range(0..pop_size.wrapping_div(2));
            let row2 = fatherpool[index2];
            let mut f2: Vec<f64> = vec![0.0; var_num];
            f2.copy_from_slice(&population[row2 * var_num..(row2 * var_num + var_num)]);

            let mut child: Vec<f64> = crossover_sbx(&f1, &f2, var_num, var_limit);
            son.append(&mut child);
            i = i.wrapping_add(2);
        } else {
            //muatation
            let index1 = rng.gen_range(0..pop_size.wrapping_div(2));
            let row1 = fatherpool[index1];
            let mut f: Vec<f64> = vec![0.0; var_num];
            f.copy_from_slice(&population[row1 * var_num..(row1 * var_num + var_num)]);

            let var_limit = species.var_limit.deref();
            match &op.mutation {
                Mutation::OneBit=>{
                    let mut child = mutation_onebit(&f, var_num, var_limit);
                    son.append(&mut child);}
                Mutation::Poly=>{
                    let mut child = mutation_poly(&f, var_num, var_limit);
                    son.append(&mut child);}
                
            }
            
            i = i.wrapping_add(1);
        }
    }
    if i == pop_size.wrapping_add(1) {
        son.resize(son.len() - var_num, 0.0);
    }
    assert_eq!(
        son.len(),
        species.population.deref().len(),
        "son num{},pop_size{}",
        son.len().wrapping_div(var_num),
        pop_size
    );
    son
}
fn crossover_sbx(
    f1: &Vec<f64>,
    f2: &Vec<f64>,
    var_num: usize,
    var_limit: &Vec<(f64, f64)>,
) -> Vec<f64> {
    //generate two offspring in one Vec
    //let population=species.population.deref();
    //let f1=population
    //let var_num=species.get_var_num();
    let mut rng = rand::thread_rng();
    let ita = 1.0;
    let mut gamma;
    let mut new_indi1: Vec<f64> = Vec::new();
    let mut new_indi2: Vec<f64> = Vec::new();
    for i in 0..var_num {
        let u: f64 = rng.gen_range(0.0..1.0);
        if u < 0.5 {
            let exp = 1_f64 / (ita + 1.0);
            gamma = (2.0 * u).powf(exp);
        } else {
            let exp = 1_f64 / (ita + 1.0);
            gamma = (1.0 / (2.0 - 2.0 * u)).powf(exp);
        }
        let mut new_var1 = 0.5 * ((1.0 + gamma) * f1[i] + (1.0 - gamma) * f2[i]);
        let mut new_var2 = 0.5 * ((1.0 - gamma) * f1[i] + (1.0 + gamma) * f2[i]);
        new_var1 = new_var1.clamp(var_limit[i].0, var_limit[i].1);
        new_var2 = new_var2.clamp(var_limit[i].0, var_limit[i].1);

        new_indi1.push(new_var1);
        new_indi2.push(new_var2);
    }
    new_indi1.append(&mut new_indi2);
    new_indi1
}
fn mutation_poly(f: &Vec<f64>, var_num: usize, var_limit: &Vec<(f64, f64)>) -> Vec<f64> {
    let ita = 1.0;
    let exp = 1.0 / (ita + 1.0);
    let mut mutated: Vec<f64> = Vec::new();
    let mut rng = rand::thread_rng();
    for i in 0..var_num {
        let u: f64 = rng.gen_range(0.0..=1.0);
        let sigma1 = (f[i] - var_limit[i].0) * (var_limit[i].1 - var_limit[i].0);
        let sigma2 = (var_limit[i].1 - f[i]) * (var_limit[i].1 - var_limit[i].0);
        let base;
        if u < 0.5 {
            base = (2.0 * u + (1.0 - 2.0 * u) * ((1.0 - sigma1).powf(ita + 1.0))).powf(exp) - 1.0;
        } else {
            base = 1.0
                - (2.0 * (1.0 - u) + 2.0 * (u - 0.5) * ((1.0 - sigma2).powf(ita + 1.0))).powf(exp);
        }
        let mut mutated_var = f[i] + base * (var_limit[i].1 - var_limit[i].0);
        mutated_var = mutated_var.clamp(var_limit[i].0, var_limit[i].1);
        mutated.push(mutated_var);
    }
    mutated
}
fn mutation_onebit(f: &Vec<f64>, var_num: usize, _t: &Vec<(f64, f64)>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..var_num);
    let mut mutated = f.clone();

    mutated[index] = (1.0 - f[index]).round();
    mutated
}
pub fn tournament(
    dist_indiv_hash: &HashMap<usize, f64>,
    fronts: &Vec<Vec<usize>>,
    pop_size: usize,
    mut last_included_front: usize,
) -> Vec<usize> {
    //select best out of 'k' randomly
    let mut rng = rand::thread_rng();
    let front_num = fronts.len();
    assert!(
        front_num >= last_included_front,
        "last_included_front index out of bound"
    );
    let mut fatherpool: Vec<usize> = Vec::new();
    if last_included_front == 0 {
        last_included_front = 1;
    }
    for _i in 0..pop_size.wrapping_div(2) {
        let r1 = rng.gen_range(0..last_included_front);
        let front_ele_num1 = fronts[r1].len();
        let tmp1 = rng.gen_range(0..front_ele_num1);
        let p1 = fronts[r1][tmp1];

        let r2 = rng.gen_range(0..last_included_front);
        let front_ele_num2 = fronts[r2].len();
        let tmp2 = rng.gen_range(0..front_ele_num2);
        let p2 = fronts[r2][tmp2];

        let index = crowd_op(
            r1,
            r2,
            dist_indiv_hash.get(&p1).unwrap(),
            dist_indiv_hash.get(&p2).unwrap(),
        );
        if index == 0 {
            fatherpool.push(p1);
        } else if index == 1 {
            fatherpool.push(p2);
        } else {
            panic!("tourment error,fn crowd_op wrong output")
        }
    }
    fatherpool
}
fn crowd_op(r1: usize, r2: usize, d1: &f64, d2: &f64) -> usize {
    if (r1 < r2) || (r1 == r2 && d1 > d2) {
        0
    } else if (r1 > r2) || (r1 == r2 && d1 < d2) {
        1
    } else {
        r1 % 2
    }
}
