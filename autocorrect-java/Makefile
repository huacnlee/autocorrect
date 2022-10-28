.PHONY: lib test

run: lib
	cd javasrc; mvn --activate-profiles debug compile exec:java  --activate-profiles debug -Dexec.mainClass="io.github.huacnlee.AutoCorrect"

javah:
	javac -h . -sourcepath javasrc/src/main/java -d classes javasrc/src/main/java/io/github/huacnlee/AutoCorrect.java

test: lib
	cd javasrc; mvn test --activate-profiles debug

lib:
	cargo build