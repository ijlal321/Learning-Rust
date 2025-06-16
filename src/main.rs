#![allow(unused, dead_code)]

use std::collections::{HashMap, hash_map};
use std::env;
use std::process::Termination;

#[derive(Eq, PartialEq, Debug, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    ordered_product: Product,
    quantity: i32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: i32, shipped: bool) -> Self {
        CustomerOrder {
            ordered_product: product,
            quantity: quantity,
            shipped: shipped,
        }
    }
}

#[derive(Debug)]

struct Customer {
    id: i32,
    orders: Vec<CustomerOrder>,
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let blender_orders = orders
        .iter()
        .filter(|order| order.ordered_product == Product::Blender)
        .collect::<Vec<&CustomerOrder>>();

    println!("{:#?}", blender_orders);

    let mico_orders: i32 = orders
        .iter()
        .filter_map(|order| {
            if (order.ordered_product == Product::Microwave) {
                Some(order.quantity)
            } else {
                None
            }
        })
        .sum();

    println!("Microwaves sum: {}", mico_orders);

    let micro_orders = orders
        .iter()
        .filter(|order| order.ordered_product == Product::Microwave)
        .map(|order| order.quantity)
        .sum::<i32>();

    println!("Microwaves sum: {}", mico_orders);

    let args = env::args();

    let user_input = args.into_iter().nth(1);

    match user_input {
        None => print_orders(&orders, 2),

        Some(user_input) => match user_input.parse::<i32>() {
            Ok(res) => print_orders(&orders, res),

            Err(err) => print_orders(&orders, 2),
        },
    }

    let mut need_inventory: HashMap<&Product, i32> = HashMap::new();
    for order in &orders {
        if order.shipped == true {
            continue;
        };
        *need_inventory.entry(&order.ordered_product).or_insert(0) += order.quantity;
    }

    println!("{:?}", need_inventory);

    let first_unshipped_order = orders.iter_mut().find(|order| order.shipped == false);

    match first_unshipped_order {
        Some(order) => {
            order.shipped = true;
            println!("Customer Order is {:?}", order);
        }
        None => println!("No unshipped order found"),
    }

        // requires creating alot of iter, so i am not going with this one, bad approach
    // let mut customers: Vec<Customer> = vec![];
    
    // for (id, order) in customer_ids_by_order.into_iter().zip(orders.into_iter()){

    //     let existing_customer_option = customers.iter_mut().find(|customer| customer.id == id);

    //     match existing_customer_option {
    //         Some(customer) => customer.orders.push(order),
    //         None => {
    //             let new_customer = Customer {id, orders : vec![order]};
    //             customers.push(new_customer);
    //         }
    //     };
    // }

    // customers.sort_by_key(|customer| customer.id);

    // println!("{:#?}", customers);


                // O(N) approach
    let mut customers: Vec<Customer> = vec![];
    
    let mut customers_map: HashMap<i32, Vec<CustomerOrder>> = HashMap::new();


        // make a hashmap with key as id, and value as list of orders for its customer
    for (id, order) in customer_ids_by_order.into_iter().zip(orders.into_iter()){
            // get customer or create if not present
        let mut customer = customers_map.entry(id).or_insert(vec![]);
            // insert order in value of map
        customer.push(order);
    };

        // populate customers vector with hashmap key (id) and values (orders)
    for (id, order) in customers_map{
        customers.push(
            Customer { id, orders: order }
        );
    }
    customers.sort_by_key(|customer| customer.id);
    println!("{:#?}", customers);

}

fn print_orders(orders: &Vec<CustomerOrder>, quantity: i32) {
    for order in orders {
        if order.quantity >= quantity {
            println!("{:#?}", order);
        }
    }
}
