FROM rust:latest

# Install dependencies for building AND running OCR
RUN apt-get update && apt-get install -y \
    libleptonica-dev \
    libtesseract-dev \
    tesseract-ocr \
    tesseract-ocr-eng \
    tesseract-ocr-mon \
    libclang-dev \
    llvm-dev \
    pkg-config \
    build-essential \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy configuration
COPY Cargo.toml Cargo.lock ./

# Pre-fetch and pre-build dependencies for caching
# We need a dummy benchmark file because Cargo.toml defines it
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    mkdir benches && echo "fn main() {}" > benches/image_processing.rs && \
    cargo build --release

# Copy real source and rebuild the actual application
COPY . .
RUN cargo clean -p table-ocr && cargo build --release

# Ensure output directories exist
RUN mkdir -p /app/files /app/output

ENV TESSDATA_PREFIX=/usr/share/tesseract-ocr/5/tessdata
ENTRYPOINT ["/app/target/release/table-ocr"]
