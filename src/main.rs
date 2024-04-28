use std::io;

use std::collections::HashMap;

use petgraph::Graph;
use petgraph::algo::find_negative_cycle;
use petgraph::prelude::*;

#[derive(Debug)]
struct Exchange {
	from: String,
	to: String,
	rate: f64,
}

type CGraph = Graph::<String, f64, Directed>;
type IndexMap = HashMap::<String, NodeIndex>;

fn parse_input(data: String) -> Result<Vec<Exchange>, String> {
	data.split(",")
	.map(|value| {
		let exchstr = value.split("/").collect::<Vec<&str>>();
		match exchstr[..] {
			[from, to, ratestr] => {
				let rate = str::parse::<f64>(ratestr);
				if rate.is_err() {
					return Err(format!("'{}' not a float", ratestr));
				}
				Ok(Exchange {
					from: from.to_string(),
					to: to.to_string(),
					rate: rate.unwrap(),
				})
			}
			_ => Err(format!("bad format '{}'", value).to_string())
		}
	})
	.collect()
}

fn update_states(
	currencies: &mut CGraph, 
	currency_index: &IndexMap,
	data: &Vec<Exchange>
) {
	
}

fn main() {
//	let graph_with_neg_cycle = Graph::<(), f32, Directed>::from_edges(&[
//		(0, 1, 1.),
//		(0, 2, 1.),
//		(0, 3, 1.),
//		(1, 3, 1.),
//		(2, 1, 1.),
//		(3, 2, -3.),
//	]);
//	let path = find_negative_cycle(&graph_with_neg_cycle, NodeIndex::new(0));
//	println!("{:?}", path);

	let currency_index = IndexMap::new();
	let mut currencies = CGraph::new();

	let index1 = currencies.add_node("USD".to_string());
	let index2 = currencies.add_node("EUR".to_string());
	println!("{:?}", index1);
	println!("{:?}", index2);

	loop {
		let mut updates = String::new();
		io::stdin().read_line(&mut updates).unwrap();
		updates = updates.trim().to_string();
		let input_data = parse_input(updates);
		match input_data {
			Ok(data) => {
				println!("{:?}", data);
				update_states(&mut currencies, &currency_index, &data);
			},
			Err(msg) => eprintln!("{}", msg)
		}
	}
}
