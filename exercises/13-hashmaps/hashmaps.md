# HashMap

## Exercises

In Visual Studio Code, create a new project for each of the following exercises using `cargo new`

## 13-0:  Number of Vowels

Create a new project called `num_vowels` that counts the **number of vowels** found in the text file called `emma.md`.  You will need to save the text file ``emma.md`` in the `src` folder of your project.  Use a `HashMap` collection to store the vowels as `keys` and the number of vowels as `values`. Create a function `num_vowels(s: &str) -> HashMap<char,u64>` that uses the **string** read from the file and returns a `HashMap` with `keys` that are vowels and `values` that represent the number of vowels found.

## 13-1: Nucleotides

Create a new project called `nucleotides`.

The genetic language of every living thing on the planet is DNA. DNA is a large molecule that is built from an extremely long sequence of individual elements called nucleotides. 4 types exist in DNA and these differ only slightly and can be represented as the following symbols: **A** for adenine, **C** for cytosine, **G** for guanine, and **T** for thymine."

Given a single stranded DNA string, compute how many times each nucleotide occurs in the string.

Using the starter code,

```rust
fn main(){
   let nucleo = "AGCTTTTCATTCTGACTGCAACGGGCAATATGTCTCTGTGTGGATTAAAAAAAGAGTGTCTGATAGCAGC";
   println!("{:?}", num_nucleotides(nucleo));
}

fn num_nucleotides(s: &str) -> HashMap<char, u64> {
	
}

```

modify the function called `num_nucleotides(s: &str) -> HashMap<char,u64>`  that returns a `HashMap` with keys that are the nucleotides:  **'A'**, **'C'**, **'G'**, and **'T'** and values that represent **how many times each nucleotide occurs** in a string called **strand**.