Portents
========

Finding references to the future in Bitcoin Ordinals.

Programs
--------

Written in Rust in order to access the Ordinals library.

### fetch-block-hashes

Outputs a csv file of block hashes and indexes.

Requires access to a Bitcoin RPC node.

### bignum-tail-search

Loads the csv file of block hashes and indexes,
converts the hash to a bignum,
then checks if the least significant 64 bits of the hash exist as an ordinal.

If so, it outputs the details to a csv file.

Files
-----

### block-lsbs-ordinals-matches.csv

Details of matching block lsbs and ordinals.

To get just the ordinal in integer notation, use:

    tail -n +2 block-lsbs-ordinals-matches.csv | cut -d, -f4  | sort
