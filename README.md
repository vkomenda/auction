## Solana Auction


### Overview

The Auction is a simple Solana program allowing a SOL holder address to start an auction on a Solana
network. Once it is started, an arbitrary number of SOL holder addresses can submit their bids until
the deadline that is set in the auction.


#### Supported Auction Type

Currently only a type of second-price auction is supported which is a simplified version of the
[eBay auction](https://en.wikipedia.org/wiki/EBay#Auction-style_listings). The rules of this auction
are:

* The winning bidder pays the second-highest bid.
* All bids but the second one are sealed. The second bid is readable throughout the auction to allow
  price discovery.
* The auction has a start time and an end time, the deadline after which bids are no longer accepted
  and the winner is announced.

As it is common in sealed bid auctions, it is the bidders' advantage to make bids only at the very
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


### Test

TODO
