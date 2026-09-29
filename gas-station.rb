# https://leetcode.com/problems/gas-station/

# @param {Integer[]} gas
# @param {Integer[]} cost
# @return {Integer}
def can_complete_circuit(gas, cost)
    return -1 if gas.sum < cost.sum

    start = 0
    tank = 0

    gas.each_with_index do |gs, idx|
        tank += gs - cost[idx]

        if tank < 0
            start = idx + 1
            tank = 0 
        end
    end
    start
end