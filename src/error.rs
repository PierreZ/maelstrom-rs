//! Error type, defined by Maelstrom

/// Errors that can be thrown by Maelstrom and/or the user
/// Taken from the [Error doc](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md#errors).
#[derive(Debug, Clone)]
pub enum Error {
    /// Indicates that the requested operation could not be completed within a timeout.
    Timeout,
    /// Thrown when a client sends an RPC request to a node which does not exist.
    NodeNotFound,
    /// Use this error to indicate that a requested operation is not supported by the current implementation.
    NotSupported,
    /// Indicates that the operation definitely cannot be performed at this time--perhaps because
    /// the server is in a read-only state, has not yet been initialized, believes its peers to be down, and so on.
    TemporarilyUnavailable,
    /// The client's request did not conform to the server's expectations, and could not possibly have been processed.
    MalformedRequest,
    /// Indicates that some kind of general, indefinite error occurred.
    Crash,
    /// Indicates that some kind of general, definite error occurred. Use this as a catch-all for errors
    /// you can't otherwise categorize, when you specifically know that the requested operation has not taken place.
    Abort,
    /// The client requested an operation on a key which does not exist (assuming the operation should not automatically create missing keys).
    KeyDoesNotExist,
    /// The client requested the creation of a key which already exists, and the server will not overwrite it.
    KeyAlreadyExist,
    /// The requested operation expected some conditions to hold, and those conditions were not met.
    PreconditionFailed,
    /// The requested transaction has been aborted because of a conflict with another transaction.
    TxnConflict,
    /// Custom error that you can use. Composed of a code and an String error
    /// codes 10000 and above are free for your own purposes.
    CustomError((u64, String)),
}

impl Error {
    /// retrieve the code of the Error
    /// Might return None if the custom error is not above 10_000
    pub fn get_code(&self) -> Option<u64> {
        match self {
            Error::Timeout => Some(0),
            Error::NodeNotFound => Some(1),
            Error::NotSupported => Some(10),
            Error::TemporarilyUnavailable => Some(11),
            Error::MalformedRequest => Some(12),
            Error::Crash => Some(13),
            Error::Abort => Some(14),
            Error::KeyDoesNotExist => Some(20),
            Error::KeyAlreadyExist => Some(21),
            Error::PreconditionFailed => Some(22),
            Error::TxnConflict => Some(30),
            Error::CustomError((code, _)) => {
                if *code > 10_000 {
                    Some(*code)
                } else {
                    None
                }
            }
        }
    }
}
