pub use item;

const INVENTORY_SIZE: usize = 30;

pub struct Character<'a>
{
	pub id: u64,
	pub name: &'a str,
	pub class: u8,
	
	pub atk: u16,
	pub def: u16,
	pub dex: u16,
	pub mgc: u16,
	
	//Base Values, growth determined by class
	pub hp: u16,
	pub mp: u16,
	pub atp: u16,
	pub mst: u16,
	pub asp: u16,
	pub acc: u16,
	pub rgn: u16,
	pub dfp: u16,
	pub evp: u16,
	pub lck: u16,
	
	pub inv: [Option<item::Item<'a>>; INVENTORY_SIZE],
}

impl<'a> Default for Character<'a>
{
	fn default() -> Character<'a>
	{
		Character
		{
			id: 0,
			name: "",
			class: 0,
			
			atk: 0,
			def: 1,
			dex: 0,
			mgc: 0,
			
			hp: 1,
			mp: 0,
			atp: 1,
			mst: 1,
			asp: 1,
			acc: 1,
			rgn: 1,
			dfp: 1,
			evp: 1,
			lck: 1,
			
			inv: [None; INVENTORY_SIZE],			
		}
	}
}

impl<'a> Character<'a>
{
	pub fn level(&self) -> u16 { self.atk + self.def + self.dex + self.mgc }

	pub fn add_item(&mut self, i: item::Item<'a>) -> bool
	{
		let idx = self.inv.iter().position(|r| { r.is_none() });
		
		if idx == None
		{
			return false;
		}
		
		self.inv[idx.unwrap()] = Some(i);
		return true;
	}
	
	pub fn del_item_by_id(&mut self, id: usize) -> bool
	{
		if self.inv[id].is_none()
		{
			return false;
		}
		else
		{
			self.inv[id] = None;
			return true;
		}	
	}
	
	pub fn del_item_by_name(&mut self, id: &'a str) -> bool
	{
		let idx = self.inv.iter().position(|r| { r.unwrap().name == id });
		
		if idx == None
		{
			return false;
		}
		self.inv[idx.unwrap()] = None;
		return true;
	}
}