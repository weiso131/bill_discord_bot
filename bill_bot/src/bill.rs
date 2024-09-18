use std::collections::HashMap;
use std::fs;
use serde_json;


const DATA_PATH: &str = "data.json";


fn serialize(bill_book: HashMap<String, HashMap<String, i32>>) -> std::io::Result<()> {
    let serialized = serde_json::to_string(&bill_book).unwrap();
    println!("{}", serialized);
    fs::write(&DATA_PATH, serialized)?;
    Ok(())
}

fn deserialize(bill_book: &mut HashMap<String, HashMap<String, i32>>) -> std::io::Result<()> {
    let serialized = fs::read_to_string(&DATA_PATH)?;
    *bill_book = serde_json::from_str(&serialized).unwrap();
    Ok(())
}




pub mod edit {
    use std::collections::HashMap;
    fn bill_book_edit(user1: &str, user2: &str, amount: i32, 
        bill_book: &mut HashMap<String, HashMap<String, i32>>) {
        /*
            edit bill book that been deserilized    
         */
        let user1_bill = bill_book.entry(user1.to_string()).or_insert(HashMap::new());
        let user1_debt_user2 = user1_bill.entry(user2.to_string()).or_insert(0);
        *user1_debt_user2 += amount;
    }

    fn bill_book_file_edit(user1: &str, user2: &str, amount: i32) {
        /*
            amount > 0 代表user1 欠 user2 錢

            deserilized the bill book
            edit it 
            and then serilized it
         */
        let mut bill_book: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let _ = super::deserialize(&mut bill_book);
        bill_book_edit(user1, user2, amount, &mut bill_book);
        bill_book_edit(user2, user1, amount * -1, &mut bill_book);
        let _ = super::serialize(bill_book);

        
    }
    
    pub fn debt(user1: &str, user2: &str, amount: i32) -> String {
        bill_book_file_edit(user1, user2, amount);
        
        format!("{user1}欠{user2} ${amount}")
    }
    pub fn pay_back(user1: &str, user2: &str, amount: i32) -> String{
        bill_book_file_edit(user1, user2, amount * -1);
        
        format!("{user1}還{user2} ${amount}")

    }
}

pub mod query {
    use std::collections::HashMap;
    pub fn show_all() -> String {
        let mut bill_book: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let _ = super::deserialize(&mut bill_book);
        let mut bill_show: String = String::new();
        for (user1, user1_bill) in &bill_book {
            for (user2, amount) in user1_bill {
                if *amount > 0 {
                    bill_show.push_str(format!("@{user1} 欠 @{user2} $ {amount}\n").as_str());
                }
            }
        }
        bill_show
    }
}
