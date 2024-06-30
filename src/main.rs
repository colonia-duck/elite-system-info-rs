/* Using a Spansh Bodies Datadump,
*  write a Tool that reads a JSON
*  Stream using a single consumer
*  and multiple worker threads.
*  The worker threads look for Planets > 120kls
*  orbiting a gas giant w/ water based life. 
*/

// using the flate2 library to stream the file

use flate2::read::GzDecoder;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufReader}; 

fn decoder(file_path: &str) -> io::Result<Vec<u8>> {

    // opens file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut decoder = GzDecoder::new(reader);

    let mut decompressed_data = Vec::new();

    std::io::copy(&mut decoder, &mut decompressed_data);

    Ok(decompressed_data) 

}

fn main() -> io::Result<()>{
    let data_file = "src/galaxy_7days.json.gz";

    let decompressed_data = decoder(data_file);

    

    println!("parsed data: {:?}", decompressed_data);
    Ok(())
    
}
