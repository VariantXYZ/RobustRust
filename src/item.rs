pub struct Item<'a>
{
	pub id: u64,
	pub name: &'a str,
	pub cat: ItemCategory,
}

pub enum ItemCategory
{
	Consumable { on_use_script: String },
	Weapon { on_hit_script: String, atk: u64, def: u64, mgc: u64 },
	Armor { on_hit_script: String, atk: u64, def: u64, mgc: u64 },
}
