# https://leetcode.com/problems/minimum-number-of-pushes-to-type-word-ii/


# @param {String} word
# @return {Integer}
BUTTONS_COUNT = 8
def minimum_pushes(word)
    word.each_byte
        .tally
        .values
        .sort! { |a, b| b <=> a }
        .each_with_index
        .sum do |count, idx|
            multiplier = 1 * (1 + (idx)/BUTTONS_COUNT)
            count * multiplier
        end
end


# @param {String} word
# @return {Integer}
BUTTONS_COUNT = 8
def minimum_pushes(word)
    pushes = 0
    stats = word.bytes.tally
    sorted_counts = stats.values.sort { |a, b| b <=> a }

    sorted_counts.each.with_index do |count, idx|
        multiplier = 1 * (1 + (idx)/BUTTONS_COUNT)
        pushes += count * multiplier
    end

    pushes
end




############################### AI ###########################

BUTTONS_COUNT = 8

def minimum_pushes(word)
  # Count character frequencies directly and sort descending
  counts = word.each_byte.tally.values.sort! { |a, b| b <=> a }

  # Accumulate pushes using index math
  counts.each_with_index.sum do |count, idx|
    count * (1 + idx / BUTTONS_COUNT)
  end
end