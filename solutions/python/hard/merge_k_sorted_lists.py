from typing import Optional


class ListNode:
    def __init__(self, val: int = 0, next: Optional[ListNode] = None):
        self.val = val
        self.next = next


def merge_two_lists(a: Optional[ListNode], b: Optional[ListNode]) -> Optional[ListNode]:
    dummy = ListNode(0)
    tail = dummy

    while a and b:
        if a.val <= b.val:
            tail.next = a
            a = a.next
        else:
            tail.next = b
            b = b.next
        tail = tail.next

    tail.next = a or b
    return dummy.next


def merge_k_lists(lists: list[Optional[ListNode]]) -> Optional[ListNode]:
    if not lists:
        return None

    # Repeatedly merge pairs (divide and conquer) until one list remains.
    interval = 1
    while interval < len(lists):
        for i in range(0, len(lists) - interval, interval * 2):
            lists[i] = merge_two_lists(lists[i], lists[i + interval])
        interval *= 2

    return lists[0]


def make_list(values: list[int]) -> Optional[ListNode]:
    head = None
    for value in reversed(values):
        head = ListNode(value, head)
    return head


def list_to_array(head: Optional[ListNode]) -> list[int]:
    result = []
    current = head
    while current:
        result.append(current.val)
        current = current.next
    return result


print(list_to_array(merge_k_lists([make_list([1, 4, 5]), make_list([1, 3, 4]), make_list([2, 6])])));  # [1,1,2,3,4,4,5,6]
print(list_to_array(merge_k_lists([])));  # []
print(list_to_array(merge_k_lists([make_list([])])));  # []
