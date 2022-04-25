use std::fs::File;
use std::io::Write;
use rand::Rng;

//チップの合計サイズ
const NUM: usize = 250000;
const EDGENUM: usize= 500;

// 1/RANDAM_NUMで不良品が発生する．一つのウェハあたり，大体25個前後の不良品が出る設定．
const RANDAM_NUM: u32 = 100;

//生成されるファイルの総数
const FILENUM: u32 = 5;

struct Chipdata{
    x_posithon: u32,
    y_position: u32,
    yeild_ratio: bool
}

//構造体の初期化に必要な関数？
//コンストラクタみたいな役割があると思われる．
impl Copy for Chipdata {}

impl Clone for Chipdata {
    fn clone(&self) -> Chipdata {
        Chipdata{
            x_posithon: self.x_posithon,
            y_position: self.y_position,
            yeild_ratio: self.yeild_ratio 
        }
    }
}


impl Chipdata{
    fn new() -> Chipdata{
        Chipdata{
            x_posithon: 0,
            y_position: 0,
            yeild_ratio: true,
        }
    }
}

fn create_data(wafer_data: & mut Vec<Chipdata>){
    for j in 0..EDGENUM{
        for k in 0..EDGENUM {
            wafer_data[j*EDGENUM + k].x_posithon = j as u32 + 1;
            wafer_data[j*EDGENUM + k].y_position = k as u32 + 1;
            wafer_data[j*EDGENUM + k].yeild_ratio = faild_chip(); 
        }
    }
    //wafer_data
    //println!("{}",wafer_data[1].x_posithon);
}

fn faild_chip() -> bool{
    let mut rng = rand::thread_rng();
    let rng_value: u32 = rng.gen_range(0..RANDAM_NUM);
    let mut chip_status: bool= true;
    if rng_value % RANDAM_NUM == 0 {
        chip_status = false;
    }
    chip_status
}

fn save_file(wafer_data: &  Vec<Chipdata>, file_number: u32) -> Result<(), Box<dyn std::error::Error>>{
    let file_name :String= format!("{}{}{}", "wafer_data", file_number.to_string(), ".csv");
    let mut wafer_data_file = File::create(file_name)?;

    let mut contents: String;
    for (i,_) in  wafer_data.iter().enumerate() {
        contents = format!("{}, {}, {}\n", wafer_data[i].x_posithon.to_string(), wafer_data[i].y_position.to_string(), wafer_data[i].yeild_ratio.to_string());
        //contents = format!("{},", i);
        wafer_data_file.write_all(contents.as_bytes())?;
        //println!("{}", i);
    }
    wafer_data_file.flush()?;
    println!("{}: Data Created",file_number);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut chip_array: Vec<Chipdata>= vec![Chipdata::new(); NUM];
    println!("Creating Wafer_Data.");
    for k in 1..=FILENUM {
        create_data(& mut chip_array);
        save_file(& chip_array,k)?;
    }
    println!("Writing finished");
    Ok(())
}
