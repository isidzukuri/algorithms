# https://leetcode.com/problems/binary-search-tree-iterator

# Definition for a binary tree node.
# class TreeNode
#     attr_accessor :val, :left, :right
#     def initialize(val = 0, left = nil, right = nil)
#         @val = val
#         @left = left
#         @right = right
#     end
# end
class BSTIterator

=begin
    :type root: TreeNode
=end
    def initialize(root)
        @stack = []
        push_left(root)
    end


=begin
    :rtype: Integer
=end
    def next()
        node = @stack.pop
        push_left(node.right) if node.right
        node.val
    end


=begin
    :rtype: Boolean
=end
    def has_next()
        !@stack.empty?
    end

    private

    def push_left(node)
        while node
            @stack << node
            node = node.left
        end
    end

end

# Your BSTIterator object will be instantiated and called as such:
# obj = BSTIterator.new(root)
# param_1 = obj.next()
# param_2 = obj.has_next()