
# Vector Store Uploader

A desktop application built with Tauri, React, and Rust for processing PDF documents and storing their vector embeddings in Pinecone. Currently in early development.

## Features

- PDF text extraction and processing
- OpenAI embeddings generation
- Vector storage in Pinecone
- Cross-platform desktop application (Windows, macOS, Linux)
- Modern React UI with Material-UI components
- Secure settings storage

## Tech Stack

- **Frontend**: React, TypeScript, Material-UI
- **Backend**: Rust, Tauri
- **Vector Processing**: OpenAI API
- **Vector Storage**: Pinecone
- **Build Tools**: Vite

## Prerequisites

- Node.js 16+
- Rust toolchain
- OpenAI API key
- Pinecone API key and index

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/vector-store-uploader.git
cd vector-store-uploader
```

2. Install dependencies:

```bash
npm install
```

3. Run the development server:

```bash
npm run tauri dev
```

## Configuration

Before using the application, you need to configure:

- OpenAI API key
- Pinecone API key
- Pinecone index host
- Pinecone namespace

These can be set through the settings dialog in the application.

## Development Status

This project is in early development. Current features:

- ✅ Basic PDF file upload
- ✅ Text extraction
- ✅ OpenAI embeddings integration
- ✅ Pinecone storage integration
- ✅ Settings management
- 🚧 Progress tracking
- 🚧 Error handling
- 🚧 Testing coverage

## Contributing

As this project is in early development, contributions are welcome. Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This is an early development version and may contain bugs or incomplete features. Use at your own risk.