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

# Accept build-time features (like dhat-heap)
ARG FEATURES=""

# Copy configuration
COPY Cargo.toml Cargo.lock ./

# Pre-fetch and pre-build dependencies for caching
# We need a dummy benchmark file because Cargo.toml defines it
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    mkdir benches && echo "fn main() {}" > benches/image_processing.rs && \
    if [ -n "$FEATURES" ]; then \
        cargo build --release --features "$FEATURES"; \
    else \
        cargo build --release; \
    fi

# Copy real source and rebuild the actual application
COPY . .
RUN cargo clean -p table-ocr && if [ -n "$FEATURES" ]; then \
        cargo build --release --features "$FEATURES"; \
    else \
        cargo build --release; \
    fi

# Ensure output directories exist
RUN mkdir -p /app/files /app/output /app/perf_data

ENV RUST_LOG=info
ENV TESSDATA_PREFIX=/usr/share/tesseract-ocr/5/tessdata
ENTRYPOINT ["/app/target/release/table-ocr"]
