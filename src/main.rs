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
	//TODO make sure pairs are unique
}

fn asset_to_index(
	asset_name: String,
	currency_index: &mut IndexMap,
	currencies: &mut CGraph
) -> NodeIndex {
	match currency_index.get(&asset_name) {
		Some(index) => *index,
		None => {
			let index = currencies.add_node(asset_name.clone());
			currency_index.insert(asset_name, index);
			index
		}
	}
}

fn update_states(
	currencies: &mut CGraph, 
	currency_index: &mut IndexMap,
	data: &Vec<Exchange>
) {
	for item in data {
		let from_index = asset_to_index(item.from.clone(), currency_index, currencies);
		let to_index = asset_to_index(item.to.clone(), currency_index, currencies);

		currencies.update_edge(from_index, to_index, item.rate);
		currencies.update_edge(to_index, from_index, 1f64/item.rate);
	}
}

/*
5/6/1,5/7/1,5/8/1,6/8/1,7/6/1,8/7/-1,0/1/1,0/2/1,0/3/1,1/3/1,2/1/1,3/2/-1
*/
fn main() {
	let mut currency_index = IndexMap::new();
	let mut currencies = CGraph::new();

	loop {
		let mut updates = String::new();
		io::stdin().read_line(&mut updates).unwrap();
		updates = updates.trim().to_string();
		let input_data = parse_input(updates);
		match input_data {
			Ok(data) => {
				update_states(&mut currencies, &mut currency_index, &data);
				let neg_cycles = currencies
					.node_indices()
					.filter_map(
						|index| find_negative_cycle(&currencies, index)
							.and_then(|negc| Some((index, negc)))
					);
				for cc in neg_cycles {
					println!("{:?}", cc);
				}
			},
			Err(msg) => eprintln!("{}", msg)
		}
	}
}
