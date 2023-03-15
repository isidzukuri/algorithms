# grid 2x3 is given:
# 1 2 3
# 4 5 6
# how many ways to travel from 1 to 6?



require 'benchmark'
require 'ruby-prof'
require 'awesome_print'


def grid_traveler_v0(rows, columns)
  return 1 if(rows == 1 && columns == 1)
  return 0 if(rows == 0 || columns == 0)

  grid_traveler_v0(rows - 1, columns) + grid_traveler_v0(rows, columns - 1)
end



def grid_traveler_v1(rows, columns, cache = {})
  return 1 if(rows == 1 && columns == 1)
  return 0 if(rows == 0 || columns == 0)

  cache["#{rows}_#{columns}"] ||=  grid_traveler_v1(rows - 1, columns, cache) + 
                                    grid_traveler_v1(rows, columns - 1, cache)
end


def grid_traveler_v2(rows, columns, cache = {})
  return 1 if(rows == 1 && columns == 1)
  return 0 if(rows == 0 || columns == 0)

  cache[[rows, columns].sort.join('_')] ||=  grid_traveler_v2(rows - 1, columns, cache) + 
                                             grid_traveler_v2(rows, columns - 1, cache)
end


def grid_traveler_v3(rows, columns, cache = {})
  return 1 if(rows == 1 && columns == 1)
  return 0 if(rows == 0 || columns == 0)

  key = rows > columns ?  "#{rows}_#{columns}" : "#{columns}_#{rows}"

  cache[key] ||=  grid_traveler_v2(rows - 1, columns, cache) + 
                                             grid_traveler_v2(rows, columns - 1, cache)
end


# p grid_traveler_v1(1, 1) #1
# p grid_traveler_v1(2, 3) #3
# p grid_traveler_v1(3, 2) #3
# p grid_traveler_v1(3, 3) #6
# p grid_traveler_v1(18, 18) #2333606220
# p grid_traveler_v1(100, 100) #22750883079422934966181954039568885395604168260154104734000


# puts Benchmark.measure { grid_traveler_v1(100, 100) } # 0.021
# puts Benchmark.measure { grid_traveler_v2(100, 100) } # 0.017

# puts Benchmark.measure { grid_traveler_v1(2000, 2000) } # 9.83
# puts Benchmark.measure { grid_traveler_v2(2000, 2000) } # 14.29
# puts Benchmark.measure { grid_traveler_v3(2000, 2000) } # 12.88

