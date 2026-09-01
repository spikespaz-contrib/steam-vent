use num_enum::{FromPrimitive, IntoPrimitive};

/// A Steam result code.
///
/// Valve documents no meaning for these codes. Unless a variant says
/// otherwise, its name is the whole of what is known about it.
#[derive(FromPrimitive, IntoPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
#[non_exhaustive]
pub enum EResult {
    /// Valve's name for code 0.
    ///
    /// Not a result Steam is observed to send, and not what an absent result
    /// decodes to -- `eresult` fields declare a default of [`EResult::Fail`].
    Invalid = 0,
    OK = 1,
    Fail = 2,
    NoConnection = 3,
    InvalidPassword = 5,
    LoggedInElsewhere = 6,
    InvalidProtocolVer = 7,
    InvalidParam = 8,
    FileNotFound = 9,
    Busy = 10,
    InvalidState = 11,
    InvalidName = 12,
    InvalidEmail = 13,
    DuplicateName = 14,
    AccessDenied = 15,
    Timeout = 16,
    Banned = 17,
    AccountNotFound = 18,
    InvalidSteamID = 19,
    ServiceUnavailable = 20,
    NotLoggedOn = 21,
    Pending = 22,
    EncryptionFailure = 23,
    InsufficientPrivilege = 24,
    LimitExceeded = 25,
    Revoked = 26,
    Expired = 27,
    AlreadyRedeemed = 28,
    DuplicateRequest = 29,
    AlreadyOwned = 30,
    IPNotFound = 31,
    PersistFailed = 32,
    LockingFailed = 33,
    LogonSessionReplaced = 34,
    ConnectFailed = 35,
    HandshakeFailed = 36,
    IOFailure = 37,
    RemoteDisconnect = 38,
    ShoppingCartNotFound = 39,
    Blocked = 40,
    Ignored = 41,
    NoMatch = 42,
    AccountDisabled = 43,
    ServiceReadOnly = 44,
    AccountNotFeatured = 45,
    AdministratorOK = 46,
    ContentVersion = 47,
    TryAnotherCM = 48,
    PasswordRequiredToKickSession = 49,
    AlreadyLoggedInElsewhere = 50,
    Suspended = 51,
    Cancelled = 52,
    DataCorruption = 53,
    DiskFull = 54,
    RemoteCallFailed = 55,
    // PasswordNotSet = 56, // removed renamed to PasswordUnset
    PasswordUnset = 56,
    ExternalAccountUnlinked = 57,
    PSNTicketInvalid = 58,
    ExternalAccountAlreadyLinked = 59,
    RemoteFileConflict = 60,
    IllegalPassword = 61,
    SameAsPreviousValue = 62,
    AccountLogonDenied = 63,
    CannotUseOldPassword = 64,
    InvalidLoginAuthCode = 65,
    // AccountLogonDeniedNoMailSent = 66, // removed renamed to AccountLogonDeniedNoMail
    AccountLogonDeniedNoMail = 66,
    HardwareNotCapableOfIPT = 67,
    IPTInitError = 68,
    ParentalControlRestricted = 69,
    FacebookQueryError = 70,
    ExpiredLoginAuthCode = 71,
    IPLoginRestrictionFailed = 72,
    // AccountLocked = 73, // removed renamed to AccountLockedDown
    AccountLockedDown = 73,
    AccountLogonDeniedVerifiedEmailRequired = 74,
    NoMatchingURL = 75,
    BadResponse = 76,
    RequirePasswordReEntry = 77,
    ValueOutOfRange = 78,
    UnexpectedError = 79,
    Disabled = 80,
    InvalidCEGSubmission = 81,
    RestrictedDevice = 82,
    RegionLocked = 83,
    RateLimitExceeded = 84,
    // AccountLogonDeniedNeedTwoFactorCode = 85, // removed renamed to AccountLoginDeniedNeedTwoFactor
    AccountLoginDeniedNeedTwoFactor = 85,
    // ItemOrEntryHasBeenDeleted = 86, // removed renamed to ItemDeleted
    ItemDeleted = 86,
    AccountLoginDeniedThrottle = 87,
    TwoFactorCodeMismatch = 88,
    TwoFactorActivationCodeMismatch = 89,
    // AccountAssociatedToMultiplePlayers = 90, // removed renamed to AccountAssociatedToMultiplePartners
    AccountAssociatedToMultiplePartners = 90,
    NotModified = 91,
    // NoMobileDeviceAvailable = 92, // removed renamed to NoMobileDevice
    NoMobileDevice = 92,
    // TimeIsOutOfSync = 93, // removed renamed to TimeNotSynced
    TimeNotSynced = 93,
    SMSCodeFailed = 94,
    // TooManyAccountsAccessThisResource = 95, // removed renamed to AccountLimitExceeded
    AccountLimitExceeded = 95,
    AccountActivityLimitExceeded = 96,
    PhoneActivityLimitExceeded = 97,
    RefundToWallet = 98,
    EmailSendFailure = 99,
    NotSettled = 100,
    NeedCaptcha = 101,
    GSLTDenied = 102,
    GSOwnerDenied = 103,
    InvalidItemType = 104,
    IPBanned = 105,
    GSLTExpired = 106,
    InsufficientFunds = 107,
    TooManyPending = 108,
    NoSiteLicensesFound = 109,
    WGNetworkSendExceeded = 110,
    AccountNotFriends = 111,
    LimitedUserAccount = 112,
    CantRemoveItem = 113,
    AccountHasBeenDeleted = 114,
    AccountHasAnExistingUserCancelledLicense = 115,
    DeniedDueToCommunityCooldown = 116,
    NoLauncherSpecified = 117,
    MustAgreeToSSA = 118,
    ClientNoLongerSupported = 119,
    /// A code with no name in this table; the payload is the code itself.
    ///
    /// Not one of Valve's variants. Its own discriminant is `i32::MIN`, outside
    /// the range Valve's codes occupy.
    #[num_enum(catch_all)]
    Unknown(i32) = i32::MIN,
}

impl EResult {
    pub fn from_result(result: i32) -> Result<(), EResult> {
        match EResult::from(result) {
            EResult::OK => Ok(()),
            err => Err(err),
        }
    }
}

#[test]
fn test_unknown_code_keeps_its_number() {
    // A code with no name here round-trips as its own value.
    assert!(matches!(
        EResult::from_result(9999),
        Err(EResult::Unknown(9999))
    ));
    assert_eq!(9999, i32::from(EResult::Unknown(9999)));

    // `Unknown`'s own discriminant is carried as data like any other code.
    assert!(matches!(
        EResult::from(i32::MIN),
        EResult::Unknown(i32::MIN)
    ));

    // Named codes map both ways, and only `OK` is a success.
    assert_eq!(EResult::RateLimitExceeded, EResult::from(84));
    assert_eq!(84, i32::from(EResult::RateLimitExceeded));
    assert_eq!(Ok(()), EResult::from_result(1));
    assert_eq!(Err(EResult::Invalid), EResult::from_result(0));
}
