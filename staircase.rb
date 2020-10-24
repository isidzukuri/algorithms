def staircase(n)
  n.times do |i|
    spaces = n - i - 1

    print(' '*spaces + '#'*(i+1) + "\n")
  end
end

staircase(4)