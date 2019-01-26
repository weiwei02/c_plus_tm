/**
 * 冒泡排序算法
 * 重复地走访过要排序的数列，一次比较两个元素，
 * 如果他们的顺序（如从大到小、首字母从A到Z）错误就把他们交换过来。
 * */
// Created by weiwei on 2019/1/27.
//
#include "sort.h"
#include <stdio.h>
int* bubble_sort(int * arr, int length) {
    int i, j, temp;
    for (i = 0; i < length - 1; i++)
        for (j = 0; j < length - 1 - i; j++)
            if (arr[j] > arr[j + 1]) {
                temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
    return arr;
}