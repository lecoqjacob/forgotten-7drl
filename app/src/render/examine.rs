use super::*;
use gridbugs::chargrid::text::StyledString;

#[derive(Clone, Copy, Debug)]
enum MessageVerb {
    See,
    Remember,
} 

pub fn examine(game: &StateScope, coord: Coord) -> Option<StyledString> {
    let mut entity_under_cursor = None;
    let visibility = &game.visibility_grid().get_visibility(coord);
    let data = &game.visibility_grid().get_data(coord);
    if let Some(data) = data{
        let verb = match visibility {
            CellVisibility::Never => None,
            CellVisibility::Previous(..) => Some(MessageVerb::Remember),
            CellVisibility::Current { .. } => Some(MessageVerb::See),
        };
        
        if let Some(verb) = verb {
            let mut depth = 0;
            data.tiles.option_for_each_enumerate(|tile,layer| {
                let layer_depth = layer_depth(layer);
                if layer_depth >= depth {
                    depth = layer_depth;
                    entity_under_cursor = Some((*tile, verb));
                }
            });
        }
        
    }

    entity_under_cursor.and_then(|(tile, verb)| {
        tile_str(tile).map(|label| match label {
            TileLabel::Literal(literal) => StyledString::plain_text(literal),
            TileLabel::Name(name) => {
                let verb_str = match verb {
                    MessageVerb::See => "see",
                    MessageVerb::Remember => "remember seeing",
                };
                StyledString::plain_text(format!("You {} {} here.", verb_str, name))
            }
        })
    })
}

enum TileLabel {
    Literal(String),
    Name(String),
}

fn tile_str(tile: Tile) -> Option<TileLabel> {
    let desc = default_tile_str(tile).unwrap_or_default().to_string();
   matches!(
        tile,
        Tile::Player
            | Tile::Npc(..)
            | Tile::DoorClosed
            | Tile::DoorOpen
            | Tile::Floor
            | Tile::CaveFloor
            | Tile::Wall
            | Tile::CaveWall
            | Tile::Grass
            | Tile::GrassCrushed
            | Tile::Water
    )
    .then(|| TileLabel::Name(desc.clone()))
    .or_else(|| matches!(
        tile,
        Tile::Weapon(..)
        )
        .then(|| TileLabel::Literal(desc.clone())))
}

fn default_tile_str(tile: Tile) -> Option<&'static str> {
    Some(match tile {
        Tile::Player => "yourself",
        Tile::Floor | Tile::CaveFloor => "the floor",
        Tile::DoorClosed | Tile::DoorOpen => "a door",
        Tile::Wall | Tile::CaveWall => "a wall",
        Tile::Grass => "dense patch of grass",
        Tile::GrassCrushed => "crushed grass",
        Tile::Water => "some lovely cave water",
        
        Tile::Weapon(wpn) => match wpn{
            WeaponType::BareHands => return None,
            WeaponType::CattleProd => "A cattle prod - can stun low-level enemies",
            WeaponType::Chainsaw => "A chainsaw - melee weapon with high DMG and limited uses.",
            WeaponType::Railgun => "A railgun - it can shoot through almost anything!",
            WeaponType::LifeStealer => "A life stealer - converts the recently deceased into health like some kind of creepy vampire. And you thought the zombies were gross!",
            WeaponType::FiftyCal => "A 50 Cal Sniper - 1 shot is all you need",
            
        }

        Tile::Npc(npc_type) => match npc_type{
            NpcType::MiniBot => "a mini-bot",
            NpcType::SecBot => "a sec-bot. an upgraded bot from mini-bot.",
            NpcType::RoboCop => "a robo-cop. a security bot for the <blank>",
            NpcType::DoomBot => "a doom-bot. its only purpose is to kill"
        }       

        Tile::Laser | Tile::Bullet => return None,
    })
}

