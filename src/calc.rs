use chrono::prelude::*;
use clap::Parser;
use treeseq_stats_prototype::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    #[arg(short, long)]
    treefile: String,
}

fn read_treefile(treefile: &str) -> tskit::TreeSequence {
    tskit::TreeSequence::load(treefile).unwrap()
}

fn summarize_treeseq(ts: &tskit::TreeSequence) {
    println!("Summary of tree sequence:");
    println!("\t{} trees", ts.num_trees());
    println!("\t{} samples", ts.num_samples());
    println!("\t{} sites", ts.sites().num_rows());
    println!("\t{} mutations", ts.mutations().num_rows());
}

fn main() {
    let options = Options::parse();
    let pre_load = Utc::now();
    let ts = read_treefile(&options.treefile);
    let post_load = Utc::now();
    println!("I/O time: {}", post_load - pre_load);
    summarize_treeseq(&ts);
    let pre_div = Utc::now();
    let diversity = diversity(&ts).unwrap();
    let post_div = Utc::now();
    println!("{diversity} {}", post_div - pre_div);
}
