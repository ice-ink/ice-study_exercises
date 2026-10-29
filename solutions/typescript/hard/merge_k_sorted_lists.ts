// Merge K Sorted Lists — hard
// Merge k sorted linked lists into one sorted linked list.

class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val: number = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

function mergeTwoLists(a: ListNode | null, b: ListNode | null): ListNode | null {
  const dummy = new ListNode(0);
  let tail: ListNode = dummy;

  while (a !== null && b !== null) {
    if (a.val <= b.val) {
      tail.next = a;
      a = a.next;
    } else {
      tail.next = b;
      b = b.next;
    }
    tail = tail.next!;
  }

  tail.next = a ?? b;
  return dummy.next;
}

function mergeKLists(lists: Array<ListNode | null>): ListNode | null {
  if (lists.length === 0) return null;

  // Repeatedly merge pairs (divide and conquer) until one list remains.
  let interval = 1;
  while (interval < lists.length) {
    for (let i = 0; i + interval < lists.length; i += interval * 2) {
      lists[i] = mergeTwoLists(lists[i]!, lists[i + interval]!);
    }
    interval *= 2;
  }

  return lists[0]!;
}

// Quick checks
function makeList(values: number[]): ListNode | null {
  let head: ListNode | null = null;
  for (let i = values.length - 1; i >= 0; i--) {
    head = new ListNode(values[i]!, head);
  }
  return head;
}

function listToArray(head: ListNode | null): number[] {
  const result: number[] = [];
  let current = head;
  while (current !== null) {
    result.push(current.val);
    current = current.next;
  }
  return result;
}

console.log(listToArray(mergeKLists([makeList([1, 4, 5]), makeList([1, 3, 4]), makeList([2, 6])]))); // [1,1,2,3,4,4,5,6]
console.log(listToArray(mergeKLists([]))); // []
console.log(listToArray(mergeKLists([makeList([])]))); // []
