package pq

import (
	"container/heap"
)

type PQInterface[T any] interface {
	PQLess(b T) bool
}

type PriorityQueue[T PQInterface[T]] struct {
	items RawPriorityQueue[T]
}
func NewPriorityQueue[T PQInterface[T]]() *PriorityQueue[T] {
	var r = new(PriorityQueue[T])
	r.items = make(RawPriorityQueue[T], 0)
	heap.Init(&r.items)

	return r
}
func (pq *PriorityQueue[T]) Len() int {
	return pq.items.Len()
}
func (pq *PriorityQueue[T]) Push(value T) {
	heap.Push(&pq.items, NewItem(value))
}
func (pq *PriorityQueue[T]) Pop() T {
	return heap.Pop(&pq.items).(*Item[T]).value
}

// https://pkg.go.dev/container/heap#example-package-PriorityQueue
type Item[T PQInterface[T]] struct {
	value T
	index int
}
func NewItem[T PQInterface[T]](value T) *Item[T] {
	return &Item[T]{value,0}
}
type RawPriorityQueue[T PQInterface[T]] []*Item[T]
func (pq RawPriorityQueue[T]) Len() int { return len(pq) }
func (pq RawPriorityQueue[T]) Less(i, j int) bool {
	var a = pq[i].value
	var b = pq[j].value

	return a.PQLess(b)
}
func (pq RawPriorityQueue[T]) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}
func (pq *RawPriorityQueue[T]) Push(x any) {
	n := len(*pq)
	item := x.(*Item[T])
	item.index = n
	*pq = append(*pq, item)
}
func (pq *RawPriorityQueue[T]) Pop() any {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil
	item.index = -1
	*pq = old[0 : n-1]
	return item
}
