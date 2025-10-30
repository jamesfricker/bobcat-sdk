//go:generate go run github.com/99designs/gqlgen generate

package main

import (
	"net/http"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/handler/extension"
	"github.com/99designs/gqlgen/graphql/handler/lru"
	"github.com/99designs/gqlgen/graphql/handler/transport"
	"github.com/stylus-developers-guild/bobcat-sdk/examples/004-bozo/graph"
	"github.com/vektah/gqlparser/v2/ast"
)

func main() {
	srv := handler.New(graph.NewExecutableSchema(graph.Config{
		Resolvers: &graph.Resolver{},
	}))
	srv.AddTransport(transport.Options{})
	srv.AddTransport(transport.GET{})
	srv.AddTransport(transport.POST{})
	srv.SetQueryCache(lru.New[*ast.QueryDocument](1000))
	srv.Use(extension.Introspection{})
	srv.Use(extension.AutomaticPersistedQuery{
		Cache: lru.New[string](100),
	})
	http.Handle("/", srv)
	panic(http.ListenAndServe(":8080", nil))
}
