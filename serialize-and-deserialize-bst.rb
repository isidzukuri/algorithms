# https://leetcode.com/problems/serialize-and-deserialize-bst

# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# Encodes a tree to a single string.
#
# @param {TreeNode} root
# @return {string}
def serialize(root)
    stack = [root]
    result = []
    while node = stack.pop
        result << node.val

        if node.right
            stack << node.right
        end
        if node.left
            stack << node.left
        end
    end
    result
end

# Decodes your encoded data to tree.
#
# @param {string} data
# @return {TreeNode}
def deserialize(data)
    root = TreeNode.new(data[0])
    stack = [root]
    (1..(data.size-1)).each do |idx|
        val = data[idx]
        child = TreeNode.new(val)

        if val < stack[-1].val
            stack[-1].left = child
        else 
            while !stack.empty? && stack[-1].val < val
                last = stack.pop
            end
            last.right = child
        end
        stack << child
    end

    root
end


# Your functions will be called as such:
# deserialize(serialize(data))


################################################################ AI

# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# Encodes a tree to a single string/array.
def serialize(root)
    return [] unless root
    stack = [root]
    result = []
    
    while node = stack.pop
        result << node.val
        stack << node.right if node.right
        stack << node.left if node.left
    end
    
    result
end

# Decodes your encoded data to tree.
def deserialize(data)
    return nil if data.empty?
    
    root = TreeNode.new(data[0])
    stack = [root]
    
    (1...data.size).each do |idx|
        parent = stack[-1]
        child = TreeNode.new(data[idx])
        
        # Pop elements from the stack to find the correct parent node
        while !stack.empty? && stack[-1].val < child.val
            parent = stack.pop
        end
        
        # Attach the child based on BST properties
        if parent.val > child.val
            parent.left = child
        else
            parent.right = child
        end
        
        stack << child
    end
    
    root
end



################################################################### AI

# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val)
#         @val = val
#         @left, @right = nil, nil
#     end
# end

# Encodes a tree to a single string.
def serialize(root)
    return "" unless root
    res = []
    stack = [root]
    
    while node = stack.pop
        res << node.val
        stack << node.right if node.right
        stack << node.left if node.left
    end
    
    res
end

# Decodes your encoded data to tree.
def deserialize(data)
    return nil if data.empty?
    vals = data
    
    helper(vals, -Float::INFINITY, Float::INFINITY, [0])
end

private

def helper(vals, min_val, max_val, index)
    return nil if index[0] >= vals.size
    
    val = vals[index[0]]
    return nil if val < min_val || val > max_val
    
    index[0] += 1
    root = TreeNode.new(val)
    root.left = helper(vals, min_val, val, index)
    root.right = helper(vals, val, max_val, index)
    
    root
end