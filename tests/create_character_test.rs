extern crate robust;

use robust::character;
use robust::item;

#[test]
fn create_character()
{
	//Create Character
	let mut char = character::Character { id: 1, name: "Test", ..Default::default() };
	assert_eq!(char.id, 1);
	assert_eq!(char.name, "Test");
	assert_eq!(char.atk, 0);
	assert_eq!(char.def, 1);
	assert_eq!(char.dex, 0);
	assert_eq!(char.mgc, 0);	
	assert_eq!(char.hp, 1);
	assert_eq!(char.mp, 0);	
	assert_eq!(char.atp, 1);
	assert_eq!(char.mst, 1);	
	assert_eq!(char.asp, 1);	
	assert_eq!(char.acc, 1);	
	assert_eq!(char.rgn, 1);	
	assert_eq!(char.dfp, 1);	
	assert_eq!(char.evp, 1);	
	assert_eq!(char.lck, 1);	
	assert_eq!(char.level(),1);
	
	//Level up attack
	char.atk += 1;
	assert_eq!(char.atk, 1);
	assert_eq!(char.level(), 2);
	
	//Get items
	let itm = item::Item { id: 1, name: "Potion", on_use_script: "Heal 5", inf: item::ItemInfo::Stackable { quantity: 1 } };
	let wpn = item::Item { id: 2, name: "Sword", on_use_script: "", inf: item::ItemInfo::Weapon { on_hit_script: "Fire 1", atk: 2, def: 0, dex: 0, mgc: 0 } };
	let arm = item::Item { id: 3, name: "Armor", on_use_script: "", inf: item::ItemInfo::Armor { on_def_script: "Ice 1", atk: 0, def: 2, dex: 0, mgc: 0 } };	
	assert!(char.inv_add(itm));	
	assert!(char.inv_add(wpn));	
	assert!(char.inv_add(arm));	
	assert_eq!(char.inv[0].unwrap().on_use_script, "Heal 5");	
	if let item::ItemInfo::Stackable { quantity, .. } = char.inv[0].unwrap().inf 
	{
		assert_eq!(quantity, 1);
	}
	else
	{
		assert!(false);
	}	
	if let item::ItemInfo::Weapon { on_hit_script, atk, def, dex, mgc } = char.inv[1].unwrap().inf 
	{
		assert_eq!(on_hit_script, "Fire 1");
		assert_eq!(atk, 2);
		assert_eq!(def, 0);
		assert_eq!(dex, 0);
		assert_eq!(mgc, 0);
	}
	else
	{
		assert!(false);
	}	
	if let item::ItemInfo::Armor { on_def_script, atk, def, dex, mgc } = char.inv[2].unwrap().inf 
	{
		assert_eq!(on_def_script, "Ice 1");
		assert_eq!(atk, 0);
		assert_eq!(def, 2);
		assert_eq!(dex, 0);
		assert_eq!(mgc, 0);	
	}
	else
	{
		assert!(false);
	}
	
	assert_eq!(char.inv_count(),3);
	
	//Take Potion
	assert!(char.inv_del_by_name("Potion"));
	assert!(!char.inv_del_by_idx(0));
	assert!(!char.inv_has_by_name("Potion"));
	assert!(!char.inv_has_by_idx(0));
	
	//Take Sword
	assert!(char.inv_del_by_idx(1));
	assert!(!char.inv_del_by_name("Sword"));
	assert!(!char.inv_has_by_name("Sword"));
	assert!(!char.inv_has_by_idx(1));
	
	//Verify Current Items
	assert!(char.inv_has_by_name("Armor"));
	assert_eq!(char.inv_count(),1);
	
	//Add 3 potions
	assert!(char.inv_add(itm));
	assert!(char.inv_add(itm));	
	assert!(char.inv_add(itm));	
	if let item::ItemInfo::Stackable { quantity, .. } = char.inv[0].unwrap().inf
	{
		assert_eq!(quantity, 3);
	}
}