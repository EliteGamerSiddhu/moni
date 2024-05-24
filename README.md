# CW721-Factory Cosmwasm Smart Contract
A simple cw721-factory contract designed for Oraichain testnet as a task of OSOD (Oraichain Season of Docs).

The cw721-factory contract needs a cw721-base contract and the NFT data to mint an NFT and output a cw721 address for them. The cw721-base acts as the collection whom the minted NFT will be a part of.

<img src="https://github.com/EliteGamerSiddhu/moni/assets/65298483/644edf2f-ed0a-4d39-b9ce-24776e1bc986" width=500 height=500>

I used my previously created cw20 token [SYPZ](https://github.com/Ansh1902396/first_token/tree/Smartypanz) & used IPFS through [Pinata](https://www.pinata.cloud/) which also provides a Gateway to access stored files, to make sure the NFT is secure and to avoid "rug pulls". In the token URI, I used the metadata standards as explained by [OpenSea Developer Docs](https://docs.opensea.io/docs/metadata-standards).

Data of each of the contracts can be seen through the [Oraichain Block Explorer](https://scan.orai.io/smart-contracts).

For example, to make this Digital Eye NFT, whose contract address is orai1djhtqhjjml3gxu54gwg68nuxts744hkd7rkqu5s3xx96y62rrt7sjw83ar , and this SmoKid NFT, whose contract address is orai1m4r7yedmn7rzac6d0439ac4m6qdhsgecvs9tkchfqwarkzdc8hfs06wppw , uses this cw721-base contract, whose contract address is orai12x8futf90m3npkhu2aadhjxrx9u0m9lef0jexqargy7jx2pthdmqpsses0 , to make each of them and make it a part of the Perspective NFT collection.

Oraichain Docs : https://docs.oraichaindao.org/cosmwasm-quick-start/writing-cw-721-nft-contract
