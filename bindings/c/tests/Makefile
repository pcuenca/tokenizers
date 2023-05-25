CC=gcc
CFLAGS=-Wall -I../include -L../target/debug -lctokenizers
TARGET=models

all: $(TARGET)

$(TARGET): $(TARGET).c
	$(CC) $(CFLAGS) -o $(TARGET) $(TARGET).c

clean:
	rm -f $(TARGET)
