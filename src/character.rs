pub use item;

pub struct Character<'a>
{
	pub id: u64,
	pub name: &'a str,
	pub atk: u64,
	pub def: u64,
	pub mgc: u64,
	pub itm: item::Item<'a>,
	pub wpn: item::Item<'a>,
	pub arm: item::Item<'a>,
}

impl<'a> Character<'a>
{
	pub fn get_item(self, i: item::Item<'a>, replace: bool)
	{
	}
}