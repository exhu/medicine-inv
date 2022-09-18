// TODO check id to be > than the previous
// TODO skip header
use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
pub struct Inventory {
    pub id: i32,
    pub expiry: NaiveDate,
    pub name: String,
    pub amount: i32,
    pub record_date: NaiveDate,
    pub origin: String,
    pub notes: String,
}


#[cfg(test)]
mod tests {
    //use super::*;

    use crate::Inventory;

    #[test]
    fn read_inventory() {
        const ROWS:&str = "id,expiry,name,amount,record_date,origin,notes
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
}
