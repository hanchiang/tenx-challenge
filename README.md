
## Data structure
Model graph as a V x V adjacency matrix, because there is a high likelihood of a dense graph and therefore minimise space wastage
Each vertex is represented by `(exchange, currency)` pair
Each edge is represented by the i, j entry in the adjacency matrix

## Price update
**Format**
`<timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>`

**Assumption**
`source_currency` and `destination_currency` cannot be the same because it is meaningless to have price updates between the same currency

## Exchange rate request
**Format**
`EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>`

**Assumption**
`source_currency` and `destination_currency` cannot be the same because the answer will be the same as `source_currency`, and this isn't a useful business question to solve

## File
Each line in the file follows the format of price update or exchange rate request ending with line break

**Operations**
If vertices don't exist, add them in the graph and add the edges.
If vertices exist, update the edges if `timestamp` is after the last updated timestamp for the vertices

## How to use
* Clone project: `git clone git@github.com:hanchiang/tenx-challenge.git`
* Create an input file in project root, e.g. `input.txt`
* Run the program and pass the input file as an argument, e.g. `cargo run input.txt`


## Note
There are over 2000 cryptocurrencies and 200 crypto exchanges as of 18 May 2019
There are multiple trading pairs that involve the same currency such as fiat and base coins, resulting in a denser graph
Price updates arrive frequently