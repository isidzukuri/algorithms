# https://leetcode.com/problems/merge-intervals

# @param {Integer[][]} intervals
# @return {Integer[][]}
def merge(intervals)
    intervals.sort_by!{|item| item[0]}

    result = [intervals.first]

    intervals[1..-1].each do |cur|
        last_merged = result.last

        if cur[0] <= last_merged[1]
            last_merged[1] = [last_merged[1], cur[1]].max
        else
            result << cur
        end
    end
    result
end
