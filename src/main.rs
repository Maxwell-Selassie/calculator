struct Contact {
    name : String,
    phone : String,
    age : i32,
}

impl Contact {
    fn is_adult(&self) -> bool {
        self.age > 18
    }
}
fn main() {
    let contact_card = Contact {
        name : String::from("Maxwell Selassie Hiamatsu"),
        phone : String::from("+1 23 45 654 423"),
        age : 34,
    };

    println!("Contact Name : {}",contact_card.name);
    println!("Phone number : {}", contact_card.phone);
    println!("Age : {}",contact_card.age);
    
    
    if contact_card.is_adult() {
        println!("You are above 18 years old. Access granted");
    }
    else {
        println!("You are less than 18 years old. Access denied");
    }
}