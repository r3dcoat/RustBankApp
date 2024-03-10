// recreating a banking application that I created in Python
use std::io;
// use std::thread::AccessError;
// use log::log;

struct Account {
    email: String,
    password: String,
    balance: i32,
}

// function to create a new account, this does not yet hash the password
// todo!("Add hashing functionality to the account creation. Comment the shit out of this too.");
fn create_account(accounts: &mut Vec<Account>){
    let mut email = String::new(); // variable for the email account.
    let mut password = String::new(); // variable for the password
    let mut password_verify = String::new(); // variable to compare to the password

    println!("Thank you for choosing to create an account with us. First we need to ask a few questions.");
    println!("Please enter your email address: ");
    io::stdin() // input that will be sent to the email variable
        .read_line(&mut email)
        .expect("Failed to read line");
    let email = email.trim().to_owned(); // trims and sets the owner of the input string.

    if accounts.iter().any(|account| account.email == email) { // if statement to find if the user email address already exists.
        println!("An account with this email already exists. Returning to sign in.");
        return;
    }

    println!("Please enter a password: ");
    io::stdin().read_line(&mut password).expect("Failed to read line"); // user input for the password
    let password = password.trim().to_owned(); // trims and sets the ownership of the password string
    println!("Please enter the password again: ");
    io::stdin().read_line(&mut password_verify).expect("Failed to read line"); // user inputs the password again to verify they did it correct.
    let password_verify = password_verify.trim().to_owned(); // trims and sets teh ownership of the password string.

    if password == password_verify { // runs only if the user passwords are the same.
        let new_account = Account { // creates a new variable with the struct
            email,
            password,
            balance: 0,
        };
        accounts.push(new_account); // pushes the new account to the accounts vector.
        println!("Account created successfully!");
    } else {
        println!("Passwords do not match. Please try again later. "); // if the user failed to input their password correctly, they are returned to the title.
        return;
    }
}

// function to log in an existing account.
// FUNCTION APPEARS TO BE IMPLEMENTED. ACCOUNT LOGS IN AND FUNCTION CLOSES.
fn logon_account(accounts: &mut Vec<Account>, logged_in: &mut bool, logged_in_account: &mut String) {
    let mut email = String::new();
    let mut password = String::new();

    println!("Enter your account email: ");
    io::stdin()
        .read_line(&mut email)
        .expect("Failed to read line");
    let email = email.trim().to_owned();

    println!("Enter your password: ");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password = password.trim().to_owned();

    if accounts.iter().any(|account| account.email == email && account.password == password) {
        // this section will need to set the logged_in function to true, and set the account that is logged in.
        // if logged in is set to true then a different menu will display.
        // the logged in account variable will determine what account gets accessed.
        println!("Welcome back {}", email);
        *logged_in = true;
        *logged_in_account = email;
    } else {
        println!("INVALID LOGON CREDENTIALS. TRY AGAIN LATER");
    }
}

// Function to deposit money
fn deposit_money(accounts: &mut Vec<Account>, logged_in_account: &mut String){
    let mut deposit = String::new();
    let target_account = logged_in_account.to_owned();
    println!("How much would you like to deposit today?");
    io::stdin()
        .read_line(&mut deposit)
        .expect("Failed to read line");
    let deposit_amount: i32 = deposit.trim().parse().expect("Please type a number!");

    if deposit_amount < 0  {
        println!("deposit amount cannot be less than $0");
        return;
    }

    for account in accounts.iter_mut() {
        if account.email == target_account {
            account.balance += deposit_amount;
            println!("Deposited ${} successfully. New balance is: ${}", deposit_amount, account.balance);
            return;
        }
    }
}

// Function to withdraw money
fn withdraw_money(accounts: &mut Vec<Account>, logged_in_account: &mut String) {
    let mut withdraw = String::new();
    let target_account = logged_in_account.to_owned();
    println!("How much would you like to deposit today?");
    io::stdin()
        .read_line(&mut withdraw)
        .expect("Failed to read line");
    let withdraw_amount: i32 = withdraw.trim().parse().expect("Please type a number!");

    if withdraw_amount < 0 {
        println!("Withdraw amount cannot be less than $0");
    }

    for account in accounts.iter_mut() {
        if account.email == target_account {
            if account.balance + withdraw_amount >= 0 {
                account.balance -= withdraw_amount;
                println!("Withdrew ${} successfully. New balance is: ${}", withdraw_amount, account.balance);
                return;
            } else {
                println!("You do not have the funds necessary for that withdraw");
                return;
            }
        }
    }
}

// function to display current balance
fn display_balance(accounts: &mut Vec<Account>, logged_in_account: &mut String) {
    let target_account = logged_in_account.to_owned();
    for account in accounts.iter_mut(){
        if account.email == target_account {
            println!("Your current account balance is {}", account.balance);
            return;
        }
    }
}

// function to transfer money
fn transfer_money(accounts: &mut Vec<Account>, logged_in_account: &mut String) {
    let mut transfer_target = String::new();
    let mut amount = String::new();

    println!("Enter the email of the account you wish to transfer to: ");
    io::stdin()
        .read_line(&mut transfer_target)
        .expect("Failed to read line");
    let transfer_target = transfer_target.trim().to_owned();

    if !accounts.iter().any(|account| account.email == transfer_target) {
        println!("The requested account was not found. Try again later.");
        return;
    }

    println!("Enter the amount you wish to transfer: ");
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let transfer_amount: i32 = amount.trim().parse().expect("Please enter a valid number");

    if transfer_amount < 0 {
        println!("Transfer amount cannot be less than 0$");
        return;
    }

    let mut source_account_balance = 0;
    let mut found_source_account = false;

    for account in accounts.iter_mut(){
        if account.email == *logged_in_account {
            if account.balance >= transfer_amount {
                account.balance -= transfer_amount;
                source_account_balance = account.balance;
                found_source_account = true;
                println!("Your current balance is ${}", account.balance);
                break;
            } else {
                println!("You do not have the required funds to complete this transfer.");
                return;
            }
        }
    }

    if found_source_account {
        for account in accounts.iter_mut() {
            if account.email == transfer_target {
                account.balance += transfer_amount;
                println!("Your transfer of ${} to {} was successful. Your new balance is ${}", transfer_amount, transfer_target, source_account_balance);
                return;
            }
        }
    }
}

fn main() {
    let mut app_running = true;
    let mut logged_in = false;
    let mut logged_in_account: String = String::new();
    let mut accounts: Vec<Account> = Vec::new();

    while app_running == true {
        println!("Welcome to Bank of Money");
        println!("How can we help you today?");
        println!("1) login");
        println!("2) Create an account");
        println!("3) Exit application");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let mut user_input = user_input.trim();
        if user_input == "2" {
            create_account(&mut accounts);
        }else if user_input == "1" {
            logon_account(&mut accounts, &mut logged_in, &mut logged_in_account);
        }else if user_input == "3" {
            app_running = false;
        } else {
            println!("Please select a valid option. ");
        }

        while logged_in == true {
            println!("How can we help you today?");
            println!("1) Deposit money");
            println!("2) Withdraw money");
            println!("3) Display Balance");
            println!("4) Transfer money");
            println!("5) Log out");

            let mut user_input2 = String::new();
            io::stdin()
                .read_line(&mut user_input2)
                .expect("Failed to read line");
            let mut user_input2 = user_input2.trim();

            if user_input2 == "1"{
                // Deposit money function will go here.
                deposit_money(&mut accounts, &mut logged_in_account);
            } else if user_input2 == "2" {
                // Withdraw money function will go here
                withdraw_money(&mut accounts, &mut logged_in_account);
            } else if user_input2 == "3" {
                // Display current balance function will go here
                display_balance(&mut accounts, &mut logged_in_account);
            } else if user_input2 == "4" {
                // Transfer money to other accounts
                transfer_money(&mut accounts, &mut logged_in_account);
            } else if user_input2 == "5" {
                // log out of the account
                logged_in = false;
            }
        }

    }


}
