TEX = pdflatex -halt-on-error -interaction nonstopmode -output-directory build
Q = >/dev/null

.SUFFIXES:
.PHONY: default clean

default: build/spec.pdf

clean:
	rm -rf build

build/%.pdf: doc/%.tex
	mkdir -p build
	for x in 1 2; do $(TEX) $^ $(Q); done
