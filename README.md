# beemovie-rs
Bee Movie crate.
# Usage
Put this in your Cargo.toml
```toml
beemovie = '1.0.1'
```
# Documentation
## sentence()
This function returns a sentence or two from the bee movie.
```rust
extern crate beemovie;
fn main() {
    println!("Sentence: {}", beemovie::sentence(1));
}
```
## script()
This function returns the entire Bee Movie script.
```rust
extern crate beemovie;
fn main() {
    println!("{}", beemovie::script());
}
```
## word()
This function returns a word from the Bee Movie.
```rust
extern crate beemovie;
fn main() {
    println!("Word: {}", beemovie::word(1));
}
```
## paragraph()
This function returns a paragraph from the Bee Movie.
```rust
extern crate beemovie;
fn main() {
    println!("Paragraph: {}", beemovie::paragraph(1));
}
```
