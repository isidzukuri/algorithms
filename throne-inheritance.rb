# https://leetcode.com/problems/throne-inheritance

class ThroneInheritance
    Node = Struct.new(:name, :children, :dead)
=begin
    :type king_name: String
=end
    def initialize(king_name)
        @root = Node.new(king_name, [])
        @nodes = {}
        @nodes[king_name] = @root
    end


=begin
    :type parent_name: String
    :type child_name: String
    :rtype: Void
=end
    def birth(parent_name, child_name)
        child = Node.new(child_name, [])
        @nodes[child_name] = child
        @nodes[parent_name].children << child
    end


=begin
    :type name: String
    :rtype: Void
=end
    def death(name)
        @nodes[name].dead = true 
    end


=begin
    :rtype: String[]
=end
    def get_inheritance_order()
        stack = [@root]
        result = []

        while cur = stack.pop
            result << cur.name unless cur.dead

            cur.children.reverse_each do |child|
                stack << child
            end
        end
        result
    end


end

# Your ThroneInheritance object will be instantiated and called as such:
# obj = ThroneInheritance.new(king_name)
# obj.birth(parent_name, child_name)
# obj.death(name)
# param_3 = obj.get_inheritance_order()


############################################################################### V2

class ThroneInheritance
    # Node = Struct.new(:name, :children, :dead)
=begin
    :type king_name: String
=end
    def initialize(king_name)
        @root = [king_name, [], false]
        @nodes = {}
        @nodes[king_name] = @root
    end


=begin
    :type parent_name: String
    :type child_name: String
    :rtype: Void
=end
    def birth(parent_name, child_name)
        child = [child_name, [], false]
        @nodes[child_name] = child
        @nodes[parent_name][1] << child
    end


=begin
    :type name: String
    :rtype: Void
=end
    def death(name)
        @nodes[name][2] = true 
    end


=begin
    :rtype: String[]
=end
    def get_inheritance_order()
        stack = [@root]
        result = []

        while cur = stack.pop
            result << cur[0] unless cur[2]

            cur[1].reverse_each do |child|
                stack << child
            end
        end
        result
    end


end

# Your ThroneInheritance object will be instantiated and called as such:
# obj = ThroneInheritance.new(king_name)
# obj.birth(parent_name, child_name)
# obj.death(name)
# param_3 = obj.get_inheritance_order()


####################################################################### AI

require 'set'

class ThroneInheritance
=begin
    :type king_name: String
=end
    def initialize(king_name)
        @king = king_name
        @children = Hash.new { |h, k| h[k] = [] }
        @dead = Set.new
    end

=begin
    :type parent_name: String
    :type child_name: String
    :rtype: Void
=end
    def birth(parent_name, child_name)
        @children[parent_name] << child_name
    end

=begin
    :type name: String
    :rtype: Void
=end
    def death(name)
        @dead.add(name)
    end

=begin
    :rtype: String[]
=end
    def get_inheritance_order()
        result = []
        dfs(@king, result)
        result
    end

    private

    def dfs(curr, result)
        # Include person in succession only if they are alive
        result << curr unless @dead.include?(curr)
        
        # Recursively visit all children in birth order
        @children[curr].each do |child|
            dfs(child, result)
        end
    end
end