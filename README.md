# Ubi Doc
todo: 概要を書く

## Usage
todo: 使い方を書く

### Basic configuration

```yaml
name: Ubi Doc
on:
  workflow_dispatch:
  pull_request:
  push:
    branches:
      - main
      - 'releases/*'

jobs:
  ubi-doc:
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
      checks: write
    steps:
      - uses: actions/checkout@v4

      - name: Ubi Doc
        uses: Glider2355/ubi-doc@v1
        with:
          output: 'docs/generated.html'  # ouput HTML file path
```