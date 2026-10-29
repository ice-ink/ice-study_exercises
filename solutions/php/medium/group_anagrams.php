<?php
// Group Anagrams — medium
// Group strings that are anagrams of each other.

function groupAnagrams(array $strs): array
{
    $groups = [];

    foreach ($strs as $str) {
        $chars = str_split($str);
        sort($chars);
        $key = implode('', $chars);
        $groups[$key][] = $str;
    }

    return array_values($groups);
}

// Quick checks
var_dump(groupAnagrams(["eat", "tea", "tan", "ate", "nat", "bat"]));
// [["eat", "tea", "ate"], ["tan", "nat"], ["bat"]]
var_dump(groupAnagrams([""]));  // [[""]]
var_dump(groupAnagrams(["a"])); // [["a"]]
var_dump(groupAnagrams(["ab", "ba", "abc"])); // [["ab", "ba"], ["abc"]]
