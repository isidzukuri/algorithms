# write a function can_sum(target_sum, numbers) that takes in target_sum and array of numbers as an arguments.
# the function should return a boolean indicating wheather or not it is possible to generate true target_sum using numbers from the array


def can_sum(target_sum, numbers, cache = {})
  return true if target_sum == 0
  return false if target_sum < 0

  key = target_sum.to_s 
  return cache[key] if cache[key] != nil

  numbers.each do |number|
    reminder = target_sum - number
  
    if can_sum(reminder, numbers, cache)
      cache[key] = true

      return true
    end
  end

  cache[key] = false
end


p can_sum(7, [2, 3]) # true
p can_sum(7, [5, 3, 4, 7]) # true
p can_sum(7, [2, 4]) # false
p can_sum(8, [2, 3, 5]) # true
p can_sum(300, [7, 14]) # false