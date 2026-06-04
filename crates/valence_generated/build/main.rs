mod attributes;
mod block;
#[path = "chunk/view.rs"]
mod chunk_view;
mod item;
#[path = "packet/id.rs"]
mod packet_id;
mod sound;
#[path = "status/effects.rs"]
mod status_effects;

pub fn main() -> anyhow::Result<()> {
    valence_build_utils::write_generated_file(attributes::build()?, "attributes.rs")?;
    valence_build_utils::write_generated_file(block::build()?, "block.rs")?;
    valence_build_utils::write_generated_file(item::build()?, "item.rs")?;
    valence_build_utils::write_generated_file(sound::build()?, "sound.rs")?;
    valence_build_utils::write_generated_file(packet_id::build()?, "packet_id.rs")?;
    valence_build_utils::write_generated_file(chunk_view::build(), "chunk_view.rs")?;
    valence_build_utils::write_generated_file(status_effects::build()?, "status_effects.rs")?;

    Ok(())
}
