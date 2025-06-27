# Semantic Search Re-Architecture Plan

## Executive Summary

The current `semantic_search.py` implementation works but contains unnecessary complexity that makes it harder to read and maintain. This plan outlines a simpler, more Pythonic approach that maintains **all existing functionality** while improving readability and maintainability.

## Current Issues

### 1. Overly Complex Class Structure
- Single class (`DocumentIndexer`) doing too much
- Class state management for what could be simpler functions
- Unnecessary persistence of model and client objects

### 2. Exception Handling Anti-patterns
- Bare `except:` clauses that hide errors
- Silent failures in configuration loading
- No clear error messages for users

### 3. Complex Data Structures
- ChromaDB returns nested lists/dicts that require confusing indexing
- Manual embedding generation adds complexity
- Metadata management is verbose

### 4. Configuration Over-engineering
- JSON config file for simple excluded directories
- Default fallbacks buried in methods
- Config loading failures silently ignored

### 5. Verbose Implementation
- 238 lines for what could be ~100 lines
- Repetitive code patterns
- Manual string formatting instead of using Python features

## Proposed Architecture

### 1. Hybrid Functional Approach
Use module-level functions with a minimal state container for expensive objects:
```python
# Module-level state (initialized once)
_model = None
_client = None

def get_model():
    """Lazy-load the sentence transformer model."""
    global _model
    if _model is None:
        _model = SentenceTransformer('all-MiniLM-L6-v2')
    return _model

def index_documents(directory: Path, extensions: List[str] = None) -> None
def search_documents(query: str, n_results: int = 5) -> List[SearchResult]
def show_stats() -> Stats
```

### 2. Enhanced Data Models
Use dataclasses with complete metadata preservation:
```python
@dataclass
class SearchResult:
    filepath: Path
    content: str
    score: float
    chunk_index: int
    total_chunks: int
    
@dataclass
class DocumentMetadata:
    filepath: str
    filename: str
    extension: str
    chunk_index: int
    total_chunks: int
    file_hash: str
    indexed_at: str

@dataclass
class Stats:
    total_files: int
    total_chunks: int
    index_location: Path
```

### 3. Configuration Support
Maintain external configuration with proper defaults:
```python
def load_config() -> Dict[str, Any]:
    """Load configuration from JSON file with defaults."""
    config_path = Path(__file__).parent / "semantic_search_config.json"
    defaults = {
        "excluded_dirs": ["target", "node_modules", ".git", "__pycache__", ".semantic_index"],
        "chunk_size": 1000,
        "overlap_ratio": 0.25,
        "default_extensions": ['.md', '.txt', '.rs', '.toml', '.log']
    }
    
    try:
        with open(config_path, 'r') as f:
            user_config = json.load(f)
            return {**defaults, **user_config}
    except FileNotFoundError:
        return defaults

# Load config once at module level
CONFIG = load_config()
INDEX_DIR = Path('tools/.semantic_index')
```

### 4. Explicit Embedding Control
- Keep manual SentenceTransformer for control over embeddings
- Maintain consistent embedding generation
- Store embeddings explicitly in ChromaDB

### 5. Comprehensive Error Handling
- Specific exceptions with file context
- Graceful degradation for missing files
- Clear user feedback with actionable messages

## Implementation Details

### Core Functions

```python
def get_client():
    """Get or create ChromaDB client (lazy initialization)."""
    global _client
    if _client is None:
        _client = chromadb.PersistentClient(path=str(INDEX_DIR))
    return _client

def get_collection():
    """Get or create the ChromaDB collection."""
    client = get_client()
    try:
        return client.get_collection("gamma_vk_docs")
    except ValueError:
        return client.create_collection(
            "gamma_vk_docs",
            metadata={"hnsw:space": "cosine"}
        )

def get_file_hash(filepath: Path) -> str:
    """Calculate MD5 hash of file for change detection."""
    return hashlib.md5(filepath.read_bytes()).hexdigest()

def should_skip_file(filepath: Path) -> bool:
    """Check if file should be skipped based on exclusion rules."""
    # Skip hidden files and directories
    if any(part.startswith('.') for part in filepath.parts):
        return True
    
    # Skip files in excluded directories
    excluded_dirs = CONFIG.get("excluded_dirs", [])
    if any(excluded_dir in filepath.parts for excluded_dir in excluded_dirs):
        return True
    
    return False

def chunk_text(text: str) -> List[str]:
    """Split text into overlapping chunks."""
    chunk_size = CONFIG.get("chunk_size", 1000)
    overlap_ratio = CONFIG.get("overlap_ratio", 0.25)
    
    words = text.split()
    if len(words) <= chunk_size:
        return [text]
    
    chunks = []
    step = int(chunk_size * (1 - overlap_ratio))
    for i in range(0, len(words), step):
        chunk = ' '.join(words[i:i + chunk_size])
        chunks.append(chunk)
        if i + chunk_size >= len(words):
            break
    
    return chunks

def index_documents(directory: Path, extensions: List[str] = None):
    """Index all documents in directory with full metadata preservation."""
    extensions = extensions or CONFIG.get("default_extensions", ['.md', '.txt'])
    collection = get_collection()
    model = get_model()
    
    files_indexed = 0
    chunks_created = 0
    
    for ext in extensions:
        for filepath in directory.rglob(f'*{ext}'):
            if should_skip_file(filepath):
                continue
                
            try:
                # Read file and generate metadata
                content = filepath.read_text(encoding='utf-8')
                file_hash = get_file_hash(filepath)
                chunks = chunk_text(content)
                
                # Process all chunks for this file
                ids = []
                embeddings = []
                documents = []
                metadatas = []
                
                for i, chunk in enumerate(chunks):
                    chunk_id = f"{filepath}::{i}"
                    
                    # Generate embedding manually for control
                    embedding = model.encode(chunk).tolist()
                    
                    # Prepare complete metadata
                    metadata = {
                        "filepath": str(filepath),
                        "filename": filepath.name,
                        "extension": ext,
                        "chunk_index": i,
                        "total_chunks": len(chunks),
                        "file_hash": file_hash,
                        "indexed_at": datetime.now().isoformat()
                    }
                    
                    ids.append(chunk_id)
                    embeddings.append(embedding)
                    documents.append(chunk)
                    metadatas.append(metadata)
                
                # Batch upsert all chunks for this file
                collection.upsert(
                    ids=ids,
                    embeddings=embeddings,
                    documents=documents,
                    metadatas=metadatas
                )
                
                chunks_created += len(chunks)
                files_indexed += 1
                print(f"Indexed: {filepath} ({len(chunks)} chunks)")
                
            except Exception as e:
                print(f"Error indexing {filepath}: {e}")
    
    print(f"\nIndexing complete: {files_indexed} files, {chunks_created} chunks")
```

### Enhanced Search

```python
def search_documents(query: str, n_results: int = 5) -> List[SearchResult]:
    """Search for documents matching query with manual embedding generation."""
    collection = get_collection()
    model = get_model()
    
    # Generate query embedding manually
    query_embedding = model.encode(query).tolist()
    
    # Search with explicit embedding
    results = collection.query(
        query_embeddings=[query_embedding],
        n_results=n_results
    )
    
    # Handle empty results
    if not results['ids'] or not results['ids'][0]:
        return []
    
    # Format results with full metadata
    search_results = []
    for i in range(len(results['ids'][0])):
        doc_id = results['ids'][0][i]
        filepath, chunk_idx = doc_id.split('::')
        metadata = results['metadatas'][0][i]
        
        # Truncate content for display
        content = results['documents'][0][i]
        if len(content) > 200:
            content = content[:200] + '...'
        
        search_results.append(SearchResult(
            filepath=Path(filepath),
            content=content,
            score=1 - results['distances'][0][i],
            chunk_index=int(chunk_idx),
            total_chunks=metadata.get('total_chunks', 1)
        ))
    
    return search_results

def show_stats() -> Stats:
    """Get comprehensive statistics about the index."""
    collection = get_collection()
    
    try:
        results = collection.get()
        
        # Calculate unique files
        unique_files = set()
        for metadata in results['metadatas']:
            unique_files.add(metadata['filepath'])
        
        return Stats(
            total_files=len(unique_files),
            total_chunks=len(results['ids']),
            index_location=INDEX_DIR
        )
    except Exception:
        # Return empty stats if collection is empty
        return Stats(
            total_files=0,
            total_chunks=0,
            index_location=INDEX_DIR
        )
```

### Clean CLI

```python
def main():
    parser = argparse.ArgumentParser(description='Semantic search for documentation')
    subparsers = parser.add_subparsers(dest='command', required=True)
    
    # Index command
    index_parser = subparsers.add_parser('index')
    index_parser.add_argument('directory', type=Path, nargs='?', default='.')
    index_parser.add_argument('--extensions', nargs='+')
    
    # Search command  
    search_parser = subparsers.add_parser('search')
    search_parser.add_argument('query')
    search_parser.add_argument('-n', '--results', type=int, default=5)
    
    # Stats command
    subparsers.add_parser('stats')
    
    args = parser.parse_args()
    
    if args.command == 'index':
        index_documents(args.directory, args.extensions)
    elif args.command == 'search':
        results = search_documents(args.query, args.results)
        display_results(results, args.query)
    elif args.command == 'stats':
        display_stats(show_stats())
```

## Benefits of New Architecture

1. **Readability**: Clear function names, obvious data flow
2. **Maintainability**: Each function has single responsibility  
3. **Testability**: Functions are easier to test than class methods
4. **Performance**: Same performance with lazy initialization
5. **Error Handling**: Clear error messages with file context
6. **Pythonic**: Uses dataclasses, pathlib, and modern Python features
7. **Feature Parity**: Maintains ALL existing functionality:
   - Manual embedding control with SentenceTransformer
   - File hash tracking for change detection
   - Hidden file filtering
   - Configuration file support
   - Rich metadata preservation
   - Comprehensive error context
8. **Moderate Size**: ~150-170 lines (vs 238) - reasonable reduction while keeping features

## Migration Strategy

1. Create new `semantic_search_v2.py` alongside existing
2. Test with same commands to ensure compatibility
3. Replace original once verified
4. No changes needed to existing index or usage patterns

## Future Enhancements (Not in Initial Refactor)

- Async file reading for better performance
- Progress bars for large directories  
- Incremental indexing (only changed files)
- Better chunk boundary detection (sentence-aware)
- Export/import index capability

## Key Changes Summary

### What's Preserved (No Loss of Functionality)
- ✅ Manual SentenceTransformer embedding generation
- ✅ File hash tracking for change detection  
- ✅ Hidden file/directory filtering
- ✅ External configuration file support
- ✅ Rich metadata (filename, extension, hash, timestamp)
- ✅ Chunk overlapping with configurable ratio
- ✅ Specific error messages with file context
- ✅ Same CLI interface and commands
- ✅ Same ChromaDB persistence location

### What's Improved
- ✨ Cleaner functional architecture with lazy initialization
- ✨ Type-safe data structures using dataclasses
- ✨ Better separation of concerns
- ✨ More testable code structure
- ✨ Clearer error handling without bare except
- ✨ Configuration with proper defaults
- ✨ More concise implementation (~30% reduction)

### What's Different (But Better)
- 🔄 Functions instead of class (but with module-level state for expensive objects)
- 🔄 Explicit configuration loading at module level
- 🔄 Batch processing of chunks per file for efficiency
- 🔄 Better empty result handling

## Summary

This refactor maintains complete feature parity while improving code quality. The hybrid functional approach (functions with minimal module-level state) provides the best of both worlds: clean API and efficient resource usage. By preserving all existing functionality while removing unnecessary complexity, we create a tool that's more maintainable without sacrificing any capabilities.