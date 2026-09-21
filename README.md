# table-ocr

Multi-threaded batch OCR for table-like images. Each image is enhanced
(`scale -> grayscale -> binarize -> denoise`), read with Tesseract, and the
recognized lines are written to a CSV.

> [!IMPORTANT]
> **Run this tool only inside Docker.** It links native Leptonica/Tesseract
> libraries and the image ships the required OCR language data. A local
> `cargo run` build is not supported.

## Build

```bash
docker build -t table-ocr .
```

## Run

```bash
docker run --rm \
  -v "${PWD}/files/number8-2:/data/images:ro" \
  -v "${PWD}/output:/data/output" \
  table-ocr \
  --input-dir /data/images \
  --output /data/output/out.csv \
  --lang eng+mon \
  --columns 1 --x-start 1270 --x-end 2050
```

Or with compose after editing the mounts and column bounds in `docker-compose.yml`:

```bash
docker compose up --build
```

## Options

| Flag | Default | Description |
| --- | --- | --- |
| `--input-dir` | — | Folder with images (`png`, `jpg`, `jpeg`, `bmp`, `tiff`) |
| `--output` | — | Output CSV path |
| `--lang` | `eng` | Tesseract language(s), e.g. `eng+mon` |
| `--columns` | `5` | Number of CSV columns per row |
| `--x-start`, `--x-end` | — | X-coordinate window to OCR, in pixels of the scaled image |
| `--target-dpi` | `300` | Scales image width to `3000px * (dpi/300)` |
| `--deskew` | `true` | Enable deskewing |
| `--skip-denoise` | `false` | Skip median/morphological denoising for clean images |

Inputs are mounted read-only; only the resulting CSV is written to the host.
