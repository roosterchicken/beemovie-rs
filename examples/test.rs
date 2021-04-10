use promptly::prompt;

fn main() {
    let number: i32 = prompt("How many things do you to generate? ").expect("an error occured");
    println!("{}", beemovie::word(number));
    println!("{}", beemovie::sentence(number));
    println!("{}", beemovie::paragraph(number));
}
