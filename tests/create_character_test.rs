extern crate robust;

use robust::character;

#[test]
fn create_character()
{
	let char = character::Character { id: 1, name: "Test", atk: 1, def: 1, mgc: 1 };
	assert_eq!(char.id, 1);
	assert_eq!(char.name, "Test");
	assert_eq!(char.atk, 1);
	assert_eq!(char.def, 1);
	assert_eq!(char.mgc, 1);
}