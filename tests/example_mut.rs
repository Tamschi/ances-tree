use ances_tree::{Node, NodeHandle};
use tiptoe::Arc;

#[test]
fn test() {
	let mut handle = Node::new(None, 1);
	{
		let mut exclusive = Arc::get_mut(&mut handle).unwrap();
		let _: &mut i32 = exclusive.value_mut();
		let _: &mut Option<NodeHandle<i32>> = exclusive.parent_mut();
	}

	let second_handle = handle.clone_handle();
	assert!(Arc::get_mut(&mut handle).is_none());
	assert_eq!(second_handle.value, 1);
}
