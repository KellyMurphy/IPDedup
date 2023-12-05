extern crate ipnet;
extern crate iprange;
use ipnet::Ipv4Net;
use iprange::IpRange;
use std::{
    env, fs,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process::exit, 
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.iter().count();
    
    if argc == 1 {
        println!("Error: Invalid number of arguments: ");
        println!("Useage: {} <input folder> <output file>", args[0]);
        println!("   Example: {} \\var\\lists\\mylists combined_list.txt", args[0]);
        exit(1);
    }
    let input_folder = &args[1];
    let output_file = &args[2];
    let mut ip_range_all: IpRange<Ipv4Net> = IpRange::<Ipv4Net>::new();
    println!("Input Folder: {input_folder}");
    println!("Output File: {output_file}");
    let paths = fs::read_dir(input_folder).unwrap();
    for path in paths {
        let pth = path.unwrap().path();
        let lines = lines_from_file(&pth);
        let lines_count = lines.iter().count();
        let ip_range: IpRange<Ipv4Net> = lines.iter().map(|s| s.parse().unwrap()).collect();

        print!(
            "Name: {}, File Count: {}, IPList count {}",
            pth.display(),
            ip_range.into_iter().count(),
            lines_count
        );
        let pre_count=ip_range_all.iter().count();
        ip_range_all = ip_range_all.merge(&ip_range);
        let added_count = ip_range_all.iter().count() - pre_count;

        println!(", Merge Complete, added {}", added_count);
    }
    println!(
        "IP's count of all lists: {}",
        ip_range_all.into_iter().count()
    );
    let mut file = File::create(output_file).unwrap();

    let ips = ip_range_all.into_iter();
    for ip_net in ips {
        writeln!(&mut file, "{}", ip_net).unwrap();
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
