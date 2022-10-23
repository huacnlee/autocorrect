.PHONY: lib test

run: lib
	javac javasrc/src/main/java/org/huacnlee/AutoCorrect.java && java -classpath javasrc/src/main/java/org/huacnlee/ -Djava.library.path=../target/debug/ AutoCorrect

javah:
	javac -h . -sourcepath javasrc/src/main/java -d classes javasrc/src/main/java/org/huacnlee/AutoCorrect.java

test: 
	cd javasrc; mvn test

lib:
	cargo build