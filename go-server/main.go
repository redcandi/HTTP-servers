package main

import (
	"fmt"
	"log"
	"net/http"
	"time"
	"github.com/gorilla/mux"
)

type Middleware func(http.HandlerFunc) http.HandlerFunc

func Logger() Middleware {
	return func(hf http.HandlerFunc) http.HandlerFunc {
		return func  (w http.ResponseWriter, r* http.Request){
			
						start := time.Now()
			defer func(){ log.Println(r.URL.Path, time.Since(start)) }()

			hf(w,r)
		}
	}
}

func Chain(f http.HandlerFunc, middlewares ...Middleware) http.HandlerFunc{
	for _,m := range middlewares{
		f = m(f)
	}
	return f
}

func main(){

	r := mux.NewRouter()
	
	r.http.HandleFunc("/",func(w http.ResponseWriter,r* http.Request){
		fmt.Fprintf(w,"Hello, World")
	})

	fs := http.FileServer(http.Dir("static/"))

    http.Handle("/static/", http.StripPrefix("/static/", fs))

	http.ListenAndServe("127.0.0.1:6969", r)
}
 












































































































































































