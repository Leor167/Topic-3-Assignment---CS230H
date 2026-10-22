




fn main() {
    let account = UserAccount{
        username: String::from("Ben"),
        email: String::from("Ben@yahoo.com")
    };
    let name;

        {
        let session = ActiveSession::new_session(1000, &account);
        name = get_username(&session); 
        println!("Session {} belongs to {} ", session.session_id, name);
        println!("email is {}", session.user.email);
        }
    

    println!("Account {} still here after session lifetime ended", name); // even thought name was defined with get_username which returns a reference to a str since it was given lifetime 'a it can still be used even after the scope ended
}

// owns its own data
struct UserAccount{
    username: String,
    email: String,
}
//Active Session contains a reference to the struct UserAccount for user
//defined lifetime parameter 'a thats tied to UserAccount. So long as UserAccount is alive it can be borrowed from, its lifetime is denoted with 'a.
struct ActiveSession<'a>{
    session_id: u32,
    user: &'a UserAccount,
}

//method implemented for ActiveSession struct
//constructor. Returns a ActiveSession struct which is referred to as Self in return area
impl<'a> ActiveSession<'a> {
    fn new_session(session_id: u32, user: &'a UserAccount) -> Self{
        ActiveSession {
            session_id,
            user
        }
    }
}
//function that takes a ActiveSession with a explicit lifetime parameter. returned string is tied to said lifetime parameter 'a
fn get_username<'a>(session: &ActiveSession<'a>) -> &'a str {
    &session.user.username
}

