pub mod greetings
{
    pub mod english; // Rust knows to look for a 'greetings' folder with English.rs 
    pub mod french
    {
        pub fn hello() -> String {"bonjour".to_string()}
        pub fn goodbye() -> String {"au revoir".to_string()}
    }
}

