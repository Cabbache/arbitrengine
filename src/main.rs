use std::io;

use std::collections::{HashMap, HashSet};

use petgraph::Graph;
use petgraph::algo::find_negative_cycle;
use petgraph::prelude::*;

type CGraph = Graph::<String, f64, Directed>;
type IndexMap = HashMap::<String, NodeIndex>;
type GraphPath = Vec<NodeIndex>;

#[derive(Clone, Debug)]
struct Exchange {
	from: String,
	to: String,
	rate: f64,
}

fn mkerror(msg: String) -> String {
	format!("ERROR {}", msg)
}

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
	.collect::<Result<Vec<Exchange>, String>>()
	.and_then(|list| {
		let unique_len = HashSet::<(String, String)>::from_iter(
			list.clone()
			.into_iter()
			.map(|exchange| (exchange.to, exchange.from))
		).len();
		match unique_len == list.len() {
			true => Ok(list),
			false => Err("duplicate entries".to_string())
		}
	})
}

fn path_to_string(path: GraphPath, currencies: &CGraph) -> String {
	let indexes = path.iter().map(|index| currencies[*index].clone());
	indexes.clone().zip(indexes.skip(1))
	.map(|(a, b)| format!("{}/{}", a, b))
	.collect::<Vec<String>>()
	.join(",")
}

fn get_neg_cycles(currencies: &CGraph) -> Vec<GraphPath> {
	currencies
	.node_indices()
	.filter_map(
		|index| find_negative_cycle(&currencies, index)
		.and_then(|mut negc| {
			if *negc.last().unwrap() != index {
				negc.push(index);
			}
			if *negc.first().unwrap() != index {
				let mut cl = vec![index];
				cl.extend(negc);
				negc = cl;
			}
			Some(negc)
		})
	).collect::<Vec<GraphPath>>()
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
				for cycle in get_neg_cycles(&currencies) {
					println!("{}", path_to_string(cycle, &currencies));
				}
			},
			Err(msg) => eprintln!("{}", mkerror(msg))
		}
	}
}
