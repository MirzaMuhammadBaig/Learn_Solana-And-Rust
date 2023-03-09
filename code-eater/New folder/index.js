import { ThirdwebSDK } from "@thirdweb-dev/sdk/solana";

const sdk = ThirdwebSDK.fromPrivateKey("devnet", "5kt2deMMM9pwvNrVxd751mJ4jKDEJPPWDmBat7yhVmdm4Y1eZmPsJJPW9KQW7MoVhkyGbN7pDFmNR28ghDJ2PEwd");

const nftCollection = await sdk.deployer.createNftCollection({
    name: "Antemater",
    description:"Blockchain Best Indeustry"
});
console.log("NFT Collection", nftCollection);
// D7tGncgoitXswRGa4L5tDywXViDViK5z3hmqq3j1PGxK