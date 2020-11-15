## Solana Auction **WIP**


### Overview

The Auction is a simple Solana program allowing an address to start an auction on a Solana
network. Once it is started, an arbitrary number addresses can submit their bids until the deadline
that is set in the auction.

This project is based on the Solana [example-helloworld](https://github.com/solana-labs/example-helloworld).


#### Supported Auction Type

Currently only a type of English auction is supported. The rules of this auction
are:

* The winner is the bidder with the highest bid.
* All bids but the highest one are sealed. The highest bid is readable throughout the auction to
  allow price discovery.
* The auction has a start time (the time of deployment) and an end time, the deadline after which
  bids are no longer accepted and the winner is announced.

As it is common in ascending auctions, it is the bidders' advantage to make bids only at the very
end of the auction since early bids only increase the final price.


### Prerequisites

* [NodeJs](https://nodejs.org/en/) and [NPM](https://www.npmjs.com/).
* [NVM](https://github.com/creationix/nvm) is recommended for downloading and selecting NodeJs versions.
* [Rust](https://www.rust-lang.org/tools/install)
* `curl` or `wget` is used by the BPF SDK install script.


### Quick Start

```bash
npm i
npm run program:build
npm run client:start
```

### WIP Status

The Solana program is very basic and is currently in development.

The client is missing basic required features.

Tests are missing.
