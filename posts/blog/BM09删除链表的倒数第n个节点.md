---
image_path: "/images/blog_images/leetcode/None.png"
title: BM09 删除链表的倒数第n个节点
date: 2024-07-04
description: 每日一题：删除链表的倒数第n个节点
project_link: none
tags:
  - learn
---
## 描述

给定一个链表，删除链表的倒数第$n$﻿个节点并返回链表的头指针例如，

给出的链表为:$1→2→3→4→5$﻿, _$n=2$_﻿.

删除了链表的倒数第_$n$_﻿个节点之后,链表变为$1→2→3→5$﻿.

数据范围：链表长度$0≤n≤1000$﻿，链表中任意节点的值满足$0≤val≤100$﻿

要求：空间复杂度_$O(1)$_﻿，时间复杂度_$O(n)$_﻿

备注：题目保证_$n$_﻿一定是有效的

## 示例1

```Plain
输入：{1,2},2
返回值：{2}
```

## 解题思路

1. 循环取得链表长度$len$﻿后再次循环找到倒数第n个节点，修改前节点后驱即可

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# 
# @param head ListNode类 
# @param n int整型 
# @return ListNode类
#
class Solution:
    def removeNthFromEnd(self , head: ListNode, n: int) -> ListNode:
        if not head:
            return None
        if n == 0:
            return head
        
        curr = head
        len = 0
        while curr:
            len += 1
            curr = curr.next
        if len == n:
            return head.next

        pre = ListNode(0)
        pre.next = head
        curr = head
        for i in list(range(len - n)):
            curr, pre = curr.next, pre.next
        pre.next, curr.next = curr.next, None
        
        return head
```