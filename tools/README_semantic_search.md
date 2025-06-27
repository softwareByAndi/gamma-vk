# Semantic Search Tool

A local semantic search tool for the Gamma-VK documentation that uses AI embeddings to find content by meaning rather than exact string matching.

## Setup

```bash
cd tools
pip install -r requirements.txt
```

## Usage

### 1. Index your documentation
```bash
# Index all documentation (first time or after major changes)
python semantic_search.py index .

# Index specific directories
python semantic_search.py index docs/

# Index only specific file types
python semantic_search.py index . --extensions .md .txt
```

### 2. Search semantically
```bash
# Search for concepts
python semantic_search.py search "how to handle buffer allocation"
python semantic_search.py search "error handling patterns"
python semantic_search.py search "vulkan initialization steps"

# Get more results
python semantic_search.py search "memory management" -n 10
```

### 3. Check index statistics
```bash
python semantic_search.py stats
```

## Features

- **Semantic Understanding**: Finds documents by meaning, not just keywords
- **Local & Private**: Everything runs on your machine, no cloud services
- **Persistent Index**: Builds once, searches many times
- **Smart Chunking**: Handles large documents by splitting with overlap
- **Fast Search**: Uses efficient vector similarity search

## Examples

```bash
# Find information about RAII patterns
python semantic_search.py search "automatic resource cleanup"

# Find debug notes about specific issues
python semantic_search.py search "type mismatch errors with vulkano"

# Find architectural decisions
python semantic_search.py search "why use builder pattern"
```

The tool creates a `.semantic_index` directory to store the vector database. You can add this to `.gitignore` if desired.