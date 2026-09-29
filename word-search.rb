# https://leetcode.com/problems/word-search

# @param {Character[][]} board
# @param {String} word
# @return {Boolean}
def exist(board, word)
    rows = board.length
    cols = board[0].length
    board_counts = Hash.new(0)
    (0...rows).each { |y| (0...cols).each { |x| board_counts[board[y][x]] += 1 } }

    chars = word.chars
    word_counts = Hash.new(0)
    chars.each { |c| word_counts[c] += 1 }

    # Ensure all required characters exist in sufficient quantities
    word_counts.each { |char, count| return false if board_counts[char] < count }

    visited = Array.new(board.length) { Array.new(board[0].length, false) }

    (0...board.size).each do |by|
        (0...board[0].size).each do |bx|
            next unless board[by][bx] == chars[0]

            return true if dfs(bx, by, 0, board, chars, visited)
        end
    end
    false
end

def dfs(x, y, char_idx, board, chars, visited)
    return false if x < 0 || y < 0 || x == board[0].count || y == board.count || visited[y][x]

    cur_val = board[y][x]

    return false unless cur_val == chars[char_idx]
    next_idx = char_idx + 1

    return true if next_idx == chars.count
    
    visited[y][x] = true

    found = dfs(x-1, y, next_idx, board, chars, visited) ||
            dfs(x+1, y, next_idx, board, chars, visited) ||
            dfs(x, y-1, next_idx, board, chars, visited) ||
            dfs(x, y+1, next_idx, board, chars, visited)

    visited[y][x] = false
    found
end