use std::error::Error;

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

// The question mark operator (point d'intérogation)
// will automatically be able to convert different
// err types into a chat err by using the
// standard libraries from trait to the type we implemented (ChatError)
//
// We use send and sync bounds to ensure that if a task that is spawned
// on another thread fails, it will be able to safely report the failure
// up to main.
// And if it doesn't, you'll be able to see it a little bit more as we
// continue our program.
