#!/usr/bin/env python3
"""
Semantic Search Tool for Gamma-VK Documentation

This tool indexes and searches through project documentation using semantic embeddings,
allowing for natural language queries instead of exact string matching.
"""

import os
import argparse
import chromadb
from chromadb.config import Settings
from sentence_transformers import SentenceTransformer
from pathlib import Path
from typing import List, Tuple, Dict
import hashlib
import json
from datetime import datetime


class DocumentIndexer:
    """Indexes markdown and other documentation files for semantic search."""
    
    def __init__(self, persist_directory: str = ".semantic_index"):
        self.persist_directory = persist_directory
        self.model = SentenceTransformer('all-MiniLM-L6-v2')
        
        # Initialize ChromaDB with persistence
        self.client = chromadb.PersistentClient(path=persist_directory)
        
        # Create or get collection
        try:
            self.collection = self.client.get_collection("gamma_vk_docs")
            print(f"Loading existing index from {persist_directory}")
        except:
            self.collection = self.client.create_collection(
                "gamma_vk_docs",
                metadata={"hnsw:space": "cosine"}
            )
            print(f"Creating new index at {persist_directory}")
    
    def _get_file_hash(self, filepath: str) -> str:
        """Get hash of file content for change detection."""
        with open(filepath, 'rb') as f:
            return hashlib.md5(f.read()).hexdigest()
    
    def _chunk_document(self, content: str, chunk_size: int = 1000) -> List[str]:
        """Split document into overlapping chunks for better search."""
        words = content.split()
        chunks = []
        overlap = chunk_size // 4  # 25% overlap
        
        for i in range(0, len(words), chunk_size - overlap):
            chunk = ' '.join(words[i:i + chunk_size])
            chunks.append(chunk)
        
        return chunks if chunks else [content]
    
    def index_directory(self, directory: str, extensions: List[str] = None):
        """Index all documents in a directory."""
        if extensions is None:
            extensions = ['.md', '.txt', '.rs', '.toml', '.log']
        
        path = Path(directory)
        files_indexed = 0
        chunks_created = 0
        
        # Get existing document IDs for updating
        existing_ids = set()
        try:
            results = self.collection.get()
            existing_ids = set(results['ids'])
        except:
            pass
        
        for ext in extensions:
            for filepath in path.rglob(f'*{ext}'):
                # Skip hidden files and directories
                if any(part.startswith('.') for part in filepath.parts):
                    continue
                
                try:
                    # Read file content
                    content = filepath.read_text(encoding='utf-8')
                    file_hash = self._get_file_hash(str(filepath))
                    
                    # Chunk the document
                    chunks = self._chunk_document(content)
                    
                    for i, chunk in enumerate(chunks):
                        chunk_id = f"{filepath}::{i}"
                        
                        # Prepare metadata
                        metadata = {
                            "filepath": str(filepath),
                            "filename": filepath.name,
                            "extension": ext,
                            "chunk_index": i,
                            "total_chunks": len(chunks),
                            "file_hash": file_hash,
                            "indexed_at": datetime.now().isoformat()
                        }
                        
                        # Generate embedding
                        embedding = self.model.encode(chunk).tolist()
                        
                        # Upsert to collection
                        self.collection.upsert(
                            ids=[chunk_id],
                            embeddings=[embedding],
                            documents=[chunk],
                            metadatas=[metadata]
                        )
                        
                        chunks_created += 1
                    
                    files_indexed += 1
                    print(f"Indexed: {filepath} ({len(chunks)} chunks)")
                    
                except Exception as e:
                    print(f"Error indexing {filepath}: {e}")
        
        print(f"\nIndexing complete: {files_indexed} files, {chunks_created} chunks")
    
    def search(self, query: str, n_results: int = 5) -> List[Dict]:
        """Search for documents semantically similar to the query."""
        # Generate query embedding
        query_embedding = self.model.encode(query).tolist()
        
        # Search in collection
        results = self.collection.query(
            query_embeddings=[query_embedding],
            n_results=n_results
        )
        
        # Format results
        formatted_results = []
        if results['ids'] and results['ids'][0]:
            for i in range(len(results['ids'][0])):
                result = {
                    'filepath': results['metadatas'][0][i]['filepath'],
                    'chunk_index': results['metadatas'][0][i]['chunk_index'],
                    'score': 1 - results['distances'][0][i],  # Convert distance to similarity
                    'content': results['documents'][0][i][:200] + '...' if len(results['documents'][0][i]) > 200 else results['documents'][0][i]
                }
                formatted_results.append(result)
        
        return formatted_results
    
    def get_stats(self) -> Dict:
        """Get statistics about the index."""
        try:
            results = self.collection.get()
            unique_files = set()
            for metadata in results['metadatas']:
                unique_files.add(metadata['filepath'])
            
            return {
                'total_chunks': len(results['ids']),
                'total_files': len(unique_files),
                'index_location': self.persist_directory
            }
        except:
            return {
                'total_chunks': 0,
                'total_files': 0,
                'index_location': self.persist_directory
            }


def main():
    parser = argparse.ArgumentParser(description='Semantic search for documentation')
    subparsers = parser.add_subparsers(dest='command', help='Commands')
    
    # Index command
    index_parser = subparsers.add_parser('index', help='Index documentation files')
    index_parser.add_argument('directory', help='Directory to index', default='.')
    index_parser.add_argument('--extensions', nargs='+', help='File extensions to index')
    
    # Search command
    search_parser = subparsers.add_parser('search', help='Search indexed documents')
    search_parser.add_argument('query', help='Search query')
    search_parser.add_argument('-n', '--results', type=int, default=5, help='Number of results')
    
    # Stats command
    stats_parser = subparsers.add_parser('stats', help='Show index statistics')
    
    args = parser.parse_args()
    
    indexer = DocumentIndexer(persist_directory='tools/.semantic_index')
    
    if args.command == 'index':
        print(f"Indexing directory: {args.directory}")
        indexer.index_directory(args.directory, args.extensions)
        
    elif args.command == 'search':
        results = indexer.search(args.query, args.results)
        
        if results:
            print(f"\nFound {len(results)} results for: '{args.query}'\n")
            for i, result in enumerate(results, 1):
                print(f"{i}. {result['filepath']} (chunk {result['chunk_index']}, score: {result['score']:.3f})")
                print(f"   {result['content']}\n")
        else:
            print(f"No results found for: '{args.query}'")
            
    elif args.command == 'stats':
        stats = indexer.get_stats()
        print(f"\nIndex Statistics:")
        print(f"  Location: {stats['index_location']}")
        print(f"  Total files: {stats['total_files']}")
        print(f"  Total chunks: {stats['total_chunks']}")
        
    else:
        parser.print_help()


if __name__ == '__main__':
    main()