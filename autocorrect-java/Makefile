.PHONY: lib test

run: lib
	javac javasrc/src/main/java/io/github/huacnlee/*.java -d javasrc/target/classes 
	java -classpath javasrc/target/classes -Djava.library.path=../target/debug/ io/github/huacnlee/AutoCorrect

javah:
	javac -h . -sourcepath javasrc/src/main/java -d classes javasrc/src/main/java/io/github/huacnlee/AutoCorrect.java

test: lib
	cd javasrc; mvn test

lib:
	cargo build