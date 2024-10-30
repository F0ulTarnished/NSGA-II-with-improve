use std::fs::{File, create_dir_all};
use std::io::{self, Write, BufRead};
use std::path::Path;
use chrono::Local;
pub fn save_vec_to_file(vec: &Vec<f64>, cur_gen:&usize,pf:&str,directory:&str) -> io::Result<()> {
    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    create_dir_all(directory)?;
    let filename=format!("Prob_{}_time{}_{}_gen_{}.txt",pf,formatted_time,directory,cur_gen);
    let path=format!("{}/{}",directory,filename);
    let mut file = File::create(path)?;
    for &value in vec {
        writeln!(file, "{}", value)?;
    }
    Ok(())
}



pub fn read_vec_from_file(filename: &str,directory:&str) -> io::Result<Vec<f64>> {
    let file_path=format!("{}/{}",directory,filename);
    let path = Path::new(&file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    
    let mut vec = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Ok(value) = line.trim().parse::<f64>() {
            vec.push(value);
        }
    }
    Ok(vec)
}
