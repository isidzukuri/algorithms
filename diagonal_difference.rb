def diagonalDifference(arr)
  size = arr.length

  diagonal1_sum = 0
  diagonal2_sum = 0
  i = 0
  loop do
    diagonal1_sum += arr[i][i]
    diagonal2_sum += arr[i][size - (i += 1)]
    
    break unless i < size
  end

  (diagonal1_sum - diagonal2_sum).abs
end

arr = [
  [1, 2, 3],
  [4, 5, 6],
  [9, 8, 9]
]

diagonalDifference(arr)
