# https://leetcode.com/problems/reverse-linked-list/

# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @return {ListNode}
def reverse_list(head)
    current = head
    prev = nil
    nxt = nil

    while current do
        nxt = current.next
        current.next = prev
        break if nxt.nil?
        prev = current
        current = nxt
    end
    current
end



def reverse_list(head)
    current = head
    prev = nil
    while current
        tmp = current.next
        current.next = prev
        prev = current
        current = tmp
    end
    prev
end