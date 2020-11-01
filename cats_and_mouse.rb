# https://www.hackerrank.com/challenges/cats-and-a-mouse/problem

require 'awesome_print'

def catAndMouse(x, y, z)
  cat_1_distance = (z - x).abs
  cat_2_distance = (z - y).abs

  if cat_1_distance < cat_2_distance
    p 'Cat A'
  elsif cat_1_distance > cat_2_distance
    p 'Cat B'
  else
    p 'Mouse C'
  end
end

catAndMouse(1, 3, 2)