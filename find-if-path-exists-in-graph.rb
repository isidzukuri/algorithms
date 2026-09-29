# https://leetcode.com/problems/find-if-path-exists-in-graph


# @param {Integer} n
# @param {Integer[][]} edges
# @param {Integer} source
# @param {Integer} destination
# @return {Boolean}
def valid_path(n, edges, source, destination)
    visited = Array.new(n, false)
    neighbors = Array.new(n) { [] }
    stack = [source]

    edges.each do |edge|
        neighbors[edge[0]] << edge[1]
        neighbors[edge[1]] << edge[0]
    end
    
    while node = stack.pop do
        return true if node == destination
        
        next if visited[node]

        visited[node] = true

        stack.push(*neighbors[node])
    end
    false
end


# @param {Integer} n
# @param {Integer[][]} edges
# @param {Integer} source
# @param {Integer} destination
# @return {Boolean}
def valid_path(n, edges, source, destination)
    visited = Array.new(n, false)
    neighbors = {}
    stack = [source]

    edges.each do |edge|
        neighbors[edge[0]] ||= []
        neighbors[edge[1]] ||= []
        neighbors[edge[0]] << edge[1]
        neighbors[edge[1]] << edge[0]
    end
    
    while node = stack.pop do
        return true if node == destination
        
        next if visited[node]

        visited[node] = true

        stack.concat(neighbors[node]) if neighbors[node]
    end
    false
end

# @param {Integer} n
# @param {Integer[][]} edges
# @param {Integer} source
# @param {Integer} destination
# @return {Boolean}
def valid_path(n, edges, source, destination)
    visited = []
    neighbors = {}
    stack = [source]

    edges.each do |edge|
        neighbors[edge[0]] ||= []
        neighbors[edge[1]] ||= []
        neighbors[edge[0]] << edge[1]
        neighbors[edge[1]] << edge[0]
    end
    
    while node = stack.pop do
        return true if node == destination
        
        next if visited.include?(node)

        visited << node

        stack.concat(neighbors[node]) if neighbors[node]
    end
    false
end


# @param {Integer} n
# @param {Integer[][]} edges
# @param {Integer} source
# @param {Integer} destination
# @return {Boolean}
def valid_path(n, edges, source, destination)
    visited = []
    matrix = Array.new(n) { Array.new(n, false) }
    stack = [source]

    edges.each do |edge|
        matrix[edge[0]][edge[1]] = true 
        matrix[edge[1]][edge[0]] = true 
    end
    
    while node = stack.pop do
        return true if node == destination
        
        next if visited.include?(node)

        visited << node

        matrix[node].each.with_index do |neighbor, idx|
            stack.push(idx) if neighbor
        end
    end
    false
end