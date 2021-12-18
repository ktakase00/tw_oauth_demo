//! キーと値のペア

/// キーと値のペアを表す構造体。
#[derive(Debug, Clone)]
pub struct StringPair {
    key: String,
    value: String,
}

impl StringPair {
    /// 初期化する。
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    /// キーと値を結合した文字列を生成する。
    pub fn make_encode_string(&self, quote_flag: bool) -> String {
        let encoded_key = urlencoding::encode(&self.key);
        let encoded_value = urlencoding::encode(&self.value);
        match quote_flag {
            true => {
                format!("{}=\"{}\"", &encoded_key, &encoded_value)
            }
            false => {
                format!("{}={}", &encoded_key, &encoded_value)
            }
        }
    }
}

/// キーと値のペアのリストを扱う構造体。
#[derive(Debug)]
pub struct StringPairList {
    list: Vec<StringPair>,
}

impl StringPairList {
    /// 初期化する。
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    /// キーと値のペアを1組追加する。
    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        let key_value_pair = StringPair::new(key, value);
        self.list.push(key_value_pair);
        self
    }

    /// クエリ文字列の書式ですべてのペアを結合した文字列を生成する。
    pub fn make_query(&self) -> String {
        let sort_list = self.make_sort_list();
        sort_list.iter()
            .map(|x| x.make_encode_string(false))
            .collect::<Vec<String>>()
            .join("&")
    }

    /// Authorization HTTPヘッダーフィールドに指定する文字列を生成する。
    pub fn make_authorization(&self) -> String {
        let sort_list = self.make_sort_list();
        let param_string = sort_list
            .iter()
            .map(|x| x.make_encode_string(true))
            .collect::<Vec<String>>()
            .join(", ");
        format!("OAuth {}", &param_string)
    }

    /// キーで並び替えたペアのリストを生成する。
    fn make_sort_list(&self) -> Vec<StringPair> {
        let mut dup_list = self.list.clone();
        dup_list.sort_by_key(|it| it.key.to_owned());
        dup_list
    }
}
