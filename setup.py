from setuptools import setup
from setuptools_rust import Binding, RustExtension

extras = {}
extras["testing"] = ["pytest"]

setup(
    name="sudachirs",
    version="0.1.0",
    description="Fast and Customizable Sudachi Tokenizers",
    long_description=open("README.md", "r", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    keywords="Japanese NLP tokenizer",
    author="Faisal Hadiputra",
    license="Apache License 2.0",
    rust_extensions=[RustExtension(
        "sudachirs.sudachirs", binding=Binding.PyO3)],
    extras_require=extras,
    classifiers=[
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Intended Audience :: Education",
        "Intended Audience :: Science/Research",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
    packages=[
        "sudachirs",
    ],
    zip_safe=False,
)
