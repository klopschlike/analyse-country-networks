use std::io::{self};
use std::net::{Ipv4Addr};
use ipnet::{Ipv4Net};
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;

use binary_search_tree::BinarySearchTree;
 
fn main() -> io::Result<()> {
    let nets = read_nets_from_directory("./zones").expect("Could not open zone files");
    let mut tree: BinarySearchTree<Ipv4Net> = BinarySearchTree::new();
    for net in nets {
        tree.insert(net);
    }

    let stdin = io::stdin();
    let handle = stdin.lock();
    let net = Ipv4Addr::new(255, 255, 255, 255);
    for line in handle.lines() {
        let str = line.unwrap();
        let ip_address: Ipv4Addr = str.parse().expect("Invalid IP");
        let single_net = Ipv4Net::with_netmask(ip_address, net).expect("Invalid Net");
        let net4 = tree.predecessor(&single_net);
        // TODO: contains has to be checked
        if let Some(net) = net4 {
            println!("{}", net);
        } else {
            println!("No matching net found for {}", ip_address);
        }
    }
    Ok(())
}

fn get_containing_net<'a>(
    nets: &'a [Ipv4Net],
    ip_address: Ipv4Addr
) -> Option<&'a Ipv4Net> {
    nets.iter().find(|net| net.contains(&ip_address))
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
