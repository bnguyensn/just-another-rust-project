struct Unit {
    // Core
    id: u32,
    name: String,
    title: String,
    level: u32,

    // Combat
    hp_max: u32,
    hp_cur: u32,
    mana_max: u32,
    mana_cur: u32,
    dmg_base: u32,
}

impl Unit {
    fn hit_target(&self, target: &mut Unit) {
        target.take_damage(self.dmg_base);
    }

    fn take_damage(&mut self, dmg: u32) {
        self.hp_cur -= dmg;
    }
}
