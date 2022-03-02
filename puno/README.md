
# puno

Decentralized Card Game based on popular game mechanics

## Card Distribution

### Requirements

 - A secure peer-to-peer transfer network

### Choosing the card

rt = total amount of cards
ri = chosen card per person
r = chosen card number
p = total number of players

Assumptions for this example situation:
- 3 players exist
- One of them want to cheat.

To choose a card, Player 2 and Player 3 generate random numbers from `ri = {0, 1 .. (rt - 1)}` and send it to Player 1.

They also store the index of this card.

For Player 1 to get their card, they do `r = (sum of ri) % rt`

For example:
```
Player 2 sent (number 10, index 1) to Player 1
Player 3 sent (number 8, index 1) to Player 1
Player 1 has (card 18, index 1)
```

#### Verification

Whenever Player 1 places down the card, they broadcast the index and card number. Player 2 and 3 verify the card by broadcasting their number for that index.

#### Duplicate cards

If Player 2 and 3 generate 10 and 8 again, Player 1 lets them know that they already have that card -- Player 1 gives them the part that they generated with the index -- they try again

##### Cheating
Assume Player 1 has 5 cards -- Player 1 can say they already have the card they gave until they run out of card lenghts.

##### Solution
Assuming Player 1 wants a different card than chosen --

Player 1 has 3 cards:
```
17 at i1 - 10, 7
23 at i2 - 20, 3
40 at i3 - 20, 20
```

Player 2 sent (number 10, index 1) to Player 1. Player 1 responds with this information that they gave them as well as to Player 2. They both verify this information. If the player lies about having the card, they have to provide indexes of the card it was at.

This means that they can only use that card once to cheat, reducing cheating methods. **However, it is not certain**

(TODO: Clean up?)

### Full Specifications

Card possibilities:
- Red, Green, Yellow, Blue (can stack with same color)
	- Numbers 1-9 (can stack with same number)
	- Plus 2
	- Plus 4
	- _11 combinations per color_
	- **Total: 44 combinations**
- Black (stacks with all)
	- Wild
	- Wild Plus 4
	- **Total: 2 * 4 = 8**
- ***Total: 52 combinations***

Card numbers are in this order, from `{0, 1 .. 52 - 1}`

### Standard Protocols

#### Getting a card

##### Packets
`card_request`: -> player broadcast
`card_number(num)`: -> player

##### Procedure

Player who requests card = Pr

Pr sends `card_request`.

All players except for Pr send `card_number(num) -> Pr`, num = a random number {0, 1, ..., (total amount of cards) - 1}

The player who wants a card takes the sum of all `card_number` packets % (total amount of cards).
