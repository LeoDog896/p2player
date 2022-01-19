#[derive(PartialEq, Debug, Clone)]
enum Color {
	Red,
	Blue,
	Yellow,
	Green,
	Black
}

impl Color {
	/// Checks if a color can stack with the current color.
	/// If it is the same color or the new card is black, it can stack
    fn can_stack(&self, color: Self) -> bool {
        *self == color || color == Self::Black
    }
}

#[derive(Debug, PartialEq)]
struct PunoNumber(u8);

impl PunoNumber {
	fn is_in_range(val: u8) -> bool {
		val <= 9
	}

	pub fn new(val: u8) -> Option<PunoNumber> {
		if Self::is_in_range(val) {
			Some(Self(val))
		} else {
			None
		}
	}
}

#[derive(Debug, PartialEq)]
enum CardType {
	Number(PunoNumber),
	Reverse,
	Skip,
	PlusTwo,
	Wild,
	WildPlusFour
}

impl CardType {
	fn can_stack(&self, card_type: Self) -> bool {
		*self == card_type
	}
}

#[derive(Debug, PartialEq)]
struct Card {
	color: Color,
	card_type: CardType
}

impl Card {

	pub fn can_stack(&self, card: Self) -> bool {
		card.color == self.color || card.card_type == self.card_type
	}

	pub fn new(color: Color, card_type: CardType) -> Card {
		Card {
			color,
			card_type
		}
	}
}

fn generate_cards() -> Vec<Card> {
	// For every color:
	[Color::Red, Color::Green, Color::Blue, Color::Yellow].iter().flat_map(|color| 
		// Repeat twice
		(0..=1).into_iter().flat_map(|_| 
			// Generate cards 1-9 + Skip/Reverse/PlusTwo cards
			(1..=9).into_iter()
				.filter_map(PunoNumber::new)
				.map(CardType::Number)
				.map(|face| Card::new(color.clone(), face))
				.chain([
					Card::new(color.clone(), CardType::Skip),
					Card::new(color.clone(), CardType::Reverse),
					Card::new(color.clone(), CardType::PlusTwo)
				])
		).chain([ 
			// Add a 0 card per color
			Card::new(color.clone(), CardType::Number(PunoNumber(0)))
		])
	// Add Wild / WildPlusFour
	).chain((0..4).into_iter().flat_map(|_| [
		Card::new(Color::Black, CardType::WildPlusFour),
		Card::new(Color::Black, CardType::Wild)
	])).collect()
}

fn main() {
	// A game of puno, based on the popular card game UNO, has:
	// Cards 1-9 twice for Red, Green, Yellow, and Blue, plus 1 0 card for those colors
	// Four wild and plus four wild cards
	let cards: Vec<Card> = generate_cards();
}
