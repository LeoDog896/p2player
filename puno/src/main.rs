enum Color {
	RED,
	BLUE,
	YELLOW,
	GREEN
}

enum CardType {
	Number(u32)
}

struct Card {
	color: Color,
	type: CardType
}

fn main() {
    println!("Hello, world!");
}
