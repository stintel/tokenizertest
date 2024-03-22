#!/usr/bin/python3

from transformers import AutoTokenizer


tokenizer = AutoTokenizer.from_pretrained("BAAI/bge-large-en-v1.5")


def print_encoded(input):
    output = tokenizer.encode(input)

    print(f"input: {repr(input)}")
    print(f"output: {output}")
    print()


print_encoded("foo")
print_encoded("foo bar")
print_encoded(["foo", "bar"])
