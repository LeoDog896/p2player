mod card;

use card::{Card, generate_cards};

fn main() {
	let cards: Vec<Card> = generate_cards();
}
