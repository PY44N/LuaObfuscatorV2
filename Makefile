CC = g++
CFLAGS = -std=c++20 -Wall -Wextra
LDFLAGS = 

SRC = $(wildcard src/*.cpp) $(wildcard src/**/*.cpp) $(wildcard src/**/**/*.cpp)
OBJ = $(SRC:.cpp=.o)

NAME = LuaObfuscator

all: exe

run: all
	./$(NAME)

exe: $(OBJ)
	$(CC) -o $(NAME) $^ $(LDFLAGS)

%.o: %.c
	$(CC) -o $@ -c $< $(CFLAGS)

clean:
	rm -rf $(OBJ)