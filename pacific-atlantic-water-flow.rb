# https://leetcode.com/problems/pacific-atlantic-water-flow

# @param {Integer[][]} heights
# @return {Integer[][]}
def pacific_atlantic(heights)
    max_row = heights.size - 1
    max_col = heights[0].size - 1
    pac = Array.new(heights.size) { Array.new(heights[0].size, nil) }
    atl = Array.new(heights.size) { Array.new(heights[0].size, nil) }
    
    result = []

    (0..max_row).each do |row| 
        dfs(row, 0, pac, heights)
        dfs(row, max_col, atl, heights)
    end
    (0..max_col).each do |col| 
        dfs(0, col, pac, heights)
        dfs(max_row, col, atl, heights)
    end

    (0..max_row).each do |row| 
        (0..max_col).each do |col|
            result << [row, col] if atl[row][col] && pac[row][col]
        end
    end

    result
end

DIRS = [
        [-1,0],
        [1,0],
        [0,-1],
        [0,1],
    ]

def dfs(row, col, visited, heights)
    visited[row][col] = true

    DIRS.each do |dir|
        next_row = row + dir[0]
        next_col = col + dir[1]

        next if next_row < 0 || next_row == heights.size
        next if next_col < 0 || next_col == heights[0].size
        next if heights[row][col] > heights[next_row][next_col]
        next if visited[next_row][next_col]

        dfs(next_row, next_col, visited, heights)
    end
end

################################################################## AI

def pacific_atlantic(heights)
    return [] if heights.empty? || heights[0].empty?
    
    rows = heights.size
    cols = heights[0].size
    
    pac = Array.new(rows) { Array.new(cols, false) }
    atl = Array.new(rows) { Array.new(cols, false) }
    
    # DFS helper moving "uphill" from the ocean borders
    dfs = lambda do |r, c, visited|
        visited[r][c] = true
        dirs = [[-1, 0], [1, 0], [0, -1], [0, 1]]
        
        dirs.each do |dr, dc|
            nr, nc = r + dr, c + dc
            next if nr < 0 || nr >= rows || nc < 0 || nc >= cols
            next if visited[nr][nc]
            # Water flows downhill from neighbor to current, 
            # so moving backward means neighbor must be >= current height
            next if heights[nr][nc] < heights[r][c]
            
            dfs.call(nr, nc, visited)
        end
    end
    
    # 1. Start DFS from Pacific borders (Top row & Left column)
    (0...rows).each { |r| dfs.call(r, 0, pac) unless pac[r][0] }
    (0...cols).each { |c| dfs.call(0, c, pac) unless pac[0][c] }
    
    # 2. Start DFS from Atlantic borders (Bottom row & Right column)
    (0...rows).each { |r| dfs.call(r, cols - 1, atl) unless atl[r][cols - 1] }
    (0...cols).each { |c| dfs.call(rows - 1, c, atl) unless atl[rows - 1][c] }
    
    # 3. Collect cells that can reach both oceans
    result = []
    (0...rows).each do |r|
        (0...cols).each do |c|
            result << [r, c] if pac[r][c] && atl[r][c]
        end
    end
    
    result
end