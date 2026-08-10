CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra -Iinclude
SRCS = $(wildcard src/*.cpp)
BIN = bin/hft_engine_cpp
BENCH = bin/hft_engine_cpp_bench
CONCURRENT = bin/hft_engine_cpp_spsc

all: $(BIN) $(BENCH) $(CONCURRENT)

$(BIN): src/book.cpp src/engine.cpp src/main.cpp
	$(CXX) $(CXXFLAGS) -o $(BIN) src/book.cpp src/engine.cpp src/main.cpp

$(BENCH): src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp
	$(CXX) $(CXXFLAGS) -o $(BENCH) src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp

$(CONCURRENT): src/book.cpp src/engine.cpp src/concurrent_demo.cpp
	$(CXX) $(CXXFLAGS) -o $(CONCURRENT) src/book.cpp src/engine.cpp src/concurrent_demo.cpp

clean:
	rm -rf $(BIN) $(BENCH)

.PHONY: all clean
