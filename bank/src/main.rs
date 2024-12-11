#[derive(Debug)]
struct Account{
    id: u32,
    balance: i64,
    holder: String,
}

impl Account{
    fn new(id: u32, holder: String) -> Account{
        Account{
            id,
            holder,
            balance:0,
        }
    }
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank{
    fn new() -> Bank{
        Bank { accounts: vec![] }
    }
    fn print_bank(&self){
        println!("{:#?}", &self);
    }
}

fn print_account(account:Account){
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    bank.print_bank();

    let account = Account::new(
        1,
        String::from("me"),
    );

    // print_account(account);
}
