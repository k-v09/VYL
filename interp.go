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
func Factorial(n int64) int64 {
	if n == 1 {
		return 1
	}
	fact := n * Factorial(n-1)
	return fact
}
/*func FormatSplit(l []string) []string {
	s := ""
	rl := []string{}
	for i, v := range(l) {
		if v == " " {
			if len(rl) != 0 {
				rl = append(rl, s)
				s = ""
			}
		} else {s += v}
		if i == len(l)-1 && len(s) != 0 {
			rl = append(rl, s)
		}
	}
	return rl
}*/

func Eval() {
	vars := make(map[string]int64)
	funcs := strings.Split("+-=/*%!", "")
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
		spl := strings.Split(line, " ")
		if len(spl) > 2 {
			NotIn := true
			for _, v := range(funcs) {
				if v == spl[0] {NotIn=false}
			}
			if NotIn && spl[1] == "=" {
				vn := VarMaker(spl)
				vars[vn[0]], vars = EvalExp(vn[1], vars)
				continue
			}
		}
		EvalExp(line, vars)
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
			// These are just the dyadics for now. "!" will be factorial when monadic.
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
		} else if tokens[i] == "!" {
			k, stack := Pop(stack)
			n, stack := Pop(stack)
			// n!k=n!/(k!*(n-k)!)
			stack = append(stack, Factorial(n)/(Factorial(k)*Factorial(n-k)))
		}
	}
	_, stack = Pop(stack)
	fmt.Println(stack)
	return stack[0], v
}	

func main() {
	Eval()
}
