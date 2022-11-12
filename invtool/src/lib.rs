// TODO check id to be > than the previous
use serde::Deserialize;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Inventory {
    pub id: i32,
    pub expiry_date: NaiveDate,
    pub name: String,
    pub amount: i32,
    pub record_date: NaiveDate,
    pub origin: String,
    pub notes: String,
}

#[derive(Debug, Deserialize)]
pub struct InventoryUsage {
    pub inventory_id: i32,
    pub open_date: NaiveDate,
    pub new_expiry_date: NaiveDate,
    pub record_date: NaiveDate,
    pub notes: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryMove {
    pub inventory_id: i32,
    pub record_date: NaiveDate,
    pub amount: i32,
    pub source: String,
    pub destination: String,
    pub notes: String,
}

pub fn available_at(location_name: &str, items: &Vec<Inventory>, 
    moves: &Vec<InventoryMove>) {

    // TODO
    // sort records by date from oldest to newest
    // track amount
}


fn sorted_moves(moves: &Vec<InventoryMove>) -> Vec<InventoryMove> {
    let mut sorted_moves: Vec<InventoryMove> = Vec::clone(&moves);
    sorted_moves.sort_by( |a,b| a.record_date.cmp(&b.record_date));
    sorted_moves
}

// one location can have several inventory items
struct InventoryAtLocation {
    pub id_to_amount: HashMap<i32, i32>,
}

impl InventoryAtLocation {
    fn new() -> InventoryAtLocation {
        InventoryAtLocation { id_to_amount: HashMap::new() }
    }

    fn add(&mut self, inventory_id: i32, amount: i32) {
        if let Some(k) = self.id_to_amount.get_mut(&inventory_id) {
            *k += amount;
        } else {
            self.id_to_amount.insert(inventory_id, amount);
        }
    }
}

#[derive(Debug)]
pub struct Amounts {
    pub inventory_id: i32,
    pub amount: i32,
    pub location: String,
}

// expects sorted moves
fn final_amounts(items: &Vec<Inventory>, moves: &Vec<InventoryMove>) -> Vec<Amounts> {
    // location maps to map of inventory_id to amount
    let mut amounts = HashMap::<String, InventoryAtLocation>::new();

    // init from origins
    for item in items {
        if let Some(k) = amounts.get_mut(&item.origin) {
            k.add(item.id, item.amount);
        } else {
            let mut atloc = InventoryAtLocation::new();
            atloc.add(item.id, item.amount);
            amounts.insert(item.origin.clone(), atloc);
        }
    }

    // TODO process moves
    // TODO may be origin is unnecessary - calc all like balances -
    // location = account, inventory = currency

    Vec::new()
}



pub fn is_expired(item: &Inventory, today: &NaiveDate, usage: &Vec<InventoryUsage>) {

}

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use super::*;

    #[test]
    fn test_sorted_moves() {
        let moves = vec![InventoryMove { inventory_id: 1,
            record_date: NaiveDate::from_ymd(2022, 3, 1),
            amount: 3, source: "aa".into(), destination: "bb".into(),
            notes: "".into() },
            InventoryMove { inventory_id: 2,
                record_date: NaiveDate::from_ymd(2022, 2, 20),
                amount: 3, source: "bb".into(), destination: "aa".into(),
                notes: "".into() } ];

        let sorted_moves = sorted_moves(&moves);

        assert_eq!(sorted_moves[0].record_date.month(), 2);
        assert_eq!(sorted_moves[1].record_date.month(), 3);

        println!("moves = {:?}, sorted = {:?}", &moves, &sorted_moves);
    }

    #[test]
    fn read_inventory() {
        const ROWS:&str = "id,expiry_date,name,amount,record_date,origin,notes
1,2025-01-01,eye drops A,1,2022-09-18,City A,
2,2025-01-01,eye drops B,1,2022-09-18,City A,";

        let mut rdr = csv::Reader::from_reader(ROWS.as_bytes());
        let mut prev_id = 0;
        for result in rdr.deserialize() {
            let record: Inventory = result.unwrap();
            assert!(prev_id < record.id);
            assert_eq!(record.origin, "City A");
            prev_id = record.id;
            println!("{:?}", &record);
        }
    }

    #[test]
    fn read_usage() {
        const ROWS:&str = "inventory_id,open_date,new_expiry_date,record_date,notes
1,2022-09-18,2022-09-25,2022-09-19,not sure of date";
        let mut rdr = csv::Reader::from_reader(ROWS.as_bytes());
        for result in rdr.deserialize() {
            let record: InventoryUsage = result.unwrap();
            assert_eq!(record.open_date, NaiveDate::from_ymd(2022, 09, 18));
            println!("{:?}", &record);
        }
    }
}
