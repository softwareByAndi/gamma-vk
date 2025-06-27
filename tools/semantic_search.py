#!/usr/bin/env python3
"""
Semantic Search Tool for Gamma-VK Documentation (Refactored)

This tool indexes and searches through project documentation using semantic embeddings,
allowing for natural language queries instead of exact string matching.
"""

import argparse
import hashlib
import json
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Any
from dataclasses import dataclass

import chromadb
from sentence_transformers import SentenceTransformer


# Data Models
@dataclass
class SearchResult:
    filepath: Path
    content: str
    score: float
    chunk_index: int
    total_chunks: int


@dataclass
class Stats:
    total_files: int
    total_chunks: int
    index_location: Path


# Configuration
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

# Module-level state for expensive objects
_model = None
_client = None


def get_model():
    """Lazy-load the sentence transformer model."""
    global _model
    if _model is None:
        _model = SentenceTransformer('all-MiniLM-L6-v2')
    return _model


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
        collection = client.get_collection("gamma_vk_docs")
        print(f"Loading existing index from {INDEX_DIR}")
        return collection
    except Exception:  # ChromaDB raises NotFoundError, but catch all exceptions
        collection = client.create_collection(
            "gamma_vk_docs",
            metadata={"hnsw:space": "cosine"}
        )
        print(f"Creating new index at {INDEX_DIR}")
        return collection


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
    
    # Note: Could track existing IDs here for incremental updates in future
    
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
        content = results['documents'][0][i][:200]
        if len(content) < 200:
            content = f"{content}..." 
        
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


def display_results(results: List[SearchResult], query: str):
    """Display search results in a formatted way."""
    if results:
        print(f"\nFound {len(results)} results for: '{query}'\n")
        for i, result in enumerate(results, 1):
            print(f"{i}. {result.filepath} (chunk {result.chunk_index}/{result.total_chunks}, score: {result.score:.3f})")
            # Preserve line breaks by indenting each line
            content_lines = result.content.split('\n')
            for line in content_lines:
                print(f"   {line}")
            print()  # Empty line between results
    else:
        print(f"No results found for: '{query}'")


def display_stats(stats: Stats):
    """Display index statistics."""
    print(f"\nIndex Statistics:")
    print(f"  Location: {stats.index_location}")
    print(f"  Total files: {stats.total_files}")
    print(f"  Total chunks: {stats.total_chunks}")


def main():
    parser = argparse.ArgumentParser(description='Semantic search for documentation')
    subparsers = parser.add_subparsers(dest='command', required=True, help='Commands')
    
    # Index command
    index_parser = subparsers.add_parser('index', help='Index documentation files')
    index_parser.add_argument('directory', nargs='?', default='.', type=Path, help='Directory to index')
    index_parser.add_argument('--extensions', nargs='+', help='File extensions to index')
    
    # Search command  
    search_parser = subparsers.add_parser('search', help='Search indexed documents')
    search_parser.add_argument('query', help='Search query')
    search_parser.add_argument('-n', '--results', type=int, default=5, help='Number of results')
    
    # Stats command
    subparsers.add_parser('stats', help='Show index statistics')
    
    args = parser.parse_args()
    
    if args.command == 'index':
        print(f"Indexing directory: {args.directory}")
        index_documents(args.directory, args.extensions)
    elif args.command == 'search':
        results = search_documents(args.query, args.results)
        display_results(results, args.query)
    elif args.command == 'stats':
        stats = show_stats()
        display_stats(stats)


if __name__ == '__main__':
    main()