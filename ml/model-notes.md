Disclaimer: this is not the final design right now, it's just a slightly better looking version of thoughts bouncing around in my head

=Representations=
==Cards==
14 dimensions

- suit (2 dimensions)
- rank (4 dimensions)
- edition (2 dimensions)
- enhancement (3 dimensions)
- seal (3 dimensions)

==Jokers==
32 dimensions

- joker type (30 dimensions)
- edition (2 dimensions)

=Transformers=

- Hand (n 14-dimensional vectors) -> Embedding (32-dimensional vector)
  - ability to determine the current "strengths" of a hand
- Deck (n 14-dimensional vectors) -> Emebedding (128-dimensional vector)
  - ability to determine the current "leaning" of the remaining deck
- Owned Jokers (n 32-dimensional vectors) -> Embedding (64-dimensional vector)
  - ability to determine the "strengths" of currently owned jokers

=Main Model=
7 total layers
5 fully connected hidden layers
lots of sigmoid
Actor-Critic (Reinforcement Learning)

==Input Neurons==

- Hand Transformer Output: 32 neurons
- Owned Joker Transformer Output: 64 neurons
- Deck Transformer Output: 128 neurons
- Blinds: 30 neurons (one-hot)
- Game Phase: 4 neurons (one-hot; blind select, shop, blind, pack)
- Ante: 38 neurons (one-hot)
- Remaining Discards: 1 neuron (integer)
- Remaining Hands: 1 neuron (integer)
- Hand Size: 1 neuron (integer)
- Money: 1 neuron (integer)
- Current Score: 1 neuron (integer)
- Required Score: 1 neuron (integer)
- Shop Reroll Cost: 1 neuron (integer)
- Tarot Cards: 22 neurons (each; one-hot)
- Planet Cards: 12 neurons (each; one-hot)
- Spectral Cards: 18 neurons (each; one-hot)
- Shop Joker: 128 neurons (32 each, up to 4)

==Output Neurons==

- Play Action: 1 neuron
- Discard Action: 1 neuron
- Card Selection: 5 neurons
- Purchase Action: 1 neuron
- Item to Purchase: minimum 7 neurons (one-hot right now, doesn't work in the general)
- Skip Blind Action: 1 neuron
- Play Blind Action: 1 neuron
