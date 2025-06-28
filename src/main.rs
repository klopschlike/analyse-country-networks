use std::io::{self};
use std::net::{Ipv4Addr};
use ipnet::{Ipv4Net};
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;

fn main() -> io::Result<()> {
    let nets = read_nets_from_directory("./zones").expect("Could not open zone files");
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let str = line.unwrap();
        let ip_address: Ipv4Addr = str.parse().expect("Invalid IP");
        for net4 in &nets {
            if net4.contains(&ip_address) {
                println!("{}", net4);
            }
        }
    }
    Ok(())
}

fn read_nets_from_directory(folder_path: &str) -> Result<Vec<Ipv4Net>, Box<dyn Error>>
{
    let mut nets: Vec<Ipv4Net> = Vec::new();
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.ends_with(".zone") {
                    let mut nets2 = read_nets_from_file(path.to_str().unwrap()).unwrap();
                    nets.append(&mut nets2)
                }
            }
        }
    }
    return Ok(nets)
}

fn read_nets_from_file(filename: &str) -> Result<Vec<Ipv4Net>, Box<dyn Error>>
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut nets = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        let net: Ipv4Net = line.parse()?;
        nets.push(net);
    }
    Ok(nets)
}
