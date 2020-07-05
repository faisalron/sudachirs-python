from sudachirs import Dictionary as RDictionary
from sudachipy.dictionary import Dictionary as PDictionary

r_tokenizer = RDictionary().create()
p_tokenizer = PDictionary().create()

text = "焦躁と言おうか、嫌悪と言おうか――酒を飲んだあとに宿酔があるように、酒を毎日飲んでいると宿酔に相当した時期がやって来る。"


def test_sudachirs(benchmark):
    benchmark(r_tokenizer.tokenize, text)


def test_sudachipy(benchmark):
    benchmark(p_tokenizer.tokenize, text)
