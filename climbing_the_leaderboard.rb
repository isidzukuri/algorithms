# https://www.hackerrank.com/challenges/climbing-the-leaderboard/problem


require 'benchmark'
require 'ruby-prof'
require 'awesome_print'

def climbingLeaderboard_1(ranked, player)
  places = []

  player.each do |round|
    leaderbord = ranked.dup
    leaderbord << round
    places << leaderbord.uniq.sort_by {|item| -item }.index(round) + 1
  end

  places
end



def climbingLeaderboard_2(ranked, player)
  places = []

  player.each do |round|
    ranked << round
    places << ranked.uniq.sort_by {|item| -item }.index(round) + 1
    ranked.delete(round)
  end

  places
end



def climbingLeaderboard_3(ranked, player)
  places = []
  leaderboard = ranked.uniq
  leaderboard_size = leaderboard.size

  player.each do |round|
    place = nil
    leaderboard.each_with_index do |val, i|
      if round >= val
        place = i + 1
        break
      end
    end

    if place
      places << place
    else
      places << leaderboard_size + 1
    end
  end

  places
end


def climbingLeaderboard_4(ranked, player)
  places = []
  leaderboard = ranked.uniq
  last_place = leaderboard.size + 1

  player.each do |round|
    place = nil
    leaderboard.each_with_index do |val, i|
      if round >= val
        place = i + 1
        break
      end
    end

    if place
      places << place
    else
      places << last_place 
    end
  end

  places
end




def climbingLeaderboard_5(ranked, player)
  places = []
  leaderboard = ranked.uniq
  leaderboard_size = leaderboard.size

  player.each do |round|
    place = nil

    place = 1 if round >= leaderboard.first
    place = (leaderboard_size + 1) if round < leaderboard.last

    if place
      places << place
      next
    end

    step = leaderboard.first / ([round, leaderboard.last, leaderboard.first].inject(:+) / 3)


    current_index = step-1

    while leaderboard_size > current_index
      if round >= leaderboard[current_index] 
        minus = 0

        loop do
          ranked_index = current_index - minus
          val = leaderboard[ranked_index] 

          if round >= val
            place = ranked_index + 1
            break
          end

          minus +=1
        end
      else
        current_index += step
      end

      if place
        places << place
        break
      end
    end
  end

  places
end


def climbingLeaderboard(ranked, player)
  places = []
  leaderboard = ranked.uniq

  player.each do |round|
    if leaderboard.last > round
      places << leaderboard.size + 1
    else
      places << binary_search(0, (leaderboard.size-1), leaderboard, round)
    end
  end
  places
end

def binary_search(from, to, leaderboard, round)
  place = nil

  first_part_last_index = from + ((to - from).to_f / 2).ceil

  if leaderboard[from] <= round
    place = from +1
  elsif leaderboard[first_part_last_index] == round
    place = first_part_last_index + 1 
  elsif leaderboard[first_part_last_index] < round
    place = binary_search(from, (first_part_last_index-1), leaderboard, round)
  elsif leaderboard[first_part_last_index] > round
    place = binary_search((first_part_last_index+1), to, leaderboard, round)
  end

  place
end








# ranked = [100, 90, 80]
# player = [70, 80, 105]

# ranked = [100, 90, 90, 80]
# player = [70, 80, 105]


# ranked = [100, 100, 50, 40, 40, 20, 10]
# player = [5, 25, 50, 120]
# ranked = [100, 90, 80, 75, 60]
# player = [50, 65, 77, 90, 102]

ranked = (1..488888).to_a
player = [200000]




puts Benchmark.measure { 100.times { climbingLeaderboard_1(ranked.clone, player) } }
puts Benchmark.measure { 100.times { climbingLeaderboard_2(ranked.clone, player) } }
puts Benchmark.measure { 100.times { climbingLeaderboard_3(ranked.clone, player) } }
puts Benchmark.measure { 100.times { climbingLeaderboard_4(ranked.clone, player) } }
puts Benchmark.measure { 100.times { climbingLeaderboard_5(ranked.clone, player) } }
puts Benchmark.measure { 100.times { climbingLeaderboard(ranked.clone, player) } }


# puts Benchmark.measure { 20000.times { climbingLeaderboard_1(ranked.clone, player) } }
# puts Benchmark.measure { 20000.times { climbingLeaderboard_2(ranked.clone, player) } }
# puts Benchmark.measure { 20000.times { climbingLeaderboard_3(ranked.clone, player) } }
# puts Benchmark.measure { 20000.times { climbingLeaderboard_4(ranked.clone, player) } }
# puts Benchmark.measure { 20000.times { climbingLeaderboard_5(ranked.clone, player) } }
# puts Benchmark.measure { 20000.times { climbingLeaderboard(ranked.clone, player) } }

# puts Benchmark.measure { 1.times { climbingLeaderboard(ranked.clone, player) } }


climbingLeaderboard(ranked, player)
