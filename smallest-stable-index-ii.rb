# https://leetcode.com/problems/smallest-stable-index-ii

# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def first_stable_index(nums, k)
    min_index = -1
    max_n = nums.size() -1
    mins = Array.new(nums.count, 0)
    mins[max_n] = nums[max_n]
    (max_n - 1).downto(0).each do |index|
        fst = mins[index + 1]
        scd = nums[index]
        mins[index] = fst < scd ? fst : scd
    end
    max = nums[0]
    (0...nums.count).each do |index|
        max = nums[index] if max < nums[index]
        score = max - mins[index]
        if score <= k
            min_index = index
            break
        end
    end    

    min_index
end



############################################## AI


# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def first_stable_index(nums, k)
  n = nums.size
  return -1 if n.zero?

  # 1. Build suffix minimums array in-place
  mins = nums.dup
  (n - 2).downto(0) do |i|
    mins[i] = mins[i + 1] if mins[i + 1] < mins[i]
  end

  # 2. Track running prefix max and return the first valid index
  max_prefix = nums[0]
  (0...n).each do |i|
    max_prefix = nums[i] if nums[i] > max_prefix
    return i if (max_prefix - mins[i]) <= k
  end

  -1
end