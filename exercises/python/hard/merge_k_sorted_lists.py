# Merge k Sorted Lists — hard
# Merge k sorted linked lists into one sorted linked list.

from typing import Optional


class ListNode:
    def __init__(self, val: int = 0, next: Optional["ListNode"] = None) -> None:
        self.val = val
        self.next = next


def merge_k_lists(lists: list[Optional[ListNode]]) -> Optional[ListNode]:
    raise NotImplementedError("TODO: implement")
