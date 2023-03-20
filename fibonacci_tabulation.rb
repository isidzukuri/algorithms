def fibonacci_tabulation(n)
  table = Array.new(n+2, 0)
  table[1] = 1

  i = 1
  while i < n
    table[i+1] += table[i]
    table[i+2] += table[i]
    i += 1
  end

  table[n]
end


p fibonacci_tabulation(6) # 8
p fibonacci_tabulation(7) # 13
p fibonacci_tabulation(8) # 21
p fibonacci_tabulation(50) # 12586269025

