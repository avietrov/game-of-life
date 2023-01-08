function neighbours(world, row, col)
  n, m = size(world)

  down = mod1(row+1, n)
  up = mod1(row-1, n)
  left = mod1(col+1, m)
  right = mod1(col-1, m)

  return (
          world[up, left] + world[up, col] + world[up, right] +
          world[row, left] + world[row, right] +
          world[down, left] + world[down, col] + world[down, right]
         )
end

function neighbours(world)
  n, m = size(world)

  return [neighbours(world, row, col) for row in 1:n, col in 1:m]
end


function willsurvive(cell, k)
  if k == 3
    return true
  elseif k == 2 && cell
    return true
  else 
    return false
  end 
end


function evolve!(world)
  ks = neighbours(world)
  for i in eachindex(world)
    world[i] = willsurvive(world[i], ks[i])
  end 
  return
end

world = zeros(Bool, 10, 10)
world[2, 3] = 1
world[3, 3] = 1
world[4, 3] = 1

using Plots
anim = @animate for i in 1:150
    heatmap(world; axis = nothing, border = :none, cbar = false, ratio = :equal)
    evolve!(world)
end
gif(anim, "gameoflife.gif"; fps = 10)
