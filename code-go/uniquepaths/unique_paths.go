package main

import "fmt"

// 不同路径
// 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为“Start” ）。
// 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为“Finish”）。
// 问总共有多少条不同的路径？

// 示例
// 输入: m = 3, n = 2
// 输出: 3
// 解释:
// 从左上角开始，总共有 3 条路径可以到达右下角。
// 1. 向右 -> 向右 -> 向下
// 2. 向右 -> 向下 -> 向右
// 3. 向下 -> 向右 -> 向右

// 输入: m = 7, n = 3
// 输出: 28

func uniquePaths(m int, n int) int {
    row, col := n, m
    // 用来保存原数组从起点开始到对应点的路径数目
    pathSums := make([][]int, row)
    for i := 0; i < len(pathSums); i++ {
        pathSums[i] = make([]int, col)
    }
    // 起始点 0,0
    startX, startY := 0, 0
    // 起点路径数目为 1
    pathSums[startY][startX] = 1

    // 第一列所有点的路径数目
    for i := startY + 1; i < row; i++ {
        // 等于上一个点的路径数目
        pathSums[i][0] = pathSums[i - 1][0]
    }

    // 第一行所有点的路径数目
    for j := startX + 1; j < col; j++ {
        // 等于前一个点的路径数目
        pathSums[0][j] = pathSums[0][j - 1]
    }

    // 其它点的路径数目等于上边和左边点的路径数目之和
    for i := startY + 1; i < row; i++ {
        for j := startX + 1; j < col; j++ {
            pathSums[i][j] = pathSums[i - 1][j] + pathSums[i][j - 1]
        }
    }
    // 返回最后一个点的路径数目
    return pathSums[row - 1][col - 1]
}

func main() {
    m := 3
    n := 2
    fmt.Printf("m = %d, n = %d\n", m, n)
    count := uniquePaths(m, n)
    fmt.Println(count)

    m = 7
    n = 3
    fmt.Printf("m = %d, n = %d\n", m, n)
    count = uniquePaths(m, n)
    fmt.Println(count)
}