---
image_path: "/images/blog_images/leetcode/None.png"
title: BM17 二分查找-I
date: 2024-07-04
description: 每日一题：二分查找-I
project_link: none
tags:
  - learn
---
## 描述

请实现无重复数字的升序数组的二分查找

给定一个 元素升序的、无重复数字的整型数组`nums`和一个目标值 `target`，写一个函数搜索`nums`中的`target`，如果目标值存在返回下标（下标从$0$﻿开始），否则返回$-1$﻿

数据范围：$0≤len(nums)≤2×10^5$﻿ ， 数组中任意值满足$|val|≤10^9$﻿

进阶：时间复杂度_$O(logn)$_﻿，空间复杂度_$O(1)$_﻿

## 示例1

```Plain
输入：[-1,0,3,4,6,10,13,14],13
返回值：6
说明：13 出现在nums中并且下标为 6
```

## 示例2

```Plain
输入：[],3
返回值：-1
说明：nums为空，返回-1
```

## 示例3

```Plain
输入：[-1,0,3,4,6,10,13,14],2
返回值：-1
说明：2不存在nums中因此返回-1
```

## 备注

```Plain
数组元素长度在⁍之间
数组每个元素都在[-9999, 9999]之间。
```

## 解题思路

实现二分查找即可

函数操作步骤

1. 检查左右边界值，当左右边界值相等或右小于左时说明无目标值，返回$-1$﻿
2. 比较子数组中点值是否为目标值，相等时返回索引`middle`
3. 根据目标值大小递归调用函数，目标值小于当前中点值则搜索前半部分，大于则搜索后半部分

```Python
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# @param nums int整型一维数组 
# @param target int整型 
# @return int整型
#
class Solution:
    def search(self , nums: List[int], target: int) -> int:
        def binarySearch(nums: List[int], target: int, left: int, right: int) -> int:
            if left <= right:
                middle = left + (right - left) // 2
                if nums[middle] == target:
                    return middle
                elif nums[middle] < target:
                    return binarySearch(nums, target, middle + 1, right)
                else:
                    return binarySearch(nums, target, left, middle - 1)
            else:
                return -1
        
        if not nums:
            return -1
        return binarySearch(nums, target, 0, nums.__len__() - 1)/
```

## 优化前代码

由于递归调用时使用了List切片运算，创建了新的子列表，导致了额外的内存分配和额外的赋值操作，算法效率低，在提交时应该是爆内存了导致没有AC

```Python
#
# 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
#
# @param nums int整型一维数组 
# @param target int整型 
# @return int整型
#
class Solution:
    def search(self , nums: List[int], target: int) -> int:
        def binarySearch(nums: List[int], target: int, index: int) -> int:
            middle = int(nums.__len__()/2)
            if nums.__len__() == 1 and nums[0] != target:
                return -1
            
            if nums[middle] == target:
                return middle + index
            elif nums[middle] < target:
                return binarySearch(nums[middle:], target, middle)
            elif nums[middle] > target:
                return binarySearch(nums[:middle], target, index + 0)
        
        if not nums:
            return -1
        return binarySearch(nums, target, 0)
```