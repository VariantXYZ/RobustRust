#[derive(Copy, Clone)]
pub struct Item<'a>
{
	pub name: &'a str,
	pub inf: ItemInfo<'a>,
}

#[derive(Copy, Clone)]
pub enum ItemInfo<'a>
{
	Consumable { on_use_script: &'a str },
	Weapon { on_hit_script: &'a str, atk: u16, def: u16, dex: u16, mgc: u16 },
	Armor { on_hit_script: &'a str, atk: u16, def: u16, dex: u16, mgc: u16 },
}