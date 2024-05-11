# Arbitrengine

## Description
A graph based arbitrage calculator with state updates

## Running

### Build from source

```bash
# Clone the repository
git clone https://github.com/Cabbache/arbitrengine
cd app

# Run the project
cargo build --release
```

### With docker

```bash
docker pull cabbache/arbitrengine
docker run -it -d -p 5000:5000 --rm cabbache/arbitrengine
netcat 127.0.0.1 5000
```

## Usage
![Example](https://cabbache.github.io/arbitrengine.gif)
