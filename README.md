# Blackjack
This simple console application lets you play the classic card game Blackjack. It supports saving & loading to a file so you can continue where you left.

![image](https://github.com/user-attachments/assets/b2ffe70d-76de-4e20-a20b-aac810561e6f)

## Gameplay
You start with $10 and the game ends if you lose all your money. If you get very rich, you may win the game. 

The dealer hits until 17. There are no soft 17's. When you win, you get 2x the bet.

The game is automatically saved every round, indicated by a text saying "saved". So if you want to quit, you can just close the app.

![image](https://github.com/user-attachments/assets/159841fb-7540-4150-b658-ba7f7caad7bc)

## Installation
Download the exe-file:
- Go to this project's releases section
- Download the exe file
  
Alternative way to install (build the app yourself): 
- Clone this project to your PC
- Open cmd in the folder you cloned the project into
- Type in ```cargo build --release```
- Once building is ready, go to ```target\release\blackjack.exe```
