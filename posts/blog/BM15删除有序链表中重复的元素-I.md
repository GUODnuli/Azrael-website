---
image_path: "/images/blog_images/leetcode/None.png"
title: BM15 删除有序链表中重复的元素-I
date: 2024-07-04
description: 每日一题：删除有序链表中重复的元素-I
project_link: none
tags:
  - learn
---
## 描述

删除给出链表中的重复元素（链表中元素从小到大有序），使链表中的所有元素都只出现一次

例如：

给出的链表为$1→1→2$﻿，返回$1→2$﻿

给出的链表为$1→1→2→3→3$﻿，返回$1→2→3$﻿

数据范围：链表长度满足$0≤n≤100$﻿，链表中任意节点的值满足$∣val∣≤100$﻿

进阶：空间复杂度_$O(1)$_﻿，时间复杂度_$O(n)$_﻿

## 示例1

```Plain
输入：{1,1,2}
返回值：{1,2}
```

## 示例2

```Plain
输入：{}
返回值：{}
```

## 解题思路

1. 设置一前一后两个指针，当后指针数值与前指针相等时更改前指针的后驱为后指针的后驱实现跳过相同数值的节点

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
# @return ListNode类
#
class Solution:
    def deleteDuplicates(self , head: ListNode) -> ListNode:
        if not head or not head.next:
            return head
        curr = head
        pos = head.next
        while pos:
            if curr.val == pos.val:
                curr.next, pos = pos.next, pos.next
            else:
                curr, pos = curr.next, pos.next
        return head
```