// Мы создаем сущности USB кабель и карта памяти. Мы хотим записать значение карты памяти в usb кабель для последующей передачи, но
// не можем сделать это напрямую, так как кабель может передавать только число i64. Адаптер Translator передает значения в кабель, записывая
// значения char последовательно в число i64 (считаем, что чисел в массиве карты памяти не больше 8, чтобы все точно поместилось). Далее USB кабель может передать то число по сети.

struct USB {
    temporary: i64
}

impl USB {
    fn get_information(&self) -> i64 {
        self.temporary
    }

    fn set_information(&mut self, i: i64) {
        self.temporary = i;
    }

    fn new(i: i64) -> USB {
        USB {
            temporary: i
        }
    }
}

struct MemoryCard {
    name: Vec<char>
}

struct Translator<'a> {
    memory_card: &'a MemoryCard
}

impl Translator<'_> {
    fn translate(&self, usb: &mut USB) {
        let mut cnt: i64 = 1;
        for symbol in self.memory_card.name.iter() {
            usb.set_information(usb.get_information() + (*symbol as i64) * cnt);
            cnt *= 8;
        }
    }
}

fn main() {
    let memory_card = MemoryCard {
        name: vec!['a', 'b', '0', '8']
    };
    let mut usb = USB::new(0);
    let translator = Translator {
        memory_card: &memory_card
    };

    translator.translate(&mut usb);

    print!("{}", usb.get_information());
}

