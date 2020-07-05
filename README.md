# sudachirs-python

An unofficial python binding for Rust implementation of [Sudachi](https://github.com/WorksApplications/Sudachi).
This repository is inspired by [sudachi.rs](https://github.com/sorami/sudachi.rs).

## Usage

```python
from sudachirs import Dictionary

tokenizer = Dictionary().create()
for morpheme in tokenizer.tokenize("明日、東京駅に行く。", mode="C"): # You can use mode A, B, or C just like sudachi does
    print(morpheme.surface(), morpheme.pos())

# Output:
# 明日 ['名詞', '普通名詞', '副詞可能', '*', '*', '*']
# 、 ['補助記号', '読点', '*', '*', '*', '*']
# 東京 ['名詞', '固有名詞', '地名', '一般', '*', '*']
# に ['助詞', '格助詞', '*', '*', '*', '*']
# 行く ['動詞', '非自立可能', '*', '*', '五段-カ行', '終止形-一般']
# 。 ['補助記号', '句点', '*', '*', '*', '*']

```

## Setup

### 1. Get the source code

```
$ git clone https://github.com/faisalron/sudachirs-python.git
```

### 2. Download a Sudachi Dictionary

You can download a dictionary zip file from [WorksApplications/SudachiDict](https://github.com/WorksApplications/SudachiDict) (choose one from `small`, `core`, or `full`), unzip it, and place the `system_*.dic` file to `src/resources/system.dic` (Note that the file name is changed to `system.dic`) .

Alternatively, you can use a quick shell script in the source code; This script will download the `core` dictionary and place it to `src/resources/system.dic`.

```
$ ./fetch_dictionary.sh
```

### 3. Build, Install

You will need setuptools-rust installed beforehand.
The built library will **contain the dictionary binary**.

```
$ pip install .
```

## Performance Benchmark

Sudachirs-python vs sudachipy==0.4.2 (Pure Python).

The Rust version is **30x faster**.

```
============================================ test session starts =============================================
platform darwin -- Python 3.6.3, pytest-5.4.3, py-1.9.0, pluggy-0.13.1
benchmark: 3.2.3 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/kaigit/Documents/GitHub/sudachirs-python.rs
plugins: celery-4.4.0, benchmark-3.2.3
collected 2 items                                                                                            

benchmark_test.py ..
---------------------------------------------------------------------------------------------- benchmark: 2 tests ----------------------------------------------------------------------------------------------
Name (time in us)             Min                    Max                   Mean                StdDev                 Median                   IQR            Outliers         OPS            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_sudachirs           305.9360 (1.0)       5,339.5090 (1.0)         455.2708 (1.0)        326.0953 (1.0)         353.3675 (1.0)        166.2295 (1.0)         57;71  2,196.4950 (1.0)        1076           1
test_sudachipy        11,755.0310 (38.42)    42,088.3221 (7.88)     15,189.1679 (33.36)    5,231.4347 (16.04)    13,739.4925 (38.88)    2,552.3510 (15.35)         2;4     65.8364 (0.03)         40           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```

Sudachirs-python vs sudachipy==0.4.9 (Cython Optimized).

The Rust version is around **8x faster**.

```
============================================ test session starts =============================================
platform darwin -- Python 3.6.3, pytest-5.4.3, py-1.9.0, pluggy-0.13.1
benchmark: 3.2.3 (defaults: timer=time.perf_counter disable_gc=False min_rounds=5 min_time=0.000005 max_time=1.0 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/kaigit/Documents/GitHub/sudachirs-python.rs
plugins: celery-4.4.0, benchmark-3.2.3
collected 2 items                                                                                            

benchmark_test.py ..
-------------------------------------------------------------------------------------------- benchmark: 2 tests --------------------------------------------------------------------------------------------
Name (time in us)            Min                   Max                  Mean                StdDev                Median                   IQR            Outliers         OPS            Rounds  Iterations
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_sudachirs          301.9370 (1.0)      1,072.3210 (1.0)        401.9116 (1.0)        146.3000 (1.0)        332.9599 (1.0)        110.1720 (1.0)       181;159  2,488.1093 (1.0)        1301           1
test_sudachipy        2,144.7670 (7.10)     9,914.5430 (9.25)     3,431.3932 (8.54)     1,542.9379 (10.55)    2,838.6400 (8.53)     1,295.3440 (11.76)       21;17    291.4268 (0.12)        135           1
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```
