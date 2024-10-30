
pub fn zdt1(var:&Vec<f64>,var_num:usize)->Vec<f64>{//one time all individual
    
    assert!(var.len()%var_num==0,"var len{}unmatch var_num{},",var.len(),var_num);
    let round=var.len().wrapping_div(var_num);
    let mut ouput:Vec<f64>=Vec::new();
    for i in 0..round{
        
        let row=&var[i.wrapping_mul(var_num)..i.wrapping_mul(var_num).wrapping_add(var_num)];
        let f1=row[0];
        let mut fake_sum=0.0;
        for j in 1..var_num{
            fake_sum=fake_sum+row[j];
        }
        let g=1.0+9.0*fake_sum/(var_num as f64-1.0);
        let f2=g*(1.0-(f1/g).sqrt());
        
        ouput.append(&mut [f1,f2].to_vec());
    }   
    ouput
}
pub fn one_minmax(var:&Vec<f64>,var_num:usize)->Vec<f64>{
    let round=var.len().wrapping_div(var_num);
    let mut ouput:Vec<f64>=Vec::new();
    for i in 0..round{
        let row=&var[i.wrapping_mul(var_num)..i.wrapping_mul(var_num).wrapping_add(var_num)];
        let sum_1=row.iter().map(|&x|x.round()).sum();
        let sum_0=(var_num as f64-sum_1)*10.0;
        ouput.append(&mut [sum_0,sum_1].to_vec());
    }
    ouput
}
