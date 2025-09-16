# Wingfoil

Wingfoil is a blazingly fast, highly scalable stream processing framework designed for latency-critical use cases such as electronic trading and real-time AI systems.

Wingfoil is a [lingua franca](https://en.wikipedia.org/wiki/Lingua_franca) of stream
processing, making it easy to receive, process, and distribute streaming data.   

## Features

- **Fast**: Ultra-low latency and high throughput with a efficent [DAG](https://en.wikipedia.org/wiki/Directed_acyclic_graph) based execution engine.  
- **Simple and obvious to use**: Define your graph of calculations; Wingfoil manages it's execution.  
- **Backtesting**: Replay historical data to backtest and optimise strategies.
- **Async/Tokio**: seemless integration, allows you to leverage async at your graph edges.
- **Multi-threading**: distribute graph execution across cores.

## Quick Start
```rust
use wingfoil::*;
use std::time::Duration;
fn main() {
    let period = Duration::from_secs(1);
    ticker(period)
        .count()
        .map(|i| format!("hello, world {:}", i))
        .print()
        .run(RunMode::RealTime, RunFor::Duration(period*3)
    );
}
```
This output is produced:
```pre
hello, world 1
hello, world 2
hello, world 3
```


<!--
Website | Guides | API Docs | Chat
Read the docs or jump straight into some examples.
-->
