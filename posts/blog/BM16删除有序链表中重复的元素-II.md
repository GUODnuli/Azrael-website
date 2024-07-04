---
image_path: "/images/blog_images/leetcode/None.png"
title: BM16 删除有序链表中重复的元素-II
date: 2024-07-04
description: 每日一题：删除有序链表中重复的元素-II
project_link: none
tags:
  - learn
---
## 描述

给出一个升序排序的链表，删除链表中的所有重复出现的元素，只保留原链表中只出现一次的元素。

例如：

给出的链表为$1→2→3→3→4→4→5$﻿

返回$1→2→5$﻿

给出的链表为$1→1→1→2→3$﻿

返回$2→3$﻿

数据范围：链表长度$0≤n≤10000$﻿，链表中的值满足$∣val∣≤1000$﻿

要求：空间复杂度_$O(n)$_﻿，时间复杂度_$O(n)$_﻿

进阶：空间复杂度_$O(1)$_﻿，时间复杂度_$O(n)$_﻿

## 示例1

```Plain
输入：{1,2,2}
返回值：{1}
```

## 示例2

```Plain
输入：{}
返回值：{}
```

## 解题思路

1. 设置三个节点，`prev`跳过区间的前节点，`start`跳过区间头节点，`end`跳过区间尾节点
2. 当头节点值与尾节点值相等时找到重复值的尾部，调整`prev`指向`end`后一个节点并后移`start`和`end`

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
# 
# @param head ListNode类 
# @return ListNode类
#
class Solution:
    def deleteDuplicates(self , head: ListNode) -> ListNode:
        if not head:
            return head

        dummyNode = ListNode(0)
        prev = dummyNode
        prev.next = head
        start = head
        end = start.next
        while end:
            if start.val == end.val:
                while end.next and end.val == end.next.val:
                    end = end.next
                prev.next = end.next
                if not prev.next:
                    return dummyNode.next
                start = prev.next
                end = start.next
            else:
                prev = prev.next
                end = end.next
                start = start.next
            
        return dummyNode.next
```