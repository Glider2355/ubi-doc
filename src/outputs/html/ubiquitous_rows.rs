use super::ubiquitous_row::UbiquitousRow;

pub struct UbiquitousRows {
    pub rows: Vec<UbiquitousRow>,
}

impl UbiquitousRows {
    pub fn new() -> Self {
        UbiquitousRows { rows: Vec::new() }
    }

    pub fn add(&mut self, row: UbiquitousRow) {
        self.rows.push(row);
    }

    pub fn sort(&mut self) {
        self.rows.sort_by(|a: &UbiquitousRow, b: &UbiquitousRow| {
            let cmp_context = a.context.cmp(&b.context);
            if cmp_context == std::cmp::Ordering::Equal {
                a.ubiquitous.cmp(&b.ubiquitous)
            } else {
                cmp_context
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_context_and_ubiquitous() {
        // UbiquitousRows を新規生成
        let mut rows = UbiquitousRows::new();

        // 複数の UbiquitousRow を、ランダムな順序で生成
        let row1 = UbiquitousRow {
            class_name: "C1".to_string(),
            ubiquitous: "z".to_string(), // 後で "z" の中で最後になるはず
            context: "A".to_string(),    // context "A" のグループ
            description: "".to_string(),
            file_path: "".to_string(),
            line_number: 0,
            github_url: "".to_string(),
        };
        let row2 = UbiquitousRow {
            class_name: "C2".to_string(),
            ubiquitous: "m".to_string(), // "m" は "z" より前
            context: "A".to_string(),
            description: "".to_string(),
            file_path: "".to_string(),
            line_number: 0,
            github_url: "".to_string(),
        };
        let row3 = UbiquitousRow {
            class_name: "C3".to_string(),
            ubiquitous: "a".to_string(), // context "B" のグループ
            context: "B".to_string(),
            description: "".to_string(),
            file_path: "".to_string(),
            line_number: 0,
            github_url: "".to_string(),
        };
        let row4 = UbiquitousRow {
            class_name: "C4".to_string(),
            ubiquitous: "a".to_string(), // 同じ context "A" で "a" は最初になるはず
            context: "A".to_string(),
            description: "".to_string(),
            file_path: "".to_string(),
            line_number: 0,
            github_url: "".to_string(),
        };
        let row5 = UbiquitousRow {
            class_name: "C5".to_string(),
            ubiquitous: "z".to_string(), // context "B" で "z" は後になる
            context: "B".to_string(),
            description: "".to_string(),
            file_path: "".to_string(),
            line_number: 0,
            github_url: "".to_string(),
        };

        // ランダムな順序で追加
        rows.add(row1);
        rows.add(row2);
        rows.add(row3);
        rows.add(row4);
        rows.add(row5);

        // ソート実行: まず context で、同一の場合は ubiquitous で昇順ソート
        rows.sort();

        // 期待するソート順:
        // context "A": まず ubiquitous "a" (row4), 次に "m" (row2), 次に "z" (row1)
        // context "B": まず ubiquitous "a" (row3), 次に "z" (row5)
        assert_eq!(rows.rows.len(), 5);

        // 1行目: context "A", ubiquitous "a"
        assert_eq!(rows.rows[0].context, "A");
        assert_eq!(rows.rows[0].ubiquitous, "a");

        // 2行目: context "A", ubiquitous "m"
        assert_eq!(rows.rows[1].context, "A");
        assert_eq!(rows.rows[1].ubiquitous, "m");

        // 3行目: context "A", ubiquitous "z"
        assert_eq!(rows.rows[2].context, "A");
        assert_eq!(rows.rows[2].ubiquitous, "z");

        // 4行目: context "B", ubiquitous "a"
        assert_eq!(rows.rows[3].context, "B");
        assert_eq!(rows.rows[3].ubiquitous, "a");

        // 5行目: context "B", ubiquitous "z"
        assert_eq!(rows.rows[4].context, "B");
        assert_eq!(rows.rows[4].ubiquitous, "z");
    }
}
