// Generated block bindings intentionally suppress broad Clippy noise from the
// included data table.
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/block.rs"));

impl std::fmt::Debug for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt_block_state(*self, f)
    }
}

impl std::fmt::Display for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt_block_state(*self, f)
    }
}

fn fmt_block_state(bs: BlockState, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    debug_assert!(
        BlockState::from_raw(bs.to_raw()).is_some(),
        "block state is valid"
    );
    let kind = bs.to_kind();
    debug_assert!(!kind.to_str().is_empty(), "block kind names are non-empty");

    write!(f, "{}", kind.to_str())?;

    let props = kind.props();

    if !props.is_empty() {
        let mut list = f.debug_list();
        for &p in kind.props() {
            struct KeyVal<'a>(&'a str, &'a str);

            impl<'a> std::fmt::Debug for KeyVal<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}={}", self.0, self.1)
                }
            }

            let Some(prop_value) = bs.get(p) else {
                debug_assert!(false, "listed block property has a value");
                continue;
            };
            list.entry(&KeyVal(p.to_str(), prop_value.to_str()));
        }
        list.finish()
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_consistency() {
        for kind in BlockKind::ALL {
            let block = kind.to_state();

            for &prop in kind.props() {
                let value = block.get(prop);
                assert!(value.is_some(), "listed property has a value");
                let Some(value) = value else {
                    continue;
                };
                let new_block = block.set(prop, value);
                assert_eq!(new_block, block);
            }
        }
    }

    #[test]
    fn blockstate_to_wall() {
        assert_eq!(BlockState::STONE.wall_block_id(), None);
        assert_eq!(
            BlockState::OAK_SIGN.wall_block_id(),
            Some(BlockState::OAK_WALL_SIGN)
        );
        assert_eq!(
            BlockState::GREEN_BANNER.wall_block_id(),
            Some(BlockState::GREEN_WALL_BANNER)
        );
        assert_ne!(
            BlockState::GREEN_BANNER.wall_block_id(),
            Some(BlockState::GREEN_BANNER)
        );
    }
}
