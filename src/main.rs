use std::io::{self};
use std::net::{Ipv4Addr};
use ipnet::{Ipv4Net};
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fs;

use std::collections::BTreeSet;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Ipv4NetWithZone {
    net: Ipv4Net,
    zone: String
}
 
fn main() -> io::Result<()> {
    let nets = read_nets_from_directory("./zones").expect("Could not open zone files");
    let mut btree: BTreeSet<Ipv4NetWithZone> = BTreeSet::new();
    for net in nets {
        btree.insert(net);
    }

    let stdin = io::stdin();
    let handle = stdin.lock();
    let net = Ipv4Addr::new(255, 255, 255, 255);
    for line in handle.lines() {
        let str = line.unwrap();
        let ip_address: Ipv4Addr = str.parse().expect("Invalid IP");
        let single_net = Ipv4Net::with_netmask(ip_address, net).expect("Invalid Net");
        let dummy_zone_net = Ipv4NetWithZone {
            net: single_net,
            zone: "dummy".to_string()
        };
        let net4 = btree.range(..dummy_zone_net).next_back();
        if let Some(net) = net4 {
            if net.net.contains(&ip_address) {
                println!("{} in net {} and zone {}", ip_address, net.net, net.zone);
            }
        } else {
            println!("No matching net found for {}", ip_address);
        }
    }
    Ok(())
}

fn read_nets_from_directory(folder_path: &str) -> Result<Vec<Ipv4NetWithZone>, Box<dyn Error>>
{
    let mut nets: Vec<Ipv4NetWithZone> = Vec::new();
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

fn read_nets_from_file(filename: &str) -> Result<Vec<Ipv4NetWithZone>, Box<dyn Error>>
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut nets = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?.trim().to_string();
        nets.push(Ipv4NetWithZone {
            net: line.parse()?,
            zone: filename.to_string()
        });
    }
    Ok(nets)
}
