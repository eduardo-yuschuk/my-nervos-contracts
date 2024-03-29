use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    TypeArgsInvalid,
    VersionInvalid,
    CollectionDataInvalid,
    CollectionTotalSmallerThanIssued,
    CollectionCellsCountError,
    CollectionIssuedInvalid = 15,
    CollectionImmutableFieldsNotSame,
    CollectionCellCannotDestroyed,
    CollectionIdIncreaseError,
    NFTDataInvalid,
    NFTCellsCountError = 20,
    NFTTokenIdIncreaseError,
    NFTAndCollectionConfigureNotSame,
    NFTCharacteristicNotSame,
    NFTDataNotSame,
    NFTClaimedToUnclaimedError = 25,
    NFTLockedToUnlockedError,
    NFTDisallowClaimed,
    NFTDisallowLocked,
    NFTCannotTransferBeforeClaim,
    NFTCannotTransferAfterClaim = 30,
    NFTExtInfoLenError,
    NFTExtInfoCannotModify,
    NFTCannotDestroyBeforeClaim,
    NFTCannotDestroyAfterClaim,
    LockedNFTCannotClaim = 35,
    LockedNFTCannotTransfer,
    LockedNFTCannotAddExtInfo,
    LockedNFTCannotDestroy,
    LockedNFTCannotUpdateCharacteristic,
    GroupInputWitnessNoneError = 40,
    PaymentNotEnough = 45,
    InvalidPaymentLockScript,
    NFTUnlockedOnCreateError,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}
