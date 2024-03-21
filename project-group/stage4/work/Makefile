all: record convert

record:
	perf record -g -T -P -a --call-graph fp julia solarium.jl

convert:
	perf script report gecko

clean:
	rm -f *perf*
	rm -f flamegraph.svg