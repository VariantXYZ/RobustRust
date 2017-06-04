pub use item;

const INVENTORY_SIZE: usize = 50;

pub struct Character<'a>
{
	pub id: u64,
	pub name: &'a str,
	pub class: u8,
	
	pub atk: u16,
	pub def: u16,
	pub dex: u16,
	pub mgc: u16,
	
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

	//Inventory Related
	pub fn inv_max(&self) -> usize { INVENTORY_SIZE }
	pub fn inv_count(&self) -> usize { self.inv.iter().fold(0, |cnt, &x| if x.is_some() { cnt + 1 } else { cnt } ) }
	pub fn inv_free_space(&self) -> usize { INVENTORY_SIZE - self.inv_count() }
	pub fn inv_has_by_idx(&self, idx: usize) -> bool { return self.inv[idx].is_some() }
	pub fn inv_find_by_id(&self, id: u32) -> Option<usize> { return self.inv.iter().position(|r| r.is_some() && r.unwrap().id == id ) }	
	pub fn inv_find_by_name<'b>(&self, name: &'b str) -> Option<usize> { return self.inv.iter().position(|r| r.is_some() && r.unwrap().name == name ) } //Should really just use inv_find_by_id
	pub fn inv_has_by_name<'b>(&self, name: &'b str) -> bool { self.inv_find_by_name(name).is_some() }	
	
	fn inv_add_single(&mut self, i: item::Item<'a>) -> bool
	{
		let idx = self.inv.iter().position(|r| { r.is_none() });
		match idx
		{
			None => false,
			_ => { self.inv[idx.unwrap()] = Some(i); return true; }
		}
	}
	
	fn inv_add_stack(&mut self, i: item::Item<'a>, qty: u8) -> bool
	{
		let idx = self.inv_find_by_id(i.id);
		match idx
		{
			None => self.inv_add_single(i),
			_ => 
				{
					if let item::ItemInfo::Stackable { ref mut quantity, .. } = self.inv[idx.unwrap()].as_mut().unwrap().inf
					{
						*quantity += qty;
						return true;
					}
					else
					{
						return false;
					}
				}
		}
	}
	
	pub fn inv_add(&mut self, i: item::Item<'a>) -> bool
	{
		match i.inf
		{
			item::ItemInfo::Stackable { quantity, .. } => self.inv_add_stack(i, quantity),
			_ => self.inv_add_single(i)
		}
	}
	
	pub fn inv_remove_by_id(&mut self, id: u32) -> bool
	{
		true
	}
	
	pub fn inv_remove_by_name<'b>(&mut self, name: &'b str) -> bool
	{
		true
	}
	
	pub fn inv_del_by_idx(&mut self, idx: usize) -> bool
	{
		let ret_val = idx < INVENTORY_SIZE && self.inv[idx].is_some();
		self.inv[idx] = None;
		return ret_val;
	}

	pub fn inv_del_by_name<'b>(&mut self, name: &'b str) -> bool
	{
		let idx = self.inv.iter().position(|r| r.is_some() && r.unwrap().name == name );
		match idx
		{
			None => false,
			_ => self.inv_del_by_idx(idx.unwrap())
		}	
	}
	
}