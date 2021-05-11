package main

import (
	"testing"
)

func BenchmarkGo10threads_1bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10, 1)
	}
}

func BenchmarkGo10000threads_1bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000, 1)
	}
}

func BenchmarkGo10000threads_10bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000, 10)
	}
}

func BenchmarkGo10000threads_100bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000, 100)
	}
}

func BenchmarkGo10000threads_1000bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000, 1000)
	}
}

func BenchmarkGo10000threads_10000bufsize(b *testing.B) {
	for i := 0; i < b.N; i++ {
		channel(10000, 10000)
	}
}
