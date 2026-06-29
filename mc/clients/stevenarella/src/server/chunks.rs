pub(crate) fn should_log_first_chunk(already_logged: bool) -> bool {
    !already_logged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_chunk_log_is_emitted_only_before_first_chunk() {
        assert!(should_log_first_chunk(false));
        assert!(!should_log_first_chunk(true));
    }
}
