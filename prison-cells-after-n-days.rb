# https://leetcode.com/problems/prison-cells-after-n-days

# @param {Integer[]} cells
# @param {Integer} n
# @return {Integer[]}
def prison_after_n_days(cells, n)
    rep = n 
    if rep > 21
        rep = n % 14 + 14
    end
    
    rep.times do
        new_state = Array.new(8)
        
        1.upto(cells.size-2).each do |cur|
            sum = cells[cur - 1] + cells[cur + 1]
            if sum == 0 || sum == 2
                new_state[cur] = 1
            else
                new_state[cur] = 0
            end
        end
        new_state[0] = 0
        new_state[7] = 0
        cells = new_state
    end
    cells
end


############################################################ AI

def prison_after_n_days(cells, n)
    seen = {}

    n.times do |day|
        # Convert the array to a string key so we can store it in a hash map
        state_key = cells.join

        # If we have seen this exact layout before, a cycle has been found!
        if seen.key?(state_key)
            cycle_length = day - seen[state_key]
            
            # Use modulo arithmetic to skip all the redundant loops
            remaining_days = (n - day) % cycle_length
            remaining_days.times { cells = compute_next_day(cells) }
            
            return cells
        end

        # Record the day we first saw this state
        seen[state_key] = day
        cells = compute_next_day(cells)
    end

    cells
end

def compute_next_day(cells)
    new_state = Array.new(8, 0)
    1.upto(6) do |i|
        new_state[i] = 1 if cells[i - 1] == cells[i + 1]
    end
    new_state
end