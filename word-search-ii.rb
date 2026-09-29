# https://leetcode.com/problems/word-search-ii

DIRECTIONS = [[1,0], [-1,0], [0,1], [0,-1]] # [x, y]

class Trie
    attr_accessor :children, :word
    
    def initialize()
        @children = {}
        @word = nil
    end

    def insert(chars)
        cur = self
        chars.each do |ch|
            cur = cur.children[ch] ||= Trie.new
        end
        cur.word = chars
    end

    def search_chars(chars)
        cur = self
        
        chars.each do |ch|
            cur = cur.children[ch]
            return false unless cur
        end
        cur.word == chars
    end
end

# @param {Character[][]} board
# @param {String[]} words
# @return {String[]}
def find_words(board, words)
    dict = Trie.new

    words.each do |word|
        dict.insert(word.chars)
    end
    result = []

    (0...board.length).each do |y|
        (0...board[0].length).each do |x|
            dfs(board, dict, x, y, result)
            if result.size == words.size 
                return result
            end
        end
    end
    result
end

def dfs(board, dict, node_x, node_y, result)
    ch = board[node_y][node_x]
    return unless cur_dict = dict.children[ch]

    if cur_dict.word 
        word = cur_dict.word.join
        cur_dict.word = nil 
        result << word
    end
    
    board[node_y][node_x] = '#' 

    DIRECTIONS.each do |dir|
        nei_x = node_x + dir[0]
        nei_y = node_y + dir[1]

        next if nei_y < 0 || nei_y >= board.size
        next if nei_x < 0 || nei_x >= board[0].size
        next if board[nei_y][nei_x] == '#'
        
        dfs(board, cur_dict, nei_x, nei_y, result)
    end
    
    board[node_y][node_x] = ch
    if cur_dict.children.empty? && cur_dict.word.nil?
        dict.children.delete(ch)
    end
    nil
end




################################################### AI

class TrieNode
  attr_accessor :children, :word

  def initialize
    @children = {}
    @word = nil
  end
end

DIRS = [[-1, 0], [1, 0], [0, -1], [0, 1]] # [dy, dx]

# @param {Character[][]} board
# @param {String[]} words
# @return {String[]}
def find_words(board, words)
  root = TrieNode.new

  # 1. Build Trie from input words
  words.each do |word|
    node = root
    word.each_char do |char|
      node.children[char] ||= TrieNode.new
      node = node.children[char]
    end
    node.word = word
  end

  result = []
  rows = board.length
  cols = board[0].length

  # 2. Search board using Trie
  (0...rows).each do |y|
    (0...cols).each do |x|
      dfs(board, y, x, root, result)
    end
  end

  result
end

def dfs(board, y, x, parent, result)
  char = board[y][x]
  curr_node = parent.children[char]

  # Return if character isn't in current Trie path
  return unless curr_node

  # If a word is matched, add to results and clear to prevent duplicate finds
  if curr_node.word
    result << curr_node.word
    curr_node.word = nil
  end

  # Mark visited in-place
  board[y][x] = '#'

  DIRS.each do |dy, dx|
    ny = y + dy
    nx = x + dx

    next if ny < 0 || ny >= board.length || nx < 0 || nx >= board[0].length
    next if board[ny][nx] == '#'

    dfs(board, ny, nx, curr_node, result)
  end

  # Backtrack in-place
  board[y][x] = char
end