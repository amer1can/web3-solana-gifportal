const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log('Starting test...');

  // Create and set the provider.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Gifportal;

  // Create an account keypair for our program to use.
  const baseAccount = anchor.web3.Keypair.generate();

  // Call initialize, pass it the params it needs!
  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log('Your transaction signature', tx);

  // Fetch data from the account.
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count', account.totalGifs.toString()); //we can use camelCase in JS and snake_case in Rust

  await program.rpc.addGif("https://media.giphy.com/media/7ZjmsISzWnreE/giphy.gif",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  })

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('GIF Count:', account.totalGifs.toString())

  console.log('GIF List:', account.gifList)

}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (err) {
    console.error('runMain error!', err);
    process.exit(1);
  }
};

runMain();

// const anchor = require('@project-serum/anchor');
//
// describe('gifportal', () => {
//
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.Provider.env());
//
//   it('Is initialized!', async () => {
//     // Add your test here.
//     const program = anchor.workspace.Gifportal;
//     const tx = await program.rpc.initialize();
//     console.log("Your transaction signature", tx);
//   });
// });
