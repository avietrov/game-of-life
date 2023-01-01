import Control.Monad (when)
data CellState = Dead | Alive

data Position = Position Integer Integer

type Generation = Position -> CellState

isAlive :: CellState -> Bool
isAlive Alive = True
isAlive Dead = False

neighbors :: Position -> [Position]
neighbors (Position x y) =
  [Position (x-1) (y-1), Position x (y-1), Position (x+1) (y-1),
   Position (x-1) y, Position (x+1) y,
   Position (x-1) (y+1), Position x (y+1), Position (x+1) (y+1)]

aliveNeighbors :: Generation -> Position -> Int
aliveNeighbors generation position = length (filter isAlive (map generation (neighbors position)))

evolution :: Generation -> Position -> CellState
evolution generation position =
  case aliveNeighbors generation position of 
    2 -> if isAlive (generation position) then Alive else Dead
    3 -> Alive
    _ -> Dead


visualizeGeneration generation =
  map (visualizeLine generation) [1..10]

visualizeLine :: Generation -> Integer -> String
visualizeLine generation y =
  concatMap (visualizeCell generation y) [1..10]

visualizeCell generation y x =
  case generation (Position x y) of 
  Alive -> ['X']
  Dead -> [' ']


bar (Position 1 2) = Alive
bar (Position 2 2) = Alive
bar (Position 3 2) = Alive
bar (Position x y) = Dead

main = mapM_ print (visualizeGeneration (evolution bar))
