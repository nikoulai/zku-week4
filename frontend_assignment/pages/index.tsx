import detectEthereumProvider from "@metamask/detect-provider";
import { Strategy, ZkIdentity } from "@zk-kit/identity";
import { generateMerkleProof, Semaphore } from "@zk-kit/protocols";
import { Contract, providers, utils } from "ethers";
import Head from "next/head";
import React, { useEffect } from "react";
import styles from "../styles/Home.module.css";
import { CustomForm } from "./form";
import Greeter from "artifacts/contracts/Greeters.sol/Greeters.json";
import { TextField } from "@material-ui/core";

export default function Home() {
  const [logs, setLogs] = React.useState("Connect your wallet and greet!");
  const [greeting, setGreeting] = React.useState("");
  // let provider2: providers.JsonRpcProvider;

  async function greet() {
    setLogs("Creating your Semaphore identity...");

    const provider = (await detectEthereumProvider()) as any;

    await provider.request({ method: "eth_requestAccounts" });

    const ethersProvider = new providers.Web3Provider(provider);
    const signer = ethersProvider.getSigner();
    console.log("signer", await signer.getAddress());
    const message = await signer.signMessage(
      "Sign this message to create your identity!"
    );

    const identity = new ZkIdentity(Strategy.MESSAGE, message);
    const identityCommitment = identity.genIdentityCommitment();
    const identityCommitments = await (
      await fetch("./identityCommitments.json")
    ).json();

    const merkleProof = generateMerkleProof(
      20,
      BigInt(0),
      identityCommitments,
      identityCommitment
    );

    setLogs("Creating your Semaphore proof...");

    const greeting = "Hello world";

    const witness = Semaphore.genWitness(
      identity.getTrapdoor(),
      identity.getNullifier(),
      merkleProof,
      merkleProof.root,
      greeting
    );

    const { proof, publicSignals } = await Semaphore.genProof(
      witness,
      "./semaphore.wasm",
      "./semaphore_final.zkey"
    );
    const solidityProof = Semaphore.packToSolidityProof(proof);

    const response = await fetch("/api/greet", {
      method: "POST",
      body: JSON.stringify({
        greeting,
        nullifierHash: publicSignals.nullifierHash,
        solidityProof: solidityProof,
      }),
    });

    if (response.status === 500) {
      const errorMessage = await response.text();

      setLogs(errorMessage);
    } else {
      setLogs("Your anonymous greeting is onchain :)");
      let greeting = await response.text();
      setGreeting(utils.parseBytes32String(greeting));
    }
    let filter = {
      address: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
      topics: [utils.id("NewGreeting(bytes32)")],
    };

    // provider2 = new providers.JsonRpcProvider("http://localhost:8545");
  }
  useEffect(() => {
    let provider2 = new providers.JsonRpcProvider("http://localhost:8545");
    // const ethersProvider = new providers.Web3Provider(provider2);
    const contract = new Contract(
      "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
      Greeter.abi,
      provider2
    );
    const getNetwork = async () => {
      const { chainId } = await provider2.getNetwork();
      console.log(chainId); // 42
    };
    getNetwork().catch((err) => {
      console.log(err);
    });

    let filter = {
      address: "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512",
      topics: [
        // the name of the event, parnetheses containing the data type of each event, no spaces
        utils.id("NewGreeting(bytes32)"),
      ],
    };
    const contractOwner = contract.connect(provider2.getSigner());
    provider2.on(filter, (greeting) => {
      console.log("***");
      // do whatever you want here
      // I'm pretty sure this returns a promise, so don't forget to resolve it
      // alert("event " + utils.parseBytes32String(greeting));
      setGreeting(utils.parseBytes32String(greeting));
    });

    contract.on("NewGreeting", (greeting) => {
      console.log("****");
      setGreeting(utils.parseBytes32String(greeting));
    });
    // return () => {
    //   contract.removeAllListeners("NewGreeting");
    // };
  }, []);

  return (
    <div className={styles.container}>
      <Head>
        <title>Greetings</title>
        <meta
          name="description"
          content="A simple Next.js/Hardhat privacy application with Semaphore."
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Greetings</h1>

        <p className={styles.description}>
          A simple Next.js/Hardhat privacy application with Semaphore.
          <CustomForm />
        </p>

        <div className={styles.logs}>{logs}</div>

        <div onClick={() => greet()} className={styles.button}>
          Greet
        </div>

        <TextField
          id="first-name"
          label="Greeting"
          value={greeting}
          margin="normal"
          color="secondary"
        />
      </main>
    </div>
  );
}
