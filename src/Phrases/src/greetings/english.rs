//! This module contains English phrases. (Module comments are in Markdown)
//! 
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!", phrases::greetings::english::hello(),
//!          username);
//! ```

// comment

/* longer
comments */

/// Applies to code that follows it.
/// In this case, it's our `hello()` function
pub fn hello() -> String {"hello".to_string()}


pub fn goodbye() -> String {"goodbye".to_string()}