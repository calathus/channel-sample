package main

import (
	"fmt"
	"strings"
	"sync"
)

type Info struct {
	p int
	s string
}

func create_ssc() chan string {
	ssc := make(chan string)
	go func() {
		defer close(ssc)
		for _, s := range data() {
			ssc <- s
		}
	}()
	return ssc
}

func create_infoc() chan *Info {
	infos := make(chan *Info)
	return infos
}

func handle_ssc(n int, ssc chan string, infos chan *Info) {
	for s := range ssc {
		infos <- &Info{p: n, s: strings.ToUpper(s)}
		// infos_sender.send(...)
	}
}

func main() {
	var width = 4

	ssc := create_ssc()
	infos := create_infoc()

	go func() {
		defer close(infos)
		var wg sync.WaitGroup
		wg.Add(width)

		for i := 0; i < width; i++ {
			go func(n int) {
				defer wg.Done()
				handle_ssc(n, ssc, infos)
			}(i)
		}
		wg.Wait()
	}()

	for i := range infos { // infos_reveiver.recv()
		fmt.Println(i)
	}
}

func data() []string {
	return []string{
		"aaa",
		"bb",
		"cc",
		"sss",
		"qq",
		"ww"}
}
