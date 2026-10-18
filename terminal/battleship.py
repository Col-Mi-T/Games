import random

# Game Settings
BOARD_SIZE = 5
NUM_SHIPS = 3
cols = [['O'] * BOARD_SIZE for _ in range(BOARD_SIZE)]
ship_dead = {
    1: False,
    2:False,
    3:False
}

def create_grid():
    ship = create_ships()
    #print(ship)
    if ship != None:
        cols[ship[0]][ship[1]] = ship[2]
    draw(cols)

def create_ships():
    global afloat
    if afloat < NUM_SHIPS:
        x= random.randint(0,BOARD_SIZE-1)
        y = random.randint(0,BOARD_SIZE-1)
        v = "X"
        afloat = afloat+1
        return [x,y,v]
    else:
        return None
  
def draw(c):
    for i in range(BOARD_SIZE):
        #print(i, i==BOARD_SIZE-1)
        if i == BOARD_SIZE-1:
            print(i, end="\n\n")
        elif i == 0:
            print(f"\t{i}",end="\t")
        else:
            print(i, end="\t")
    for i in range(BOARD_SIZE):
        col = c[i]
        line = "\t".join(col)
        print(i,"\t",line,"\n\n")
    

def play():
    while True:
        create_grid()
        row = int(input("Guess a row: "))
        col = int(input("Guess a col: "))

if __name__ == "__main__":
    play()