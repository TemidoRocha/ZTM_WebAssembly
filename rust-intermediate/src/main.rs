struct BankAccount{
  balance: i32, verified: bool
}

fn print_balance(account: &BankAccount){
  println!("{:?}", account.balance);
}
fn print_verified(account: &BankAccount){
  println!("{:?}", account.verified);
}

fn main() {
    let my_account = BankAccount{
      balance: 20,
      verified: true
    };
    
    /**
      like this, an error will be displayed because the variable my_account will be lost as soon as the main fn is finished
      we need to allow the functions to borrow print_balance and print_verified the values for reading purposes from the
      variable my_account owned by fn main
      
      Not possible like this:
       print_balance(my_account);
       print_verified(my_account);
      
      The borrow feature is implemented by adding the &
     */
    print_balance(&my_account);
    print_verified(&my_account);
    
}
