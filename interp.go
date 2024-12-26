package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"strconv"
	"regexp"
//	"math"
)

// I know I know regex is dumb... IT'S COMPACT OKAY?!? SHUT UP BEN
func IsDigit(n string) bool {
	re := regexp.MustCompile(`^\d+$`)
	if re.MatchString(n) {
		return true
	}
	return false
}
func Pop(l []int64) (int64, []int64) {
	return l[len(l)-1], l[:len(l)-1]
}
func SplitTo(l string, n int) []string {
	return strings.SplitN(l, "", n)
}
func VarMaker(l []string) []string {
	s := ""; n := len(l)
	for i := 0; i < n - 2; i++ {
		s += l[i+2]
	}
	return []string{l[0], s}
}

func Eval() {
	vars := make(map[string]int64)
	file, err := os.Open("file.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	lines := []string{}
	for scanner.Scan() {
		line := scanner.Text()
		lines = append(lines, line)
		vn := VarMaker(strings.Split(line, " "))
		vars[vn[0]], vars = EvalExp(vn[1], vars)
	}
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
	fmt.Println(vars)
}

func EvalExp(s string, v map[string]int64) (int64, map[string]int64) {
	tokens := strings.Split(s, "")
	stack := []int64{}
	for i := 0; i < len(tokens); i++ {
		if IsDigit(tokens[i]){
			num, err := strconv.ParseInt(tokens[i], 10, 64)
			if err != nil {
				panic(err)
			}
			stack = append(stack, num)
		} else if _, exists := v[tokens[i]]; exists {
			stack = append(stack, v[tokens[i]])
		} else if tokens[i] == "+" {
			rh, stack := Pop(stack)
			lh, stack := Pop(stack)
			stack = append(stack, lh+rh)
		} else if tokens[i] == "-" {
			rh, stack := Pop(stack)
			lh, stack := Pop(stack)
			stack = append(stack, lh-rh)
		} else if tokens[i] == "*" {
			rh, stack := Pop(stack)
			lh, stack := Pop(stack)
			stack = append(stack, lh*rh)
		} else if tokens[i] == "/" {
			rh, stack := Pop(stack)
			lh, stack := Pop(stack)
			stack = append(stack, lh/rh)
		} else if tokens[i] == "%" {
			rh, stack := Pop(stack)
			lh, stack := Pop(stack)
			stack = append(stack, lh%rh)
		}
	}
	_, stack = Pop(stack)
	fmt.Println(stack)
	return stack[0], v
}	

func main() {
	Eval()
}