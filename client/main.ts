import {
  establishConnection,
  establishPayer,
  loadProgram,
  // sayHello,
  // reportHellos,
} from './auction';

async function main() {
  console.log("Creating an auction on Solana...");

  // Establish connection to the cluster
  await establishConnection();

  // Determine who pays for the fees
  await establishPayer();

  // Load the program if not already loaded
  await loadProgram();

  // // Say hello to an account
  // await sayHello();

  // // Find out how many times that account has been greeted
  // await reportHellos();

  console.log('Success');
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);
