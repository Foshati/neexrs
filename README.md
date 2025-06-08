# 🚀 neex

**The Modern Build System for Polyrepo-in-Monorepo Architecture**

A powerful, fast, and modern command runner built in Rust - inspired by `npm-run-all` but with superpowers! ⚡

## ✨ Features

- **🏃‍♂️ Parallel & Sequential Execution** - Run commands simultaneously or one by one
- **🎨 Beautiful Output** - Colored output with spinners and progress indicators  
- **🖥️ Server Mode** - Perfect for running multiple development servers
- **⚡ Lightning Fast** - Built in Rust for maximum performance
- **🛡️ Smart Error Handling** - Configurable error handling strategies
- **📊 Execution Reports** - Detailed timing and status information

## 🚀 Installation

### From Source (Recommended)
```bash
git clone https://github.com/yourusername/neexrs
cd neexrs
cargo install --path .
```

### Using Cargo
```bash
cargo install neexrs
```

## 📖 Usage

### Sequential Execution
Run commands one after another:
```bash
neex run "npm test" "npm run build" "npm run deploy"
# or short form
neex s "npm test" "npm run build" "npm run deploy"
```

### Parallel Execution  
Run commands simultaneously for faster execution:
```bash
neex runx "npm run test:unit" "npm run test:e2e" "npm run lint"
# or short form  
neex p "npm run test:unit" "npm run test:e2e" "npm run lint"
```

### Server Mode
Perfect for development - run multiple servers:
```bash
neex servers "npm run dev:frontend" "npm run dev:backend" "npm run dev:api"
# or short form
neex srv "npm run dev:frontend" "npm run dev:backend" "npm run dev:api"
```

## 🎯 Command Examples

### Frontend Development
```bash
# Run all tests in parallel
neex p "npm run test:unit" "npm run test:integration" "npm run lint"

# Start development environment
neex srv "npm run dev" "npm run storybook" "npm run api:mock"

# Build and deploy sequence
neex s "npm run clean" "npm run build" "npm run test:e2e" "npm run deploy"
```

### Microservices Development
```bash
# Start all services
neex srv "cd auth-service && npm start" "cd user-service && npm start" "cd payment-service && npm start"

# Run all service tests
neex p "cd auth-service && npm test" "cd user-service && npm test" "cd payment-service && npm test"
```

## ⚙️ Options

### Global Options
- `--no-color` / `-c` - Disable colored output
- `--quiet` / `-q` - Minimal output mode

### Parallel Options (`runx`)
- `--sequential` / `-s` - Run sequentially instead of parallel
- `--max-parallel` / `-j N` - Limit concurrent jobs
- `--group` - Group output by command

### Server Options (`servers`)
- `--port` / `-p` - Base port number
- `--restart` - Auto-restart on failure
- `--env-file` / `-e` - Load environment file

## 🆚 vs npm-run-all

| Feature | neex | npm-run-all |
|---------|------|-------------|
| **Performance** | ⚡ Rust (Fast) | 🐌 Node.js |
| **Memory Usage** | 🪶 Low | 🐘 High |
| **Colored Output** | 🎨 Beautiful | ⚪ Basic |
| **Server Mode** | ✅ Built-in | ❌ No |
| **Progress Indicators** | ✅ Spinners | ❌ No |
| **Cross Platform** | ✅ Yes | ✅ Yes |
| **Error Handling** | 🛡️ Advanced | ⚠️ Basic |

## 🏗️ Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/neexrs
cd neexrs

# Build in release mode
cargo build --release

# Install locally
cargo install --path .
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo test --all-features

# Test the CLI
./target/release/neex --help
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Inspired by [npm-run-all](https://github.com/mysticatea/npm-run-all)
- Built with ❤️ using [Rust](https://www.rust-lang.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Async runtime by [tokio](https://tokio.rs/)

---

**Made with ❤️ for developers who value speed and productivity**