---
image_path: "/images/blog_images/leetcode/None.png"
title: BM05 合并k个已排序的链表
date: 2024-07-04
description: 每日一题：合并k个已排序的链表
project_link: none
tags:
  - learn
---
## 描述

合并$k$﻿个升序的链表并将结果作为一个升序的链表返回其头节点。

数据范围：节点总数 $0≤n≤5000$﻿，每个节点的val满足$∣val∣<=1000$﻿

要求：时间复杂度_$O(nlogn)$_﻿

## 示例1

```Plain
输入：[{1,2,3},{4,5,6,7}]
返回值：{1,2,3,4,5,6,7}
```

## 示例2

```Plain
输入：[{1,2},{1,4,5},{6}]
返回值：{1,1,2,4,5,6}
```

## 解题思路

### 明确问题与步骤

1. 使用冒泡排序在lists中寻找最小值节点，将其放置于列表首位
2. 与[[BM04合并两个排序的链表]]相同，使副链表逐步合并进入主链表
3. 异常场景考虑：空节点、两链表不同长度

```Python
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# 
# @param lists ListNode类一维数组 
# @return ListNode类
#
class Solution:
    def mergeKLists(self , lists: List[ListNode]) -> ListNode:
        len = lists.__len__()
        if not lists:
            return None
        if len == 1:
            return lists[0]

        index = 0
        while not lists[0]:
            index += 1
            if lists[index]:
                lists[0], lists[index] = lists[index], lists[0]
            if index == len - 1 and not lists[0]:
                return None

        for i in list(range(len)):
            if not lists[i]:
                pass
            elif lists[i].val < lists[0].val:
                lists[i], lists[0] = lists[0], lists[i]

        cur = lists[0]
        head = lists[0]
        pre = ListNode(0)
        pre.next = cur

        for i in lists[1:]:
            if not i:
                continue
                
            addNode = i
            while cur and addNode:
                if cur.val > addNode.val:
                    pre.next, addNode.next, pre, addNode = addNode, cur, addNode, addNode.next 
                else:
                    cur, pre = cur.next, cur
            
            if addNode:
                pre.next = addNode
            
            cur = head
        
        return head
```