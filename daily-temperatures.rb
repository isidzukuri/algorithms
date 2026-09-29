# https://leetcode.com/problems/daily-temperatures

# @param {Integer[]} temperatures
# @return {Integer[]}
def daily_temperatures(temperatures)
    len = temperatures.size
    result = Array.new(len, 0)
    stack = []
    (len-1).downto(0) do |idx|
        while last_idx = stack.last
            if temperatures[last_idx] <= temperatures[idx]
                stack.pop
            else
                result[idx] = last_idx - idx
                break
            end
        end
        stack << idx
    end
    result
end


############################################### AI

# @param {Integer[]} temperatures
# @return {Integer[]}
def daily_temperatures(temperatures)
    n = temperatures.size
    result = Array.new(n, 0)
    stack = [] # Store indices instead of values

    (n - 1).downto(0) do |i|
        # Pop indices from the stack while the stack's temperature is less than or equal to the current temperature
        while !stack.empty? && temperatures[stack.last] <= temperatures[i]
            stack.pop
        end

        # If the stack is not empty, the top of the stack is the index of the next warmer day
        result[i] = stack.last - i unless stack.empty?

        # Push the current index onto the stack
        stack.push(i)
    end

    result
end


############################################### AI

# @param {Integer[]} temperatures
# @return {Integer[]}
def daily_temperatures(temperatures)
    n = temperatures.size
    result = Array.new(n, 0)
    
    # Temperatures are typically between 30 and 100.
    # next_warm[t] stores the index of the closest future day with temperature t.
    next_warm = Array.new(101, Float::INFINITY)

    (n - 1).downto(0) do |i|
        current_temp = temperatures[i]
        
        # Find the earliest index among all temperatures strictly greater than current_temp
        closest_idx = Float::INFINITY
        (current_temp + 1).upto(100) do |warmer_temp|
            closest_idx = [closest_idx, next_warm[warmer_temp]].min
        end

        # If a warmer day was found, calculate the distance
        result[i] = closest_idx == Float::INFINITY ? 0 : closest_idx - i
        
        # Update the latest seen index for the current temperature
        next_warm[current_temp] = i
    end

    result
end