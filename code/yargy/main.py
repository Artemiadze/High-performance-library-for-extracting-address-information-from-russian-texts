from yargy import Parser

from rules import *


if __name__ == "__main__":
    parser = Parser(ADDR)
    text = "Россия, обл. Курская, р-н Золотухинский, рп Золотухино, ул. Куйбышева, дом 42"
    for match in parser.findall(text):
        print(match.fact)