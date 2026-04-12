    /// 向前移动一个字符（position 为字节偏移，便于与源码索引一致）
    fn next_char(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
        }
        // 更新缓存：peek 新的下一个字符
        // 克隆迭代器来获取下一个字符（仅在前进时执行，不是热路径）
        // After advancing, self.chars is positioned before current character i.
        // To get i+1 (the next character after current): clone, step once, then peek.
        let mut cloned = self.chars.clone();
        cloned.next();
        self.cached_next = cloned.peek().copied();
    }
