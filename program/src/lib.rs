#![cfg(feature = "program")]

use byteorder::{ByteOrder, LittleEndian};
use num_derive::FromPrimitive;
use solana_sdk::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint,
    entrypoint::ProgramResult,
    info,
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    pubkey::Pubkey,
    timing::timestamp,
};
use std::convert::TryInto;
use std::mem;
use thiserror::Error;

type Bid = u32;

/// The auction summary length.
const SUMMARY_DATA_LEN: usize = 8 + 32 + 4;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
enum Error {
    #[error("Auction Ended")]
    AuctionEnded,
}

impl From<Error> for ProgramError {
    fn from(e: Error) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for Error {
    fn type_of() -> &'static str {
        "Error"
    }
}

/// Public auction information.
// TODO: split into read-only and writable parts for better performance.
struct AuctionSummary {
    /// The deadline timestamp.
    deadline: u64,
    /// Winning bid value.
    winning_bid: Bid,
    /// Winning bidder Pubkey.
    winning_pubkey: Pubkey,
}

impl Sealed for AuctionSummary {}

impl Pack for AuctionSummary {
    const LEN: usize = SUMMARY_DATA_LEN;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        Ok(AuctionSummary {
            deadline: LittleEndian::read_u64(&src[0..7]),
            winning_bid: LittleEndian::read_u32(&src[8..11]),
            winning_pubkey: Pubkey::new(&src[12..43]),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        LittleEndian::write_u64(&mut dst[0..7], self.deadline);
        LittleEndian::write_u32(&mut dst[8..11], self.winning_bid);
        mem::replace::<[u8; 32]>(
            &mut dst[12..43].try_into().expect("cannot copy pubkey"),
            Pubkey::to_bytes(self.winning_pubkey),
        );
    }
}

/// Program instructions.
// TODO: Add an instruction to cancel a bid. Perhaps remove GetSummary.
enum Instruction {
    /// Get the auction deadline and the current maximum bid pubkey and value.
    GetSummary,
    /// Place a bid.
    PlaceBid(Bid),
}

impl Sealed for Instruction {}

impl Pack for Instruction {
    const LEN: usize = 1 + 4;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        match src[0] {
            0 => Ok(Instruction::GetSummary),
            1 => Ok(Instruction::PlaceBid(LittleEndian::read_u32(&src[1..4]))),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }

    fn pack_into_slice(&self, _dst: &mut [u8]) {}
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation.
fn process_instruction(
    program_id: &Pubkey,      // Public key of the auction account.
    accounts: &[AccountInfo], // Accounts: 1) auction, 2) bidder.
    instruction_data: &[u8],  // Instruction
) -> ProgramResult {
    info!("Auction BPF program entrypoint");
    let t = timestamp();
    let instruction = Instruction::unpack_unchecked(&instruction_data)?;
    let accounts_iter = &mut accounts.iter();
    let auction_account = next_account_info(accounts_iter)?;
    if auction_account.owner != program_id {
        info!("Auction account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    if auction_account.try_data_len()? < SUMMARY_DATA_LEN {
        info!("Auction account data length is too small");
        return Err(ProgramError::AccountDataTooSmall);
    }
    let mut summary_data = auction_account.try_borrow_mut_data()?;
    let mut summary = AuctionSummary::unpack_unchecked(&summary_data)?;
    match instruction {
        Instruction::GetSummary => {
            // TODO
        }
        Instruction::PlaceBid(bid) => {
            if t > summary.deadline {
                return Err(Error::AuctionEnded.into());
            }
            let bidder_account = next_account_info(accounts_iter)?;
            let bidder_pubkey = bidder_account.unsigned_key().clone();
            if bid > summary.winning_bid {
                summary.winning_bid = bid;
                summary.winning_pubkey = bidder_pubkey;
            }
        }
    }
    AuctionSummary::pack(summary, &mut summary_data)
}

#[cfg(test)]
mod test {
    // TODO
}

// Required to support info! in tests
#[cfg(not(target_arch = "bpf"))]
solana_sdk::program_stubs!();
