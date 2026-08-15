/// Progress information emitted while downloading a Modrinth version file.
///
/// `downloaded` is the number of bytes downloaded and written so far.
///
/// `total` is the content length reported by the server, if available. Some
/// file hosts may not provide a `Content-Length` header, in which case this is
/// `None`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DownloadProgress {
    /// Number of bytes downloaded and written so far.
    pub downloaded: u64,

    /// Total expected bytes, if known.
    pub total: Option<u64>,
}

impl DownloadProgress {
    /// Returns download progress as a fraction from `0.0` to `1.0`, if the
    /// total size is known.
    ///
    /// Returns `None` if the server did not provide a content length.
    ///
    /// # Examples
    /// ```
    /// # use modrinth_wrapped::DownloadProgress;
    /// let progress = DownloadProgress {
    ///     downloaded: 25,
    ///     total: Some(100),
    /// };
    ///
    /// assert_eq!(progress.fraction(), Some(0.25));
    /// ```
    pub fn fraction(self) -> Option<f64> {
        let total = self.total?;

        if total == 0 {
            return Some(1.0);
        }

        Some((self.downloaded as f64 / total as f64).clamp(0.0, 1.0))
    }
}
