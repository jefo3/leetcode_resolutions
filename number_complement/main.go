package main

import (
	"fmt"
	"math/bits"
)

func findComplement(num int) int {
	if num == 1 {
		return 0
	}

	bitLength :=  bits.Len(uint(num))
	
	mask := (1 << bitLength) - 1
	
	return num ^ mask
}

func main () {
	res := findComplement(10)
	fmt.Println(res)
}