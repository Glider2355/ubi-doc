document.addEventListener("DOMContentLoaded", () => {
    const tableBody = document.querySelector("#ubitable tbody");
    const keywordInput = document.getElementById("keyword-input");
    const contextSelect = document.getElementById("context-select");
  
    if (!tableBody || !contextSelect) return;
  
    // --- 1. ユニークなContext値を抽出して<select>に追加 ---
    const contextSet = new Set();
    const rows = tableBody.querySelectorAll("tr");
    
    rows.forEach(row => {
      const cells = row.querySelectorAll("td");
      // Context列は3番目 (0-based index: 2)
      if (cells.length >= 3) {
        const contextText = cells[2].innerText.trim();
        if (contextText) {
          contextSet.add(contextText);
        }
      }
    });
  
    // 重複を除いた値を昇順ソート
    const uniqueContexts = Array.from(contextSet).sort();
    
    // 既存の "すべて" オプションを残し、追加する
    uniqueContexts.forEach(ctx => {
      const option = document.createElement("option");
      option.value = ctx;
      option.textContent = ctx;
      contextSelect.appendChild(option);
    });
  
    // --- 2. フィルタ処理 ---
    function filterTable() {
      const keyword = (keywordInput?.value || "").toLowerCase().trim();
      const selectedContext = contextSelect?.value || "";
    
      Array.from(rows).forEach(row => {
        const rowText = row.innerText.toLowerCase();
        const cells = row.querySelectorAll("td");
        const contextText = cells.length >= 3 ? cells[2].innerText.trim() : "";
    
        const matchKeyword = rowText.includes(keyword);
        const matchContext = !selectedContext || (contextText === selectedContext);
    
        if (matchKeyword && matchContext) {
          row.classList.remove("hide");
        } else {
          row.classList.add("hide");
        }
      });
    }
    
    // --- 3. イベント登録 ---
    if (keywordInput) {
      keywordInput.addEventListener("input", filterTable);
    }
    contextSelect.addEventListener("change", filterTable);
  });
  