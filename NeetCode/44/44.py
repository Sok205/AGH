class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        n = len(lists)

        if not n:
            return None

        if n == 1:
            return lists[0]

        def sort_list_of_nodes(l1: ListNode, l2: ListNode) -> ListNode:
            """
            Return a sorted list of two ListNode objects
            """
            dummy = ListNode()
            res = dummy
            while l1 and l2:
                if l1.val < l2.val:
                    dummy.next = l1
                    l1 = l1.next
                else:
                    dummy.next = l2
                    l2 = l2.next
                dummy = dummy.next

            while l1:
                dummy.next = l1
                l1 = l1.next
                dummy = dummy.next

            while l2:
                dummy.next = l2
                l2 = l2.next
                dummy = dummy.next

            return res.next

        res = lists.pop()

        for i in lists:
            print(res)
            res = sort_list_of_nodes(res, i)

        return res