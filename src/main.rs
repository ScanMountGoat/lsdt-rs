
use std::path::PathBuf;
use std::io::BufReader;
use std::io::Seek;
use std::io::Read;
use std::io::SeekFrom;
use std::fs::File;
use std::io::Cursor;
mod ls;
mod rf;

pub use ls::ls_str::crc32;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dt: PathBuf = args[1].as_str().into();
    let ls: PathBuf = args[2].as_str().into();
    let out_dir: PathBuf = args[3].as_str().into();
    extract(ls,dt, out_dir);
}

fn extract(_ls_file: PathBuf, _dt_file: PathBuf,_out_folder: PathBuf) {
    let ls = ls::LSFile::open(_ls_file).unwrap(); 

    let rf_file_info = ls.find("resource");
    //Resorce file info get!

    //let d = File::open(_dt_file).unwrap();
    print!("RF Compressed is {0} Bytes\nRF Offset is {1}\n",rf_file_info.size,rf_file_info.offset);
    let  mut dt = BufReader::new(File::open(_dt_file).unwrap());
    //Load our Data file into memory
    dt.seek(SeekFrom::Start(rf_file_info.offset as u64)).unwrap();
    //Seek to our RF file
    let mut rf_data = vec![0u8;rf_file_info.size as usize];
    //Alocate memory for rf
    dt.read_exact(&mut rf_data).unwrap();
    //Read from buffer into that alocated memory
    let mut rf_cursor = Cursor::new(rf_data);
    let rf = rf::RFFile::read(&mut rf_cursor);
    println!("{0} is at pos {1}",rf.header.magic,rf_cursor.position());
    println!("{0}",rf.data.name_offset);



}