# apca_datav2
This crate provides bindings for Alpaca's Data API v2. It provides both the
realtime and historical data information. The point of this crate is by no means
to provide a full coverage of all the available api. But only the market data
api (v2). I tried to stay faithful to the code and documentation provided by 
Alpaca. The original doc is to be found here:
https://alpaca.markets/docs/api-documentation/api-v2/market-data/alpaca-data-api-v2/

## Why is it v2 ?
The v2 by no means intends to say that these are the 2nd implementations of 
bindings for Alpaca's market data. Rather do they mean the bindings are meant
to exploit the 2nd version of Alpaca's API. (v1 is decommissioned on august 26, 
2021)

## Note
As far as I am concerned, this project is mostly thought of as a big fat pull
request to be applied on top of https://github.com/d-e-s-o/apca. But the truth
is, I feel a bit lazy about making a smooth integration for it in `apca`.