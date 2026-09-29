# https://leetcode.com/problems/insert-interval

# @param {Integer[][]} intervals
# @param {Integer[]} new_interval
# @return {Integer[][]}
def insert(intervals, new_interval)
    result = []
    cur_n = 0

    while cur_n < intervals.count && intervals[cur_n][1] < new_interval[0]
        result << intervals[cur_n]
        cur_n +=1
    end

    while cur_n < intervals.count && intervals[cur_n][0] <= new_interval[1]
        new_interval[0] = [new_interval[0], intervals[cur_n][0]].min
        new_interval[1] = [new_interval[1], intervals[cur_n][1]].max
        cur_n +=1
    end

    result << new_interval
    result += intervals[cur_n..]
    result
end