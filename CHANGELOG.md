## Parity-Ethereum [v2.5.9](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.8)

Parity Ethereum v2.5.9-stable is a patch release that adds the block numbers for activating the Istanbul hardfork on test networks: Ropsten, Görli, Rinkeby and Kovan.

The full list of included changes:

* ethcore/res: activate Istanbul on Ropsten, Görli, Rinkeby, Kovan (#11068)
* [json-spec] make blake2 pricing spec more readable (#11034)

## Parity-Ethereum [v2.5.8](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.8)

Parity Ethereum v2.5.8-stable is a patch release that improves security, stability and performance.

* The most noteworthy improvement in this release is incorporating all the EIPs required for the Istanbul hard fork.
* This release also fixes certain security and performance issues, one of which was suspected to be consensus-threatening but turned out to be benign. Thanks to Martin Holst Swende and Felix Lange from the Ethereum Foundation for bringing the suspicious issue to our attention.

The full list of included changes:

* add more tx tests (#11038)
* Fix parallel transactions race-condition (#10995)
* Add blake2_f precompile (#11017)
* [trace] introduce trace failed to Ext (#11019)
* Edit publish-onchain.sh to use https (#11016)
* Fix deadlock in network-devp2p (#11013)
* EIP 1108: Reduce alt_bn128 precompile gas costs (#11008)
* xDai chain support and nodes list update (#10989)
* EIP 2028: transaction gas lowered from 68 to 16 (#10987)
* EIP-1344 Add CHAINID op-code (#10983)
* manual publish jobs for releases, no changes for nightlies (#10977)
* [blooms-db] Fix benchmarks (#10974)
* Verify transaction against its block during import (#10954)
* Better error message for rpc gas price errors (#10931)
* tx-pool: accept local tx with higher gas price when pool full (#10901)
* Fix fork choice (#10837)
* Cleanup unused vm dependencies (#10787)
* Fix compilation on recent nightlies (#10991)
* Don't build rpc with ethcore test-helpers (#11048) 
* EIP 1884 Re-pricing of trie-size dependent operations  (#10992)
* Implement EIP-1283 reenable transition, EIP-1706 and EIP-2200  (#10191)

## Parity-Ethereum [v2.5.7](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.7)

Parity Ethereum v2.5.7-stable is a bugfix release that fixes a potential DoS attack in the trace_call RPC method. This is a critical upgrade for anyone running Parity nodes with RPC exposed to the public internet (and highly recommended for anyone else). For details see this blog post.

## Parity-Ethereum [v2.5.6](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.6)

Parity-Ethereum v2.5.6-stable is a bugfix release that improves stability.

* Allow specifying hostnames for node URLs
* Fix a bug where archive nodes were losing peers

The full list of included changes:

* Kaspersky AV whitelisting (#10919)
* Avast whitelist script (#10900) 
* Docker images renaming (#10863) 
* Remove excessive warning (#10831) 
* Allow --nat extip:your.host.here.org (#10830) 
* When updating the client or when called from RPC, sleep should mean sleep (#10814)
* added new ropsten-bootnode and removed old one (#10794)
* ethkey no longer uses byteorder (#10786) 
* Do not drop the peer with None difficulty (#10772)
* docs: Update Readme with TOC, Contributor Guideline. Update Cargo package descriptions (#10652)

## Parity-Ethereum [v2.5.5](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.5)

Parity-Ethereum v2.5.5-stable is a minor release that improves performance and stability.
This release stabilises the 2.5 branch.

As of today, Parity-Ethereum 2.4 reaches end of life and everyone is
encouraged to upgrade.

## Parity-Ethereum [v2.5.4](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.4)

Parity Ethereum v2.5.4-beta is a security update that addresses servo/rust-smallvec#148

The full list of included changes:

* cargo update -p smallvec ([#10822](https://github.com/paritytech/parity-ethereum/pull/10822))

## Parity-Ethereum [v2.5.3](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.3)

Parity-Ethereum 2.5.3-beta is a bugfix release that improves performance and stability.

* EthereumClassic: activate the Atlantis Hardfork
* Clique: fix time overflow
* State tests: treat empty accounts the same as non-existant accounts (EIP 1052)
* Networking: support discovery-only peers (geth bootnodes)
* Snapshotting: fix unclean shutdown while snappshotting is under way

The full list of included changes:

* ethcore/res: activate atlantis classic hf on block 8772000 ([#10766](https://github.com/paritytech/parity-ethereum/pull/10766))
* fix docker tags for publishing ([#10741](https://github.com/paritytech/parity-ethereum/pull/10741))
* fix: aura don't add `SystemTime::now()` ([#10720](https://github.com/paritytech/parity-ethereum/pull/10720))
* Treat empty account the same as non-exist accounts in EIP-1052 ([#10775](https://github.com/paritytech/parity-ethereum/pull/10775))
* DevP2p: Get node IP address and udp port from Socket, if not included in PING packet ([#10705](https://github.com/paritytech/parity-ethereum/pull/10705))
* Add a way to signal shutdown to snapshotting threads ([#10744](https://github.com/paritytech/parity-ethereum/pull/10744))

## Parity-Ethereum [v2.5.2](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.2)

Parity-Ethereum 2.5.2-beta is a bugfix release that improves performance and stability.

Among others, it enables the _Atlantis_ hardfork on **Morden** and **Kotti** Classic networks.

The full list of included changes:

* [CI] allow cargo audit to fail ([#10676](https://github.com/paritytech/parity-ethereum/pull/10676))
* Reset blockchain properly ([#10669](https://github.com/paritytech/parity-ethereum/pull/10669))
* new image ([#10673](https://github.com/paritytech/parity-ethereum/pull/10673))
* Update publishing ([#10644](https://github.com/paritytech/parity-ethereum/pull/10644))
* enable lto for release builds ([#10717](https://github.com/paritytech/parity-ethereum/pull/10717))
* Use RUSTFLAGS to set the optimization level ([#10719](https://github.com/paritytech/parity-ethereum/pull/10719))
* ethcore: enable ECIP-1054 for classic ([#10731](https://github.com/paritytech/parity-ethereum/pull/10731))

## Parity-Ethereum [v2.5.1](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.1)

Parity-Ethereum 2.5.1-beta is a bugfix release that improves performance and stability. 

Among others, it enables the Petersburg hardfork on **Rinkeby** and **POA-Core** Network, as well as the **Kovan** Network community hardfork.

The full list of included changes:

* ci: publish docs debug ([#10638](https://github.com/paritytech/parity-ethereum/pull/10638))

## Parity-Ethereum [v2.5.0](https://github.com/paritytech/parity-ethereum/releases/tag/v2.5.0)

Parity-Ethereum 2.5.0-beta is a minor release that improves performance and stabilizes the 2.5 branch by marking it as beta release. 

- This release adds support for the Clique consensus engine ([#9981](https://github.com/paritytech/parity-ethereum/pull/9981))
  - This enables Parity-Ethereum users to use the Görli, the Kotti Classic, and the legacy Rinkeby testnet. To get started try `parity --chain goerli`; note that light client support is currently not yet fully functional.
- This release removes the dead chain configs for Easthub and Ethereum Social ([#10531](https://github.com/paritytech/parity-ethereum/pull/10531))

As of today, Parity-Ethereum 2.3 reaches end of life and everyone is encouraged to upgrade.

The full list of included changes:

* fix(light cull): poll light cull instead of timer ([#10559](https://github.com/paritytech/parity-ethereum/pull/10559))

