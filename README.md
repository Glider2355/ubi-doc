# Ubi Doc

Ubi Doc is an open-source GitHub Action that generates an HTML table representing your ubiquitous language directly from your code's doc comments. By using specific annotations within your documentation, Ubi Doc automatically produces an HTML file that visually displays your project's core terms and their contexts.

## Overview

When you include ubiquitous language definitions in your code documentation using special annotations, Ubi Doc parses these comments and generates an HTML file containing a comprehensive table of terms, contexts, and descriptions. This output facilitates clear communication and shared understanding of the project's terminology among team members.

## How to Use

### Writing Doc Comments

Add doc comments in your source code using the following annotations:

- `@ubiquitous`: **Ubiquitous Language** – the term or phrase to document.
- `@context`: **Context** – the context or scenario where the term is used.
- `@description`: **Description** – an explanation of the term.

For example (PHP):

```php
/**
 * @ubiquitous Order
 * @context E-commerce
 * @description Represents a customer's purchase order.
 */
```

### Adding the GitHub Action

To integrate Ubi Doc into your workflow, add the following GitHub Action configuration to your repository.

Example Basic configuration: 

```yml
name: Generate Ubiquitous

on:
  push:
    # your default branch
    branches: [ main ]
  workflow_dispatch:

jobs:
  test-action:
    runs-on: ubuntu-latest

    permissions:
      contents: write
      pull-requests: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Run Ubi Doc Action
        uses: ./
        with:
          # Directory path to output HTML
          output_dir: 'docs'
          repo: ${{ github.repository }}
          branch: ${{ github.ref_name }}

      - name: Check for differences in ubi-doc directory
        id: diffcheck
        run: |
          echo "Checking for differences in ubi-doc compared to origin/main..."
          ls -la docs
          git fetch origin main
          git add docs
          if git diff --cached --quiet "origin/main" -- "docs"; then
            echo "No differences found in ubi-doc. Exiting."
            echo "difffound=false" >> "$GITHUB_OUTPUT"
          else
            echo "Differences found, continuing..."
            echo "difffound=true" >> "$GITHUB_OUTPUT"
          fi

      - name: Close existing auto-generated HTML PRs
        if: steps.diffcheck.outputs.difffound == 'true'
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          echo "Closing existing auto-generated HTML PRs..."
          # List previous PRs, extract their numbers, and close them one by one
          gh pr list \
            --search "head:auto-generated-html" \
            --state open \
            --json number \
            --limit 100 \
            | jq -r '.[].number' \
            | while read -r pr; do
                echo "Closing PR #$pr"
                gh pr close "$pr" --delete-branch
              done

      - name: Create Pull Request (via gh CLI)
        if: steps.diffcheck.outputs.difffound == 'true'
        env:
          GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          # 1. Set Git user configuration
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"

          # 2. Create a unique branch name: auto-generated-html/<run_id>
          BRANCH_NAME="auto-generated-html/${{ github.run_id }}"
          git checkout -b "$BRANCH_NAME"

          # 3. Commit the changes
          git add docs
          git commit -m $'chore: Add generated HTML\n\nAuto-commit by GitHub Actions.'

          # 4. Push the branch to the remote repository
          git push origin "$BRANCH_NAME"

          # 5. Create a Pull Request
          gh pr create \
            --base main \
            --head "$BRANCH_NAME" \
            --title "Add generated HTML via GitHub Actions" \
            --body "This Pull Request includes the latest auto-generated HTML files."
```

### Auto-merging and Deployment
- Auto-merge Generated PRs: Optionally, configure your workflow or additional automation to merge the auto-generated pull requests once they pass the required checks.
- Deploy via GitHub Pages: Publish the generated HTML files (located in the output directory, e.g., docs) using GitHub Pages. This makes your ubiquitous language table publicly accessible.