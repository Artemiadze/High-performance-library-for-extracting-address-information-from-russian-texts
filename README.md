# Высокопроизводительная библиотека для извлечения структурированноц информации из текстов на русском языке #
---
### Курсовая работа студента группы 23КНТ6 Власова Артёма ИМИКН ВШЭ НН (2026 год) 
---

## Run `yargy` (Python)

```bash
cd code/yargy
python3 -m venv .venv
source .venv/bin/activate
python3 -m pip install -U pip
python3 -m pip install "setuptools<81" yargy pymorphy2-dicts-ru
python3 main.py
```

## Run `yargy-rs` (Rust)

```bash
cd code/yargy-rs
cargo run
```