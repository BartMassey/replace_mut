pub trait ReplaceMutExt<T> {
    /// In-place replacement of a `target` pattern by a
    /// `subst` pattern in self. This is assumed to replace
    /// all non-overlapping occurrences without
    /// re-evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::replace_mut::ReplaceMutExt;
    /// let mut s = "aaaa".to_string();
    /// s.replace_mut("aa", "baa");
    /// assert_eq!(s, "baabaa");
    /// ```
    fn replace_mut(&mut self, target: T, subst: T);
}

impl ReplaceMutExt<&str> for String {
    fn replace_mut(&mut self, target: &str, subst: &str) {
        let ntarget = target.len();
        let nsubst = subst.len();
        let mut end = 0;
        while let Some(start) = self[end..].find(target) {
            let tstart = start + end;
            let tend = tstart + ntarget;
            self.replace_range(tstart..tend, subst);
            end = tstart + nsubst;
        }
    }
}

#[test]
fn test_replace_mut() {
    let mut s = "aaaa".to_string();
    s.replace_mut("aa", "baa");
    assert_eq!(s, "baabaa");
}
