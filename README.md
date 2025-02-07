# Ubiquitous Generator
todo: 概要を書く

## Usage
todo: 使い方を書く

### Basic configuration

```yaml
name: Ubiquitous Generator
on:
  workflow_dispatch:
  pull_request:
  push:
    branches:
      - main
      - 'releases/*'

jobs:
  ubiquitous-generator:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      checks: write
    steps:
      - uses: actions/checkout@v4

      - name: Ubiquitous Generator
        uses: Glider2355/ubiquitous-generator@v1
        with:
          output: 'docs/generated.html'  # ouput HTML file path
```