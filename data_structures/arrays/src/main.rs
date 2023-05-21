// Problem Statement:
// Develop a program to manage the inventory of a small store. The store has space to store exactly 100 items. Each item in the store has a unique identifier (integer) and a quantity.

//     Initialize the Inventory: Initialize an array of size 100 to represent the store's inventory. Each index in the array will represent an item ID, and the value at each index will represent the quantity of that item. For simplicity, let's assume that all item IDs are integers from 0 to 99. Initialize the array such that each item's quantity is set to 0.

//     Add Items to the Inventory: Write a function add_item that takes in the item's ID and the quantity to be added. The function should add the quantity to the item's current quantity in the inventory.

//     Remove Items from the Inventory: Write a function remove_item that takes in the item's ID and the quantity to be removed. If the store has enough of the item, the function should subtract the quantity from the item's current quantity in the inventory and return true. If the store does not have enough of the item, the function should not change the inventory and should return false.

//     Check Item Quantity: Write a function check_item_quantity that takes an item ID and returns the current quantity of that item in the inventory.

//     Total Inventory: Write a function total_inventory that returns the total quantity of all items in the inventory.


fn main() {
    let mut inventory: [u32; 100] = [0;100];
    inventory[0] = 45;
    inventory[4] = 9;
    total_inventory(&inventory);
    add_item(&mut inventory, 4, 4);
    let mut quantity = check_item_quantity(&inventory, 4); 
    println!("{}", quantity);
    remove_item(&mut inventory, 4, 14);
    remove_item(&mut inventory, 4, 12);
    quantity = check_item_quantity(&inventory, 4); 
    println!("{}", quantity);
    total_inventory(&inventory);
}

fn add_item(inventory: &mut [u32; 100], item_id: usize, quantity: u32) {
    inventory[item_id] += quantity;
}

fn remove_item(inventory: &mut [u32; 100], item_id: usize, quantity: u32) {
    if inventory[item_id] >= quantity { 
        inventory[item_id] += quantity;
    } else {
        println!("Not enough quantity.");
    };
}   

fn check_item_quantity(inventory: &[u32; 100], item_id: usize) -> u32 {
    return inventory[item_id];
}

fn total_inventory(inventory: &[u32; 100]){
    for (index, item) in inventory.iter().enumerate() {
        println!("Item {}: {}", index+1, item);
    }
}


