# Using Enums in Rust

In Visual Studio Code, create a folder called **Enums** and then open it. Now open a new Terminal window.

## Exercises

Complete the following exercises.

## 09-0:  Traffic Light System

   - Create an `enum` named `TrafficLight` with three variants: `Red`, `Yellow`, and `Green`.
   - Implement a method `duration()` that returns the duration each light stays on (e.g., `Red` for 30 seconds, `Yellow` for 5 seconds, and `Green` for 60 seconds).

## 09-1:  Shape Calculation

   - Define an `enum` `Shape` with variants: `Circle`, `Square`, and `Rectangle`.
   - Each variant should **hold data** relevant to that shape (e.g., radius for `Circle`, side length for `Square`, and width and height for `Rectangle`).
   - Implement methods to calculate the area for each shape.

## 09-2:  Card Deck Management

   - Create an `enum` `Suit` for the four suits in a deck of cards: `Hearts`, `Diamonds`, `Clubs`, and `Spades`.
   - Create another `enum` `Rank` for the ranks in a deck (e.g., `Two`, `Three`, `Four`, up to `Ace`).
   - Define a `struct` `Card` that uses these [enums](/notes/13-enums/enums.md) to represent a playing card.
   - **Write a function** to create a full deck of cards (52 cards) and **another function** to shuffle this deck.