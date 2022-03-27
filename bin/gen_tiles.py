#!/usr/bin/env python3
import os,sys,re,xml
from random import Random
from pprint import pprint

d='debug';opt={d:0} # enables if opt[d]>=1: print(f'[{d}] info')
sizeX=3
sizeY=3
tiles_per_sheet = 48
min_edge_type = 2 #perimeter edges and !perimeter
max_edge_type = 23
rotations=4 # same as sides to the tiles
corner_count = 4 # we might want an L shaped board with 5 concave corners

if len(sys.argv) >= 2:
  sizeX = int(sys.argv[1])
if len(sys.argv) >= 3:
  sizeY = int(sys.argv[2])
if len(sys.argv) >= 4:
  max_edge_type = int(sys.argv[3])

tiles = sizeX * sizeY
g = [] #generated tiles
marche = ( sizeX + sizeY-2 ) * 2 # the frontier of the game
interior = tiles - marche
# number of interior tiles * rotations + edges*3 + corners*2 [all divided by 2 because the pairs must match]
max_possible_pairs = int(((interior*rotations) + ((marche - 4)*3) + (corner_count*2))/2)
if max_edge_type > max_possible_pairs: max_edge_type = max_possible_pairs
edge_count = marche - corner_count

if opt[d]>=1:
  print(f"[i] Going to generate tiles for a {sizeX} x {sizeY} board with {marche} perimeter pieces and {interior} non-edge pieces")
  print(f"[i] with {corner_count} corners and {edge_count} edges")

# given an edge what is the corrosponding edge of the neightbour?
# [ used to locate where in the neighbours array (list) of edges to write the matching edge pair ]
e2n = {
0: 2, #top -> down
1: 3, #right -> left
2: 0, #down -> top
3: 1  #left -> right
}

n2l = {
0: 'T', #top -> down
1: 'R', #right -> left
2: 'D', #down -> top
3: 'L'  #left -> right
}

def get_rand(lower=1,upper=max_edge_type):
  r = Random()
  if lower > upper:
    h = lower
    lower = upper
    upper = h
  return r.randint(lower,upper)

def index_to_XY(index):
  # expects an index from the tile array (list) and returns (x,y) tuple
  #  + used by find_neighbours(index):
  return ( ( index % sizeX, int(index / sizeX) ) )

def xy_to_index(this_x,this_y):
  return this_x + (this_y * sizeX)

# convert index,direction -> index || None (if ocean)
def find_neighbour(index,d,x,y): #index,direction,my_x,my_y
  #(x,y) = index_to_XY(index) we've already calculated this, so juts passit
  #(nx,ny) = (0,0)
  if d == 0: #above
    nx = x
    ny = y-1
    if ny < 0: return None
  elif d == 1: #east # PASS
    nx = x+1
    ny = y
    if nx > sizeX-1: return None
  elif d == 2: #down # PASS
    nx = x
    ny = y+1
    if ny > sizeY-1: return None
  elif d == 3: #west
    nx = x-1
    ny = y
    if nx < 0: return None
    #if nx < 0:
    #  print(f'Left of ({x},{y}) is ({nx},{ny})')
    #  return None
  
  return xy_to_index(nx,ny)
  
def dump_board_coordinates():
  row = ''
  for i in range(tiles):
    x,y = index_to_XY(i)
    row += f'({x},{y}) '
    if x == sizeX-1: print(row); row=''


def dump_next_coordinates(direction):
  row = ''
  for i in range(tiles):
    x,y = index_to_XY(i)
    ni = find_neighbour(i,direction,x,y)
    if ni is not None: nx,ny = index_to_XY(ni)
    else:
      if opt[d]>=10: print(f'Sadly ({x},{y}) in {direction} has {ni} neighbour')
      nx = '?'; ny = '?'
    row += f'({nx},{ny}) '
    if x == sizeX-1: print(row); row=''

def d2l(d):
  # direction (number:0..3) to letter
  return n2l[d]

def test_coordinates():
  """
  debug verification that the code can
  correctly identify the neighbour coordinates
  """
  dump_board_coordinates()
  print()
  for direction in range(4): #lets look in 0..3 directions
    print(f'{d2l(direction)}:')
    dump_next_coordinates(direction)

if opt[d]>=10: sys.exit(0)

# initialise the board
board = []
for i in range(sizeY*3):
  board.append([])
  for j in range(sizeX*3):
    board[i].append('.')
"""
 .T.
 LxR
 .D.

"""

def dump_ascii(board):
  # dumps to stderr to make it easier to filer from the xml on stdout
  for i in board:
    row = ''
    for j in i:
      if type(j) is int:
        row += f'{j:02} '
      else:
        row += f'{j:2} '
    sys.stderr.write(row + "\n")
    #print(row)

tile_set = {}
for i in range(tiles):
  x,y = index_to_XY(i)
  for direction in range(4): #lets look in 0..3 directions
    d = direction
    # we could improve performance by skipping this if tile_set[i][d]
    ni = find_neighbour(i,d,x,y)
    if ni is not None:
      (nx,ny) = index_to_XY(ni)
      edge = get_rand(2,max_edge_type) #production
      # NOTE: we should have the option to check that this tile doesn't 
      # NOTE: already have two of this edge already
      (n0,n1,n2,n3) = (None,None,None,None)
      if d == 0: #top
        bx = (x*3)+1; by = (y*3)  # this tiles edge
        vx = (nx*3)+1; vy = (ny*3)+2  # voisins edge
        m0 = edge
        n2 = edge
      elif d == 1: #right
        bx = (x*3)+2; by = (y*3)+1
        vx = (nx*3); vy = (ny*3)+1
        m1 = edge
        n3 = edge
      elif d == 2: #down
        bx = (x*3)+1; by = (y*3)+2
        vx = (nx*3)+1; vy = (ny*3)
        m2 = edge
        n0 = edge
      elif d == 3: #left
        bx = (x*3); by = (y*3)+d-2
        vx = (nx*3)+2; vy = (ny*3)+1
        m3 = edge
        n1 = edge
      is_set = board[by][bx]
      if is_set == '.':
        board[by][bx] = str(edge)
        board[vy][vx] = str(edge)
        nd = e2n[d]
        if nd != 0 and ni+1 in tile_set:
          n0 = tile_set[ni+1][0]
        if nd != 1 and ni+1 in tile_set:
          n1 = tile_set[ni+1][1]
        if nd != 2 and ni+1 in tile_set:
          n2 = tile_set[ni+1][2]
        if nd != 3 and ni+1 in tile_set:
          n3 = tile_set[ni+1][3]
        tile_set[ni+1] = [n0,n1,n2,n3]


        if i+1 in tile_set:
          tile_set[i+1][d] = edge
        else:
          if d != 0:
            m0 = tile_set[ni+1][0]
          if d != 1:
            m1 = tile_set[ni+1][1]
          if d != 2:
            m2 = tile_set[ni+1][2]
          if d != 3:
            m3 = tile_set[ni+1][3]
          tile_set[i+1] = [m0,m1,m2,m3]
    else:
      edge = 'A'
      if d == 0: #top
        bx = (x*3)+d+1; by = (y*3)+d
      elif d == 1: #right
        bx = (x*3)+d+1; by = (y*3)+d
      elif d == 2: #down
        bx = (x*3)+d-1; by = (y*3)+d
      elif d == 3: #left
        bx = (x*3); by = (y*3)+d-2
      #print(f'({x},{y}) -> {n2l[d]}:{by},{bx} = ({i},{d}): _,_')
      board[by][bx] = edge
      if i+1 in tile_set:
        tile_set[i+1][d] = edge
      else:
        tile_set[i+1] = [edge,None,None,None]

dump_ascii(board)

"""
 massage tile_set dict into dict(t) 
"""
t = {}
for i,tle in tile_set.items():
  t[f'tile{i}'] = {"id": i }
  for e in range(len(tle)):
    t[f'tile{i}'][f'edge{e}'] = {"value": tle[e]}
    
"""
 This is the desired XML output
#<?xml version="1.0" ?>
#<e1>
#        <tile id='1'>
#                <edge value="1"/>
#                <edge value="3"/>
#                <edge value="4"/>
#                <edge value="2"/>
#        </tile>...
#</e1>
"""

import dict2xml_reimund
print('<?xml version="1.0" ?>' + re.sub("<edge\d*","\t<edge", re.sub("tile\d*","tile", dict2xml_reimund.dict2xml(t, 'e1').replace("<","\n<").replace("A","1"))))


#import dict2xml_thomaswpp  # This requires we massage the tile_set dict into a different form
#print(dict2xml_thomaswpp.dict2xml(t), 'e1')
#print('<?xml version="1.0" ?>\n' + re.sub("<edge\d*","\t<edge", re.sub("tile\d*","tile", dict2xml_thomaswpp.dict2xml(t, 'e1').replace("<","\n<"))))
sys.exit(0)

