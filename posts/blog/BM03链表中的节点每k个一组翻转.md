---
image_path: "/images/blog_images/leetcode/None.png"
title: BM03 链表中的节点每k个一组翻转
date: 2024-07-04
description: 每日一题：链表中的节点每k个一组翻转
project_link: none
tags:
  - learn
---
## **描述**

将给出的链表中的节点每$k$﻿个一组翻转，返回翻转后的链表如果链表中的节点数不是$k$﻿的倍数，将最后剩下的节点保持原样你不能更改节点中的值，只能更改节点本身。

数据范围： $0≤n≤2000$﻿ ，$1≤k≤2000$﻿ ，链表中每个元素都满足$0≤val≤1000$﻿

要求：空间复杂度_$O(1)$_﻿，时间复杂度_$O(n)$_﻿

例如：

给定的链表是 1→2→3→4→5

对于_$k=2$_﻿, 你应该返回 2→1→4→3→5

对于_$k=3$_﻿, 你应该返回 3→2→1→4→5

## 示例1

```Plain
输入：{1,2,3,4,5},2
返回值：{2,1,4,3,5}
```

## 示例2

```Plain
输入：{},1
返回值：{}
```

## 解题思路

### 明确问题与策略

1. 计算链表长度
2. 根据长度确定需要进行多少次完整的反转
3. 对每个k节点进行反转

### 翻转链表的一部分

为了简化问题，我们可以先编写一个辅助函数来每次翻转链表的k个节点。

这个函数可以接受链表的开始节点和结束节点，并返回翻转后的链表部分的开始节点和结束节点。

```Python
def reverse_partial(start: ListNode, end: ListNode) -> (ListNode, ListNode):
    prev, curr = None, start
    while curr != end:
        curr.next, prev, curr = prev, curr, curr.next
    return prev, start
```

### 整合进入主函数

现在我们使用**reverse_partial**函数逐步反转链表，每次翻转k个节点，直到没有足够的节点为止

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# @param head ListNode类 
# @param k int整型 
# @return ListNode类
#
class Solution:
    def reverseKGroup(self , head: ListNode, k: int) -> ListNode:
        def reverse_partial(start: ListNode, end: ListNode) -> (ListNode, ListNode):
            prev, curr = None, start
            while curr != end:
                curr.next, prev, curr = prev, curr, curr.next
            return prev, start
        
        if not head or k == 1:
            return head

        length = 0
        temp = head
        while temp:
            length += 1
            temp = temp.next
        
        if length < k:
            return head
        
        dummy = ListNode(0)
        dummy.next = head
        prev_group_end = dummy

        while length >= k:
            group_start = prev_group_end.next
            group_end = prev_group_end.next
            for _ in range(k):
                group_end = group_end.next
            
            new_start, new_end = reverse_partial(group_start, group_end)
            prev_group_end.next = new_start
            new_end.next = group_end
            prev_group_end = new_end
            length -= k
        
        return dummy.next
```

## 首次提交（该答案可读性较差且代码冗余过多）

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# @param head ListNode类 
# @param k int整型 
# @return ListNode类
#
class Solution:
    def reverseKGroup(self , head: ListNode, k: int) -> ListNode:
        cur = head
        len = 0
        while cur != None:
            len += 1
            cur = cur.next
            
        if len < k or len <= 1 or k == 1:
            return head

        if len == k:
            preNode = ListNode(1)
            nextNode = head.next
            while preNode != None:
                head.next = preNode.next
                preNode.next = head
                if nextNode != None:
                    head = nextNode
                    nextNode = nextNode.next
                else:
                    preNode = None

            return head
        
        totalgroup = int(len / k)
        groupcount = 0
        cur = head
        cur2 = cur.next
        pre = ListNode(None)
        pre.next = head
        pos = ListNode(None) 
        lastNode = ListNode(None)

        while groupcount < totalgroup:
            groupcount += 1

            count = 0
            while count < k:
                cur = cur.next
                count += 1
                if count == k - 1:
                    lastNode = cur
                if count == k:
                    pos = cur
                    pre.next.next = pos
                    cur = pre.next
                    pre.next = lastNode
                    pre = cur
            
            cashNode = None
            count = 0
            while count < k:
                if count < k - 1:
                    cashNode = cur2.next
                    cur2.next = cur
                    cur = cur2
                    cur2 = cashNode
                    if groupcount == 1:
                        head = cur
                else:
                    cur = cur2
                    cur2 = cur2.next
                count += 1
        
        return head
```