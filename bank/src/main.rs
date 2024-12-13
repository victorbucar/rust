#[derive(Debug)]
struct Account{
    id: u32,
    balance: i32,
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
    fn deposit(&mut self, amount: i32) -> i32{
        self.balance += amount;
        self.balance
    }
    fn withdraw(&mut self, amount: i32) -> i32{
        self.balance -= amount;
        self.balance
    }
    fn summary(&self) -> String{
        format!("Account {} is owned by {} and has balance of {}", self.id, self.holder, self.balance)
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
    fn add_account(&mut self, account: Account){
        self.accounts.push(account);
    }
    fn total_balance(&self) -> i32 {
        self.accounts
            .iter()
            .map(|account| account.balance)
            .sum()
    }
    fn summary(&self) -> Vec<String>{
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn print_account(account:Account) -> Account{
    println!("{:#?}", account);
    account
}

fn change_account(account: &mut Account){
    account.balance = 10;
}

fn print_balance(account: &Account) {
    println!("{}", account.balance);
}

fn main() {
    let mut bank = Bank::new();
    // bank.print_bank();
   
    let mut account = Account::new(
        1,
        String::from("me"),
    );
    // let list_of_accounts = vec![account];
    // println!("{:#?}", list_of_accounts);
    // println!("{:#?}", &mut account);
    let amount = 300;
    account.deposit(amount);
    account.summary();
    bank.add_account(account);
    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());
    

    // println!("{:#?}", &mut account)

    

 


}
