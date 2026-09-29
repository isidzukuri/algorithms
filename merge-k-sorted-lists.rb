# https://leetcode.com/problems/merge-k-sorted-lists/

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode[]} lists
# @return {ListNode}
def merge_k_lists(lists)
  vals = []

  lists.each do |list|
    curr = list
    while curr
      vals << curr.val
      curr = curr.next
    end
  end

  vals.sort!

  head = ListNode.new(0)
  curr = head
  vals.each do |v|
    curr.next = ListNode.new(v)
    curr = curr.next
  end

  head.next
end




def merge_k_lists(lists)
    return lists if lists.empty?

    head = lists[0]

    until lists[1..].count(nil) == lists.count - 1 do 
        idx = 1
        
        while idx < lists.count do
            node = lists[idx]
            if node.nil? 
                idx +=1
                next
            end

            if head.nil?
                head = node
                lists[0] = head
                lists[idx] = nil 

                idx +=1
                next
            end


            if head.val == node.val
                head_next = head.next
                node_next = node.next
                node.next = head_next
                head.next = node
                lists[idx] = node_next

                idx +=1
                next
            end


            if head.val > node.val
                node_next = node.next
                node.next = head
                head = node
                lists[0] = head
                lists[idx] = node_next

                idx +=1
                next
            end

            if head.val < node.val
                nxt = head
                tmp_head = head
                loop do 
                    if nxt == nil
                        node_next = node.next
                        lists[idx] = node_next
                        tmp_head.next = node
                        node.next = nil
                        break
                    elsif nxt.val >= node.val
                        node_next = node.next
                        lists[idx] = node_next

                        node.next = nxt
                        tmp_head.next = node

                        break
                    end
                    tmp_head = nxt
                    nxt = nxt.next
                end

                idx +=1
                next
            end
        end
    end


    lists[0]
end


###################################################################### AI

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end

# @param {ListNode[]} lists
# @return {ListNode}
def merge_k_lists(lists)
  return nil if lists.nil? || lists.empty?

  while lists.length > 1
    merged_lists = []

    # Merge lists in pairs
    (0...lists.length).step(2) do |i|
      l1 = lists[i]
      l2 = (i + 1 < lists.length) ? lists[i + 1] : nil
      merged_lists << merge_two_lists(l1, l2)
    end

    lists = merged_lists
  end

  lists[0]
end

# Helper method to merge two sorted linked lists
def merge_two_lists(l1, l2)
  dummy = ListNode.new(0)
  tail = dummy

  while l1 && l2
    if l1.val < l2.val
      tail.next = l1
      l1 = l1.next
    else
      tail.next = l2
      l2 = l2.next
    end
    tail = tail.next
  end

  tail.next = l1 || l2
  dummy.next
end