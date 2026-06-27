use tokio_util::sync::CancellationToken;

/// Shared cancellation token for graceful shutdown of all background tasks.
///
/// One root token is stored in [`RuntimeContext`](super::runtime_context::RuntimeContext).
/// Each subsystem gets a `child()` token; cancelling the root cancels every
/// child. Tasks poll `token.cancelled()` in their loops.
pub struct ShutdownToken {
    token: CancellationToken,
}

impl ShutdownToken {
    pub fn new() -> Self {
        Self {
            token: CancellationToken::new(),
        }
    }

    /// Create a child token — cancelling the root also cancels this.
    pub fn child(&self) -> CancellationToken {
        self.token.child_token()
    }

    /// Cancel the root token. All children are cancelled too.
    pub fn cancel(&self) {
        self.token.cancel();
    }

    /// Whether this token has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.token.is_cancelled()
    }
}

impl Default for ShutdownToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_cancellation_propagates() {
        let root = ShutdownToken::new();
        let child = root.child();
        assert!(!child.is_cancelled());
        root.cancel();
        assert!(child.is_cancelled());
        assert!(root.is_cancelled());
    }
}
