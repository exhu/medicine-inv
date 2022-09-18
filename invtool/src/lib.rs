// TODO check id to be > than the previous
use serde::Deserialize;
use chrono::NaiveDate;

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

#[derive(Debug, Deserialize)]
pub struct InventoryMove {
    // TODO

}

pub fn available_at(location_name: &str, items: &Vec<Inventory>, 
    moves: &Vec<InventoryMove>) {

}

pub fn is_expired(item: &Inventory, usage: &Vec<InventoryUsage>) {

}

#[cfg(test)]
mod tests {
    use super::*;

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
