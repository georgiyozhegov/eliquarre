# Eliquarre
Eliquarre - fast text filtering with classic ML (logistic regression)

# Setup
```bash
git clone https://github.com/georgiyozhegov/eliquarre
cd eliquarre
cargo build
```

# Usage

## Ask
To create labeled data for training use 
```bash
eliquarre ask <texts.txt>
```
This will generate output *samples.jsonl*

**NOTE**: input file format should look like this:
```
text
{split_token}
text
{split_token}
...
```
You can also specify some options in *eliquarre.json*
```json
{
     "samples": 40,
}
```
This option sets number of samples to ask = 40.

## Train
To train model on labeled data use
```bash
eliquarre train samples.jsonl
```
This will train & save model to *model.json*.

## Filter
In progress...
