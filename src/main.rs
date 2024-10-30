
use utils::{ checkpoint::read_vec_from_file, objective::{one_minmax, zdt1}, operators::{MODE, MODEL}, population::Species};

mod utils;
fn main() {

//*****   /|
//*****    |
//*****   ---, set limit of every var,in form of (min,max)
    let var_limit_inside=[(0.0,1.0);601].to_vec();
//*****   ---
//*****  (  /
//*****    ---, set parameters below  
    let mut species:Species=Species::new(76, 
                                        2000, 
                                        0.0,  
                                        601, 
                                        2, 
                                        var_limit_inside);
//*****   ---
//*****    --|
//*****   ---, choose whether load a population or not
    //whether load trained population,need to denote the file name
    //**if not, 
    //**if yes,the file contains the pop need to be in folder "checkpoint"
    let load_ck=false;
    if load_ck{
        let pop=read_vec_from_file("Prob_OneMinMax_g_time2024-10-30_12-29-02_checkpoint_gen_3100.txt","checkpoint").unwrap();
    species.update_population(&pop);
    }
    
    

//*****   |  |
//*****    --|
//*****      |, choose the problem
    species.update_obj(one_minmax);
    
//*****   ---
//*****   (-)
//*****   --), choose operater ,model,mode
        //** operator for now only support changing mutation op,see definition
        //** model is Origin CurDist
        //** mode ,see definition */
    let op=utils::operators::Operator{mutation:utils::operators::Mutation::OneBit};
    let model=MODEL::Origin;
    let mode=MODE::Mei;

    
    let mei=utils::nsga_ii_c::nsga_ii_c_fn(&mut species,one_minmax,&mode,&op,&model);
    //println!("{:?}",species.get_aei());
    
    if mode==MODE::Mei{
        let _=species.plot_obj_figure(3100);
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
        
        let saving=utils::checkpoint::save_vec_to_file(&mei, &species.get_max_gen(), &pf_name,&"mei");
    }
    
    

}

//fail work
// fn mei_plot(mei:Vec<f64>,max_gen:usize,n:usize,N:usize)-> Result<(), Box<dyn std::error::Error>>{

//     let x_axis:Vec<usize>=(1..=100).chain(3100..=3100).collect();
//     let file_name=format!("MEI");
//     let cur_mei=read_vec_from_file("Prob_OneMinMax_c_time2024-10-29_20-06-34_mei_gen_3100.txt","mei");
//     let ori_mei=read_vec_from_file("Prob_OneMinMax_o_time2024-10-29_20-16-03_mei_gen_3100.txt", "mei");
//     let root = BitMapBackend::new(&file_name, (800, 600)).into_drawing_area();
//         root.fill(&WHITE)?;
        
        
//         let caption=format!("n={},N={}",n,N);
//         let mut chart = ChartBuilder::on(&root)
//             .caption(caption, ("sans-serif", 50).into_font())
//             .margin(5)
//             .x_label_area_size(30)
//             .y_label_area_size(30)
//             .build_cartesian_2d(0f64..500f64, 0f64..100f64)?;

    
//         chart
//             .configure_mesh()
//             .x_desc("X Axis")
//             .y_desc("Y Axis")
//             .draw()?;

//         chart.draw_series(
//             x_axis.iter().zip(mei.iter()).map(|(&x, &y)| {
//             Circle::new((x as f64, y), 5, RED.filled())
//             }),)?;

    
//         //chart.draw_series(LineSeries::new(
//         //    x_obj.iter().zip(y_obj.iter()).map(|(&x, &y)| (x, y)),
//         //    &BLUE,
//         //    ))?;
//         root.present()?;
//         Ok(())
// }
