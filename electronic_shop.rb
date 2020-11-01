# https://www.hackerrank.com/challenges/electronics-shop/problem

require 'awesome_print'

def getMoneySpent(keyboards, drives, b)
  highest_sum = -1

  keyboards.each do |keyboard|
    drives.each do |drive|
      sum = keyboard + drive

      next if sum > b
      
      highest_sum = sum if sum > highest_sum
    end
  end

  highest_sum
end


keyboards = [40, 50, 60]  
drives = [5, 8, 12] 
b = 60

getMoneySpent(keyboards, drives, b)