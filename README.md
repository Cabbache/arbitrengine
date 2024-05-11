# Arbitrengine

## Description
A graph based arbitrage calculator with state updates. Uses [petgraph::algo::bellman\_ford::find\_negative\_cycle](https://docs.rs/petgraph/latest/petgraph/algo/bellman_ford/fn.find_negative_cycle.html)

## Running

### Build from source

```bash
# Clone the repository
git clone https://github.com/Cabbache/arbitrengine
cd app

# Run the project
cargo run --release
```

### With docker

```bash
docker pull cabbache/arbitrengine
docker run -it -d -p 5000:5000 --rm cabbache/arbitrengine
netcat 127.0.0.1 5000
```

## Usage
![Example](https://cabbache.github.io/arbitrengine.gif)

## Issues
- It does not find all cycles
- [https://github.com/petgraph/petgraph/issues/642](https://github.com/petgraph/petgraph/issues/642)
