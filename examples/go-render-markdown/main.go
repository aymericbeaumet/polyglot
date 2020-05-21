package main

import (
	"fmt"
	"io/ioutil"

	"github.com/russross/blackfriday/v2"
)

func main() {
	input, err := ioutil.ReadFile("sample.md")
	if err != nil {
		panic(err)
	}

	out := blackfriday.Run(input)

	s := string(out)
	fmt.Println(s)
}
