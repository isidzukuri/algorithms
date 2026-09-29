# https://leetcode.com/problems/first-unique-character-in-a-string

# @param {String} s
# @return {Integer}
def first_uniq_char(s)
    bytes = s.bytes
    counts = bytes.tally
    byte = counts.find {|key, val| val == 1 }&.first
    bytes.index(byte) || -1
end


# @param {String} s
# @return {Integer}
def first_uniq_char(s)
    counts = s.chars.tally
    char = counts.find {|key, val| val == 1 }&.first
    char ? s.index(char) : -1
end


# AI

# @param {String} s
# @return {Integer}
def first_uniq_char(s)
  counts = Hash.new(0)
  codepoints = s.codepoints # Returns integer array of Unicode code points

  # Step 1: Count integer codepoints (Faster than string keys in Hash)
  codepoints.each { |cp| counts[cp] += 1 }

  # Step 2: Find the first codepoint with count == 1
  codepoints.each_with_index do |cp, i|
    return i if counts[cp] == 1
  end

  -1
end


# @param {String} s
# @return {Integer}
def first_uniq_char(s)
  counts = Array.new(256, 0)
  len = s.bytesize

  # Step 1: Count frequencies using getbyte (no array allocations)
  0.upto(len - 1) do |i|
    counts[s.getbyte(i)] += 1
  end

  # Step 2: Return first index with count == 1
  0.upto(len - 1) do |i|
    return i if counts[s.getbyte(i)] == 1
  end

  -1
end