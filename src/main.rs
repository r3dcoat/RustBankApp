// recreating a banking application that I created in Python
use std::io;

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

fn main() {
    let mut app_running = true;
    let mut _logged_in = false;
    let mut _logged_in_account: String = String::new();
    let mut accounts: Vec<Account> = Vec::new();

    while app_running == true {
        println!("Welcome to Bank of Money");
        println!("How can we help you today?");
        println!("1) login");
        println!("2) Create an account");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let mut user_input = user_input.trim();
        if user_input == "2" {
            create_account(&mut accounts);
        } if user_input == "1" {

        } if user_input == "quit" {
            app_running = false;
        } else {
            println!("Please select a valid option. ");
        }

    }


}
