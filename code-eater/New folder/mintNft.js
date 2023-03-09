import { ThirdwebSDK } from "@thirdweb-dev/sdk/solana";
import {readFileSync} from 'fs';;

const sdk = ThirdwebSDK.fromPrivateKey("devnet", "5kt2deMMM9pwvNrVxd751mJ4jKDEJPPWDmBat7yhVmdm4Y1eZmPsJJPW9KQW7MoVhkyGbN7pDFmNR28ghDJ2PEwd");

const myNftCollection = await sdk.getNFTCollection(
    "D7tGncgoitXswRGa4L5tDywXViDViK5z3hmqq3j1PGxK"
);

const meta = {
    name: "Antematter 2",
    description:"Blockchain industry",
    image:readFileSync("./code.jpg")
};

const mintNft = await myNftCollection.mint(meta);
console.log("mint nft", mintNft);

const nftMinted = await myNftCollection.getAll();
console.log("Nft Minted", nftMinted);

// mint nft AfnFkCma9KP9vHFyUbsHYnySc9XsHov1BpYzwJUf6jGU
// Nft Minted [
//   {
//     metadata: {
//       id: '9KEMhUXBAX98LMYwDeCGf23FpHKhJYvGmN7q2zs9WW8N',
//       uri: 'https://gateway.ipfscdn.io/ipfs/QmNRCCqCrq6SSkbCXByigv8H1aPieoXdXVgEDZnF75e7wp/0',
//       name: 'Antematter 2',
//       symbol: '',
//       description: 'Blockchain industry',
//       image: 'https://gateway.ipfscdn.io/ipfs/QmSVudVY6UQ8CD3gWaEM2rzxCBNkJgRELriGgf3hgrTAxt/0'
//     },
//     owner: 'Gva1Bw1NdDaTSrYY5HRK76i86FffHLkXHjhhfFWg7D8F',
//     supply: '1',
//     type: 'metaplex'
//   },