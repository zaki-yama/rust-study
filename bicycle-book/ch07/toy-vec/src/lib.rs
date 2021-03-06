pub struct ToyVec<T> {
    // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    elements: Box<[T]>,
    // ベクタの長さ
    len: usize,
}

#[allow(
    clippy::len_without_is_empty,
    clippy::new_without_default,
    unconditional_recursion
)]
impl<T: Default> ToyVec<T> {
    // newはキャパシティ(容量)が0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size) // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>() // Vec<T>に収集してから
            .into_boxed_slice() // Box<[T]>に変換する
    }

    pub fn len(&self) -> usize {
        self.len()
    }

    pub fn capacity(&self) -> usize {
        self.elements.len()
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.elements[self.len] = element;
        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(&self.elements[index])
        } else {
            None
        }
    }

    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        match self.get(index) {
            Some(v) => v,
            None => default,
        }
        // 上の式はunwrap_orコンビネータで書き換えられる
        // self.get(index).unwrap_or(default)
    }

    // 戻り値が参照でないことに注目。所有権ごと返す
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    fn grow(&mut self) {
        if self.capacity() == 0 {
            self.elements = Self::allocate_in_heap(1);
        } else {
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            let old_elements = std::mem::replace(&mut self.elements, new_elements);

            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    // 説明のためにライフタイムを明示しているが、本当は省略できる
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements, // Iter構造体の定義より、ライフタイムは'vecになる
            len: self.len,
            pos: 0,
        }
    }
}

// ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vec T型の値が
// 生存している間は、ToyVecは変更できない
pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>, // ToyVec構造体のelementsを指す不変の参照
    len: usize,               // ToyVecの長さ
    pos: usize,               // 次に返す要素のインデックス
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型(トレイトに関連付いた型)で、このイテレータがイテレートする要素の型を指定する
    type Item = &'vec T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }
}
