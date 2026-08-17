CXX = g++
CXXFLAGS = -std=c++17 -O2 -Wall -Wextra -Iinclude
SRCS = $(wildcard src/*.cpp)
BIN = bin/hft_engine_cpp
BENCH = bin/hft_engine_cpp_bench
CONCURRENT = bin/hft_engine_cpp_spsc
GBENCH = bin/hft_engine_cpp_gbench

all: $(BIN) $(BENCH) $(CONCURRENT) $(GBENCH)

$(BIN): src/book.cpp src/engine.cpp src/main.cpp
	$(CXX) $(CXXFLAGS) -o $(BIN) src/book.cpp src/engine.cpp src/main.cpp

$(BENCH): src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp
	$(CXX) $(CXXFLAGS) -o $(BENCH) src/book.cpp src/engine.cpp src/generator.cpp src/stats.cpp src/benchmark.cpp

$(CONCURRENT): src/book.cpp src/engine.cpp src/concurrent_demo.cpp
	$(CXX) $(CXXFLAGS) -o $(CONCURRENT) src/book.cpp src/engine.cpp src/concurrent_demo.cpp

# Google Benchmark target: only builds if "benchmark" is available via pkg-config
$(GBENCH):
	@if pkg-config --exists benchmark; then \
		CXXFLAGS_GB=`pkg-config --cflags benchmark`; \
		LIBS_GB=`pkg-config --libs benchmark`; \
		$(CXX) $(CXXFLAGS) $$CXXFLAGS_GB -o $(GBENCH) src/book.cpp src/engine.cpp src/generator.cpp bench/bench_gbench.cpp $$LIBS_GB; \
		echo "Built Google Benchmark binary: $(GBENCH)"; \
	else \
		echo "Google Benchmark not found via pkg-config; to build, install benchmark (and googletest), or follow cpp/third_party/benchmark instructions in the README."; \
		exit 0; \
	fi

clean:
	rm -rf $(BIN) $(BENCH)

.PHONY: all clean
