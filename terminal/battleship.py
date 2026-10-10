import random

# Game Settings
BOARD_SIZE = 5
NUM_SHIPS = 3

def create_board():
    """Creates a blank board grid filled with ocean waves (~)."""
    return [["~" for _ in range(BOARD_SIZE)] for _ in range(BOARD_SIZE)]

def print_board(board):
    """Beautifully prints the board matrix to the terminal with row/column headers."""
    print("\n   " + " ".join(str(i) for i in range(BOARD_SIZE)))  # Column index
    for idx, row in enumerate(board):
        print(f"{idx}  " + " ".join(row))

def place_ships():
    """Generates secret coordinate points for the hidden enemy fleet."""
    ships = set()
    while len(ships) < NUM_SHIPS:
        x = random.randint(0, BOARD_SIZE - 1)
        y = random.randint(0, BOARD_SIZE - 1)
        ships.add((x, y))
    return ships

def main():
    print("=================================")
    print("   WELCOME TO TERMINAL BATTLESHIP ")
    print("=================================")
    print(f"Find and destroy the {NUM_SHIPS} hidden enemy ships!")
    
    board = create_board()
    hidden_ships = place_ships()
    ships_destroyed = 0
    turns = 10

    while turns > 0 and ships_destroyed < NUM_SHIPS:
        print_board(board)
        print(f"\nRemaining Turns: {turns} | Ships Destroyed: {ships_destroyed}/{NUM_SHIPS}")
        
        try:
            # Take target input coordinates from the user
            row = int(input(f"Enter Row (0-{BOARD_SIZE-1}): "))
            col = int(input(f"Enter Column (0-{BOARD_SIZE-1}): "))
            
            # Boundary Check
            if row < 0 or row >= BOARD_SIZE or col < 0 or col >= BOARD_SIZE:
                print("💥 Out of bounds! Aim within the grid coordinates.")
                continue
                
            # Duplicate Shot Check
            if board[row][col] == "X" or board[row][col] == "O":
                print("⚠️ You already fired a shell there! Pick new coordinates.")
                continue
                
            # Hit or Miss Calculations
            if (row, col) in hidden_ships:
                print("\n🔥 DIRECT HIT! An enemy ship was blown apart!")
                board[row][col] = "X"  # X marks a Hit
                ships_destroyed += 1
            else:
                print("\n💦 Splash... Total miss.")
                board[row][col] = "O"  # O marks a Miss
                
            turns -= 1
            
        except ValueError:
            print("❌ Invalid input! Please enter numbers only.")

    # Game Over Conditions
    print_board(board)
    if ships_destroyed == NUM_SHIPS:
        print("\n🎉 VICTORY! You completely neutralized the enemy fleet!")
    else:
        print("\n💀 GAME OVER. Your fleet ran out of ammunition.")
        print("The enemy ships were located at:")
        for x, y in hidden_ships:
            print(f" - Row {x}, Col {y}")

if __name__ == "__main__":
    main()
