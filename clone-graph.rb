# https://leetcode.com/problems/clone-graph/

# Definition for a Node.
# class Node
#     attr_accessor :val, :neighbors
#     def initialize(val = 0, neighbors = nil)
#     @val = val
#     neighbors = [] if neighbors.nil?
#         @neighbors = neighbors
#     end
# end

# @param {Node} node
# @return {Node}
def cloneGraph(node)
    return unless node
    
    cloned = { node => Node.new(node.val) }
    queue = [node]

    while cur = queue.shift do
        cur.neighbors.each do |nei|
            unless cloned[nei]
                cloned[nei] = Node.new(nei.val)
                queue.push(nei)
            end

            cloned[cur].neighbors.push(cloned[nei])
        end

    end
    cloned[node]
end