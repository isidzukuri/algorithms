# https://leetcode.com/problems/design-neighbor-sum-service

class NeighborSum

    attr_reader :grid

=begin
    :type grid: Integer[][]
=end
    def initialize(grid)
        @limit = grid.size - 1
        @grid = grid
        @coords = {}

        grid.each_with_index do |row, r_id|
            row.each_with_index do |val, c_id|
                @coords[val] = [r_id, c_id]
            end
        end
    end


=begin
    :type value: Integer
    :rtype: Integer
=end
    def adjacent_sum(value)
        cur = @coords[value]

        sum = 0

        sum += grid[cur[0] - 1][cur[1]] if cur[0] > 0
        sum += grid[cur[0] + 1][cur[1]] if cur[0] < @limit

        sum += grid[cur[0]][cur[1] - 1] if cur[1] > 0
        sum += grid[cur[0]][cur[1] + 1] if cur[1] < @limit

        sum
    end


=begin
    :type value: Integer
    :rtype: Integer
=end
    def diagonal_sum(value)
        cur = @coords[value]

        sum = 0

        sum += grid[cur[0] - 1][cur[1] - 1] if cur[0] > 0 && cur[1] > 0
        sum += grid[cur[0] + 1][cur[1] + 1] if cur[0] < @limit && cur[1] < @limit

        sum += grid[cur[0] - 1][cur[1] + 1] if cur[0] > 0 && cur[1] < @limit
        sum += grid[cur[0] + 1][cur[1] - 1] if cur[0] < @limit && cur[1] > 0

        sum
    end


end

# Your NeighborSum object will be instantiated and called as such:
# obj = NeighborSum.new(grid)
# param_1 = obj.adjacent_sum(value)
# param_2 = obj.diagonal_sum(value)



