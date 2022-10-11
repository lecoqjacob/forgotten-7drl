use gridbugs::chargrid::text::StyledString;

use crate::prelude::*;

pub fn weapon_name_text(weapon_name: WeaponType) -> StyledString {
    let t = |s: &str, c| StyledString {
        string: s.to_string(),
        style: Style::new().with_foreground(c).with_bold(true),
    };
    let name = weapon_name.to_string();
    let color = match weapon_name {
        WeaponType::BareHands => Rgba32::new_grey(255),
        WeaponType::CattleProd => color::SHOCK.saturating_scalar_mul_div(3, 2),
        WeaponType::Chainsaw => color::CHAINSAW.saturating_scalar_mul_div(3, 2),
        WeaponType::Railgun => color::PLASMA,
        WeaponType::LifeStealer => color::HEALTH,
        WeaponType::FiftyCal => color::GAUS.saturating_scalar_mul_div(3, 2),
    };

    t(name.as_str(), color)
}

pub fn enemy_text(enemy: NpcType) -> StyledString {
    let t = |s: &str, c| StyledString {
        string: s.to_string(),
        style: Style::new().with_foreground(c).with_bold(true),
    };
    match enemy {
        NpcType::MiniBot => t("MiniBot", color::MINIBOT),
        NpcType::SecBot => t("Secbot", color::SECBOT),
        NpcType::RoboCop => t("RoboCop", color::ROBOCOP.saturating_scalar_mul_div(3, 2)),
        NpcType::DoomBot => t("DoomBot", color::DOOMBOT.saturating_scalar_mul_div(3, 2)),
    }
}

pub fn render_weapon(title: &str, weapon: &Weapon, player: &Player, ctx: Ctx, fb: &mut FrameBuffer) {
    let plain = Style::new().with_foreground(Rgba32::new_grey(255)).with_bold(false);
    StyledString { string: title.to_string(), style: plain }.render(&(), ctx, fb);

    // Weapon name
    weapon_name_text(weapon.name).render(&(), ctx.add_y(1), fb);

    // Ammo
    let mut show_ammo = true;
    if let Some(ammo) = weapon.ammo.as_ref() {
        StyledString { string: format!("AMMO: {}/{}\n", ammo.current, ammo.max), style: plain }.render(
            &(),
            ctx.add_y(2),
            fb,
        );
    } else {
        show_ammo = false;
        // StyledString { string: "AMMO: -".to_string(), style: plain }.render(&(), ctx.add_y(2), fb);
    }

    StyledString { string: format!("PEN(♦): {}\n", weapon.pen), style: plain }.render(
        &(),
        ctx.add_y(if show_ammo { 3 } else { 2 }),
        fb,
    );

    let extra = if player.traits.double_damage { "x2" } else { "" };
    StyledString { string: format!("DMG(♥): {}{}\n", weapon.dmg, extra), style: plain }.render(
        &(),
        ctx.add_y(if show_ammo { 4 } else { 3 }),
        fb,
    );

    for &ability in weapon.abilities.iter() {
        weapon_ability_text(ability).render(&(), ctx.add_y(if show_ammo { 5 } else { 4 }), fb);
    }
}

pub fn weapon_ability_text(weapon_ability: WeaponAbility) -> StyledString {
    match weapon_ability {
        WeaponAbility::KnockBack => StyledString {
            string: "Knocks Back".to_string(),
            style: Style::new().with_foreground(Rgba32::new_rgb(0xFF, 0x44, 0x00)),
        },
        WeaponAbility::Shock => StyledString {
            string: "Chance to stun".to_string(),
            style: Style::new().with_foreground(color::SHOCK),
        },
        WeaponAbility::LifeSteal => StyledString {
            string: "Restores Health".to_string(),
            style: Style::new().with_foreground(color::HEALTH),
        },
    }
}

pub fn render_empty_weapon_slot(title: &str, ctx: Ctx, fb: &mut FrameBuffer) {
    let style = Style::new().with_foreground(Rgba32::new_grey(255)).with_bold(false);
    StyledString { string: title.to_string(), style }.render(&(), ctx, fb);
    StyledString { string: "(empty)".to_string(), style }.render(&(), ctx.add_y(1), fb);
}
