# https://www.hackerrank.com/challenges/counting-valleys/problem
# require 'awesome_print'

def countingValleys(steps, path)
  valleys = 0
  above_sea_level = 0
  valley_starts = nil
  valley_ends = nil



  
  path.split('').each do |step|
    above_sea_level_before_step = above_sea_level

    if step == 'U'
      above_sea_level += 1
    else
      above_sea_level -= 1
    end

    if above_sea_level == -1 && above_sea_level_before_step == 0
      valley_starts = true
    end

    if above_sea_level == 0 && above_sea_level_before_step == -1
      valley_ends = true
    end

    if valley_starts && valley_ends
      valleys += 1
      valley_starts = nil
      valley_ends = nil
    end
  end

  valleys
end



steps = 8
path = 'UDDDUDUU'

countingValleys steps, path
