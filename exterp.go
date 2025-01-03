package main

import (
	"fmt"
	"os"
	"bufio"
)

/*
func Eval(s string, v map[string]int64) (int64, map[string]int64)  {

}
*/

func extrp () {
	vars := make(map[string]int64)
	file, err := os.Open("file.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	//lines := []string{}
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}
	fmt.Println(vars)
}
