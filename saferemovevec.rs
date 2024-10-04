use std::ptr;

// Создаем трейт для безопасного удаления элемента
pub trait SafeRemoveVec<T> {
    fn safe_remove(&mut self, index: usize) -> Option<T>;
}

impl<T> SafeRemoveVec<T> for Vec<T> {
    // Функция для безопасного удаления элемента вектора
    fn safe_remove(&mut self, index: usize) -> Option<T> {
        let len = self.len();
        if index >= len {
            return None;
        }

        unsafe {
            let ret;
            {
                // Копируем незащищенно (небезопасно) элемент массива в стек. Имеем ссылку на элемент массива и на стек. Удаляем элемент из вектора.
                let ptr = self.as_mut_ptr().offset(index as isize);
                ret = ptr::read(ptr);
                ptr::copy(ptr.offset(1), ptr, len - index - 1);
            }
            self.set_len(len - 1);
            Some(ret)
        }
    }
}