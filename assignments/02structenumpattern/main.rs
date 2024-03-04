


mod structenum;
fn main(){
    println!("Struct + Enum + Match")

    // Use the structenum module to create a new Book instance
    let mut book = structenum::Book::new("The Rust Programming Language".to_string(), "Steve Klabnik and Carol Nichols".to_string());
    
    // Display the initial status of the book
    println!("{}", book.display_status());

    // Check out the book for 14 days
    book.check_out(14);
    println!("{}", book.display_status());

    // Mark the book as being read
    book.mark_as_being_read();
    println!("{}", book.display_status());

    // Return the book
    book.return_book();
    println!("{}", book.display_status());

    // Send the book for repair
    book.send_for_repair("Cover needs replacement".to_string());
    println!("{}", book.display_status());

    // Report the book as lost
    book.report_lost();
    println!("{}", book.display_status());
}


