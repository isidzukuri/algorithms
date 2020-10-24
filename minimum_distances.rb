def minimumDistances(a)
  distances = []

  a.each do |item|
    indexes = a.each_index.select{|i| a[i] == item}

    next unless indexes.length >=1

    indexes.each_cons(2) { |chunk| distances << chunk[1] - chunk[0] }
  end

  distances.min || -1
end



minimumDistances([7, 1, 3, 4, 1, 7])