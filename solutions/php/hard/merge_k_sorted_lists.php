<?php

// Merge K Sorted Lists — hard
// Merge k sorted linked lists into one sorted linked list.

class ListNode
{
    public function __construct(public int $val = 0, public ?ListNode $next = null)
    {
    }
}

function mergeTwoLists(?ListNode $a, ?ListNode $b): ?ListNode
{
    $dummy = new ListNode(0);
    $tail = $dummy;

    while ($a !== null && $b !== null) {
        if ($a->val <= $b->val) {
            $tail->next = $a;
            $a = $a->next;
        } else {
            $tail->next = $b;
            $b = $b->next;
        }
        $tail = $tail->next;
    }

    $tail->next = $a ?? $b;
    return $dummy->next;
}

function mergeKLists(array $lists): ?ListNode
{
    if (count($lists) === 0) {
        return null;
    }

    // Repeatedly merge pairs (divide and conquer) until one list remains.
    $interval = 1;
    $n = count($lists);
    while ($interval < $n) {
        for ($i = 0; $i + $interval < $n; $i += $interval * 2) {
            $lists[$i] = mergeTwoLists($lists[$i], $lists[$i + $interval]);
        }
        $interval *= 2;
    }

    return $lists[0];
}

// Helpers for the quick checks.
function makeList(array $values): ?ListNode
{
    $head = null;
    for ($i = count($values) - 1; $i >= 0; $i--) {
        $head = new ListNode($values[$i], $head);
    }
    return $head;
}

function listToArray(?ListNode $head): array
{
    $result = [];
    $current = $head;
    while ($current !== null) {
        $result[] = $current->val;
        $current = $current->next;
    }
    return $result;
}

// Quick checks
var_dump(listToArray(mergeKLists([makeList([1, 4, 5]), makeList([1, 3, 4]), makeList([2, 6])]))); // [1,1,2,3,4,4,5,6]
var_dump(listToArray(mergeKLists([]))); // []
var_dump(listToArray(mergeKLists([makeList([])]))); // []
