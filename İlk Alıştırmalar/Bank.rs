#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
    fn summary(&self) ->String{
        format!("{} Kullanıcı Bakiyesi Özeti {} TL",self.holder,self.balance)
    }
    fn deposit(&mut self, amount:i32) ->i32{
        self.balance += amount;
        self.balance
    }
    fn withdraw(&mut self,amount:i32) ->i32{
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: vec![],
        }
    }
    fn total_balance(&self) -> i32 {
    self.accounts.iter().map(|account| account.balance).sum()
}

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
}
}

fn add_account_print(account: Account, bank: &mut Bank) {
    bank.accounts.push(account);
    println!("{:#?}", bank);
}

fn account_printer(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    
    let mut account = Account::new(1, String::from("Mustafa Şenlik"));
    let mut bank = Bank::new();
    
    account.deposit(500);
    account.withdraw(250);
    println!("{:#?}", account.summary());

    // Hesabı bankaya ekliyoruz
    add_account_print(account, &mut bank);
    
    // Bankanın özetini ve toplam bakiyesini yazdırıyoruz
    println!("{:#?}", bank.summary());
    println!("Toplam Bakiye: {} TL", bank.total_balance());


}
