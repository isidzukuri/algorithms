# https://leetcode.com/problems/path-sum

# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
# @param {TreeNode} root
# @param {Integer} target_sum
# @return {Boolean}
def has_path_sum(root, target_sum)
    return false unless root
    stack = [[root, target_sum]]

    while item = stack.pop do
        sum = item[1] - item[0].val

        if sum == 0 && item[0].left.nil? && item[0].right.nil?
            return true
        end

        if item[0].left
            stack << [item[0].left, sum]
        end
        
        if item[0].right
            stack << [item[0].right, sum]
        end
    end
    false
end





def has_path_sum(root, target_sum)
    return false unless root

    sum = target_sum - root.val
    return true if sum == 0 && root.left.nil? && root.right.nil?

    if root.left
        return true if has_path_sum(root.left, sum)
    end

    if root.right
        return true if has_path_sum(root.right, sum)
    end
    false
end
