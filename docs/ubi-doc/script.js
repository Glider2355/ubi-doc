document.addEventListener("DOMContentLoaded", () => {
    const tableBody = document.querySelector("#ubitable tbody");
    const keywordInput = document.getElementById("keyword-input");
    const contextSelect = document.getElementById("context-select");
  
    if (!tableBody) return;
  
    // フィルタ関数
    function filterTable() {
      const keyword = keywordInput?.value.toLowerCase().trim() || "";
      const contextValue = contextSelect?.value || "";
  
      Array.from(tableBody.querySelectorAll("tr")).forEach(row => {
        const rowText = row.innerText.toLowerCase();
        const contextCell = row.querySelectorAll("td")[2]; // 0-based index
        const contextText = contextCell ? contextCell.innerText : "";
  
        // 条件1: キーワード (全セル対象)
        const matchKeyword = rowText.includes(keyword);
        // 条件2: contextフィルタ (空文字ならスルー)
        const matchContext = !contextValue || contextText === contextValue;
  
        if (matchKeyword && matchContext) {
          row.classList.remove("hide");
        } else {
          row.classList.add("hide");
        }
      });
    }
  
    // イベント登録
    if (keywordInput) {
      keywordInput.addEventListener("input", filterTable);
    }
    if (contextSelect) {
      contextSelect.addEventListener("change", filterTable);
    }
  });
  