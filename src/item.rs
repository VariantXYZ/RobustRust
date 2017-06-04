#[derive(Copy, Clone)]
pub struct Item<'a>
{
	pub id: u32, //Is there a way to force this to be unique?
	pub name: &'a str,
	pub on_use_script: &'a str,
	pub inf: ItemInfo<'a>,
}

#[derive(Copy, Clone)]
pub enum ItemInfo<'a>
{
	Stackable { quantity: u8 },

	//Equipment Types
	Weapon { on_hit_script: &'a str, atk: u16, def: u16, dex: u16, mgc: u16 },
	Armor { on_def_script: &'a str, atk: u16, def: u16, dex: u16, mgc: u16 },
}

