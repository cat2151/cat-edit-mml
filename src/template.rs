/// MMLテンプレートを管理するモジュール
pub struct MmlTemplate;

impl MmlTemplate {
    /// 利用可能なMMLテンプレートのリスト
    const TEMPLATES: &'static [&'static str] = &[
        // テンプレート1: 基本的なスケール
        "cdefgab",
        
        // テンプレート2: Cメジャースケール（オクターブ付き）
        "c4d4e4f4g4a4b4>c4",
        
        // テンプレート3: 基本的なメロディパターン
        "c4e4g4e4 c4e4g4e4 f4a4>c4<a4 g4b4>d4<b4",
        
        // テンプレート4: ドレミの歌
        "c4d4e4c4 c4d4e4c4 e4f4g2 e4f4g2",
        
        // テンプレート5: 簡単なコード進行（和音）
        "ceg4 fac4 gbd4 ceg4",
        
        // テンプレート6: テンポ指定付きメロディ
        "t120 c8d8e8f8g8a8b8>c8 <b8a8g8f8e8d8c2",
        
        // テンプレート7: マルチトラックの例
        "A o4 cdefgab>c\nB o3 c2e2g2c2",
        
        // テンプレート8: 空のテンプレート
        "",
    ];

    /// 指定されたインデックスのテンプレートを取得する
    /// インデックスが範囲外の場合は最初のテンプレートを返す
    pub fn get_template(index: usize) -> &'static str {
        Self::TEMPLATES.get(index).unwrap_or(&Self::TEMPLATES[0])
    }

    /// テンプレートの総数を取得する
    pub fn template_count() -> usize {
        Self::TEMPLATES.len()
    }

    /// テンプレートのタイトルを取得する（デバッグ用）
    pub fn get_template_title(index: usize) -> String {
        match index {
            0 => "基本スケール".to_string(),
            1 => "Cメジャースケール（オクターブ付き）".to_string(),
            2 => "基本メロディパターン".to_string(),
            3 => "ドレミの歌".to_string(),
            4 => "コード進行".to_string(),
            5 => "テンポ指定付きメロディ".to_string(),
            6 => "マルチトラック".to_string(),
            7 => "空のテンプレート".to_string(),
            _ => format!("テンプレート {}", index + 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template() {
        // 有効なインデックスでテンプレートを取得
        assert_eq!(MmlTemplate::get_template(0), "cdefgab");
        assert_eq!(MmlTemplate::get_template(1), "c4d4e4f4g4a4b4>c4");
        
        // 無効なインデックスは最初のテンプレートを返す
        let invalid_index = MmlTemplate::template_count() + 10;
        assert_eq!(MmlTemplate::get_template(invalid_index), "cdefgab");
    }

    #[test]
    fn test_template_count() {
        assert!(MmlTemplate::template_count() > 0);
    }

    #[test]
    fn test_get_template_title() {
        assert_eq!(MmlTemplate::get_template_title(0), "基本スケール");
        assert_eq!(MmlTemplate::get_template_title(7), "空のテンプレート");
    }
}