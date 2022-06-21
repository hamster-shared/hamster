use codec::{Decode, Encode};
use sp_std::vec::Vec;
use frame_support::{Parameter, RuntimeDebug, BoundedVec};
use sp_runtime::traits::AtLeast32BitUnsigned;

// /*
//     Use BondundedVec to Specify that the interpretation is up to 64 bytes
// */
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct AuthInfo<BlockNumber,Moment, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub hash : Vec<u8>,
    pub accountld : AccountId,
    pub blocknumber : BlockNumber,
    pub description: BoundedVec<u8, frame_support::traits::ConstU32<64>>,
    pub orgcode : Vec<u8>,
    pub file: Vec<u8>,
    pub people: Vec<u8>,
    pub creation: Moment,
}

impl<BlockNumber,Moment, AccountId> AuthInfo<BlockNumber,Moment, AccountId>
    where BlockNumber: Parameter + AtLeast32BitUnsigned{
    pub fn new( hash: Vec<u8>,
                accountld: AccountId,
                blocknumber: BlockNumber,
                description: BoundedVec<u8, frame_support::traits::ConstU32<64>>,
                orgcode: Vec<u8>,
                file: Vec<u8>,
                people: Vec<u8>,
                creation: Moment
    ) -> Self {
        Self{
            hash,
            accountld,
            blocknumber,
            description,
            orgcode,
            file,
            people,
            creation
        }
    }
}
//
// /*
// The Explanation of status:
//     0: Allow organization to define rights
//     1: Not allow organization to define rights
//     ...
// */
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct OrgInfo {
    pub code : Vec<u8>,
    pub name : Vec<u8>,
    pub status: u8,
}

impl OrgInfo {
    pub fn new( code: Vec<u8>,
                name: Vec<u8>,
                status: u8) -> Self {
        Self {
            code,
            name,
            status,
        }
    }
}