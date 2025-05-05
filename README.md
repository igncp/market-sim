# Market Simulator

A basic financial market simulator to explore implementing the mechanics
involved in markets and practice building simulators.

The current stack is based on these technologies:

- Rust
    - `actix-web`: for the server
    - `fake`: for generating fake data
- Redis: for saving the state of the simulation
- Prometheus and Grafana: for monitoring and visualizations

The code is deterministic, using a provided seed approach to generate
randomness in the simulation.
